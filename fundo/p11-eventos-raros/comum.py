"""Métricas comuns do fundo/p11 (eventos raros).

A decomposição de Murphy é a MESMA função do H3 (`h3/estocologia/calibracao.py`), carregada por caminho do
`codigo/h3/` copiado byte a byte — 10 caixas de massa igual. A AUC é Mann–Whitney com posto médio nos empates.
O bootstrap reamostra as linhas do cofre com reposição (semente 2027), refazendo as caixas dentro de cada
reamostragem e recalculando a climatologia do cofre dentro dela.
"""

from __future__ import annotations

import importlib.util
from pathlib import Path

import numpy as np
from scipy import stats

RAIZ = Path(__file__).resolve().parent
CAIXAS = 10
B_BOOT = 2000
SEMENTE = 2027


def _carregar(nome: str, caminho: Path):
    spec = importlib.util.spec_from_file_location(nome, caminho)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


H3_CALIBRACAO = _carregar("h3_calibracao", RAIZ / "codigo" / "h3" / "estocologia" / "calibracao.py")


def murphy(p, y, caixas: int = CAIXAS) -> dict:
    """A do H3: 10 caixas de massa igual."""
    return H3_CALIBRACAO.murphy(np.asarray(p, float), np.asarray(y, float), caixas)


def murphy_largura(p, y, caixas: int = CAIXAS) -> dict:
    """A de `fundo/p09-escala-dados/escala/raro_indo.py`: 10 caixas de mesma LARGURA. Só para a conferência."""
    p, y = np.asarray(p, float), np.asarray(y, float)
    b = np.minimum((p * caixas).astype(int), caixas - 1)
    ybar = y.mean()
    rel = res = 0.0
    for k in range(caixas):
        s = b == k
        if s.any():
            rel += s.sum() * (p[s].mean() - y[s].mean()) ** 2
            res += s.sum() * (y[s].mean() - ybar) ** 2
    n = y.size
    return {"confiabilidade": rel / n, "resolucao": res / n, "incerteza": float(ybar * (1 - ybar))}


def auc(p, y) -> float:
    p, y = np.asarray(p, float), np.asarray(y, float)
    n1, n0 = int(y.sum()), int((1 - y).sum())
    if n1 == 0 or n0 == 0:
        return float("nan")
    r = stats.rankdata(p)
    return float((r[y == 1].sum() - n1 * (n1 + 1) / 2) / (n1 * n0))


def decis(p_ordem, p_alvo, y_ordem):
    """Escore de ranking -> probabilidade: as bordas são os decis de massa igual da ordem de `p_ordem`, e cada
    caixa leva a taxa de eventos do desenvolvimento. `np.searchsorted` com side="right" nos dois lados."""
    bordas = np.quantile(np.asarray(p_ordem, float), np.linspace(0.1, 0.9, CAIXAS - 1))
    k_ordem = np.searchsorted(bordas, np.asarray(p_ordem, float), side="right")
    k_alvo = np.searchsorted(bordas, np.asarray(p_alvo, float), side="right")
    taxa = np.full(CAIXAS, float(np.mean(y_ordem)))
    for k in range(CAIXAS):
        s = k_ordem == k
        if s.any():
            taxa[k] = float(np.mean(np.asarray(y_ordem, float)[s]))
    return taxa[k_alvo], taxa, bordas


def caixas_de_massa_igual(p, caixas: int = CAIXAS):
    """A previsão constante dentro de cada caixa (a média da caixa). A identidade de Murphy é exata só para ela."""
    p = np.asarray(p, float)
    medias = np.empty(len(p))
    for c in np.array_split(np.argsort(p, kind="stable"), caixas):
        if len(c):
            medias[c] = p[c].mean()
    return medias


def metricas(p, y, p_c) -> dict:
    p, y, p_c = np.asarray(p, float), np.asarray(y, float), np.asarray(p_c, float)
    m = murphy(p, y)
    p_caixa = caixas_de_massa_igual(p)
    brier_caixa = float(np.mean((p_caixa - y) ** 2))
    brier = float(np.mean((p - y) ** 2))
    brier_c = float(np.mean((p_c - y) ** 2))
    return {"brier": brier, "REL": m["REL"], "RES": m["RES"], "UNC": m["UNC"],
            "identidade_brier_menos_rel_mais_res_menos_unc": float(brier - (m["REL"] - m["RES"] + m["UNC"])),
            "brier_da_previsao_constante_na_caixa": brier_caixa,
            "identidade_com_previsao_constante_na_caixa": float(brier_caixa - (m["REL"] - m["RES"] + m["UNC"])),
            "res_menos_rel": float(m["RES"] - m["REL"]),
            "brier_climatologia": brier_c, "habilidade": float(1 - brier / brier_c) if brier_c > 0 else float("nan"),
            "auc": auc(p, y), "bate_o_UNC": bool(brier < m["UNC"])}


def bootstrap(caminhos: dict, y, p_c, B: int = B_BOOT, semente: int = SEMENTE) -> dict:
    """IC90 de (RES − REL), da habilidade e da AUC por reamostragem do cofre com reposição."""
    y, p_c = np.asarray(y, float), np.asarray(p_c, float)
    n = y.size
    rng = np.random.default_rng(semente)
    acum = {k: {"res_menos_rel": [], "habilidade": [], "auc": []} for k in caminhos}
    for _ in range(B):
        i = rng.integers(0, n, n)
        yy, cc = y[i], p_c[i]
        bc = float(np.mean((cc - yy) ** 2))
        for k, p in caminhos.items():
            pp = np.asarray(p, float)[i]
            m = murphy(pp, yy)
            br = float(np.mean((pp - yy) ** 2))
            acum[k]["res_menos_rel"].append(m["RES"] - m["REL"])
            acum[k]["habilidade"].append(1 - br / bc)
            acum[k]["auc"].append(auc(pp, yy))
    return {k: {q: [float(np.quantile(v, 0.05)), float(np.quantile(v, 0.95))] for q, v in d.items()} for k, d in acum.items()}


def criterio(m: dict) -> dict:
    a = m["res_menos_rel_ic90"][0] > 0
    b = m["habilidade"] >= 0.05
    return {"res_maior_que_rel_com_ic90_acima_de_zero": bool(a), "habilidade_maior_igual_a_0_05": bool(b),
            "vence": bool(a and b), "leitura_estrita_ic90_da_habilidade_acima_de_0_05": bool(m["habilidade_ic90"][0] > 0.05)}


def juntar(nome, titulo, fonte, dados, y, caminhos, p_c, B: int = B_BOOT, extra: dict | None = None) -> dict:
    """Monta o resultado de um caso: métricas por caminho, IC90, critério."""
    y = np.asarray(y, float)
    ic = bootstrap(caminhos, y, p_c, B=B)
    out = {}
    for k, p in caminhos.items():
        m = metricas(p, y, p_c)
        m["res_menos_rel_ic90"] = ic[k]["res_menos_rel"]
        m["habilidade_ic90"] = ic[k]["habilidade"]
        m["auc_ic90"] = ic[k]["auc"]
        m["criterio"] = criterio(m)
        out[k] = m
    r = {"caso": nome, "titulo": titulo, "fonte": fonte, **dados, "caminhos": out,
         "vencem": sorted(k for k, m in out.items() if m["criterio"]["vence"]),
         "bootstrap": {"reamostragens": B, "semente": SEMENTE, "caixas": CAIXAS}, "disponivel": True}
    if extra:
        r.update(extra)
    return r
