"""Portão 1 do Projeto C (dados reais): invariantes do processo gerador de cauda e do lote (Hypothesis)."""

import numpy as np
from hypothesis import given, strategies as st

from estocologia.agentes import Loja
from projetos.c_real import (JANELA, LIMITE, ORIGENS, PERIODOS_ANOS, REGRAS, RETENCAO, TETO, ProjetoC, alavancagem_da_regra,
                             dados_reais, excesso_camada, g, hill_ic, kelly, laplace, ln_riqueza, maior_sequencia, mercado)

P = st.floats(0.0, 0.2)
P_POS = st.floats(1e-4, 0.2)
BARREIRA = mercado()["barreira"]
F = st.floats(0.0, BARREIRA * 0.999)      # domínio em que g é finita (1 + fπ − f > 0)
proj = ProjetoC()


def test_dados_reais_completos():
    d = dados_reais()
    assert len(d) == 2167
    assert len({r[2] for r in d}) == 132          # todo mês de 1980–90 tem ao menos um sinistro: a unidade nunca some
    assert len({r[0] for r in d}) == len(d)
    assert min(r[4] for r in d) >= 1.0
    assert sum(len(proj.gerar_lote(p, None)) for p in PERIODOS_ANOS) == len(d)


@given(P)
def test_g_zero_sem_exposicao(p):
    assert g(0.0, p) == 0.0


@given(P, F, F)
def test_g_concava_em_f(p, f1, f2):
    assert g((f1 + f2) / 2, p) >= (g(f1, p) + g(f2, p)) / 2 - 1e-12


@given(P_POS)
def test_kelly_maximiza_g_abaixo_da_barreira(p):
    fs = np.linspace(0, BARREIRA * 0.999, 1501)
    f = kelly(p)
    assert 0 <= f < BARREIRA
    assert g(f, p) >= g(fs, p).max() - 1e-9


@given(P_POS, P_POS)
def test_kelly_decresce_em_p(p1, p2):
    lo, hi = sorted((p1, p2))
    assert kelly(lo) >= kelly(hi) - 1e-6


@given(st.integers(0, JANELA), st.integers(0, JANELA))
def test_kelly_estimado_monotonico_em_k(k1, k2):
    lo, hi = sorted((k1, k2))
    for regra in REGRAS:   # mais meses atingidos ⇒ exposição menor ou igual, sempre em [0, teto]
        assert 0 <= alavancagem_da_regra(regra, hi) <= alavancagem_da_regra(regra, lo) + 1e-6 <= TETO + 1e-6
    assert alavancagem_da_regra("kelly_kn", 0) == TETO      # a alegação: janela limpa ⇒ teto


@given(st.integers(1, 10_000), st.data())
def test_laplace_nunca_zero_nem_um(n, data):
    k = data.draw(st.integers(0, n))
    p = laplace(k, n)
    assert 0 < p < 1
    assert kelly(p) < BARREIRA     # quem usa Laplace nunca cruza a barreira de ruína


@given(st.floats(0.0, TETO), st.lists(st.floats(0.0, 1.0), min_size=1, max_size=60))
def test_ruina_sse_fator_nao_positivo(f, s):
    lnw = ln_riqueza(f, s)
    pi = mercado()["premio_mes"]
    assert bool(np.isinf(lnw)) == any(1 + f * pi - f * x <= 0 for x in s)
    if not np.isinf(lnw):
        assert np.isfinite(lnw)


@given(st.floats(0.0, 1e4), st.floats(0.0, 1e4))
def test_excesso_da_camada_limitado_e_monotonico(a, b):
    lo, hi = sorted((a, b))
    assert 0.0 <= excesso_camada(lo) <= excesso_camada(hi) <= LIMITE
    assert (excesso_camada(hi) > 0) == (hi > RETENCAO)


@given(st.floats(0.1, 100.0), st.sampled_from([25, 50, 100]))
def test_hill_invariante_a_escala(c, k):
    x = np.array([r[4] for r in dados_reais()])
    assert abs(hill_ic(c * x, k)["alpha"] - hill_ic(x, k)["alpha"]) < 1e-8


@given(st.lists(st.booleans(), max_size=80))
def test_maior_sequencia(v):
    m = maior_sequencia(v)
    assert 0 <= m <= len(v)
    assert (m == 0) == (not any(v))
    assert "1" * m in "".join("1" if b else "0" for b in v)
    assert "1" * (m + 1) not in "".join("1" if b else "0" for b in v)


@given(st.integers(0, 2**32 - 1), st.sampled_from([1, 2, 3, 4, 97, 98, 99]), st.sampled_from(["mundo", "deriva", "patologico"]))
def test_lote_cumpre_contrato_e_e_deterministico(seed, periodo, cenario):
    a = proj.gerar_lote(periodo, np.random.default_rng(seed), cenario)
    b = proj.gerar_lote(periodo, np.random.default_rng(seed), cenario)
    assert a == b
    loja = Loja.__new__(Loja)
    c = proj.contrato()
    assert len(a) >= 25
    assert all(Loja.validar_linha(loja, r, c) is None for r in a[::17])
    assert len({r["sinistro"] for r in a}) == len(a)
    for r in a[::17]:
        assert r["excesso_camada"] == float(excesso_camada(r["perda"]))
        assert r["origem"] in ORIGENS
        if cenario == "mundo" and periodo in PERIODOS_ANOS:
            assert r["origem"] == "real"                       # o mundo devolve só linhas reais do período
            a0, a1 = PERIODOS_ANOS[periodo]
            assert a0 <= r["ano"] <= a1
        elif cenario != "mundo":
            assert r["origem"].startswith("sintetico")         # perturbações sempre rotuladas
        else:
            assert r["origem"] == "reamostra_real"


def test_patologico_tem_surto_e_deriva_infla_severidade():
    rng = np.random.default_rng(0)
    pat = proj.gerar_lote(97, rng, "patologico")
    dados = {c: np.array([r[c] for r in pat]) for c in ("mes", "perda")}
    _, mx, _ = proj._meses(dados)
    assert maior_sequencia(mx > RETENCAO) >= 8
    der = proj.gerar_lote(3, rng, "deriva")
    base = proj.gerar_lote(3, rng)
    assert all(abs(d["perda"] - 3 * b["perda"]) < 1e-9 for d, b in zip(der, base))
