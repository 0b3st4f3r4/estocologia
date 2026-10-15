"""Metropolis adaptativo (Haario, Saksman & Tamminen 2001) — o amostrador do H2.

No H1, o passeio aleatório de passo fixo falhava no R-hat em ~40% das sementes com d ≥ 5. Aqui:
1. o ponto de partida é o MAP (BFGS) e a proposta inicial é a inversa da hessiana;
2. no aquecimento, a covariância da proposta aprende com a própria cadeia (2,38²/d · Σ̂) e a escala
   segue Robbins–Monro rumo à aceitação de 0,234;
3. ao fim do aquecimento, a proposta CONGELA: a amostragem que conta é Metropolis homogêneo,
   portanto válido (a adaptação não contamina as amostras guardadas);
4. metade dos passos é independente (Metropolis–Hastings com proposta t₅ centrada no MAP, escala da
   hessiana inflada): em GLMs com n grande a posterior é quase gaussiana e esse núcleo quase não
   rejeita. A mistura de dois núcleos válidos com a mesma estacionária é válida.
"""

from __future__ import annotations

import numpy as np
from scipy import optimize

from estocologia import estat


def hessiana(f, x: np.ndarray) -> np.ndarray:
    """Hessiana por diferenças centrais (a hess_inv do BFGS é aproximação grosseira demais para a proposta)."""
    d = x.size
    h = 1e-4 * (1 + np.abs(x))
    H = np.empty((d, d))
    f0 = f(x)
    for i in range(d):
        for j in range(i, d):
            if i == j:
                e = np.zeros(d)
                e[i] = h[i]
                H[i, i] = (f(x + e) - 2 * f0 + f(x - e)) / h[i] ** 2
            else:
                ei, ej = np.zeros(d), np.zeros(d)
                ei[i], ej[j] = h[i], h[j]
                H[i, j] = H[j, i] = (f(x + ei + ej) - f(x + ei - ej) - f(x - ei + ej) + f(x - ei - ej)) / (4 * h[i] * h[j])
    return H


def map_e_hessiana(logpost, x0: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    """MAP por BFGS e a covariância de Laplace (inversa da hessiana de −log posterior no MAP)."""
    r = optimize.minimize(lambda x: -logpost(x), x0, method="BFGS", options={"gtol": 1e-7, "maxiter": 5000})
    Hn = hessiana(lambda x: -logpost(x), r.x)
    w, V = np.linalg.eigh(0.5 * (Hn + Hn.T))
    w = np.clip(w, 1e-8 * max(w.max(), 1e-8), None)   # garante positividade
    return r.x, (V / w) @ V.T


NU = 5.0          # graus de liberdade da proposta independente (caudas mais pesadas que a posterior)
INFLA = 1.5       # inflação da escala da hessiana na proposta independente


def _logt(x, m, Linv, d):
    z = Linv @ (x - m)
    return -0.5 * (NU + d) * np.log1p(z @ z / NU)


def cadeia(logpost, x0, cov0, n: int, aquec: int, rng, centro=None) -> tuple[np.ndarray, float]:
    d = x0.size
    L = np.linalg.cholesky(cov0)
    centro = x0 if centro is None else centro
    Lt = np.linalg.cholesky(INFLA * cov0)
    Ltinv = np.linalg.inv(Lt)
    escala = 2.38 / np.sqrt(d)
    x, lp = x0.copy(), logpost(x0)
    hist = np.empty((aquec, d))
    out = np.empty((n, d))
    aceit = 0
    for i in range(aquec + n):
        if rng.random() < 0.5:   # passo independente
            z = rng.standard_normal(d) * np.sqrt(NU / rng.chisquare(NU))
            y = centro + Lt @ z
            ly = logpost(y)
            a = np.isfinite(ly) and np.log(rng.random()) < (ly - lp) - (_logt(y, centro, Ltinv, d) - _logt(x, centro, Ltinv, d))
            if a:
                x, lp = y, ly
            if i >= aquec:
                out[i - aquec] = x
                aceit += a
            else:
                hist[i] = x
            continue
        y = x + escala * (L @ rng.standard_normal(d))
        ly = logpost(y)
        a = np.isfinite(ly) and np.log(rng.random()) < ly - lp
        if a:
            x, lp = y, ly
        if i < aquec:
            hist[i] = x
            escala *= np.exp((float(a) - 0.234) / np.sqrt(i + 1))
            if i >= 200 and i % 50 == 0:
                C = np.cov(hist[i // 2:i + 1].T).reshape(d, d) + 1e-10 * np.eye(d)
                try:
                    L = np.linalg.cholesky(C)
                except np.linalg.LinAlgError:
                    pass
        else:
            out[i - aquec] = x
            aceit += a
    return out, aceit / n


def amostrar(logpost, x0, rng, cadeias: int = 4, n: int = 2000, aquec: int = 2000) -> dict:
    """Devolve amostras (cadeias × n × d) e diagnósticos por coordenada (R-hat dividido, ESS bulk)."""
    xm, H = map_e_hessiana(logpost, np.asarray(x0, float))
    ch, ac = [], []
    for _ in range(cadeias):
        ini = xm + rng.multivariate_normal(np.zeros(xm.size), 2 * H)   # dispersas em torno do MAP
        a, t = cadeia(logpost, ini, H, n, aquec, rng, centro=xm)
        ch.append(a)
        ac.append(t)
    A = np.stack(ch)
    rh = [estat.rhat(A[:, :, j]) for j in range(A.shape[2])]
    es = [estat.ess(A[:, :, j]) for j in range(A.shape[2])]
    return {"amostras": A, "map": xm, "aceitacao": float(np.mean(ac)), "rhat_max": float(max(rh)), "ess_min": float(min(es)),
            "rhat": rh, "ess": es}
