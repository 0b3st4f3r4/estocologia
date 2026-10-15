"""Leitura comum dos diários do H3 para o ponto 8 (só leitura; os diários são conferidos pelo SHA-256)."""
from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

import numpy as np
import pandas as pd

AQUI = Path(__file__).resolve().parent
sys.path.insert(0, str(AQUI / "codigo"))
from estocologia import calibracao as C  # noqa: E402

REPO = AQUI.parents[1]
H3_REPO = REPO / "h3" / "saida"
H3_MNT = Path("/mnt/project-files/estocologia/h3/saida")
REPLAYS = AQUI / "saida" / "replays"
SEMENTE = 20261015

# fluxo: (rótulo, nome da pasta)
FLUXOS = {"voos": "voos 2013 (NY)", "ciclones": "ciclones (HURDAT2)", "prisoes": "prisões Toronto 1997–2002",
          "prisoes20": "prisões Toronto 2020–2021", "voos23": "voos 2023 (NY)", "prisoes_ri": "Rhode Island 2005–2015"}


def somas_esperadas() -> dict:
    out = {}
    for l in (REPO / "FORA_DO_GIT.sha256").read_text().splitlines():
        h, p = l.split()
        out[p] = h
    for p in ("h3/saida/prisoes/diario.parquet", "h3/saida/ciclones/diario.parquet"):
        out[p] = hashlib.sha256((REPO / p).read_bytes()).hexdigest()
    return out


def diario(fluxo: str) -> pd.DataFrame:
    """Diário do H3 (no disco compartilhado), conferido contra a soma do repositório."""
    for base in (H3_REPO, H3_MNT):
        p = base / fluxo / "diario.parquet"
        if p.exists():
            h = hashlib.sha256(p.read_bytes()).hexdigest()
            esperado = somas_esperadas().get(f"h3/saida/{fluxo}/diario.parquet")
            assert esperado is None or h == esperado, (fluxo, h, esperado)
            return pd.read_parquet(p)
    raise FileNotFoundError(fluxo)


def resultados(fluxo: str) -> dict:
    return json.loads((H3_REPO / fluxo / "resultados.json").read_text())


def cofre(d: pd.DataFrame, corte: int) -> pd.DataFrame:
    return d[d["y"].notna() & (d["t_emissao"] >= corte)]


def mes(t) -> str:
    return pd.to_datetime(int(t), unit="s", utc=True).strftime("%Y-%m-%d")
