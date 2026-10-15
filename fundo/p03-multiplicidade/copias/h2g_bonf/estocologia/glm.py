"""Modelos lineares generalizados mínimos (gaussiano e logístico), sem caixa-preta.

Servem ao ICP (muitos ajustes rápidos), ao nulo de permutação e ao ponto de partida do MCMC.
O erro padrão por agrupamento (sanduíche de Liang–Zeger) é o que respeita a unidade independente.
"""

from __future__ import annotations

import numpy as np
from scipy import special

FAMILIAS = ("gaussiana", "binomial")


def ajustar(X: np.ndarray, y: np.ndarray, familia: str, ridge: float = 1e-6, iter_max: int = 60) -> dict:
    """Ajusta y ~ X (X já inclui o intercepto). Devolve beta, cov (modelo), log-verossimilhança e μ.

    Pré: X n × p com n > p. Para a binomial, IRLS com um ridge minúsculo contra separação perfeita.
    """
    n, p = X.shape
    if familia == "gaussiana":
        beta, *_ = np.linalg.lstsq(X, y, rcond=None)
        mu = X @ beta
        rss = float(np.sum((y - mu) ** 2))
        s2 = rss / max(n - p, 1)
        cov = s2 * np.linalg.pinv(X.T @ X)
        ll = -0.5 * n * (np.log(2 * np.pi * max(rss, 1e-300) / n) + 1)
        return {"beta": beta, "cov": cov, "ll": float(ll), "mu": mu, "rss": rss, "n": n, "p": p}
    if familia != "binomial":
        raise ValueError(familia)
    beta = np.zeros(p)
    I = ridge * np.eye(p)
    for _ in range(iter_max):
        eta = X @ beta
        mu = special.expit(eta)
        w = np.clip(mu * (1 - mu), 1e-10, None)
        H = X.T @ (X * w[:, None]) + I
        g = X.T @ (y - mu) - ridge * beta
        passo = np.linalg.solve(H, g)
        beta = beta + passo
        if np.max(np.abs(passo)) < 1e-9:
            break
    eta = X @ beta
    mu = special.expit(eta)
    w = np.clip(mu * (1 - mu), 1e-10, None)
    cov = np.linalg.inv(X.T @ (X * w[:, None]) + I)
    ll = float(np.sum(y * eta - np.logaddexp(0, eta)))
    return {"beta": beta, "cov": cov, "ll": ll, "mu": mu, "n": n, "p": p}


def cov_agrupada(X: np.ndarray, y: np.ndarray, ajuste: dict, grupos: np.ndarray, familia: str) -> np.ndarray:
    """Covariância sanduíche por agrupamento: o pão é o inverso da informação, o recheio soma escores por grupo."""
    r = y - ajuste["mu"]
    if familia == "gaussiana":
        pao = np.linalg.pinv(X.T @ X)
    else:
        pao = ajuste["cov"]
    _, g = np.unique(grupos, return_inverse=True)
    S = np.zeros((g.max() + 1, X.shape[1]))
    np.add.at(S, g, X * r[:, None])
    G = S.shape[0]
    corr = G / max(G - 1, 1)
    return corr * pao @ (S.T @ S) @ pao


def logverossimilhanca(beta: np.ndarray, X: np.ndarray, y: np.ndarray, familia: str, log_sigma: float = 0.0) -> float:
    eta = X @ beta
    if familia == "binomial":
        return float(np.sum(y * eta - np.logaddexp(0, eta)))
    s = np.exp(log_sigma)
    return float(-y.size * (log_sigma + 0.5 * np.log(2 * np.pi)) - 0.5 * np.sum((y - eta) ** 2) / s**2)
