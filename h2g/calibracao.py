"""Calibração do teste de invariância K = 2 em simulação (antes dos dados reais; ver ../h2b/DECLARACAO_K2.md).

Cada cenário gera (X expandido K = 2, X linear K = 1, y, ambiente). Sob H0 a lei de Y dado X é a mesma em todos
os ambientes; os ambientes mudam só a distribuição de X. Sob H1 o ambiente desloca Y | X (locação ou escala).
A taxa de rejeição a 5% é medida para o teste K = 2 e, para comparação, para o K = 1.

Uso:  python3 calibracao.py [sementes]      (grava saida/calibracao.json)
"""

from __future__ import annotations

import json
import os
import sys
from pathlib import Path

import numpy as np

from estocologia.icp import expandir, p_invariancia, p_invariancia_np

RAIZ = Path(__file__).resolve().parent
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))
F_DISCRETO = np.array([0.0, 1.0, 1.3, 1.4])          # resposta côncava em X discreto (como a coparticipação)
P_X = np.array([[0.55, 0.25, 0.15, 0.05], [0.25, 0.25, 0.25, 0.25], [0.05, 0.15, 0.25, 0.55]])


def _ruido(rng, tipo, n):
    if tipo == "t3":
        return rng.standard_t(3, n)
    if tipo == "lognormal":
        z = rng.lognormal(0, 1, n)
        return z - np.exp(0.5)
    raise ValueError(tipo)


def gerar(cenario: str, rng, n: int = 4500):
    """Devolve (y, ambiente, x). x é o candidato bruto (uma coluna)."""
    e = rng.integers(0, 3, n)
    if cenario.startswith("cont"):
        x = rng.normal(e - 1.0, 1.0)
        f = np.tanh(1.5 * x) + 0.2 * x
    else:
        u = rng.random(n)
        x = (u[:, None] > np.cumsum(P_X[e], 1)).sum(1).astype(float)
        f = F_DISCRETO[x.astype(int)]
    if cenario in ("discreto_t3", "cont_t3"):
        y = f + _ruido(rng, "t3", n)
    elif cenario == "discreto_lognormal":
        y = f + _ruido(rng, "lognormal", n)
    elif cenario == "discreto_hetero_t3":
        y = f + (1 + 0.6 * x) * _ruido(rng, "t3", n)
    elif cenario == "discreto_massa_zero":
        # forma (não só locação/escala) dependendo de X: P(y = 0) cai com x; positivos log-normais
        zero = rng.random(n) < np.array([0.45, 0.30, 0.22, 0.18])[x.astype(int)]
        y = np.where(zero, 0.0, rng.normal(5 + 0.3 * x, 1.4, n))
    elif cenario == "H1_locacao":
        y = f + np.array([0.0, 0.15, -0.10])[e] + _ruido(rng, "t3", n)
    elif cenario == "H1_escala":
        y = f + np.array([1.0, 1.25, 1.0])[e] * _ruido(rng, "t3", n)
    else:
        raise ValueError(cenario)
    return y, e, x


CENARIOS_H0 = ["discreto_t3", "discreto_lognormal", "discreto_hetero_t3", "cont_t3", "discreto_massa_zero"]
CENARIOS_H1 = ["H1_locacao", "H1_escala"]


def taxas(cenario: str, sementes: int = 200, alpha: float = 0.05, n: int = 4500) -> dict:
    rej2, rej1 = 0, 0
    for s in range(sementes):
        rng = np.random.default_rng(10_000 + s)
        y, e, x = gerar(cenario, rng, n)
        Xe = np.hstack([np.ones((n, 1)), expandir(x[:, None])[0]])
        Xl = np.column_stack([np.ones(n), x])
        rej2 += p_invariancia_np(Xe, y, e) < alpha
        rej1 += p_invariancia(Xl, y, e, "gaussiana") < alpha
    return {"cenario": cenario, "sementes": sementes, "n": n, "rejeicao_K2": rej2 / sementes, "rejeicao_K1": rej1 / sementes}


if __name__ == "__main__":
    m = int(sys.argv[1]) if len(sys.argv) > 1 else 200
    out = {"H0": [taxas(c, m) for c in CENARIOS_H0], "H1": [taxas(c, m) for c in CENARIOS_H1],
           # n do porte do RAND (≈ 15 mil linhas de treino): onde a forma depende de X, o K = 2 também passa do α
           "H0_n_grande": [taxas(c, m, n=15_000) for c in ("discreto_t3", "discreto_hetero_t3", "discreto_massa_zero")]}
    SAIDA.mkdir(parents=True, exist_ok=True)
    (SAIDA / "calibracao.json").write_text(json.dumps(out, indent=1, ensure_ascii=False))
    for k, v in out.items():
        for r in v:
            print(k, json.dumps(r, ensure_ascii=False))
