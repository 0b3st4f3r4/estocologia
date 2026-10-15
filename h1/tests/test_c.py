"""Portão 1 do Projeto C: invariantes do processo gerador de cauda (Hypothesis)."""

import numpy as np
from hypothesis import given, strategies as st

from estocologia.agentes import Loja
from projetos.c_cauda import (FUT, HIST, PERDA, REGRAS, TETO, ProjetoC, alavancagem_da_regra, g, kelly, laplace,
                              ln_riqueza)

P = st.floats(0.0, 0.05)
P_POS = st.floats(1e-6, 0.019)
F = st.floats(0.0, 1.99)          # domínio em que g é finita (f·0,5 < 1)
proj = ProjetoC()


@given(P)
def test_g_zero_sem_alavancagem(p):
    assert g(0.0, p) == 0.0


@given(P, F, F)
def test_g_concava_em_f(p, f1, f2):
    assert g((f1 + f2) / 2, p) >= (g(f1, p) + g(f2, p)) / 2 - 1e-12


@given(P_POS)
def test_kelly_fechado_maximiza_g(p):
    fs = np.linspace(0, 1.99, 2001)
    assert g(kelly(p), p) >= g(fs, p).max() - 1e-9


@given(st.integers(0, HIST), st.integers(0, HIST))
def test_kelly_estimado_monotonico_em_p_hat(k1, k2):
    lo, hi = sorted((k1, k2))
    for regra in REGRAS:   # mais perdas vistas ⇒ alavancagem menor ou igual, sempre em [0, teto]
        assert 0 <= alavancagem_da_regra(regra, hi) <= alavancagem_da_regra(regra, lo) <= TETO
    assert kelly(lo / HIST) >= kelly(hi / HIST)


@given(st.integers(1, 10_000), st.data())
def test_laplace_nunca_zero_nem_um(n, data):
    k = data.draw(st.integers(0, n))
    p = laplace(k, n)
    assert 0 < p < 1
    assert kelly(p) < 2      # quem usa Laplace nunca cruza a barreira de ruína


@given(st.floats(0.0, TETO), st.integers(0, FUT))
def test_ruina_sse_barreira_e_perda(f, m):
    lnw = ln_riqueza(f, FUT - m, m)
    assert bool(np.isinf(lnw)) == (f * PERDA <= -1 and m > 0)
    if not np.isinf(lnw):
        assert np.isfinite(lnw)


@given(st.integers(0, 2**32 - 1), st.sampled_from(["mundo", "deriva", "patologico"]))
def test_lote_cumpre_contrato_e_e_deterministico(seed, cenario):
    a = proj.gerar_lote(1, np.random.default_rng(seed), cenario)
    b = proj.gerar_lote(1, np.random.default_rng(seed), cenario)
    assert a == b
    loja = Loja.__new__(Loja)
    c = proj.contrato()
    assert all(Loja.validar_linha(loja, r, c) is None for r in a[::37])
    assert len({r["fundo"] for r in a}) == len(a)
    for r in a[::37]:   # a alavancagem é a da regra declarada aplicada ao próprio histórico
        assert r["alavancagem"] == alavancagem_da_regra(r["regra"], r["perdas_hist"])
