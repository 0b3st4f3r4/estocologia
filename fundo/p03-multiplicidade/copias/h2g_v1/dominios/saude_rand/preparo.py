"""RAND Health Insurance Experiment (sampleSelection::RandHIE): pessoa-ano; ambiente = regime de coparticipação sorteado."""

from pathlib import Path

import numpy as np
import pandas as pd


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "sampleSelection_RandHIE.csv")
    d = d.dropna(subset=["meddol", "coins", "idp", "lpi", "fmde", "linc", "xage", "female", "disea", "physlm", "site", "zper"])
    return pd.DataFrame({
        "y": np.log1p(d["meddol"].astype(float).clip(lower=0)),
        "ambiente": "c" + d["coins"].astype(int).astype(str),       # regime sorteado: c0, c25, c50, c95, c100
        "unidade": d["zper"].astype(int),
        "coparticipacao": d["coins"].astype(float) / 100,
        "franquia_individual": d["idp"].astype(float),
        "pagamento_log": d["lpi"].astype(float),
        "teto_log": d["fmde"].astype(float),
        "renda_log": d["linc"].astype(float),
        "idade": d["xage"].astype(float),
        "mulher": d["female"].astype(float),
        "doencas_cronicas": d["disea"].astype(float),
        "limitacao_fisica": d["physlm"].astype(float),
        "local": d["site"].astype(int).astype(str),
    })
