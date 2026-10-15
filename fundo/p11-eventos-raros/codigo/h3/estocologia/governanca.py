"""F5 (privacidade) e F6 (justiça) em operação contínua, sobre o fluxo de prisões.

F5. A operação publica, ao fim de cada ano, a taxa de liberação por cor da pele, e republica o
acumulado de todos os anos até ali. Assim cada pessoa entra em todas as publicações seguintes, e o ε
dela se soma (composição sequencial). Cada publicação usa ε = 0,5 por célula, com ruído de Laplace
nas contagens (sensibilidade 1). O orçamento por pessoa é 2,0.

O disjuntor de privacidade recusa a parte de uma publicação que levaria alguém acima do orçamento.
Na prática, as coortes antigas saem do acumulado e a publicação passa a ser só das coortes que
ainda têm saldo. Os bits perdidos por célula são I(K; K̃) contra H(K) (estat.bits_perdidos_laplace).

F6. Trilema da justiça (Kleinberg, Mullainathan & Raghavan 2016; Chouldechova 2017). Quando as
taxas-base dos grupos diferem e o previsor não é perfeito, três coisas não valem juntas:
- a calibração dentro de cada grupo;
- o equilíbrio da classe positiva (FNR iguais);
- o equilíbrio da classe negativa (FPR iguais).

O F6 mede as três por ano, com a decisão do contrato (marcar para revisão se p < 0,70). O desfecho
de interesse é não ser liberado (y = 0).
"""

from __future__ import annotations

from collections import Counter

import numpy as np
import pandas as pd

from .estat import bits_perdidos_laplace, taxas_por_grupo

EPS_POR_PUBLICACAO = 0.5
ORCAMENTO_EPS = 2.0


def k_anonimato(df: pd.DataFrame, quase: list[str]) -> dict:
    c = Counter(map(tuple, df[quase].astype(str).to_numpy()))
    tam = np.array(list(c.values()))
    d2 = df.assign(age=(df["age"] // 5 * 5).astype(int))
    c2 = Counter(map(tuple, d2[quase].astype(str).to_numpy()))
    tam2 = np.array(list(c2.values()))
    return {"k": int(tam.min()), "fracao_unicos": float((tam == 1).sum() / len(df)),
            "k_idade_em_faixas_de_5": int(tam2.min()), "fracao_unicos_faixas": float((tam2 == 1).sum() / len(df))}


def f5_publicacoes(det: pd.DataFrame, y: pd.Series, rng) -> dict:
    """det: detenções com year e colour; y: liberação (0/1) alinhada. Simula as publicações anuais."""
    gasto = Counter()                      # ε gasto por coorte (ano)
    pubs, recusas = [], []
    anos = sorted(det["year"].unique())
    for ano in anos:
        candidatos = [a for a in anos if a <= ano]
        aceitas = [a for a in candidatos if gasto[a] + EPS_POR_PUBLICACAO <= ORCAMENTO_EPS + 1e-12]
        recusadas = [a for a in candidatos if a not in aceitas]
        if recusadas:
            recusas.append({"publicacao": int(ano), "coortes_recusadas": [int(a) for a in recusadas],
                            "motivo": f"orçamento ε = {ORCAMENTO_EPS} por pessoa esgotado"})
        for a in aceitas:
            gasto[a] += EPS_POR_PUBLICACAO
        sel = det["year"].isin(aceitas)
        cel = []
        for cor, g in det[sel].groupby("colour"):
            k, n = int(y[g.index].sum()), len(g)
            k_ruid = k + rng.laplace(0, 1 / EPS_POR_PUBLICACAO)
            n_ruid = n + rng.laplace(0, 1 / EPS_POR_PUBLICACAO)
            bits = bits_perdidos_laplace(min(n, 400), k / n, EPS_POR_PUBLICACAO, rng, m=40_000)
            cel.append({"cor": cor, "n": n, "taxa_verdadeira": k / n, "taxa_publicada": float(k_ruid / n_ruid),
                        "erro_absoluto": float(abs(k_ruid / n_ruid - k / n)), "bits_perdidos": bits["bits_perdidos"],
                        "H_bits": bits["H_bits"]})
        pubs.append({"publicacao": int(ano), "coortes": [int(a) for a in aceitas], "celulas": cel})
    return {"eps_por_publicacao": EPS_POR_PUBLICACAO, "orcamento_eps": ORCAMENTO_EPS,
            "eps_gasto_por_coorte": {int(k): v for k, v in gasto.items()},
            "orcamento_respeitado": all(v <= ORCAMENTO_EPS + 1e-12 for v in gasto.values()),
            "publicacoes": pubs, "recusas_do_disjuntor_de_privacidade": recusas}


def f6_trilema(diario: pd.DataFrame, limiar: float = 0.70, tol: float = 0.05) -> dict:
    """diario: previsões oficiais resolvidas de prisões, com 'grupo', 'ano', 'p' e 'y'."""
    out = {}
    for ano, g in diario.groupby("ano"):
        g = g[g["grupo"].isin(["Black", "White"])]
        y0 = (g["y"] == 0).to_numpy()                     # desfecho de interesse: não liberado
        flag = (g["p"] < limiar).to_numpy()
        t = taxas_por_grupo(y0, flag, g["grupo"].to_numpy())
        cal = {grp: {"p_nao_liberacao_media": float(1 - gg["p"].mean()), "taxa_nao_liberacao": float(1 - gg["y"].mean())}
               for grp, gg in g.groupby("grupo")}
        dif_cal = max(abs(v["p_nao_liberacao_media"] - v["taxa_nao_liberacao"]) for v in cal.values())
        fpr = {k: v["fpr"][0] for k, v in t.items()}
        fnr = {k: v["fnr"][0] for k, v in t.items()}
        base = {k: v["taxa_base"][0] for k, v in t.items()}
        crit = {"calibracao_dentro_dos_grupos": dif_cal <= tol,
                "equilibrio_classe_negativa_FPR": abs(fpr["Black"] - fpr["White"]) <= tol,
                "equilibrio_classe_positiva_FNR": abs(fnr["Black"] - fnr["White"]) <= tol}
        out[int(ano)] = {"n": int(len(g)), "taxa_base_nao_liberacao": base, "calibracao": cal, "fpr": fpr, "fnr": fnr,
                         "criterios_satisfeitos": crit, "quantos_valem": int(sum(crit.values())),
                         "taxas_base_diferem": abs(base["Black"] - base["White"]) > tol,
                         "detalhe_wilson": t}
    return out
