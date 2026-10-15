"""Portão de saída do H3. Os critérios estão em CRITERIOS.md, declarados antes de abrir qualquer cofre;
o hash daquele arquivo entra em cada pré-registro.

Uso:  python3 portoes.py [--reproducao]
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

import numpy as np
import pandas as pd

from estocologia import calibracao as C
from estocologia import governanca as G
from estocologia import performativo as P
from estocologia.armazem import Armazem, Recusa, Relogio
from estocologia.barramento import Barramento
from estocologia.motor import sha_arquivo

RAIZ = Path(__file__).resolve().parent
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))
DADOS = Path(os.environ.get("ESTOCOLOGIA_DADOS", RAIZ.parent / "dados-reais"))
FLUXOS = ["energia", "central", "bolsa", "voos", "ciclones", "prisoes"]
VOLATEIS = {"segundos", "quando"}


def carregar(saida: Path) -> dict:
    out = {}
    for f in FLUXOS:
        p = saida / f / "resultados.json"
        if p.exists():
            r = json.loads(p.read_text())
            r["_diario"] = pd.read_parquet(saida / f / "diario.parquet")
            out[f] = r
    return out


def sem_volateis(o):
    if isinstance(o, dict):
        return {k: sem_volateis(v) for k, v in o.items() if k not in VOLATEIS and not k.startswith("_")}
    if isinstance(o, list):
        return [sem_volateis(v) for v in o]
    if isinstance(o, float):
        return round(o, 8)
    return o


def cofre(r) -> pd.DataFrame:
    d = r["_diario"]
    return d[d["y"].notna() & (d["t_emissao"] >= r["corte"])]


def dev(r) -> pd.DataFrame:
    d = r["_diario"]
    return d[d["y"].notna() & (d["t_emissao"] < r["corte"])]


def niveis(d: pd.DataFrame) -> list[float]:
    return [float(c.split("modelo_q")[1]) for c in d.columns if c.startswith("modelo_q")]


# ---------------- G1 ----------------
def g1(res, reproducao):
    t = subprocess.run([sys.executable, "-m", "pytest", "-q", "-p", "no:cacheprovider", "tests"], cwd=RAIZ, capture_output=True, text=True)
    ok = t.returncode == 0
    det = f"pytest: {t.stdout.strip().splitlines()[-1] if t.stdout.strip() else t.stderr[-300:]}"
    if reproducao:
        with tempfile.TemporaryDirectory() as tmp:
            env = {**os.environ, "ESTOCOLOGIA_SAIDA": tmp}
            subprocess.run([sys.executable, "rodar.py"], cwd=RAIZ, env=env, check=True, capture_output=True)
            res2 = carregar(Path(tmp))
            iguais = {}
            for k in res:
                a = {kk: v for kk, v in res[k].items() if kk not in ("K",)}
                b = {kk: v for kk, v in res2.get(k, {}).items() if kk not in ("K",)}
                iguais[k] = sem_volateis(a) == sem_volateis(b) and res[k]["_diario"].equals(res2[k]["_diario"])
        ok &= all(iguais.values())
        det += f"; reprodução limpa idêntica (resultados e diário inteiro): {iguais}"
    return ok, det


# ---------------- G2 ----------------
def g2(res):
    linhas, ok = {}, True
    for nome, r in res.items():
        d = cofre(r)
        if r["tipo"] == "binario":
            clima = C.climatologia_movel(r["_diario"])[cofre_mask(r)]
            m = C.binario(d, clima)
            passa = abs(m["citl"]) <= 0.02 and m["ece"] <= 0.03 and m["habilidade_vs_climatologia"] > 0
        elif nome == "bolsa":
            q = C.oficial_quantis(d, niveis(d))
            viol = (d["y"].to_numpy() < q[:, 0])
            m = {"n": int(len(d)), "kupiec": C.kupiec(viol, 0.01), "christoffersen": C.christoffersen(viol),
                 **C.quantis(d, niveis(d))}
            passa = 0.005 <= m["kupiec"]["taxa"] <= 0.015 and m["kupiec"]["p"] > 0.05
        else:
            m = C.quantis(d, niveis(d))
            passa = 0.86 <= m["cobertura_90"] <= 0.94 and m["habilidade_intervalo_vs_reserva"] > 0
        m["passa"] = bool(passa)
        linhas[nome] = m
        ok &= passa
    ok &= len(res) == len(FLUXOS)
    return ok, linhas


def cofre_mask(r):
    d = r["_diario"]
    return (d["y"].notna() & (d["t_emissao"] >= r["corte"])).to_numpy()


# ---------------- G3 ----------------
def banca_vazamento() -> dict:
    out = {}
    with tempfile.TemporaryDirectory() as tmp:
        rel = Relogio(1_000)
        arm = Armazem(Path(tmp), rel, corte_cofre=5_000, orcamento_por_agente=10**9)
        arm.ingerir("t", pd.DataFrame({"x": np.arange(100.0), "t_disp": np.arange(0, 10_000, 100)}), 8)

        def tenta(f):
            try:
                f()
                return False
            except Recusa:
                return True
        out["ler_o_futuro"] = tenta(lambda: arm.ler("t", ["x"], "agent-f4-frontier", 0, 2_000))
        out["select_estrela"] = tenta(lambda: arm.ler("t", ["*"], "agent-f4-frontier", 0, 1_000))
        rel.avancar(6_000)
        out["cofre_antes_do_preregistro"] = tenta(lambda: arm.ler("t", ["x"], "agent-f4-frontier", 0, 6_000))
        out["agente_errado_abre_cofre"] = tenta(lambda: arm.destrancar("a" * 64, "agent-f4-frontier"))
        arm.destrancar("b" * 64, "agent-f3-lifecycle")
        out["segunda_abertura_do_cofre"] = tenta(lambda: arm.destrancar("c" * 64, "agent-f3-lifecycle"))
        out["voltar_o_relogio"] = tenta(lambda: rel.avancar(10))
        lido = arm.ler("t", ["x"], "agent-f4-frontier", 0, 6_000)
        out["leitura_legitima_funciona"] = bool(len(lido) == 60 and lido["t_disp"].max() <= 6_000)
    return out


def canario(r) -> dict:
    """Ganho de habilidade do previsor que recebe a coluna canário (igual ao rótulo, publicada quando o rótulo
    se torna conhecível). Ajuste no desenvolvimento, avaliação no cofre. Honesto: a coluna que o motor viu
    na emissão. Controle positivo (relógio desligado): a coluna verdadeira, isto é, o próprio rótulo."""
    d = r["_diario"][r["_diario"]["y"].notna()]
    p = np.clip(np.where(d["origem"] == "modelo", d["modelo"], d["reserva"]).astype(float), 0.01, 0.99)
    lg = np.log(p / (1 - p))
    y = d["y"].to_numpy(float)
    tr = (d["t_emissao"] < r["corte"]).to_numpy()
    te = ~tr
    clima = C.climatologia_movel(r["_diario"])[r["_diario"]["y"].notna().to_numpy()]
    from fluxos.base import logistica, sigmoide

    def hab(cols):
        X = np.column_stack([np.ones(len(y)), lg, *cols])
        b = logistica(X[tr], y[tr], lam=1e-4)
        q = np.clip(sigmoide(X[te] @ b), 0.01, 0.99)
        return 1 - np.mean((q - y[te]) ** 2) / np.mean((clima[te] - y[te]) ** 2)
    base = hab([])
    honesto = hab([d["canario"].to_numpy(float) - 0.5])
    vazado = hab([y - 0.5])
    return {"habilidade_sem_canario": float(base), "ganho_canario_honesto": float(honesto - base),
            "ganho_canario_vazado": float(vazado - base),
            "canario_visto_na_emissao_diferente_de_0_5": int((d["canario"] != 0.5).sum())}


def g3(res):
    det, ok = {}, True
    for nome, r in res.items():
        d = r["_diario"]
        res_ = d[d["y"].notna()]
        a = {"leituras_com_futuro": r["varredura"]["leituras_com_futuro"],
             "estado_do_futuro_na_emissao": int((d["estado_tmax"] > d["t_emissao"]).sum()),
             "rotulo_ja_conhecido_na_emissao": int((res_["t_rotulo"] <= res_["t_emissao"]).sum()),
             "contagens_do_motor": {k: r["contagens"][k] for k in ("estado_do_futuro", "rotulo_ja_conhecido")}}
        bus = Barramento(SAIDA / nome / "barramento.jsonl")
        tipos = [f.tipo for f in bus.fichas]
        a["preregistro_antes_do_cofre"] = ("preregistro" in tipos and "cofre_aberto" in tipos
                                           and tipos.index("preregistro") < tipos.index("cofre_aberto")
                                           and tipos.count("cofre_aberto") == 1)
        a["emissoes_no_cofre_depois_da_abertura"] = bool(r["preregistro"])
        o = (a["leituras_com_futuro"] == 0 and a["estado_do_futuro_na_emissao"] == 0 and a["rotulo_ja_conhecido_na_emissao"] == 0
             and a["preregistro_antes_do_cofre"] and all(v == 0 for v in a["contagens_do_motor"].values()))
        if r["tipo"] == "binario":
            c = canario(r)
            a["canario"] = c
            o &= c["ganho_canario_honesto"] <= 0.02 and c["ganho_canario_vazado"] > 0.2
        a["passa"] = bool(o)
        det[nome] = a
        ok &= o
    banca = banca_vazamento()
    ok &= all(banca.values())
    return ok, {"fluxos": det, "banca": banca}


# ---------------- G4 ----------------
def goodhart(res) -> dict:
    out = {}
    for nome, r in res.items():
        dv, cf = dev(r), cofre(r)
        if r["tipo"] == "binario":
            d = r["_diario"]
            clima_all = C.climatologia_movel(d)
            mv = (d["y"].notna() & (d["t_emissao"] < r["corte"])).to_numpy()
            mc = cofre_mask(r)
            hd = C.binario(dv, clima_all[mv])["habilidade_vs_climatologia"]
            hc = C.binario(cf, clima_all[mc])["habilidade_vs_climatologia"]
            # banca 1: o que se abstém na zona incerta recebe o reserva ali
            p_of = C.oficial_binario(cf)
            pm = cf["modelo"].to_numpy(float)
            y = cf["y"].to_numpy(float)
            incerto = (pm > 0.3) & (pm < 0.7)
            p_abs = np.where(incerto, cf["reserva"].to_numpy(float), p_of)
            b_hon, b_abs = np.mean((p_of - y) ** 2), np.mean((p_abs - y) ** 2)
            b_res_incerto = np.mean((cf["reserva"].to_numpy(float)[incerto] - y[incerto]) ** 2) if incerto.any() else 0.0
            b_abs_incerto = np.mean((p_abs[incerto] - y[incerto]) ** 2) if incerto.any() else 0.0
            # banca 2: o constante na climatologia
            cl = clima_all[mc]
            hab_const = 1 - np.mean((cl - y) ** 2) / np.mean((cl - y) ** 2)
            ln_cap = float(np.sum(np.where(y == 1, np.log(cl / cl), np.log((1 - cl) / (1 - cl)))))
            peso_const = 1.0 if (hab_const > 0 and ln_cap > 0) else 0.0
            out[nome] = {"indice_goodhart": float(hd - hc), "habilidade_dev": float(hd), "habilidade_cofre": float(hc),
                         "abstencao": {"fracao_incerta": float(incerto.mean()), "brier_honesto": float(b_hon), "brier_abstencao": float(b_abs),
                                       "abstencao_recebe_o_escore_do_reserva": bool(np.isclose(b_abs_incerto, b_res_incerto)),
                                       "abstencao_nao_fica_a_frente": bool(b_abs >= b_hon - 1e-12)},
                         "constante_na_climatologia": {"habilidade": float(hab_const), "ln_capital": ln_cap, "peso": peso_const}}
        else:
            hd = dev(r) if len(dv) else None
            ms = C.quantis(dv, niveis(dv)) if len(dv) else {}
            mc = C.quantis(cf, niveis(cf))
            k = "habilidade_intervalo_vs_reserva"
            if k in mc:
                out[nome] = {"indice_goodhart": float(ms[k] - mc[k]), "habilidade_dev": ms[k], "habilidade_cofre": mc[k]}
            else:
                pin = lambda x, q: np.mean([np.mean(np.maximum(n * (x["y"] - x[f"{q}_q{n:g}"]), (n - 1) * (x["y"] - x[f"{q}_q{n:g}"]))) for n in niveis(x)])
                hdv = 1 - pin(dv, "modelo") / pin(dv, "reserva")
                hcf = 1 - pin(cf, "modelo") / pin(cf, "reserva")
                out[nome] = {"indice_goodhart": float(hdv - hcf), "habilidade_dev": float(hdv), "habilidade_cofre": float(hcf),
                             "nota": "habilidade de pinball do modelo contra o reserva"}
    return out


def g4(res, perf):
    p6, p0 = perf["r=0.6"], perf["r=0.0"]
    a = p6["deteccao_M11"]["p"] < 0.01 and p0["deteccao_M11"]["p"] > 0.05
    b = abs(p6["controlado"]["vies_relativo_vs_demanda_exogena"]) <= 0.03
    gh = goodhart(res)
    bin_ = {k: v for k, v in gh.items() if "abstencao" in v}
    c = all(v["abstencao"]["abstencao_recebe_o_escore_do_reserva"] and v["abstencao"]["abstencao_nao_fica_a_frente"]
            and v["constante_na_climatologia"]["peso"] == 0 for v in bin_.values())
    c &= banca_vazamento()["segunda_abertura_do_cofre"]
    return bool(a and b and c), {"a_deteccao": bool(a), "b_controle": bool(b), "c_banca_goodhart": bool(c),
                                 "laco_performativo": perf, "goodhart": gh}


# ---------------- G5 ----------------
def g5(res):
    det, ok = {}, True
    for nome, r in res.items():
        v = r["varredura"]
        reg = json.loads((SAIDA / nome / "registro_varredura.json").read_text())
        estrela = [x for x in reg if "*" in x["colunas"]]
        eq = {t: round(x["varreduras_equivalentes"], 3) for t, x in v["por_tabela"].items()}
        acima = {a: g for a, g in v["gasto_por_agente"].items() if g > v["orcamento_por_agente"]}
        o = not estrela and all(x <= 3 for x in eq.values()) and not acima
        det[nome] = {"leituras": v["leituras"], "varreduras_equivalentes_por_tabela": eq, "select_estrela": len(estrela),
                     "agentes_acima_do_orcamento": list(acima), "passa": bool(o)}
        ok &= o
    return ok, det


# ---------------- informativos ----------------
def g6(res):
    if "prisoes" not in res:
        return {}
    r = res["prisoes"]
    d = pd.read_csv(DADOS / "h2" / "carData_Arrests.csv")
    y = (d["released"] == "Yes").astype(int)
    f5 = {"k_anonimato": G.k_anonimato(d, ["colour", "year", "age", "sex", "employed", "citizen"]),
          **G.f5_publicacoes(d, y, np.random.default_rng(2027))}
    dd = r["_diario"][r["_diario"]["y"].notna()].copy()
    dd["p"] = C.oficial_binario(dd)
    f6 = G.f6_trilema(dd)
    return {"F5": f5, "F6": f6}


def g7(res):
    out, total = {}, 0
    for nome, r in res.items():
        ev = r["eventos_operacao"]
        conta = lambda tipo, e, fase: sum(1 for x in ev if x["tipo"] == tipo and x["evento"] == e and ((x["t"] < r["corte"]) == (fase == "dev")))
        out[nome] = {"previsoes": int(len(r["_diario"])), "contagens": r["contagens"], "decisoes_C_L": r["decisoes_C_L"],
                     "alarmes_deriva": {"dev": conta("deriva", "ligou", "dev"), "cofre": conta("deriva", "ligou", "cofre")},
                     "disjuntor_abriu": {"dev": conta("disjuntor", "abriu", "dev"), "cofre": conta("disjuntor", "abriu", "cofre")},
                     "K": r["K"]}
        total += len(r["_diario"])
    g = json.loads((SAIDA / "hipergrafo_resumo.json").read_text()) if (SAIDA / "hipergrafo_resumo.json").exists() else {}
    return {"fluxos": out, "previsoes_total": total, "hipergrafo": g}


def main():
    res = carregar(SAIDA)
    if not res:
        print("nada em", SAIDA, "— rode rodar.py antes")
        return 1
    perf = P.rodar(DADOS / "bank_calls.csv")
    out = {}
    out["G1 simulabilidade e reprodução"] = g1(res, "--reproducao" in sys.argv)
    out["G2 calibração fora do tempo"] = g2(res)
    out["G3 zero vazamento temporal"] = g3(res)
    out["G4 performatividade e Goodhart"] = g4(res, perf)
    out["G5 contenção de varredura"] = g5(res)
    quebras = {n: Barramento(SAIDA / n / "barramento.jsonl").verificar() for n in res}
    out["Trilha do barramento"] = (all(not q for q in quebras.values()), {n: q or "íntegra" for n, q in quebras.items()})
    saida = {k: {"ok": bool(o), "detalhe": d} for k, (o, d) in out.items()}
    saida["G6 F5/F6 (informativo)"] = g6(res)
    saida["G7 operação (informativo)"] = g7(res)
    saida["criterios_sha256"] = sha_arquivo(RAIZ / "CRITERIOS.md")
    saida["saida_do_H3"] = all(o for o, _ in out.values())
    (SAIDA / "portoes.json").write_text(json.dumps(saida, indent=1, ensure_ascii=False, default=float))
    for k, (o, d) in out.items():
        print(f"{'PASSA' if o else 'FALHA'}  {k}")
    print("SAÍDA DO H3:", "aprovada" if saida["saida_do_H3"] else "NÃO aprovada")
    return 0 if saida["saida_do_H3"] else 1


if __name__ == "__main__":
    sys.exit(main())
