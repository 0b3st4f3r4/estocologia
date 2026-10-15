"""Ponto 7 (fundo/p07-icp): ICP robusto ao agrupamento (farmacologia, STAR) e réplica da genética com 40 genes Y.

Implementa ../DECLARACAO.md (hash em ../DECLARACAO.sha256). Roda sobre uma cópia do código do H2g; nada em h2g/ muda.

Uso:  python3 icp_robusto.py [farmacologia|star|genetica ...]   (sem argumento: as três)
Saída: ../saida/<area>.json
"""

from __future__ import annotations

import importlib.util
import json
import multiprocessing as mp
import os
import sys
import time
from itertools import combinations
from pathlib import Path

import numpy as np
import pandas as pd
from scipy import stats

RAIZ = Path(__file__).resolve().parent
sys.path.insert(0, str(RAIZ))
from estocologia import dominio as D  # noqa: E402
from estocologia.funil import Desenho  # noqa: E402
from estocologia.icp import (LIMIAR_SATURACAO, PISO_ESCALA, _ols_ajuste, acerto_1nn, expandir,  # noqa: E402
                             icp, p_invariancia_np)

DADOS = Path(os.environ.get("ESTOCOLOGIA_DADOS", "/mnt/project-files/estocologia/dados-reais/h2b"))
SAIDA = Path(os.environ.get("P07_SAIDA", RAIZ.parent / "saida"))
B_PERM, SEMENTE_PERM = 999, 7
SEMENTE_GEN, N_Y_NOVOS = 7007, 40


# ---------------- dados de treino, como o funil os monta ----------------
def treino_de(nome: str):
    p = D.carregar(RAIZ / "dominios" / nome)
    df = p.preparar(DADOS)
    df = df.assign(ambiente=df["ambiente"].astype(str))
    tr = df[~df["ambiente"].isin(set(map(str, p.ambientes["cofre"])))].reset_index(drop=True)
    des = Desenho(p, tr)
    blocos = {v: des.bloco(tr, v) for v in p.icp_candidatos}
    return p, tr, blocos


# ---------------- testes ----------------
def residuo_u(Xe, y):
    """O mesmo u = r / ŝ de p_invariancia_np."""
    r = y - _ols_ajuste(Xe, y)
    a = np.abs(r)
    s = np.maximum(_ols_ajuste(Xe, a), PISO_ESCALA * a.mean())
    return r / s


def wald_cr(z, ae, cl, m, G):
    """Wald agrupado CR1 de que os coeficientes das m − 1 indicadoras de ambiente são zero; referência F(q, G − 1)."""
    n = z.size
    Dm = np.zeros((n, m))
    Dm[:, 0] = 1.0
    Dm[np.arange(n), ae] = 1.0          # ambiente 0 é a referência (coluna 0 = intercepto)
    pao = np.linalg.inv(Dm.T @ Dm)
    beta = pao @ (Dm.T @ z)
    e = z - Dm @ beta
    U = np.zeros((G, m))
    np.add.at(U, cl, Dm * e[:, None])
    V = pao @ (U.T @ U) @ pao * (G / (G - 1)) * ((n - 1) / (n - m))
    Vr, b = V[1:, 1:], beta[1:]
    q = int(np.linalg.matrix_rank(Vr))
    if q == 0:
        return 1.0
    W = float(b @ np.linalg.pinv(Vr) @ b)
    return float(stats.f.sf(W / q, q, G - 1))


def p_cr(u, ae, cl, m, G):
    z = stats.rankdata(u)
    med = np.array([np.median(u[ae == g]) for g in range(m)])
    d = np.abs(u - med[ae])
    p_loc, p_esc = wald_cr(z, ae, cl, m, G), wald_cr(d, ae, cl, m, G)
    return min(1.0, 2 * min(p_loc, p_esc)), p_loc, p_esc


class Permutador:
    """Permuta o rótulo de ambiente de blocos inteiros (grupo × ambiente) dentro do grupo."""

    def __init__(self, ae, cl, m, B, semente):
        bloco_par = cl.astype(np.int64) * m + ae
        self.ub, self.blk = np.unique(bloco_par, return_inverse=True)
        self.blk = self.blk.ravel()
        b_cl, b_ae = self.ub // m, self.ub % m
        rng = np.random.default_rng(semente)
        self.rotulos = np.empty((B + 1, self.ub.size), dtype=np.int64)
        self.rotulos[0] = b_ae
        idx_por_cl = [np.flatnonzero(b_cl == c) for c in np.unique(b_cl)]
        for k in range(1, B + 1):
            r = b_ae.copy()
            for ix in idx_por_cl:
                r[ix] = b_ae[rng.permutation(ix)]
            self.rotulos[k] = r
        self.m, self.n_blk = m, self.ub.size
        self.tam = np.bincount(self.blk, minlength=self.n_blk).astype(float)

    def p(self, u):
        """p da permutação (H de Kruskal–Wallis e F de Brown–Forsythe), vetorizado sobre as B + 1 rotulagens."""
        m = self.m
        R = stats.rankdata(u)
        somaR = np.bincount(self.blk, weights=R, minlength=self.n_blk)
        L = self.rotulos                                     # (B + 1) × blocos
        oh = np.zeros((L.shape[0], self.n_blk, m))
        np.put_along_axis(oh, L[:, :, None], 1.0, axis=2)
        n_g = np.einsum("kbg,b->kg", oh, self.tam)
        S_g = np.einsum("kbg,b->kg", oh, somaR)
        Tk = np.sum(np.where(n_g > 0, S_g ** 2 / np.maximum(n_g, 1), 0.0), axis=1)   # monótono no H de KW
        ordem = np.argsort(u, kind="stable")
        us = u[ordem]
        K = L.shape[0]
        E = L.astype(np.int8)[:, self.blk]                  # (B + 1) × linhas: ambiente de cada linha
        Es = E[:, ordem]
        med = np.zeros((K, m))
        for g in range(m):
            lin, col = np.nonzero(Es == g)                   # por linha, posições em ordem crescente de u
            ng = np.bincount(lin, minlength=K)
            ini = np.concatenate([[0], np.cumsum(ng)[:-1]])
            ok = ng > 0
            k1, k2 = (ng - 1) // 2, ng // 2
            v1 = np.where(ok, us[col[np.minimum(ini + k1, col.size - 1)]] if col.size else 0.0, 0.0)
            v2 = np.where(ok, us[col[np.minimum(ini + k2, col.size - 1)]] if col.size else 0.0, 0.0)
            med[:, g] = (v1 + v2) / 2
        d = np.abs(u[None, :] - np.take_along_axis(med, E.astype(np.intp), axis=1))
        N = u.size
        ne = n_g
        chave = (np.arange(K)[:, None] * m + E).ravel()
        sd = np.bincount(chave, weights=d.ravel(), minlength=K * m).reshape(K, m)
        sd2 = np.sum(d * d, axis=1)
        mm = np.sum(ne > 0, axis=1)
        mg = np.where(ne > 0, sd / np.maximum(ne, 1), 0.0)
        dbar = d.mean(axis=1)
        entre = np.sum(ne * (mg - dbar[:, None]) ** 2, axis=1) / (mm - 1)
        dentro = (sd2 - np.sum(ne * mg ** 2, axis=1)) / (N - mm)
        Tb = entre / dentro
        eps = 1e-12
        p_loc = (1 + np.sum(Tk[1:] >= Tk[0] * (1 - eps))) / Tk.size
        p_esc = (1 + np.sum(Tb[1:] >= Tb[0] * (1 - eps))) / Tb.size
        return min(1.0, 2 * min(p_loc, p_esc)), float(p_loc), float(p_esc)


_CTX: dict = {}


def _um_S(S):
    c = _CTX
    n = c["y"].size
    Xe = np.hstack([np.ones((n, 1))] + [c["exp"][v][0] for v in S])
    sat = acerto_1nn([c["blocos"][v] for v in S], c["amb"]) >= LIMIAR_SATURACAO
    u = residuo_u(Xe, c["y"])
    p0 = p_invariancia_np(Xe, c["y"], c["amb"])
    pc, pcl, pce = p_cr(u, c["ae"], c["cl"], c["m"], c["G"])
    t = {"S": list(S), "saturado": bool(sat), "p_original": p0, "p_cr": pc, "p_cr_loc": pcl, "p_cr_esc": pce}
    if not sat:
        pp, ppl, ppe = c["perm"].p(u)
        t.update({"p_perm": pp, "p_perm_loc": ppl, "p_perm_esc": ppe})
    return t


def resumo_icp(testes, chave_p, alpha):
    fam = [t for t in testes if not t["saturado"]]
    aceitos = [t["S"] for t in fam if t[chave_p] > alpha]
    S_hat = sorted(set.intersection(*map(set, aceitos))) if aceitos else []
    melhor = max(fam, key=lambda t: t[chave_p])
    return {"S_hat": S_hat, "n_aceitos": len(aceitos), "vazio_aceito": [] in aceitos,
            "melhor": {"S": melhor["S"], "p": melhor[chave_p]},
            "p_do_S_hat_original": None, "aceitos": aceitos}


def icp_robusto(nome, grupo_col, alpha=0.05):
    t0 = time.time()
    p, tr, blocos = treino_de(nome)
    y = tr["y"].to_numpy(float)
    amb = tr["ambiente"].to_numpy(str)
    rot, ae = np.unique(amb, return_inverse=True)
    ae = ae.ravel()
    _, cl = np.unique(tr[grupo_col].astype(str).to_numpy(), return_inverse=True)
    cl = cl.ravel()
    m, G = rot.size, int(cl.max() + 1)
    # reprodução com o ICP original (mesma função do H2g)
    orig = icp(blocos, y, amb, "gaussiana", alpha=alpha, teste="residuos_np", regra_saturacao=True)
    perm = Permutador(ae, cl, m, B_PERM, SEMENTE_PERM)
    nomes = sorted(blocos)
    _CTX.update(blocos=blocos, y=y, amb=amb, ae=ae, cl=cl, m=m, G=G, perm=perm,
                exp={v: expandir(blocos[v]) for v in nomes})
    todos = [S for k in range(len(nomes) + 1) for S in combinations(nomes, k)]
    with mp.get_context("fork").Pool(int(os.environ.get("P07_PROCESSOS", "4"))) as pool:
        testes = pool.map(_um_S, todos, chunksize=4)
    n = y.size
    out = {"dominio": nome, "area": p.area, "n_linhas": int(n), "ambientes": rot.tolist(), "grupo": grupo_col,
           "n_grupos": G, "n_blocos_perm": int(perm.n_blk), "alpha": alpha, "B_perm": B_PERM,
           "linhas_por_grupo": {"min": int(np.bincount(cl).min()), "mediana": float(np.median(np.bincount(cl))),
                                "max": int(np.bincount(cl).max())},
           "original_funcao_h2g": {"S_hat": orig["S_hat"], "n_aceitos": len(orig["aceitos"]),
                                   "melhor": orig["melhor_conjunto"]}}
    S_orig = orig["S_hat"]
    for chave in ("p_original", "p_cr", "p_perm"):
        r = resumo_icp(testes, chave, alpha)
        r["p_do_S_hat_original"] = next(t[chave] for t in testes if sorted(t["S"]) == sorted(S_orig) and not t["saturado"])
        r["p_do_vazio"] = next(t[chave] for t in testes if t["S"] == [])
        out[chave.replace("p_", "teste_")] = r
    out["segundos"] = round(time.time() - t0, 1)
    out["testes"] = testes
    return out


# ---------------- genética: 40 Y novos ----------------
def _comum():
    spec = importlib.util.spec_from_file_location("kemmeren_comum", RAIZ / "dominios" / "_kemmeren" / "comum.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def preparar_y(C, d, y):
    """comum.preparar com o Y dado (mesma pré-seleção, ambientes e orientação de x1)."""
    d = d[d["deleted_gene"] != y].reset_index(drop=True)
    xs, sinal = C.candidatos(d, y)
    out = pd.DataFrame({"y": d[y].astype(float), "ambiente": C.ambiente(d), "unidade": d["sample_id"].astype(str)})
    for i, g in enumerate(xs, 1):
        out[f"x{i}"] = d[g].astype(float) * (sinal if i == 1 else 1.0)
    out["plate"] = d["plate"].astype(float)
    return out, {"y": y, **{f"x{i}": g for i, g in enumerate(xs, 1)}, "sinal_x1": sinal}, d


def validar(d_full, genes, S_hat, tr, blocos, y_gene):
    """Efeito forte de intervenção (Meinshausen et al. 2016) para cada gene de Ŝ."""
    S = [v for v in S_hat]
    n = tr.shape[0]
    X = np.hstack([np.ones((n, 1))] + [blocos[v].reshape(n, -1) for v in S])
    beta, *_ = np.linalg.lstsq(X, tr["y"].to_numpy(float), rcond=None)
    res = []
    Y = d_full[y_gene].to_numpy(float)
    for j, v in enumerate(S, 1):
        if v == "plate":
            continue
        g = genes[v]
        coef_bruto = float(beta[j]) * (genes["sinal_x1"] if v == "x1" else 1.0)
        linha = np.flatnonzero(d_full["deleted_gene"].to_numpy() == g)
        if linha.size != 1:
            res.append({"x": v, "gene": g, "erro": f"{linha.size} linhas de mutante"})
            continue
        i = int(linha[0])
        xg = d_full[g].to_numpy(float)
        outros = np.delete(Y, i)
        q = float(np.mean(outros < Y[i]))
        coer = (q <= 0.05) if coef_bruto > 0 else (q >= 0.95)
        res.append({"x": v, "gene": g, "coef_bruto": coef_bruto, "delecao_funcionou": bool(xg[i] <= np.delete(xg, i).min()),
                    "quantil_Y_no_mutante": q, "fora_da_faixa": bool(Y[i] < outros.min() or Y[i] > outros.max()),
                    "sinal_coerente": bool((q < 0.5) == (coef_bruto > 0)), "coerencia_fraca": bool(coer)})
        r = res[-1]
        r["validacao_estrita"] = bool(r["delecao_funcionou"] and r["fora_da_faixa"])
    return res


def genetica():
    t0 = time.time()
    C = _comum()
    p8 = D.carregar(RAIZ / "dominios" / "genetica_y08")
    d = C._tabela(str(C._caminho(DADOS)))
    genes = [c for c in d.columns if c not in C.META]
    deletados = set(d["deleted_gene"].dropna())
    antigos = C.ys(DADOS)
    nunca = sorted(g for g in genes if g not in deletados and g not in set(antigos))
    novos = list(np.random.default_rng(SEMENTE_GEN).choice(nunca, N_Y_NOVOS, replace=False))
    alpha = 0.05 / N_Y_NOVOS
    out = {"y_antigos_h2f": antigos, "sorteio": f"default_rng({SEMENTE_GEN}).choice(sorted(nunca_deletados − 10 do H2f), "
           f"{N_Y_NOVOS})", "n_pool": len(nunca), "y_novos": novos, "alpha_por_y": alpha, "por_y": []}

    def um(y, a, rotulo):
        df, gmap, dfull = preparar_y(C, d, y)
        df = df.assign(ambiente=df["ambiente"].astype(str))
        tr = df[df["ambiente"] != "del_AD"].reset_index(drop=True)
        des = Desenho(p8, tr)
        blocos = {v: des.bloco(tr, v) for v in p8.icp_candidatos}
        r = icp(blocos, tr["y"].to_numpy(float), tr["ambiente"].to_numpy(str), "gaussiana", alpha=a,
                teste="residuos_np", regra_saturacao=True)
        com_gene = [v for v in r["S_hat"] if v != "plate"]
        val = validar(dfull, gmap, r["S_hat"], tr, blocos, y) if com_gene else []
        return {"rotulo": rotulo, "y": y, "genes": gmap, "alpha": a, "S_hat": r["S_hat"], "n_aceitos": len(r["aceitos"]),
                "melhor": r["melhor_conjunto"], "situacao": r["situacao"], "S_hat_tem_gene": bool(com_gene),
                "validacao": val}

    out["y08_original"] = um(antigos[7], 0.005, "y08 (H2f)")
    for k, y in enumerate(novos, 1):
        out["por_y"].append(um(y, alpha, f"n{k:02d}"))
        print("genetica", k, y, out["por_y"][-1]["S_hat"], flush=True)
    com = [r for r in out["por_y"] if r["S_hat_tem_gene"]]
    out["resumo"] = {"y_com_S_hat_com_gene": [r["rotulo"] for r in com],
                     "n_y_com_S_hat_nao_vazio": sum(bool(r["S_hat"]) for r in out["por_y"]),
                     "genes_validados_estrito": [(r["rotulo"], v["x"], v["gene"]) for r in com for v in r["validacao"]
                                                 if v.get("validacao_estrita")],
                     "genes_coerencia_fraca": [(r["rotulo"], v["x"], v["gene"]) for r in com for v in r["validacao"]
                                               if v.get("coerencia_fraca")],
                     "n_genes_em_S_hat": sum(len(r["validacao"]) for r in com)}
    out["segundos"] = round(time.time() - t0, 1)
    return out


if __name__ == "__main__":
    pedidos = sys.argv[1:] or ["farmacologia", "star", "genetica"]
    SAIDA.mkdir(parents=True, exist_ok=True)
    for a in pedidos:
        if a == "farmacologia":
            r = icp_robusto("farmacologia_qt", "unidade")
        elif a == "star":
            r = icp_robusto("educacao_star_turmas", "unidade")
        elif a == "genetica":
            r = genetica()
        else:
            raise SystemExit(a)
        (SAIDA / f"{a}.json").write_text(json.dumps(r, indent=1, ensure_ascii=False, default=float))
        print(a, "ok", r.get("segundos"), flush=True)
