"""Portão 1 do Projeto A (dados reais): invariantes do crescimento multiplicativo no índice (Hypothesis)."""

import numpy as np
from hypothesis import assume, given, strategies as st
from hypothesis.extra.numpy import arrays

from estocologia.agentes import Loja
from projetos.a_real import (ProjetoA, coortes, g_emp, g_quad, gama_de, kelly_quad, l_ruina, ln_fator, por_ano)

R = arrays(np.float64, st.integers(2, 60), elements=st.floats(-0.3, 0.3))     # retornos diários plausíveis
L = st.floats(0.0, 3.0)
proj = ProjetoA()


@given(R)
def test_g_zero_sem_exposicao(r):
    assert g_emp(0.0, r) == 0.0


@given(R, L)
def test_media_aritmetica_domina_a_temporal(r, l):
    # ln(1 + x) ≤ x: a taxa temporal nunca passa da leitura aritmética L·μ (a falácia, em uma linha)
    assert g_emp(l, r) <= l * np.mean(r) + 1e-12


@given(R, L, L)
def test_g_concava_em_L(r, l1, l2):
    a, b, m = g_emp(l1, r), g_emp(l2, r), g_emp((l1 + l2) / 2, r)
    assert m >= (a + b) / 2 - 1e-9


@given(R, st.floats(0.01, 10.0))
def test_ruina_absorvente(r, l):
    assume(abs(1 + l * r.min()) > 1e-9)          # fronteira exata de ponto flutuante fica de fora
    lr = l_ruina(r)
    assert (g_emp(l, r) == -np.inf) == (l >= lr)


@given(st.floats(-0.5, 0.5), st.floats(0.001, 0.5))
def test_kelly_quad_maximiza(alfa, V):
    ls = np.linspace(-5, 5, 2001)
    assert g_quad(kelly_quad(alfa, V), alfa, V) >= g_quad(ls, alfa, V).max() - 1e-12


@given(st.floats(0.01, 0.99), st.floats(0.01, 1.0))
def test_gama_sinal_de_p_ano(p, s):
    assert np.sign(gama_de(p, s)) == np.sign(p - 0.5) or abs(p - 0.5) < 1e-12


@given(arrays(np.int64, st.integers(1, 80), elements=st.integers(1885, 1890)), st.data())
def test_por_ano_preserva_o_total(anos, data):
    anos = np.sort(anos)
    x = data.draw(arrays(np.float64, anos.size, elements=st.floats(-1, 1)))
    u, s = por_ano(anos, x)
    assert np.isclose(s.sum(), x.sum()) and list(u) == sorted(set(anos.tolist()))


@given(R)
def test_jensen_conjunto_vs_tempo(r):
    lnf = ln_fator(r, 1.0)
    n = r.size
    ini = np.arange(0, n // 2)
    fim = ini + n // 2 - 1
    lw = coortes(lnf, ini, fim)
    assert np.mean(np.exp(lw)) >= np.exp(np.mean(lw)) * (1 - 1e-12)


@given(st.sampled_from([1, 2, 3, 4, 97, 98, 99]), st.integers(0, 2**32 - 1))
def test_lote_cumpre_contrato_e_e_deterministico(periodo, seed):
    cen = {97: "patologico", 98: "deriva"}.get(periodo, "mundo")
    a = proj.gerar_lote(periodo, np.random.default_rng(seed), cen)
    b = proj.gerar_lote(periodo, np.random.default_rng(seed), cen)
    assert a == b
    loja = Loja.__new__(Loja)
    c = proj.contrato()
    assert all(Loja.validar_linha(loja, r, c) is None for r in a[::97])
    assert len({(r["ano"], r["dia"]) for r in a}) == len(a)            # chave primária única no lote
    origem = {r["origem"] for r in a}
    assert origem == ({"real"} if periodo <= 4 else {"perturbado"} if periodo in (97, 98) else {"reamostrado"})
