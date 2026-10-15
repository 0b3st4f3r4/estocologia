"""H2f: os 10 pacotes de genética carregam e a pré-seleção usa só as amostras selvagens."""

from pathlib import Path

import numpy as np
import pandas as pd

from estocologia import dominio as D

RAIZ = Path(__file__).resolve().parents[1]
DADOS = RAIZ.parent / "dados-reais" / "h2b"


def test_genetica():
    vistos = set()
    for k in range(1, 11):
        p = D.carregar(RAIZ / "dominios" / f"genetica_y{k:02d}")
        df = p.preparar(DADOS)
        assert D.checar_tabela(p, df) == []
        assert set(df["ambiente"]) == {"obs", "del_EJ", "del_KP", "del_AD"}
        assert (df["ambiente"] == "obs").sum() == 262
        g = df.attrs["genes"]
        assert g["y"] not in vistos
        vistos.add(g["y"])
        assert p.meta["alfa_icp"] == 0.005


def test_preselecao_ignora_mutantes():
    """Mudar os valores dos mutantes não muda os candidatos."""
    import importlib.util
    spec = importlib.util.spec_from_file_location("c", RAIZ / "dominios" / "_kemmeren" / "comum.py")
    c = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(c)
    d = c._tabela(str(c._caminho(DADOS))).copy()
    y = c.ys(DADOS)[0]
    a = c.candidatos(d, y)
    genes = [x for x in d.columns if x not in c.META]
    mut = d["kind"] != "observational"
    d.loc[mut, genes] = np.random.default_rng(0).normal(size=(mut.sum(), len(genes)))
    assert c.candidatos(d, y) == a
