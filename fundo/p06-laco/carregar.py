"""Carrega o AnonymousBank 1999 (Technion SEE Lab) a partir dos dois espelhos no GitHub e confere que coincidem."""
from __future__ import annotations

import sys
from pathlib import Path

import numpy as np
import pandas as pd

DADOS = Path("/mnt/project-files/estocologia/dados-reais/fundo/p06-laco")
COLS = ["vru.line", "call_id", "customer_id", "priority", "type", "date", "vru_entry", "vru_exit", "vru_time",
        "q_start", "q_exit", "q_time", "outcome", "ser_start", "ser_exit", "ser_time", "server"]


def _seg(s: pd.Series) -> np.ndarray:
    p = s.astype(str).str.extract(r"^(\d+):(\d\d):(\d\d)$").apply(pd.to_numeric)
    return (p[0] * 3600 + p[1] * 60 + p[2]).to_numpy(float)


def ler_jorgesislema() -> pd.DataFrame:
    d = pd.read_csv(DADOS / "jorgesislema/Call_Center_1999_DataSet.csv", sep=";", dtype={"customer_id": str})
    d["date"] = pd.to_datetime(d["date"])
    return d[COLS]


def ler_datawookie() -> pd.DataFrame:
    d = pd.read_csv(DADOS / "datawookie/call-centre.csv", dtype=str)
    d["date"] = pd.to_datetime("19" + d["date"].astype(str), format="%Y%m%d", errors="coerce")
    return d[COLS]


def carregar() -> pd.DataFrame:
    d = ler_jorgesislema()
    d["customer_id"] = d["customer_id"].astype(str).str.strip()
    for c in ["vru_entry", "vru_exit", "q_start", "q_exit", "ser_start", "ser_exit"]:
        d[c + "_s"] = _seg(d[c])
    d["priority"] = pd.to_numeric(d["priority"], errors="coerce")
    d["q_time"] = pd.to_numeric(d["q_time"], errors="coerce")
    return d


def _norm(d: pd.DataFrame) -> pd.DataFrame:
    d = d.copy()
    for c in COLS:
        d[c] = d[c].astype(str).str.strip()
    d["date"] = d["date"].str[:10]
    for c in ["vru_entry", "vru_exit", "q_start", "q_exit", "ser_start", "ser_exit"]:
        d[c] = _seg(d[c])
    d["customer_id"] = d["customer_id"].str.upper()
    return d


if __name__ == "__main__":
    a, b = _norm(ler_jorgesislema()), _norm(ler_datawookie())
    print("linhas", len(a), len(b))
    k = ["date", "vru.line", "call_id", "vru_entry"]
    m = a.merge(b, on=k, how="outer", suffixes=("_a", "_b"), indicator=True)
    print("pareadas pela chave (data, vru.line, call_id, vru_entry):", int((m["_merge"] == "both").sum()),
          "sem par:", int((m["_merge"] != "both").sum()))
    for c in COLS:
        if c not in k:
            print(f"{c:12s} fração igual: {(m[c + '_a'] == m[c + '_b']).mean():.6f}")
    sci = a["customer_id"].str.contains("E+", regex=False)
    print("customer_id em notação científica (corrompido nos dois espelhos):", int(sci.sum()))
    sys.exit(0)
