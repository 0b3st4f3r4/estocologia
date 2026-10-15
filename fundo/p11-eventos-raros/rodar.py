"""Ponto 11 do fundo (bloco F): resolução e não só calibração onde o evento é raro.

Três casos (indo, covid, ri), três caminhos cada (C climatologia, L logística reajustada, R escore de ranking),
com a decomposição de Murphy (REL, RES, UNC) e IC90 por bootstrap, e o critério declarado em DECLARACAO.md:
um caminho vence se RES > REL com IC90 acima de zero E habilidade de Brier >= +0,05.

Uso:  OMP_NUM_THREADS=1 python3 rodar.py [caso ...]      (sem nomes, roda os três)

Lê só o que já está no repositório: os armazens do p09 para os dois casos de saúde e o replay do H3d (escrito
por `codigo/h3/rodar.py prisoes_ri` dentro desta pasta) para Rhode Island. Nada inventado, nada escrito fora de
`fundo/p11-eventos-raros/`.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import duckdb
import numpy as np
import pandas as pd
from scipy import special

RAIZ = Path(__file__).resolve().parent
REPO = RAIZ.parents[1]
sys.path.insert(0, str(RAIZ / "codigo" / "p09"))

from estocologia import dominio as D        # noqa: E402
from estocologia import funil as FU         # noqa: E402
from estocologia import glm as G            # noqa: E402

import comum as C                           # noqa: E402

SAIDA = RAIZ / "saida"
CORTE_RI = 1356998400                       # ts("2013-01-01"), o corte publicado do H3d
HASH_RI = "577de4d8111705a9babcc337f7e855df2629eba9f3403b0e7a45f8b9588c1062"
SAUDE = REPO / "fundo" / "p09-escala-dados" / "saida_saude"
INDO_GLOB = str(SAUDE / "saude_indometacina_ercp" / "armazem" / "saude_indometacina_ercp" / "**" / "*.parquet")
COVID_GLOB = str(SAUDE / "saude_covid_vacina_paises" / "armazem" / "saude_covid_vacina_paises" / "**" / "*.parquet")
DOM_INDO = RAIZ / "codigo" / "p09" / "dominios" / "saude_indometacina_ercp"
DOM_COVID = RAIZ / "codigo" / "p09" / "dominios" / "saude_covid_vacina_paises"
DIARIO_RI = SAIDA / "h3_ri" / "prisoes_ri" / "diario.parquet"
RES_RI = REPO / "h3" / "saida" / "prisoes_ri" / "resultados.json"


def ler(glob: str, ordem: str) -> pd.DataFrame:
    """Leitura determinística: glob fixo e ordem explícita."""
    con = duckdb.connect()
    con.execute("SET threads=1")
    df = con.execute(f"SELECT * FROM read_parquet('{glob}', hive_partitioning=true) ORDER BY {ordem}").df()
    con.close()
    return df


def logit(p):
    p = np.clip(np.asarray(p, float), 1e-9, 1 - 1e-9)
    return np.log(p / (1 - p))


def json_res(o):
    """Serializa numpy e nan (nan vira null, para o json ser estrito)."""
    if isinstance(o, dict):
        return {k: json_res(v) for k, v in o.items()}
    if isinstance(o, (list, tuple)):
        return [json_res(v) for v in o]
    if isinstance(o, (np.integer,)):
        return int(o)
    if isinstance(o, (np.floating, float)):
        v = float(o)
        return None if not np.isfinite(v) else v
    if isinstance(o, (np.bool_, bool)):
        return bool(o)
    return o


# ---------------------------------------------------------------- caso indo
def caso_indo():
    fonte = SAUDE / "raro_indo.json"
    pub = json.loads(fonte.read_text())
    df = ler(INDO_GLOB, "ambiente, unidade")
    tr, cf = df[df.ambiente != "UM"], df[df.ambiente == "UM"]
    VARS = ["indometacina", "risco", "idade", "homem"]
    mu, sd = tr[VARS].mean(), tr[VARS].std()

    def X(d):
        return np.column_stack([np.ones(len(d)), ((d[VARS] - mu) / sd).to_numpy(float)])

    ajuste = G.ajustar(X(tr), tr.y.to_numpy(float), "binomial")
    pc = special.expit(X(cf) @ ajuste["beta"])
    sc_tr, sc_cf = X(tr) @ ajuste["beta"], X(cf) @ ajuste["beta"]
    y, y_tr = cf.y.to_numpy(float), tr.y.to_numpy(float)
    clima = float(y_tr.mean())

    # conferência: método do próprio escala/raro_indo.py (caixas de largura igual, IC90 com semente 2027)
    brier = float(np.mean((pc - y) ** 2))
    brier_c = float(np.mean((clima - y) ** 2))
    rng = np.random.default_rng(C.SEMENTE)
    hb = []
    for _ in range(C.B_BOOT):
        i = rng.integers(0, y.size, y.size)
        hb.append(1 - np.mean((pc[i] - y[i]) ** 2) / np.mean((clima - y[i]) ** 2))
    conferencia = {
        "metodo": "escala/raro_indo.py do p09: caixas de largura igual, bootstrap semente 2027",
        "publicado": {"n_cofre": pub["n_cofre"], "eventos_cofre": pub["eventos_cofre"], "brier": pub["brier"],
                      "brier_clima": pub["brier_clima"], "habilidade": pub["habilidade"],
                      "habilidade_ic90_bootstrap": pub["habilidade_ic90_bootstrap"], "murphy": pub["murphy_modelo"]},
        "reproduzido": {"n_cofre": int(len(y)), "eventos_cofre": int(y.sum()), "brier": brier, "brier_clima": brier_c,
                        "habilidade": 1 - brier / brier_c,
                        "habilidade_ic90_bootstrap": np.quantile(hb, [0.05, 0.95]).tolist(),
                        "murphy": C.murphy_largura(pc, y)},
    }
    conferencia["max_diferenca_absoluta"] = max(
        abs(conferencia["publicado"]["brier"] - brier),
        abs(conferencia["publicado"]["brier_clima"] - brier_c),
        abs(conferencia["publicado"]["habilidade"] - (1 - brier / brier_c)),
        *(abs(conferencia["publicado"]["murphy"][k] - conferencia["reproduzido"]["murphy"][k])
          for k in ("confiabilidade", "resolucao", "incerteza")))
    conferencia["bate_dentro_de_1e-9"] = bool(conferencia["max_diferenca_absoluta"] < 1e-9)

    p_r, taxas, bordas = C.decis(sc_tr, sc_cf, y_tr)
    dados = {"n_treino": int(len(tr)), "eventos_treino": int(y_tr.sum()), "taxa_treino": clima,
             "n_cofre": int(len(y)), "eventos_cofre": int(y.sum()), "taxa_cofre": float(y.mean()),
             "evento": "pancreatite pós-CPRE no centro de Michigan (paciente)",
             "dados_lidos": "armazem do p09 (parte.parquet); o CSV bruto não está no repositório",
             "taxas_do_ranking_no_desenvolvimento": taxas.tolist()}
    return C.juntar("indo", "Indometacina retal e pancreatite pós-CPRE (Elmunzer et al. 2012)",
                    str(fonte.relative_to(REPO)), dados, y,
                    {"C": np.full(y.size, clima), "L": pc, "R": p_r}, np.full(y.size, clima),
                    extra={"conferencia": conferencia})


# ---------------------------------------------------------------- caso covid
def caso_covid():
    fonte = SAUDE / "saude_covid_vacina_paises" / "resultados.json"
    pub = json.loads(fonte.read_text())
    df = ler(COVID_GLOB, "ambiente, unidade, mes")
    tr, cf = df[df.ambiente != "South_America"], df[df.ambiente == "South_America"]
    T = float(np.quantile(tr.y.to_numpy(float), 0.90))
    y_tr = (tr.y.to_numpy(float) > T).astype(float)
    y = (cf.y.to_numpy(float) > T).astype(float)
    pac = D.carregar(DOM_COVID)
    des = FU.Desenho(pac, tr)
    X, nomes = des.matriz(tr, [pac.chave, *pac.ajuste])
    Xc, _ = des.matriz(cf, [pac.chave, *pac.ajuste])
    ajuste = G.ajustar(X, y_tr, "binomial")
    pc = special.expit(Xc @ ajuste["beta"])
    clima = float(y_tr.mean())
    p_r, taxas, bordas = C.decis(X @ ajuste["beta"], Xc @ ajuste["beta"], y_tr)
    conferencia = {
        "linhas_no_armazem": int(len(df)), "n_treino": int(len(tr)), "n_cofre": int(len(cf)),
        "publicado_filtro2": {"n_linhas": pub["filtro2"]["n_linhas"], "n_unidades": pub["filtro2"]["n_unidades"]},
        "ambientes": sorted(df.ambiente.unique().tolist()),
        "limiar_T_do_treino": T, "desenho": nomes,
        "publicado_f3": pub["status"],
        "bate": bool(int(len(tr)) == pub["filtro2"]["n_linhas"]),
    }
    dados = {"n_treino": int(len(tr)), "eventos_treino": int(y_tr.sum()), "taxa_treino": clima,
             "n_cofre": int(len(y)), "eventos_cofre": int(y.sum()), "taxa_cofre": float(y.mean()),
             "evento": f"mortes por milhão no mês seguinte acima de T = q90 do treino = {T:.6f} (país-mês)",
             "dados_lidos": "armazem do p09 (parte.parquet); o CSV bruto não está no repositório",
             "binarizacao_declarada": "T = quantil 0,90 amostral de y no treino; y_raro = 1 se y > T",
             "taxas_do_ranking_no_desenvolvimento": taxas.tolist()}
    return C.juntar("covid", "Painel país-mês de vacinação e mortes por COVID-19 (Our World in Data, 2021-2022)",
                    str(fonte.relative_to(REPO)), dados, y,
                    {"C": np.full(y.size, clima), "L": pc, "R": p_r}, np.full(y.size, clima),
                    extra={"conferencia": conferencia})


# ---------------------------------------------------------------- caso ri
def caso_ri():
    pub = json.loads(RES_RI.read_text())
    if not DIARIO_RI.exists():
        return {"caso": "ri", "disponivel": False,
                "motivo": "o replay do H3d não gerou saida/h3_ri/prisoes_ri/diario.parquet nesta execução"}
    df = ler(str(DIARIO_RI), "t_emissao, id")
    df = df[df.y.notna()].reset_index(drop=True)
    dev, cf = df[df.t_emissao < CORTE_RI], df[df.t_emissao >= CORTE_RI]
    y_tr, y = dev.y.to_numpy(float), cf.y.to_numpy(float)
    sc_tr, sc_cf = logit(dev.modelo_cru), logit(cf.modelo_cru)
    p_r, taxas, bordas = C.decis(sc_tr, sc_cf, y_tr)
    p_c = cf.reserva.to_numpy(float)
    p_l = cf.modelo.to_numpy(float)
    oficial = np.where(cf.origem.to_numpy() == "modelo", p_l, p_c)
    rep = json.loads((SAIDA / "h3_ri" / "prisoes_ri" / "resultados.json").read_text())
    placar = rep["placar"]["cofre"]
    perda_modelo = float(np.mean((p_l - y) ** 2))
    perda_reserva = float(np.mean((p_c - y) ** 2))
    conferencia = {
        "hash_especificacao_do_replay": rep["hash_especificacao"],
        "hash_especificacao_publicado": pub["hash_especificacao"], "hash_publicado": HASH_RI,
        "hash_bate": bool(rep["hash_especificacao"] == HASH_RI == pub["hash_especificacao"]),
        "n_cofre": int(len(y)), "n_cofre_publicado": int(placar["n"]),
        "perda_modelo": perda_modelo, "perda_modelo_publicada": float(placar["perda_modelo"]),
        "perda_reserva": perda_reserva, "perda_reserva_publicada": float(placar["perda_reserva"]),
        "habilidade_modelo_vs_reserva": float(1 - perda_modelo / perda_reserva),
        "habilidade_oficial_vs_reserva_publicada": float(placar["habilidade_oficial_vs_reserva"]),
        "max_diferenca_absoluta": max(abs(perda_modelo - placar["perda_modelo"]), abs(perda_reserva - placar["perda_reserva"])),
        "bate_dentro_de_1e-9": bool(max(abs(perda_modelo - placar["perda_modelo"]),
                                        abs(perda_reserva - placar["perda_reserva"])) < 1e-9),
        "fracao_oficial_reserva": float((cf.origem.to_numpy() != "modelo").mean()),
    }
    dados = {"n_treino": int(len(dev)), "eventos_treino": int(y_tr.sum()), "taxa_treino": float(y_tr.mean()),
             "n_cofre": int(len(y)), "eventos_cofre": int(y.sum()), "taxa_cofre": float(y.mean()),
             "evento": "prisão do motorista numa parada da polícia estadual de Rhode Island (parada)",
             "dados_lidos": "replay do H3d em saida/h3_ri/prisoes_ri/diario.parquet (o diario.parquet publicado está fora do Git)",
             "taxas_do_ranking_no_desenvolvimento": taxas.tolist()}
    r = C.juntar("ri", "Prisão do motorista em paradas de trânsito de Rhode Island (Stanford Open Policing, 2005-2015)",
                 "h3/saida/prisoes_ri/resultados.json", dados, y,
                 {"C": p_c, "L": p_l, "R": p_r}, p_c, extra={"conferencia": conferencia})
    ic_of = C.bootstrap({"of": oficial}, y, p_c)["of"]
    r["oficial_h3_informativo"] = {"informativo": True, "fora_do_criterio": True, **C.metricas(oficial, y, p_c),
                                   "res_menos_rel_ic90": ic_of["res_menos_rel"],
                                   "habilidade_ic90": ic_of["habilidade"], "auc_ic90": ic_of["auc"]}
    return r


CASOS = {"indo": caso_indo, "covid": caso_covid, "ri": caso_ri}


def main():
    pedidos = sys.argv[1:] or list(CASOS)
    SAIDA.mkdir(parents=True, exist_ok=True)
    resumo = {}
    for nome in pedidos:
        r = CASOS[nome]()
        (SAIDA / f"{nome}.json").write_text(json.dumps(json_res(r), indent=1, ensure_ascii=False))
        resumo[nome] = {"n_cofre": r.get("n_cofre"), "taxa_cofre": r.get("taxa_cofre"),
                        "vencem": r.get("vencem", []),
                        "disponivel": r.get("disponivel", True),
                        "motivo": r.get("motivo")}
        print(nome, json.dumps(resumo[nome], ensure_ascii=False), flush=True)
    geral = {"casos": resumo,
             "veredito": ("nenhum caminho vence em nenhum caso" if not any(v["vencem"] for v in resumo.values())
                          else "vencem: " + "; ".join(f"{k}:{','.join(v['vencem'])}" for k, v in resumo.items() if v["vencem"])),
             "criterio": "RES > REL com IC90 de (RES - REL) acima de zero E habilidade de Brier >= +0,05 (DECLARACAO.md)"}
    (SAIDA / "resumo.json").write_text(json.dumps(json_res(geral), indent=1, ensure_ascii=False))
    print(json.dumps(geral["veredito"], ensure_ascii=False))


if __name__ == "__main__":
    main()
