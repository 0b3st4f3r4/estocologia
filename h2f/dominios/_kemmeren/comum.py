"""Kemmeren et al. (2014): preparo comum dos 10 domínios genetica_yNN (../../DECLARACAO_H2f.md)."""

from functools import lru_cache
from pathlib import Path

import numpy as np
import pandas as pd

ARQ = "kemmeren2014_obs_int.parquet"
META = ["sample_id", "kind", "deleted_gene", "deleted_gene_col_index_1based", "plate"]
SEMENTE, N_Y, N_X = 2026, 10, 9


def _caminho(dados: Path) -> Path:
    return Path(dados).parent / "h2f" / ARQ


@lru_cache(maxsize=1)
def _tabela(caminho: str) -> pd.DataFrame:
    return pd.read_parquet(caminho)


def ys(dados: Path) -> list[str]:
    d = _tabela(str(_caminho(dados)))
    genes = [c for c in d.columns if c not in META]
    deletados = set(d["deleted_gene"].dropna())
    nunca = sorted(g for g in genes if g not in deletados)
    return list(np.random.default_rng(SEMENTE).choice(nunca, N_Y, replace=False))


def ambiente(d: pd.DataFrame) -> np.ndarray:
    letra = d["deleted_gene"].fillna("").str[1]
    return np.where(d["kind"] == "observational", "obs",
                    np.where(letra.isin(list("ABCD")), "del_AD", np.where(letra.isin(list("EFGHIJ")), "del_EJ", "del_KP")))


def candidatos(d: pd.DataFrame, y: str) -> tuple[list[str], float]:
    """Os N_X genes deletados de maior |correlação| com Y nas amostras observacionais (treino) e o sinal de x1."""
    obs = d[d["kind"] == "observational"]
    deletados = sorted(set(d["deleted_gene"].dropna()) - {y})
    X = obs[deletados].to_numpy(float)
    v = obs[y].to_numpy(float)
    Xc, vc = X - X.mean(0), v - v.mean()
    r = (Xc.T @ vc) / np.sqrt((Xc ** 2).sum(0) * (vc ** 2).sum() + 1e-300)
    ordem = np.argsort(-np.abs(r), kind="stable")[:N_X]
    return [deletados[i] for i in ordem], float(np.sign(r[ordem[0]]) or 1.0)


def preparar(dados: Path, k: int) -> pd.DataFrame:
    d = _tabela(str(_caminho(dados)))
    y = ys(dados)[k - 1]
    d = d[d["deleted_gene"] != y].reset_index(drop=True)
    xs, sinal = candidatos(d, y)
    out = pd.DataFrame({"y": d[y].astype(float), "ambiente": ambiente(d), "unidade": d["sample_id"].astype(str)})
    for i, g in enumerate(xs, 1):
        out[f"x{i}"] = d[g].astype(float) * (sinal if i == 1 else 1.0)
    out["plate"] = d["plate"].astype(float)
    out.attrs["genes"] = {"y": y, **{f"x{i}": g for i, g in enumerate(xs, 1)}, "sinal_x1": sinal}
    return out
