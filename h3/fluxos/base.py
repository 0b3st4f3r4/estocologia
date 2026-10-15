"""Base dos fluxos do H3. Um fluxo é o pacote de domínio do F7 para operação contínua: contrato, eventos com
t_disp, agenda de ticks, estado, modelo, reserva determinístico, escopo, sensores e decisão C/L."""

from __future__ import annotations

from datetime import datetime, timezone

import numpy as np
import pandas as pd


def segundos(serie) -> pd.Series:
    """Datas (texto ou datetime) -> segundos UTC, independente da resolução interna do pandas."""
    d = pd.to_datetime(serie, utc=True)
    return ((d - pd.Timestamp("1970-01-01", tz="UTC")) // pd.Timedelta(seconds=1)).astype("int64")


def ts(s: str) -> int:
    return int(pd.Timestamp(s, tz="UTC").timestamp())


def data_utc(t: int) -> datetime:
    return datetime.fromtimestamp(int(t), tz=timezone.utc)


def ridge(X: np.ndarray, y: np.ndarray, lam: float = 1e-3) -> np.ndarray:
    XtX = X.T @ X
    return np.linalg.solve(XtX + lam * np.diag(np.r_[0.0, np.ones(X.shape[1] - 1)]) * np.trace(XtX) / X.shape[1], X.T @ y)


def logistica(X: np.ndarray, y: np.ndarray, lam: float = 1e-4, iters: int = 30, w=None) -> np.ndarray:
    """IRLS com crista leve; w são pesos de observação."""
    n, d = X.shape
    w = np.ones(n) if w is None else w
    b = np.zeros(d)
    R = lam * n * np.eye(d)
    R[0, 0] = 0.0
    for _ in range(iters):
        eta = np.clip(X @ b, -30, 30)
        mu = 1 / (1 + np.exp(-eta))
        W = w * mu * (1 - mu) + 1e-9
        g = X.T @ (w * (y - mu)) - R @ b
        H = (X * W[:, None]).T @ X + R
        passo = np.linalg.solve(H, g)
        b += passo
        if np.max(np.abs(passo)) < 1e-7:
            break
    return b


def sigmoide(z):
    return 1 / (1 + np.exp(-np.clip(z, -30, 30)))


class FluxoBase:
    nome = area = tipo = ""
    niveis = None
    linhas_por_grupo: dict = {}
    sensores: dict = {}
    disjuntor: dict = {"janela": 500}
    cabecalho: dict = {}
    contrato: dict = {}

    def parametros(self) -> dict:
        return {k: v for k, v in vars(type(self)).items() if k.isupper()}

    def tabela_rotulos(self, ev) -> pd.DataFrame:      # binários: id, y, t_disp do rótulo
        raise NotImplementedError

    def precisa_ajuste(self, agora: int) -> bool:
        return False
