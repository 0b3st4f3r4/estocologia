"""Portão 1 do Projeto A: invariantes do processo gerador multiplicativo (Hypothesis)."""

import numpy as np
from hypothesis import given, strategies as st

from estocologia.agentes import Loja
from projetos.a_multiplicativo import LIMIAR_K, T, ProjetoA, g, kelly, ln_riqueza

P = st.floats(0.01, 0.99)
F = st.floats(0.0, 1.0)
proj = ProjetoA()


@given(P)
def test_g_zero_sem_aposta(p):
    assert g(0.0, p) == 0.0


@given(P, F, F)
def test_g_concava_em_f(p, f1, f2):
    assert g((f1 + f2) / 2, p) >= (g(f1, p) + g(f2, p)) / 2 - 1e-12


@given(P, P)
def test_kelly_em_0_1_e_monotona(p1, p2):
    lo, hi = sorted((p1, p2))
    assert 0 <= kelly(lo) <= kelly(hi) <= 1


@given(P)
def test_kelly_maximiza_g(p):
    fs = np.linspace(0, 1, 1001)
    assert g(kelly(p), p) >= g(fs, p).max() - 1e-6


@given(st.integers(0, T))
def test_limiar_de_caras(k):
    assert (ln_riqueza(k, T) > 0) == (k >= LIMIAR_K)


@given(st.integers(0, 2**32 - 1))
def test_lote_cumpre_contrato_e_e_deterministico(seed):
    a = proj.gerar_lote(1, np.random.default_rng(seed))
    b = proj.gerar_lote(1, np.random.default_rng(seed))
    assert a == b
    loja = Loja.__new__(Loja)
    c = proj.contrato()
    assert all(Loja.validar_linha(loja, r, c) is None for r in a[::97])
