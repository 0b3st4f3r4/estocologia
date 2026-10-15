"""Ferramentas numéricas compartilhadas pelos agentes. Nada aqui é caixa-preta: cada função
implementa o procedimento citado nos documentos (Vehtari et al. 2021 para R-hat/ESS, Talts et al.
2018 para SBC, Vovk/Lei et al. para predição conforme, Hill 1975, SHELF v4, Dwork 2006)."""

from __future__ import annotations

import numpy as np
from scipy import optimize, special, stats


# ---------- MCMC (F4) ----------
def metropolis(logpost, x0, n: int, passo, rng) -> np.ndarray:
    """Metropolis de passeio aleatório com proposta gaussiana diagonal, em escala irrestrita.
    Devolve n × d."""
    x = np.atleast_1d(np.asarray(x0, dtype=float))
    passo = np.broadcast_to(np.asarray(passo, dtype=float), x.shape)
    lp = logpost(x)
    out = np.empty((n, x.size))
    for i in range(n):
        y = x + passo * rng.standard_normal(x.size)
        ly = logpost(y)
        if np.log(rng.random()) < ly - lp:
            x, lp = y, ly
        out[i] = x
    return out


def _z_rank(c: np.ndarray) -> np.ndarray:
    r = stats.rankdata(c.ravel()).reshape(c.shape)
    return stats.norm.ppf((r - 0.375) / (c.size + 0.25))


def _rhat_basico(c: np.ndarray) -> float:
    m, n = c.shape
    B = n * c.mean(1).var(ddof=1)
    W = c.var(1, ddof=1).mean()
    return float(np.sqrt(((n - 1) / n * W + B / n) / W))


def rhat(cadeias: np.ndarray) -> float:
    """R-hat dividido, normalizado por postos e dobrado (Vehtari et al. 2021). cadeias: m × n."""
    n = cadeias.shape[1] // 2
    c = np.concatenate([cadeias[:, :n], cadeias[:, n:2 * n]])
    dobrada = np.abs(c - np.median(c))
    return max(_rhat_basico(_z_rank(c)), _rhat_basico(_z_rank(dobrada)))


def ess(cadeias: np.ndarray) -> float:
    """ESS bulk (sobre postos normalizados), com autocorrelação somada até o primeiro par negativo (Geyer)."""
    c = _z_rank(cadeias)
    m, n = c.shape
    c = c - c.mean(1, keepdims=True)
    acf = np.mean([np.correlate(x, x, "full")[n - 1:] / (x.var() * n) for x in c], axis=0)
    s, t = 0.0, 1
    while t + 1 < n:
        par = acf[t] + acf[t + 1]
        if par < 0:
            break
        s += par
        t += 2
    return float(m * n / (1 + 2 * s))


def sbc(amostra_priori, simula_dados, posterior_amostras, rng, n_sim=200, L=99) -> dict:
    """Simulation-Based Calibration: postos de θ verdadeiro entre L sorteios da posterior devem ser uniformes."""
    postos = np.empty(n_sim, dtype=int)
    for i in range(n_sim):
        th = amostra_priori(rng)
        dados = simula_dados(th, rng)
        post = posterior_amostras(dados, rng, L)
        postos[i] = int(np.sum(post < th))
    caixas = np.bincount(postos * 10 // (L + 1), minlength=10)
    chi2, p = stats.chisquare(caixas)
    return {"n_sim": n_sim, "L": L, "caixas": caixas.tolist(), "chi2": float(chi2), "p_uniformidade": float(p)}


# ---------- Predição conforme (F4) ----------
def conforme_intervalo(calibracao: np.ndarray, alpha: float) -> tuple[float, float]:
    """Intervalo conforme dividido para um escore escalar trocável: quantis empíricos corrigidos."""
    n = calibracao.size
    k_lo = int(np.floor((n + 1) * alpha / 2))
    k_hi = int(np.ceil((n + 1) * (1 - alpha / 2)))
    s = np.sort(calibracao)
    return float(s[max(k_lo - 1, 0)]), float(s[min(k_hi - 1, n - 1)])


# ---------- Deriva (F3) ----------
def deriva(ref: np.ndarray, novo: np.ndarray) -> dict:
    ks = stats.ks_2samp(ref, novo)
    return {"ks": float(ks.statistic), "p_ks": float(ks.pvalue), "wasserstein": float(stats.wasserstein_distance(ref, novo))}


# ---------- Caudas (Escalonomia) ----------
def hill(x: np.ndarray, k: int) -> float:
    """Estimador de Hill do expoente de cauda α com as k maiores observações (positivas)."""
    s = np.sort(x[x > 0])[::-1]
    k = min(k, s.size - 1)
    return float(1 / np.mean(np.log(s[:k] / s[k])))


# ---------- SHELF (F7) ----------
def ajustar_beta_quartis(q25: float, q50: float, q75: float) -> dict:
    """Ajusta Beta(a, b) aos três quartis elicitados (mínimos quadrados nos quantis, como o SHELF faz)."""
    alvo = np.array([q25, q50, q75])

    def perda(lab):
        a, b = np.exp(lab)
        return np.sum((stats.beta.ppf([0.25, 0.5, 0.75], a, b) - alvo) ** 2)

    r = optimize.minimize(perda, x0=np.log([10.0, 10.0]), method="Nelder-Mead", options={"xatol": 1e-10, "fatol": 1e-14, "maxiter": 5000})
    a, b = np.exp(r.x)
    ajust = stats.beta.ppf([0.05, 0.25, 0.5, 0.75, 0.95], a, b)
    return {"a": float(a), "b": float(b), "quantis_ajustados": {"q05": ajust[0], "q25": ajust[1], "q50": ajust[2], "q75": ajust[3], "q95": ajust[4]},
            "erro_max_quartis": float(np.max(np.abs(ajust[1:4] - alvo)))}


# ---------- Privacidade diferencial (F5) ----------
def laplace_proporcao(k: int, n: int, eps: float, rng) -> float:
    """Libera k/n com ruído de Laplace; sensibilidade 1/n."""
    return float(k / n + rng.laplace(0, 1 / (n * eps)))


def bits_perdidos_laplace(n: int, p: float, eps: float, rng, m=200_000) -> dict:
    """Informação mútua I(K; K̃) entre a contagem verdadeira K ~ Bin(n, p) e a liberação com Laplace,
    comparada com H(K) (o que a liberação exata transmitiria). Estimativa por Monte Carlo com
    densidade exata da mistura."""
    ks = np.arange(n + 1)
    pk = stats.binom.pmf(ks, n, p)
    suporte = pk > 1e-12
    ks, pk = ks[suporte], pk[suporte] / pk[suporte].sum()
    b = 1 / eps
    K = rng.choice(ks, size=m, p=pk)
    z = K + rng.laplace(0, b, m)
    dens = lambda zz, k: np.exp(-np.abs(zz - k) / b) / (2 * b)
    mistura = np.zeros(m)
    for k, w in zip(ks, pk):
        mistura += w * dens(z, k)
    I = float(np.mean(np.log2(dens(z, K) / mistura)))
    H = float(-(pk * np.log2(pk)).sum())
    return {"I_bits": I, "H_bits": H, "bits_perdidos": H - I}


def k_anonimato(linhas: list[dict], quase_ids: list[str]) -> int:
    from collections import Counter
    c = Counter(tuple(r[q] for q in quase_ids) for r in linhas)
    return min(c.values()) if c else 0


# ---------- Justiça (F6) ----------
def taxas_por_grupo(y: np.ndarray, yhat: np.ndarray, grupo: np.ndarray, score: np.ndarray | None = None) -> dict:
    """Taxa-base, FPR, FNR e VPP por grupo, com IC de Wilson de 95%."""
    def wilson(k, n):
        if n == 0:
            return [float("nan")] * 3
        p = k / n
        z = 1.96
        c = (p + z * z / (2 * n)) / (1 + z * z / n)
        h = z * np.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / (1 + z * z / n)
        return [float(p), float(c - h), float(c + h)]

    out = {}
    for g in np.unique(grupo):
        s = grupo == g
        yy, hh = y[s], yhat[s]
        out[str(g)] = {
            "n": int(s.sum()),
            "taxa_base": wilson(int(yy.sum()), int(s.sum())),
            "fpr": wilson(int((hh & ~yy).sum()), int((~yy).sum())),
            "fnr": wilson(int((~hh & yy).sum()), int(yy.sum())),
            "vpp": wilson(int((hh & yy).sum()), int(hh.sum())),
        }
    return out


# ---------- Crisonomia: valor da informação ----------
def evpi(amostras_theta: np.ndarray, utilidade, acoes) -> dict:
    """EVPI = E[max_a U(a, θ)] − max_a E[U(a, θ)]: cota superior de qualquer EVSI."""
    U = np.array([[utilidade(a, th) for th in amostras_theta] for a in acoes])
    melhor = int(np.argmax(U.mean(1)))
    return {"evpi": float(U.max(0).mean() - U.mean(1).max()), "acao_atual": acoes[melhor]}


def evsi_beta_bernoulli(a: float, b: float, n_extra: int, utilidade, acoes, rng, m=4_000) -> float:
    """EVSI de n_extra novas observações Bernoulli sob priori Beta(a, b), por pré-posteriori."""
    th_grade = stats.beta.ppf((np.arange(400) + 0.5) / 400, a, b)
    U0 = np.array([[utilidade(ac, th) for th in th_grade] for ac in acoes])
    base = U0.mean(1).max()
    ths = rng.beta(a, b, m)
    ks = rng.binomial(n_extra, ths)
    ganho = 0.0
    cache: dict[int, float] = {}
    for k in ks:
        if k not in cache:
            pg = stats.beta.ppf((np.arange(200) + 0.5) / 200, a + k, b + n_extra - k)
            Uk = np.array([[utilidade(ac, th) for th in pg] for ac in acoes]).mean(1)
            cache[k] = Uk.max()
        ganho += cache[k]
    return float(ganho / m - base)
