"""fundo/p09: os dois pacotes de saúde carregam, a tabela tem o formato declarado e a regra R1 é determinística."""

import os
from pathlib import Path

import pandas as pd
import pytest

from estocologia import dominio as D

RAIZ = Path(__file__).resolve().parents[1]
DADOS = Path(os.environ.get("P09_DADOS", "/mnt/project-files/estocologia/dados-reais/fundo/p09-escala-dados"))
pytestmark = pytest.mark.skipif(not DADOS.exists(), reason="dados do p09 fora do alcance")


def test_indometacina():
    p = D.carregar(RAIZ / "dominios" / "saude_indometacina_ercp")
    df = p.preparar(DADOS)
    assert D.checar_tabela(p, df) == []
    assert df.groupby("ambiente").size().to_dict() == {"IU": 413, "UM": 164, "outros": 25}
    assert set(df["y"]) == {0.0, 1.0} and set(df["indometacina"]) == {0.0, 1.0}
    assert df["unidade"].is_unique


def test_covid_vacina():
    p = D.carregar(RAIZ / "dominios" / "saude_covid_vacina_paises")
    df = p.preparar(DADOS)
    assert D.checar_tabela(p, df) == []
    assert len(df) == 3167 and df.loc[df.ambiente == "South_America", "unidade"].nunique() == 10
    assert df["vacinados"].between(0, 1).all()
    assert df["mes"].min() == "2021-01" and df["mes"].max() == "2022-12"


def test_regra_r1_deterministica():
    import sys
    sys.path.insert(0, str(RAIZ / "escala"))
    from gerar import selecionar
    df = pd.DataFrame({"grupo": ["a", "b", "c"] * 120, "y": range(360), "x1": [i % 7 for i in range(360)],
                       "x2": [(i * 13) % 11 for i in range(360)]})
    df["y"] = df["y"].sample(frac=1, random_state=1).to_numpy()
    e1, m1 = selecionar(df)
    e2, _ = selecionar(df)
    assert m1 == "ok" and e1 == e2
    assert e1["ambiente"] == "grupo" and e1["alvo"] == "y" and e1["chave"] == "x1" and e1["cofre"] == "c"
