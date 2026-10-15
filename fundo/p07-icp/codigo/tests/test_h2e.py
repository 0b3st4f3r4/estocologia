"""H2e: os dois pacotes novos carregam, passam no contrato e a tabela preparada tem o formato declarado."""

from pathlib import Path

import numpy as np

from estocologia import dominio as D

RAIZ = Path(__file__).resolve().parents[1]
DADOS = RAIZ.parent / "dados-reais" / "h2b"


def _checa(nome, n_amb, cofre):
    p = D.carregar(RAIZ / "dominios" / nome)
    df = p.preparar(DADOS)
    assert D.checar_tabela(p, df) == []
    amb = set(df["ambiente"])
    assert len(amb) == n_amb and cofre in amb
    assert np.isfinite(df[["y", *p.candidatos]].to_numpy(float)).all()
    return df


def test_tunel_luz():
    df = _checa("fisica_tunel_luz", 54, "uniform_red_strong")
    assert not df["ambiente"].str.startswith(("uniform_osr_c_", "uniform_v_c_")).any()
    assert df["red"].max() <= 1.0


def test_tunel_vento():
    df = _checa("engenharia_tunel_vento", 32, "r18")
    assert (df.groupby("ambiente").size() == 500).all()
    assert (df.groupby("ambiente")["unidade"].nunique() == 10).all()
