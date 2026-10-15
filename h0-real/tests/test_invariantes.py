"""Portão 1 (Φ1): invariantes da fila com c(t) atendentes e da fórmula da alegação (Hypothesis)."""

import numpy as np
from hypothesis import given, settings, strategies as st

from estocologia.central import _fcfs, conforme, erlang_c, escala, media_por_chamada, nivel_servico

sementes = st.integers(min_value=0, max_value=2**32 - 1)
INF = np.inf


def lindley(chegadas, servico):
    e = np.zeros(chegadas.size)
    for i in range(1, chegadas.size):
        e[i] = max(0.0, e[i - 1] + servico[i - 1] - (chegadas[i] - chegadas[i - 1]))
    return e


@st.composite
def fila(draw):
    n = draw(st.integers(1, 80))
    gaps = draw(st.lists(st.floats(0, 50, allow_nan=False), min_size=n, max_size=n))
    serv = draw(st.lists(st.floats(0, 200, allow_nan=False), min_size=n, max_size=n))
    G = draw(st.integers(1, 6))
    c = draw(st.lists(st.integers(1, 6), min_size=G, max_size=G))
    return np.cumsum(gaps), np.array(serv), np.array(c, dtype=np.int64)


@given(fila())
def test_espera_nao_negativa_e_primeiro_nao_espera(f):
    t, s, c = f
    e, a = _fcfs(t, s, np.full(t.size, INF), c, 100.0)
    assert np.all(e >= 0) and e[0] == 0 and not a.any()


@given(fila())
def test_um_atendente_e_lindley(f):
    t, s, _ = f
    e, _ = _fcfs(t, s, np.full(t.size, INF), np.array([1], dtype=np.int64), 1e9)
    assert np.allclose(e, lindley(t, s), atol=1e-6)


@given(fila(), st.integers(1, 3))
def test_mais_atendentes_nunca_pioram_a_espera(f, extra):
    # FCFS com c constante: o vetor de Kiefer–Wolfowitz é monótono no número de servidores
    t, s, c = f
    k = np.array([c.min()], dtype=np.int64)
    e1, _ = _fcfs(t, s, np.full(t.size, INF), k, 1e9)
    e2, _ = _fcfs(t, s, np.full(t.size, INF), k + extra, 1e9)
    assert np.all(e2 <= e1 + 1e-6)


@given(fila(), st.floats(0, 100))
def test_desistente_espera_exatamente_a_paciencia(f, pac):
    t, s, c = f
    e, a = _fcfs(t, s, np.full(t.size, pac), c, 100.0)
    assert np.all(e <= pac + 1e-9) and np.allclose(e[a], pac)


@given(st.integers(1, 300), st.floats(0.0, 0.999))
def test_erlang_c_e_probabilidade_e_cresce_com_a_carga(c, rho):
    a = rho * c
    p = erlang_c(c, a)
    assert 0 <= p <= 1 + 1e-12
    assert erlang_c(c, a * 0.9) <= p + 1e-12


@given(st.floats(0.001, 2.0), st.floats(30, 600))
@settings(max_examples=60)
def test_escala_e_a_menor_que_cumpre_a_meta(lam, aht):
    c = escala(lam, aht)
    assert nivel_servico(c, lam, aht) >= 0.8
    assert c == 1 or nivel_servico(c - 1, lam, aht) < 0.8


@given(sementes, st.integers(1, 30))
@settings(max_examples=40)
def test_media_por_chamada_e_a_media_simples(seed, dias):
    rng = np.random.default_rng(seed)
    n = rng.integers(1, 20, dias)
    e = rng.exponential(10, n.sum())
    d = np.repeat(np.arange(dias), n)
    m, ep = media_por_chamada(e, d)
    assert np.isclose(m, e.mean()) and ep >= 0


@given(sementes, st.integers(20, 200))
@settings(max_examples=40)
def test_conforme_cobre_a_propria_calibracao(seed, n):
    x = np.random.default_rng(seed).normal(size=n)
    lo, hi = conforme(x, 0.1)
    assert lo <= hi and np.mean((x >= lo) & (x <= hi)) >= 0.85
