"""Experimento de campo de Bertrand & Mullainathan (2004): currículos fictícios com nomes sorteados enviados a anúncios
de emprego em Boston e Chicago. Só tradução de nomes, tipos e recortes; nenhuma modelagem."""

from pathlib import Path

import numpy as np
import pandas as pd

# características do anúncio (iguais para os ~4 currículos enviados ao mesmo anúncio): definem o pseudoanúncio
COLUNAS_ANUNCIO = ["city", "minimum", "equal", "wanted", "requirements", "reqexp", "reqcomm", "reqeduc", "reqcomp",
                   "reqorg", "industry"]


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "AER_ResumeNames.csv")
    sim = lambda c: (d[c] == "yes").astype(float)
    qualidade = d["quality"].map({"high": "alta", "low": "baixa"})
    return pd.DataFrame({
        "y": (d["call"] == "yes").astype(float),                          # retorno do empregador (ligação ou e-mail)
        "ambiente": (d["city"] + "_" + qualidade).astype(str),            # cidade × banco de qualidade sorteado
        # o anúncio não tem identificador na base: agrupa-se pelas características do anúncio (agrupamento mais
        # grosso que o anúncio verdadeiro, logo conservador para o erro padrão)
        "unidade": d.groupby(COLUNAS_ANUNCIO).ngroup().astype(int),
        "afro": (d["ethnicity"] == "afam").astype(float),                 # nome sorteado de sonoridade afro-americana
        "feminino": (d["gender"] == "female").astype(float),
        "experiencia_log": np.log1p(d["experience"].astype(float)),       # retorno decrescente dos anos de experiência
        "honras": sim("honors"),
        "lacuna": sim("holes"),
        "email": sim("email"),
        "faculdade": sim("college"),
        "habilidades_especiais": sim("special"),
    })
