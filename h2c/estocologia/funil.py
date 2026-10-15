"""O funil escalável do H2 (PLAN.md §2): quatro filtros por domínio, do mais barato ao mais caro.

F1  Crisonomia: EVPI da decisão sob a priori SHELF contra o custo da pesquisa (sem tocar nos dados).
F2  Morfonomia/unidade: porta dos fundos no DAG do F7 e informatividade na unidade independente.
F3  Teste severo: a chave contra o nulo de permutação dentro do ambiente.
F4  Fronteira: ICP entre ambientes, MCMC adaptativo + SBC, pré-registro, cofre fora do ambiente,
    predição conforme, portão M15 e decisão do F8.

Um domínio para no primeiro filtro que não passa, e o motivo fica no barramento. Nenhum agente lê o
cofre antes do pré-registro; toda leitura passa pelo Armazem (projeção explícita, EXPLAIN, orçamento).
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

import numpy as np
from scipy import special, stats

from estocologia import dominio as D
from estocologia import estat
from estocologia.amostrador import amostrar
from estocologia.armazem import Armazem
from estocologia.barramento import Barramento, Ficha
from estocologia.diario import Diario, Previsao
from estocologia.dsep import porta_dos_fundos
from estocologia.glm import ajustar, cov_agrupada, logverossimilhanca
from estocologia.icp import icp

ALFA_NULO = 0.05
PERMUTACOES = 200
MAX_DIARIO = 2000


def _canon(o) -> str:
    return json.dumps(o, sort_keys=True, ensure_ascii=False, default=float)


def publicar(bus: Barramento, p: D.Pacote, estagio: int, agente: str, tipo: str, payload: dict) -> Ficha:
    return bus.publicar(Ficha(p.nome, estagio, agente, tipo, p.cabecalho, json.loads(_canon(payload))))


# ---------------- matriz de desenho ----------------
class Desenho:
    """Codifica candidatos com estatísticas do TREINO: numéricos padronizados (a chave só centrada, para a
    priori SHELF valer na unidade natural), categóricos em indicadoras sem o nível mais frequente."""

    def __init__(self, p: D.Pacote, treino):
        self.p, self.info = p, {}
        for v, d in p.candidatos.items():
            if d["tipo"] == "num":
                x = treino[v].astype(float).to_numpy()
                sd = 1.0 if v == p.chave else (x.std() or 1.0)
                self.info[v] = ("num", float(x.mean()), float(sd))
            else:
                cont = treino[v].astype(str).value_counts()
                self.info[v] = ("cat", list(cont.index[1:]), None)

    def bloco(self, df, v) -> np.ndarray:
        t, a, b = self.info[v]
        if t == "num":
            return ((df[v].astype(float).to_numpy() - a) / b)[:, None]
        s = df[v].astype(str).to_numpy()
        return np.column_stack([(s == lv).astype(float) for lv in a]) if a else np.zeros((len(df), 0))

    def matriz(self, df, vars_) -> tuple[np.ndarray, list]:
        cols, nomes = [np.ones((len(df), 1))], ["intercepto"]
        for v in vars_:
            B = self.bloco(df, v)
            cols.append(B)
            nomes += [v] if B.shape[1] == 1 and self.info[v][0] == "num" else [f"{v}={lv}" for lv in self.info[v][1]]
        return np.hstack(cols), nomes


# ---------------- F7 ----------------
def f7(p: D.Pacote, bus) -> dict:
    pri = D.priori_da_chave(p)
    payload = {"pergunta": p.pergunta, "alegacao": p.alegacao, "glossario": p.glossario, "fontes": p.fontes,
               "unidade": p.unidade, "ambientes": p.ambientes,
               "shelf": {**p.shelf, "ajuste_normal": pri,
                         "painel": p.shelf.get("painel", "literatura publicada; nenhum especialista humano consultado")},
               "termo_escopo": p.termo_escopo, "dag": p.dag}
    publicar(bus, p, 1, "agent-f7-domain", "pacote_dominio", payload)
    return pri


# ---------------- F1 ----------------
def filtro1(p, pri, bus, rng) -> dict:
    th = D.theta(p, rng.normal(pri["media"], pri["dp"], 20_000))
    U = np.stack([D.utilidade(p, a, th) for a in ("adotar", "manter")])
    evpi = float(U.max(0).mean() - U.mean(1).max())
    acao = ("adotar", "manter")[int(np.argmax(U.mean(1)))]
    r = {"evpi_priori": evpi, "custo_pesquisa": p.decisao["custo_pesquisa"], "acao_sem_pesquisa": acao,
         "p_adotar_compensa_priori": float(np.mean(U[0] > 0)), "passa": evpi > p.decisao["custo_pesquisa"]}
    publicar(bus, p, 1, "agent-m15-orchestrator", "filtro1_evpi", r)
    return r


# ---------------- F2 ----------------
def filtro2(p, pri, treino, des, bus) -> dict:
    pdf = porta_dos_fundos([tuple(a) for a in p.dag], p.chave, "y", p.ajuste)
    X, nomes = des.matriz(treino, [p.chave, *p.ajuste])
    y = treino["y"].to_numpy(float)
    f = ajustar(X, y, p.familia)
    V = cov_agrupada(X, y, f, treino["unidade"].to_numpy(), p.familia)
    se = float(np.sqrt(V[1, 1]))
    se_ing = float(np.sqrt(f["cov"][1, 1]))
    n_eff = int(treino["unidade"].nunique())
    informativo = se < pri["dp"]
    r = {"porta_dos_fundos": pdf, "n_linhas": int(len(treino)), "n_unidades": n_eff,
         "ep_chave_por_unidade": se, "ep_chave_ingenuo": se_ing, "efeito_de_desenho": (se / se_ing) ** 2 if se_ing > 0 else None,
         "dp_priori": pri["dp"], "informativo": bool(informativo),
         "alerta_unidade": "uma linha por unidade: nenhum agrupamento declarado; o EP pode ser otimista se houver dependência não modelada" if n_eff == len(treino) else None, "beta_chave_mv": float(f["beta"][1]),
         "passa": bool(pdf["valido"] and informativo)}
    publicar(bus, p, 2, "agent-f2-data", "filtro2_unidade_dsep", r)
    return r


# ---------------- F3 ----------------
def _z_chave(X, y, g, fam):
    f = ajustar(X, y, fam)
    V = cov_agrupada(X, y, f, g, fam)
    return f["beta"][1] / np.sqrt(max(V[1, 1], 1e-300))


def filtro3(p, treino, des, bus, rng) -> dict:
    X, _ = des.matriz(treino, [p.chave, *p.ajuste])
    y = treino["y"].to_numpy(float)
    g = treino["unidade"].to_numpy()
    amb = treino["ambiente"].astype(str).to_numpy()
    z_obs = _z_chave(X, y, g, p.familia)
    # a chave é permutada no nível em que foi atribuída (v2):
    # - constante na unidade: valores trocados entre unidades (em todo o treino: unidades podem cruzar ambientes);
    # - sorteada dentro da unidade (pacote declara chave_atribuida: dentro_da_unidade): troca dentro de cada unidade;
    # - observacional e variando dentro da unidade: troca as médias entre unidades E os desvios dentro de cada uma,
    #   para o nulo quebrar as duas associações (entre e dentro).
    k = treino[p.chave].to_numpy(float)
    por_unidade = treino.groupby("unidade")[p.chave].nunique().max() == 1
    dentro = p.meta.get("chave_atribuida") == "dentro_da_unidade"
    us, inv = np.unique(g, return_inverse=True)
    med_u = np.bincount(inv, weights=k) / np.bincount(inv)
    idx_u = [np.flatnonzero(inv == i) for i in range(us.size)]
    nivel = ("entre unidades" if por_unidade else "dentro da unidade (sorteio declarado)" if dentro
             else "médias entre unidades + desvios dentro da unidade")
    zs = []
    for _ in range(PERMUTACOES):
        if por_unidade:
            kk = rng.permutation(med_u)[inv]
        else:
            kk = k.copy()
            for s_ in idx_u:
                kk[s_] = rng.permutation(k[s_])
            if not dentro:
                kk = kk - med_u[inv] + rng.permutation(med_u)[inv]
        Xp = X.copy()
        Xp[:, 1] = kk - k.mean()
        zs.append(abs(_z_chave(Xp, y, g, p.familia)))
    zs = np.array(zs)
    pv = float((1 + np.sum(zs >= abs(z_obs))) / (1 + zs.size))
    r = {"z_obs": float(z_obs), "nulo_q95": float(np.quantile(zs, 0.95)), "p_permutacao": pv, "permutacoes": PERMUTACOES,
         "nivel_da_permutacao": nivel, "passa": pv < ALFA_NULO}
    publicar(bus, p, 3, "agent-f1-code", "filtro3_nulo", r)
    return r


# ---------------- F4 ----------------
def _logpost(p, X, y, pri, peso=1.0):
    """Priori: chave ~ SHELF; demais coeficientes ~ N(0, 2,5²) na escala padronizada; log σ ~ N(log sd(y), 2²).

    v2: a verossimilhança iid é temperada por peso = min(1, 1/efeito de desenho da chave) — o ajuste de
    magnitude da verossimilhança composta (Pauli, Racugno & Ventura 2011, citado de memória). Sem isso a
    posterior ignora o agrupamento e fica estreita demais quando há dependência dentro da unidade."""
    fam = p.familia
    sdy = float(np.std(y)) if fam == "gaussiana" else 1.0
    escala_outros = 2.5 * (sdy if fam == "gaussiana" else 1.0)

    def lp(x):
        b = x[:X.shape[1]]
        ls = x[X.shape[1]] if fam == "gaussiana" else 0.0
        out = peso * logverossimilhanca(b, X, y, fam, ls)
        out += -0.5 * ((b[1] - pri["media"]) / pri["dp"]) ** 2
        out += -0.5 * np.sum((b[2:] / escala_outros) ** 2)
        out += -0.5 * (b[0] / (10 * escala_outros + 10 * abs(np.mean(y)))) ** 2
        if fam == "gaussiana":
            out += -0.5 * ((ls - np.log(sdy)) / 2) ** 2
        return out
    return lp


def _sbc(p, X, pri, rng, n_sim=60, L=19, n_dados=400) -> dict:
    """SBC no modelo completo, com covariáveis reamostradas do treino e coeficiente da chave tirado da priori."""
    fam = p.familia
    idx_all = np.arange(X.shape[0])
    postos = []
    for _ in range(n_sim):
        idx = rng.choice(idx_all, n_dados)
        Xs = X[idx]
        b = np.zeros(X.shape[1])
        b[1] = rng.normal(pri["media"], pri["dp"])
        b[2:] = rng.normal(0, 0.5, X.shape[1] - 2)
        b[0] = rng.normal(0, 0.5) if fam == "binomial" else rng.normal(0, 1)
        eta = Xs @ b
        if fam == "binomial":
            ys = (rng.random(n_dados) < special.expit(eta)).astype(float)
        else:
            ys = eta + rng.normal(0, 1, n_dados)

        def lp(x, Xs=Xs, ys=ys):
            bb = x[:Xs.shape[1]]
            ls = x[Xs.shape[1]] if fam == "gaussiana" else 0.0
            o = logverossimilhanca(bb, Xs, ys, fam, ls) - 0.5 * ((bb[1] - pri["media"]) / pri["dp"]) ** 2
            o += -0.5 * np.sum((bb[2:] / 0.5) ** 2) - 0.5 * (bb[0] / (0.5 if fam == "binomial" else 1.0)) ** 2
            if fam == "gaussiana":
                o += -0.5 * (ls / 2) ** 2
            return o
        x0 = np.zeros(X.shape[1] + (fam == "gaussiana"))
        r = amostrar(lp, x0, rng, cadeias=1, n=L * 20, aquec=400)
        post = r["amostras"][0, ::20, 1]
        postos.append(int(np.sum(post < b[1])))
    postos = np.array(postos)
    caixas = np.bincount(postos * 5 // (L + 1), minlength=5)
    chi2, pv = stats.chisquare(caixas)
    return {"n_sim": n_sim, "L": L, "caixas": caixas.tolist(), "p_uniformidade": float(pv)}


def _prever(p, X, beta, ls=None):
    eta = X @ beta
    return special.expit(eta) if p.familia == "binomial" else eta


def _perda(p, y, yhat):
    return float(np.mean((y - yhat) ** 2))


def filtro4(p, pri, treino, des, arm: Armazem, bus, diario: Diario, colunas, rng, deff: float | None = None) -> dict:
    fam = p.familia
    amb = treino["ambiente"].astype(str).to_numpy()
    y = treino["y"].to_numpy(float)
    # (a) ICP entre os ambientes de treino
    blocos = {v: des.bloco(treino, v) for v in p.icp_candidatos}
    r_icp = icp(blocos, y, amb, fam, teste="residuos_np")   # H2c: teste K = 2 (DECLARACAO_K2)
    r_icp_pub = {k: v for k, v in r_icp.items() if k != "testes"}
    r_icp_pub["p_por_subconjunto"] = {"+".join(t["S"]) or "∅": t["p"] for t in r_icp["testes"]}
    publicar(bus, p, 4, "agent-m15-orchestrator", "icp_morfonomia", r_icp_pub)
    # (b) MCMC adaptativo no modelo ajustado
    X, nomes = des.matriz(treino, [p.chave, *p.ajuste])
    peso = float(min(1.0, 1.0 / deff)) if deff else 1.0
    lp = _logpost(p, X, y, pri, peso)
    x0 = np.zeros(X.shape[1] + (fam == "gaussiana"))
    x0[0] = np.mean(y) if fam == "gaussiana" else special.logit(np.clip(np.mean(y), 0.01, 0.99))
    if fam == "gaussiana":
        x0[-1] = np.log(np.std(y))
    mc = amostrar(lp, x0, rng)
    A = mc["amostras"].reshape(-1, mc["amostras"].shape[2])
    bchave = A[:, 1]
    sbc = _sbc(p, X, pri, rng)
    th = D.theta(p, bchave, taxa=float(np.mean(y)) if fam == "binomial" else None)
    post = {"beta_chave_media": float(bchave.mean()), "beta_chave_ic90": np.quantile(bchave, [0.05, 0.95]).tolist(),
            "theta_media": float(th.mean()), "theta_ic90": np.quantile(th, [0.05, 0.95]).tolist(),
            "p_beta_positivo": float(np.mean(bchave > 0)), "rhat_max": mc["rhat_max"], "ess_min": mc["ess_min"],
            "aceitacao": mc["aceitacao"], "sbc": sbc, "parametros": nomes + (["log_sigma"] if fam == "gaussiana" else []),
            "dp_posterior_sobre_dp_priori": float(bchave.std() / pri["dp"]), "peso_verossimilhanca": peso}
    publicar(bus, p, 3, "agent-f4-frontier", "posterior", post)
    # (c) conforme (v2): escores de calibração deixando um ambiente de treino de fora por vez, para o intervalo
    #     já carregar a mudança entre ambientes (o cofre é um ambiente novo, não uma amostra do treino)
    bm = A.mean(0)[:X.shape[1]]
    esc_cal = []
    for a in np.unique(amb):
        s_ = amb == a
        fo = ajustar(X[~s_], y[~s_], fam)
        ph = _prever(p, X[s_], fo["beta"])
        esc_cal.append(y[s_] - ph if fam == "gaussiana" else 1 - np.where(y[s_] == 1, ph, 1 - ph))
    esc_cal = np.concatenate(esc_cal)
    if fam == "gaussiana":
        lo, hi = estat.conforme_intervalo(esc_cal, 0.1)
    else:
        qn = np.quantile(esc_cal, min(1.0, np.ceil((esc_cal.size + 1) * 0.9) / esc_cal.size))
    # (d) pré-registro: congela especificação, Ŝ e previsões antes de abrir o cofre
    S_hat = r_icp["S_hat"]
    todos = list(p.icp_candidatos)
    Xi, _ = des.matriz(treino, S_hat)
    Xa, _ = des.matriz(treino, todos)
    fi, fa = ajustar(Xi, y, fam), ajustar(Xa, y, fam)
    prereg = {"dominio": p.nome, "modelo": nomes, "beta_media": bm.tolist(), "S_hat": S_hat, "icp_aceitos": r_icp["aceitos"],
              "beta_invariante": fi["beta"].tolist(), "beta_todos": fa["beta"].tolist(),
              "clima": float(np.mean(y)), "mediana_treino": float(np.median(y)), "K": p.meta.get("tentativas_de_especificacao", 1)}
    h = hashlib.sha256(_canon(prereg).encode()).hexdigest()
    publicar(bus, p, 3, "agent-f3-lifecycle", "preregistro", {**prereg, "hash": h})
    # (e) o F3 abre o cofre: outro ambiente, nunca visto
    arm.destrancar(p.nome, h, "agent-f3-lifecycle")
    cof = arm.ler(p.nome, colunas, "agent-f3-lifecycle", ambientes=p.ambientes["cofre"])
    yv = cof["y"].to_numpy(float)
    Xv, _ = des.matriz(cof, [p.chave, *p.ajuste])
    Xvi, _ = des.matriz(cof, S_hat)
    Xva, _ = des.matriz(cof, todos)
    prev = {"modelo_ajustado": _prever(p, Xv, bm), "icp_invariante": _prever(p, Xvi, fi["beta"]),
            "todos_os_candidatos": _prever(p, Xva, fa["beta"]), "climatologia": np.full(yv.size, np.mean(y))}
    perda = {k: _perda(p, yv, v) for k, v in prev.items()}
    hab = {k: 1 - perda[k] / perda["climatologia"] for k in prev if k != "climatologia"}
    if fam == "gaussiana":
        res = yv - prev["modelo_ajustado"]
        cob = float(np.mean((res >= lo) & (res <= hi)))
    else:
        pv = prev["modelo_ajustado"]
        conj1 = (1 - pv) <= qn
        conj0 = pv <= qn
        cob = float(np.mean(np.where(yv == 1, conj1, conj0)))
    deriva = estat.deriva(y, yv)
    # diário: evento binário (y = 1, ou y acima da mediana do treino), no máximo MAX_DIARIO linhas sorteadas
    sel = rng.choice(yv.size, min(MAX_DIARIO, yv.size), replace=False)
    med = float(np.median(y))
    sig = float(np.exp(A[:, -1].mean())) if fam == "gaussiana" else None
    for nome_prev, v in prev.items():
        if nome_prev == "climatologia":
            continue
        for i in sel:
            pr = v[i] if fam == "binomial" else 1 - stats.norm.cdf(med, v[i], sig)
            base = float(np.mean(y)) if fam == "binomial" else float(np.mean(y > med))
            diario.registrar(Previsao(f"{p.nome}-{nome_prev}-{i}", p.nome, nome_prev, "y=1" if fam == "binomial" else f"y>{med:.4g}",
                                      float(np.clip(pr, 0.01, 0.99)), float(np.clip(base, 0.01, 0.99)), f"cofre: ambiente {p.ambientes['cofre']}",
                                      bool(p.meta.get("performativo", False)), "causa" if nome_prev == "icp_invariante" else "flutuação",
                                      Xv.shape[1] - 1 if nome_prev == "modelo_ajustado" else len(S_hat if nome_prev == "icp_invariante" else todos),
                                      2 if nome_prev == "icp_invariante" else 1))
            diario.resolver(f"{p.nome}-{nome_prev}-{i}", int(yv[i] == 1) if fam == "binomial" else int(yv[i] > med))
    plac = diario.placar(projeto=p.nome)
    fora = {"ambientes_cofre": p.ambientes["cofre"], "n_cofre": int(yv.size), "perda": perda, "habilidade": hab,
            "cobertura_conforme": cob, "deriva_y": deriva, "placar_diario": plac,
            "invariante_vence_todos_fora_do_ambiente": bool(perda["icp_invariante"] <= perda["todos_os_candidatos"])}
    publicar(bus, p, 3, "agent-f3-lifecycle", "fora_do_ambiente", fora)
    portao = {"rhat_ok": mc["rhat_max"] < 1.01, "ess_ok": mc["ess_min"] > 400, "sbc_ok": sbc["p_uniformidade"] > 0.01,
              "conforme_ok": cob >= 0.85, "modelo_vence_climatologia": hab["modelo_ajustado"] > 0,
              "trilha_integra": not bus.verificar()}
    aprovado = all(portao.values())
    publicar(bus, p, 5, "agent-m15-orchestrator", "portao_m15", {**portao, "aprovado": aprovado})
    return {"icp": r_icp_pub, "posterior": post, "preregistro_hash": h, "fora_do_ambiente": fora, "portao_m15": portao,
            "aprovado": aprovado, "_theta": th}


# ---------------- F8 ----------------
def f8(p, th, bus) -> dict:
    U = {a: D.utilidade(p, a, th) for a in ("adotar", "manter")}
    acao = max(U, key=lambda a: U[a].mean())
    pc = float(np.mean(U["adotar"] > 0))
    evpi = float(np.maximum(U["adotar"], U["manter"]).mean() - max(u.mean() for u in U.values()))
    r = {"acao": acao, "p_adotar_compensa": pc, "evpi_posterior": evpi,
         "frequencias_naturais": f"Em {round(100 * pc)} de cada 100 mundos compatíveis com os dados e a priori, adotar '{p.decisao['acao']}' compensa o custo.",
         "circuit_breaker": "suspender a recomendação se a deriva KS do desfecho num ambiente novo tiver p < 0,01 ou se o ICP deixar de aceitar o conjunto invariante",
         "escopo": p.termo_escopo}
    publicar(bus, p, 6, "agent-f8-decision", "decisao", r)
    return r


# ---------------- F5/F6 ----------------
def f5f6(p, treino, bus) -> dict:
    qi = p.meta.get("quase_ids", [])
    if qi and p.meta.get("tem_titulares", True):
        k = int(treino.groupby(qi).size().min())
    else:
        k = None
    out = {"tem_titulares": p.meta.get("tem_titulares", True), "quase_ids": qi, "k_anonimato_bruto": k,
           "dados": "reais (" + ", ".join(p.arquivos) + ")", "responsavel_humano": {"nome": "sil", "assinatura": "PENDENTE"}}
    g = p.meta.get("grupo_justica")
    if g and g in treino.columns:
        taxas = treino.groupby(g)["y"].agg(["mean", "size"])
        out["justica"] = {"grupo": g, "taxa_base_por_grupo": {str(i): [float(r["mean"]), int(r["size"])] for i, r in taxas.iterrows()}}
    publicar(bus, p, 2, "agent-f5-governance", "governanca", out)
    return out


# ---------------- domínio inteiro ----------------
def rodar_dominio(pasta: Path, dados: Path, arm: Armazem, bus: Barramento, diario: Diario, semente: int) -> dict:
    rng = np.random.default_rng(semente)
    p = D.carregar(pasta)
    df = p.preparar(dados)
    erros = D.checar_tabela(p, df)
    if erros:
        raise D.PacoteInvalido(f"{p.nome}: {erros}")
    df = df.assign(ambiente=df["ambiente"].astype(str))
    ing = arm.ingerir(p.nome, df, cofre=p.ambientes["cofre"])
    r: dict = {"dominio": p.nome, "area": p.area, "titulo": p.titulo, "alegacao": p.alegacao, "ingestao": ing,
               "K": p.meta.get("tentativas_de_especificacao", 1)}
    pri = f7(p, bus)
    r["priori"] = pri
    r["filtro1"] = filtro1(p, pri, bus, rng)
    if not r["filtro1"]["passa"]:
        r["status"] = "podado no F1: a pesquisa não muda a decisão (EVPI < custo)"
        return r
    colunas = ["y", "unidade", "ambiente", *p.candidatos]
    treino = arm.ler(p.nome, colunas, "agent-f2-data", excluir=p.ambientes["cofre"])
    des = Desenho(p, treino)
    r["governanca"] = f5f6(p, treino, bus)
    r["filtro2"] = filtro2(p, pri, treino, des, bus)
    if not r["filtro2"]["passa"]:
        r["status"] = "podado no F2: " + ("ajuste não fecha a porta dos fundos" if not r["filtro2"]["porta_dos_fundos"]["valido"]
                                          else "dados na unidade independente não informam mais que a priori")
        return r
    r["filtro3"] = filtro3(p, treino, des, bus, rng)
    if not r["filtro3"]["passa"]:
        r["status"] = "podado no F3: a chave não vence o nulo de permutação"
        return r
    f4 = filtro4(p, pri, treino, des, arm, bus, diario, colunas, rng, deff=r["filtro2"].get("efeito_de_desenho"))
    th = f4.pop("_theta")
    r["filtro4"] = f4
    if f4["aprovado"]:
        r["decisao"] = f8(p, th, bus)
        r["status"] = "aprovado no F4: decisão emitida"
    else:
        falhas = [k for k, v in f4["portao_m15"].items() if not v]
        r["status"] = "reprovado no F4 (M15): " + ", ".join(falhas)
    return r
