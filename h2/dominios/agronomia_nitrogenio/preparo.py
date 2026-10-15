"""Las Rosas (Córdoba, Argentina): monitor de colheita de milho em 1999 e 2001, com faixas de nitrogênio sorteadas
dentro de blocos. Só tradução de nomes, tipos e recortes; nenhuma modelagem."""

from pathlib import Path

import pandas as pd


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "agridat_lasrosas.corn.csv")
    n = d["nitro"].astype(float)
    return pd.DataFrame({
        "y": d["yield"].astype(float),                                   # rendimento de grãos (quintais/ha)
        "ambiente": d["topo"].astype(str),                               # posição topográfica: W, HT, E, LO
        "unidade": (d["year"].astype(str) + "_" + d["rep"] + "_" + d["nf"]).astype(str),   # a faixa sorteada
        "nitrogenio": n,                                                 # dose de N (kg/ha)
        "nitrogenio_quad": (n - 65.0) ** 2 / 100.0,                      # curvatura da resposta (rendimento decrescente)
        "brilho_solo": d["bv"].astype(float),                            # brilho do solo (proxy de matéria orgânica)
        "ano_2001": (d["year"] == 2001).astype(float),
    })
