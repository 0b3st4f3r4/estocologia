"""Portão 1 (Φ1): invariantes estocásticos do processo gerador, por propriedades (Hypothesis)."""

import numpy as np
from hypothesis import given, settings, strategies as st

from estocologia.lab0 import MU, W, ecdf, fila_do_dia, lindley, media_por_paciente

taxas = st.floats(min_value=0.0, max_value=11.9, allow_nan=False)
sementes = st.integers(min_value=0, max_value=2**32 - 1)


@st.composite
def fila(draw):
    n = draw(st.integers(min_value=1, max_value=60))
    gaps = draw(st.lists(st.floats(0, 5, allow_nan=False), min_size=n, max_size=n))
    serv = draw(st.lists(st.floats(0, 5, allow_nan=False), min_size=n, max_size=n))
    return np.cumsum(gaps), np.array(serv)


@given(fila())
def test_lindley_espera_nao_negativa_e_primeiro_nao_espera(f):
    e = lindley(*f)
    assert np.all(e >= 0) and e[0] == 0


@given(fila(), st.floats(0.0, 1.0))
def test_lindley_monotona_no_servico(f, fator):
    # atender mais rápido (serviço encolhido) nunca aumenta a espera de ninguém
    chegadas, serv = f
    assert np.all(lindley(chegadas, serv * fator) <= lindley(chegadas, serv) + 1e-9)


@given(taxas, taxas)
def test_W_crescente_e_convexa(a, b):
    lo, hi = sorted((a, b))
    assert W(lo) <= W(hi)
    # convexidade no ponto médio: é o que faz E[W(Λ)] ≥ W(E[Λ]) (Jensen)
    assert W((lo + hi) / 2) <= (W(lo) + W(hi)) / 2 + 1e-12


@given(st.lists(st.floats(-1e6, 1e6, allow_nan=False), min_size=1, max_size=200))
def test_ecdf_e_uma_distribuicao(x):
    xs, F = ecdf(x)
    assert np.all(np.diff(xs) >= 0) and np.all(np.diff(F) > 0)
    assert 0 < F[0] <= 1 and F[-1] == 1


@given(sementes, st.integers(1, 30))
@settings(max_examples=50)
def test_media_por_paciente_e_a_media_simples(seed, dias):
    rng = np.random.default_rng(seed)
    n = rng.integers(1, 20, dias)
    e = rng.exponential(10, n.sum())
    d = np.repeat(np.arange(dias), n)
    m, ep = media_por_paciente(e, d)
    assert np.isclose(m, e.mean()) and ep >= 0


@given(sementes, taxas)
@settings(max_examples=30, deadline=None)
def test_semente_determina_o_dia(seed, lam):
    a = fila_do_dia(lam, MU, 10, np.random.default_rng(seed))
    b = fila_do_dia(lam, MU, 10, np.random.default_rng(seed))
    assert np.array_equal(a, b)
