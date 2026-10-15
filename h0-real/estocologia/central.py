"""H0 em domínio real: a central de chamadas de um banco (agent-f1-code + agent-f2-data).

O Lab 0 dizia que uma fila dimensionada pela taxa média falha porque a demanda varia de um dia
para o outro e W(λ) é convexa. Aqui a demanda não é mais sorteada: são as chegadas reais, a cada
5 minutos, de 164 dias úteis de 2003 na central de um grande banco norte-americano
(dados-reais/bank_calls.csv). O que continua suposto, e declarado como tal, é o tempo de
atendimento (a base não o traz) e a ausência de abandono.

A alegação (L2) é a prática padrão de dimensionamento: para cada meia hora, o Erlang C sobre a
taxa média do perfil histórico, com a meta de 80% das chamadas atendidas em até 20 s.

Unidades: tempo em segundos a partir das 07:00; taxas em chamadas por segundo, salvo onde dito.
"""

from __future__ import annotations

import datetime as dt
import os
from pathlib import Path

import numpy as np
from numba import njit

SEMENTE = 2003
BIN = 300.0                    # a base tem um intervalo a cada 5 min
POR_GRUPO = 6                  # escala por meia hora (6 intervalos de 5 min)
AHT = 240.0                    # tempo médio de atendimento suposto: 4 min (F7; não está na base)
PACIENCIA = 180.0             # paciência média suposta: 3 min, exponencial (F7; não está na base)
META_SL, META_T = 0.80, 20.0   # 80% atendidas em até 20 s
CORTE = dt.date(2003, 7, 1)    # treino: mar–jun; cofre fora do tempo: jul–out
DADOS = Path(os.environ.get("ESTOCOLOGIA_DADOS", Path(__file__).resolve().parents[2] / "dados-reais")) / "bank_calls.csv"


# ---------------- dados (agent-f2-data) ----------------
def carregar(caminho: Path = DADOS) -> tuple[np.ndarray, np.ndarray]:
    """Devolve (datas, contagens dia × intervalo). Pós: 169 intervalos por dia, nenhum faltando."""
    import pandas as pd

    d = pd.read_csv(caminho, parse_dates=["DateTime"])
    d["dia"] = d.DateTime.dt.date
    d["i"] = ((d.DateTime.dt.hour - 7) * 60 + d.DateTime.dt.minute) // 5
    m = d.pivot(index="dia", columns="i", values="Calls")
    if m.isna().any().any() or m.shape[1] != 169:
        raise ValueError("base de chamadas incompleta")
    return np.array(m.index), m.to_numpy(dtype=np.int64)


def grupos(n_bins: int) -> np.ndarray:
    """Índice do intervalo de escala (meia hora) de cada intervalo de 5 min."""
    return np.arange(n_bins) // POR_GRUPO


# ---------------- a fórmula da alegação (L2) ----------------
def erlang_c(c: int, a: float) -> float:
    """Probabilidade de esperar na M/M/c com carga a = λ·AHT (Erlangs). Pré: c > a ≥ 0.
    Pela recursão estável de Erlang B: B_k = a B_{k−1} / (k + a B_{k−1})."""
    if not (c > a >= 0):
        raise ValueError(f"fila instável: c={c}, a={a}")
    b = 1.0
    for k in range(1, c + 1):
        b = a * b / (k + a * b)
    return c * b / (c - a * (1 - b))


def nivel_servico(c: int, lam: float, aht: float = AHT, t: float = META_T) -> float:
    """P(espera ≤ t) estacionária na M/M/c."""
    a = lam * aht
    if c <= a:
        return 0.0
    return 1 - erlang_c(c, a) * np.exp(-(c - a) * t / aht)


def espera_media_erlang(c: int, lam: float, aht: float = AHT) -> float:
    a = lam * aht
    return erlang_c(c, a) * aht / (c - a)


def escala(lam: float, aht: float = AHT, meta: float = META_SL, t: float = META_T) -> int:
    """Menor número de atendentes que cumpre a meta pela fórmula. Pós: nivel_servico ≥ meta."""
    if lam <= 0:
        return 1
    c = int(np.floor(lam * aht)) + 1
    while nivel_servico(c, lam, aht, t) < meta:
        c += 1
    return c


def escala_do_perfil(perfil_bin: np.ndarray, aht: float = AHT, fator: float = 1.0) -> np.ndarray:
    """Atendentes por meia hora a partir de um perfil de chegadas esperadas por intervalo de 5 min."""
    g = grupos(perfil_bin.size)
    lam = np.bincount(g, weights=perfil_bin) / (np.bincount(g) * BIN)
    return np.array([escala(fator * x, aht) for x in lam], dtype=np.int64)


# ---------------- o mundo (L1): fila FCFS com c(t) atendentes ----------------
@njit(cache=True)
def _fcfs(chegadas, servico, paciencia, c, dur):
    """Espera de cada chamada numa fila FCFS com c[j] atendentes no intervalo j (de duração dur).

    Quem teria de esperar mais que a própria paciência desiste: espera a paciência e não ocupa
    ninguém (exato em FCFS, porque a espera oferecida não depende de quem chega depois).

    Um atendente k está em turno no intervalo j se k < c[j]; quem sai de turno termina a chamada
    em curso (sem preempção). Depois do último intervalo vale a escala do último (a fila é esvaziada).
    """
    G = c.size
    K = c.max()
    nxt = np.full((K, G + 1), -1, dtype=np.int64)
    for k in range(K):
        prox = -1
        for j in range(G - 1, -1, -1):
            if k < c[j]:
                prox = j
            nxt[k, j] = prox
    livre = np.zeros(K)
    espera = np.empty(chegadas.size)
    desistiu = np.zeros(chegadas.size, dtype=np.bool_)
    for i in range(chegadas.size):
        a = chegadas[i]
        melhor, kb = np.inf, -1
        for k in range(K):
            t0 = livre[k] if livre[k] > a else a
            j = int(t0 // dur)
            if j >= G:
                j = G - 1
                v = t0 if k < c[j] else np.inf
            elif k < c[j]:
                v = t0
            else:
                jj = nxt[k, j]
                v = jj * dur if jj >= 0 else np.inf
            if v < melhor:
                melhor, kb = v, k
        if melhor - a > paciencia[i]:
            espera[i] = paciencia[i]
            desistiu[i] = True
        else:
            espera[i] = melhor - a
            livre[kb] = melhor + servico[i]
    return espera, desistiu


def chegadas_do_dia(contagens: np.ndarray, rng) -> np.ndarray:
    """Instantes de chegada (s) espalhados uniformemente dentro de cada intervalo de 5 min."""
    n = int(contagens.sum())
    base = np.repeat(np.arange(contagens.size) * BIN, contagens)
    return np.sort(base + rng.uniform(0, BIN, n))


def servico(n: int, rng, aht: float = AHT, dist: str = "exp") -> np.ndarray:
    """Tempo de atendimento suposto: exponencial (a da fórmula) ou lognormal com CV 1."""
    if dist == "exp":
        return rng.exponential(aht, n)
    s2 = np.log(2.0)  # CV = 1
    return rng.lognormal(np.log(aht) - s2 / 2, np.sqrt(s2), n)


def dia(contagens: np.ndarray, c: np.ndarray, rng, aht: float = AHT, dist: str = "exp",
        paciencia: float = np.inf) -> tuple[np.ndarray, np.ndarray]:
    """Esperas (s) e desistências de todas as chamadas de um dia, com a escala c por meia hora.
    paciencia é a média de uma paciência exponencial (inf: ninguém desiste, a cota pessimista)."""
    t = chegadas_do_dia(contagens, rng)
    s = servico(t.size, rng, aht, dist)
    pac = np.full(t.size, np.inf) if np.isinf(paciencia) else rng.exponential(paciencia, t.size)
    return _fcfs(t, s, pac, c.astype(np.int64), BIN * POR_GRUPO)


# ---------------- estatística na unidade independente (o dia) ----------------
def media_por_chamada(e: np.ndarray, d: np.ndarray) -> tuple[float, float]:
    """Média por chamada e erro padrão com o dia como unidade (razão de somas)."""
    soma, n = np.bincount(d, weights=e), np.bincount(d)
    m = soma.sum() / n.sum()
    return float(m), float(np.sqrt(np.sum((soma - m * n) ** 2)) / n.sum())


def erro_ingenuo(e: np.ndarray) -> float:
    """Erro padrão tratando cada chamada como independente (a pseudorreplicação que Φ2 barra)."""
    return float(e.std(ddof=1) / np.sqrt(e.size))


def dispersao(totais: np.ndarray) -> float:
    """Índice de dispersão Var/média dos totais diários: 1 sob Poisson com taxa fixa."""
    return float(totais.var(ddof=1) / totais.mean())


def conforme(calib: np.ndarray, alpha: float) -> tuple[float, float]:
    """Intervalo conforme dividido (quantis empíricos corrigidos) para um escore escalar."""
    n = calib.size
    s = np.sort(calib)
    lo = int(np.floor((n + 1) * alpha / 2))
    hi = int(np.ceil((n + 1) * (1 - alpha / 2)))
    return float(s[max(lo - 1, 0)]), float(s[min(hi - 1, n - 1)])
