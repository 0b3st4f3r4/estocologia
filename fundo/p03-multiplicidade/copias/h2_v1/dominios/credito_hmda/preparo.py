"""Pedidos de financiamento imobiliário na região de Boston (HMDA 1990, estudo do Fed de Boston).
Só tradução de nomes, tipos e recortes; nenhuma modelagem."""

from pathlib import Path

import pandas as pd


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "AER_HMDA.csv")
    sim = lambda c: (d[c] == "yes").astype(float)
    faixa = pd.cut(d["unemp"], [0, 2.05, 3.25, 4.5, 11], labels=["baixo", "medio", "alto", "muito_alto"])
    return pd.DataFrame({
        "y": sim("deny"),                                            # pedido negado
        "ambiente": faixa.astype(str),                               # faixa de desemprego do setor do requerente
        "unidade": d["rownames"].astype(int),                        # o pedido: a base não identifica o banco
        "afro": sim("afam"),                                         # requerente negro, como registrado pelo banco
        "razao_pagamento": d["pirat"].clip(upper=1.5).astype(float),  # dívida total / renda (3 valores > 1,5 truncados)
        "razao_habitacao": d["hirat"].clip(upper=1.5).astype(float),  # despesa com moradia / renda
        "ltv_medio": ((d["lvrat"] > 0.8) & (d["lvrat"] <= 0.95)).astype(float),   # limiares usados pelos bancos
        "ltv_alto": (d["lvrat"] > 0.95).astype(float),
        "historico_consumo": d["chist"].astype(float),               # 1 (bom) a 6 (pior)
        "historico_hipoteca": d["mhist"].astype(float),              # 1 (bom) a 4 (pior)
        "registro_publico": sim("phist"),                            # falência, protesto etc.
        "seguro_negado": sim("insurance"),                           # seguro hipotecário negado
        "autonomo": sim("selfemp"),
        "solteiro": sim("single"),
        "condominio": sim("condomin"),
    })
