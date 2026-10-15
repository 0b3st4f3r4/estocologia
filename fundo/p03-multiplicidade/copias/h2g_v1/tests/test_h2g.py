"""H2g: o pacote de farmacologia carrega e a tabela tem o formato declarado."""

from pathlib import Path

from estocologia import dominio as D

RAIZ = Path(__file__).resolve().parents[1]
DADOS = RAIZ.parent / "dados-reais" / "h2b"


def test_farmacologia():
    p = D.carregar(RAIZ / "dominios" / "farmacologia_qt")
    df = p.preparar(DADOS)
    assert D.checar_tabela(p, df) == []
    assert set(df["ambiente"]) == {"placebo", "dofetilida", "quinidina", "ranolazina", "verapamil"}
    assert df["unidade"].nunique() == 22
    for b, c in (("dofetilida", "c_dof"), ("quinidina", "c_qui"), ("ranolazina", "c_ran")):
        assert (df.loc[df["ambiente"] != b, c] == 0).all()
    assert (df.groupby("ambiente").size() >= 300).all()
