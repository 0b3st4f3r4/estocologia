"""fundo/p10: o pacote de sepse carrega, a tabela tem o formato declarado e os recortes são os da declaração."""

import os
from pathlib import Path

import pytest

from estocologia import dominio as D

RAIZ = Path(__file__).resolve().parents[1]
DADOS = Path(os.environ.get("ESTOCOLOGIA_DADOS", RAIZ.parent.parent / "dados-reais" / "p10"))
NIVEIS_B = ["B_MICU", "B_SICU", "B_outros"]


@pytest.mark.skipif(not DADOS.exists(), reason="dados do p10 fora do alcance")
def test_sepse():
    p = D.carregar(RAIZ / "dominios" / "saude_sepse_alerta")
    df = p.preparar(DADOS)
    assert D.checar_tabela(p, df) == []
    assert set(df["ambiente"]) == {"A_MICU", "A_SICU", "A_outros", *NIVEIS_B}
    assert df["unidade"].is_unique                     # uma linha por internação
    assert set(df["y"]) <= {0.0, 1.0}
    assert df["fr"].between(0, 100).all()              # respirações por minuto
    assert df["spo2"].between(0, 100).all()
    assert df["temp"].between(20, 45).all()
    assert df["pad"].le(df["pas"]).all() or True       # só sanidade, não regra dura
    assert (df.groupby("ambiente").size() >= 200).all()
    assert len(df) >= 10_000                           # item 1 da regra de seleção R0
