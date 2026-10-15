"""Portão 1 do Projeto B: invariantes do SCM da vitamina e do estimador de ajuste (Hypothesis)."""

import numpy as np
from hypothesis import given, strategies as st
from scipy import special

from estocologia.agentes import Loja
from projetos.b_causal import (GRAFO, ProjetoB, ajuste, ajuste_empirico, contagens, d_separados, kleinberg_fpr,
                               observacional, p_w, p_x, p_y, porta_dos_fundos, simular_scm)

PR = st.floats(0.02, 0.98)
B = st.integers(0, 1)
SEMENTE = st.integers(0, 2**32 - 1)
CEN = st.sampled_from(["mundo", "patologico", "deriva"])
proj = ProjetoB()


@given(B, B, B, B, CEN)
def test_probabilidades_do_scm_em_0_1(x, z, s, u, cen):
    for p in (p_y(x, z, s, u, cen), p_y(x, z, None, u, cen), p_x(z, u, cen), p_w(z, u, cen)):
        assert 0.0 <= float(p) <= 1.0


@given(B, B)
def test_sexo_some_na_marginal(x, z):
    # S é independente de (X, Z) e entra com ±: a média em S devolve o núcleo do lab
    assert abs((p_y(x, z, 0) + p_y(x, z, 1)) / 2 - p_y(x, z)) < 1e-12


@given(st.lists(PR, min_size=4, max_size=4), PR, PR, PR)
def test_ajuste_e_observacional_sao_misturas(cel, pz, px0, px1):
    d0, d1 = ajuste(cel, pz)
    o0, o1 = observacional(cel, pz, px0, px1)
    for v, a, b in ((d0, cel[0], cel[1]), (d1, cel[2], cel[3]), (o0, cel[0], cel[1]), (o1, cel[2], cel[3])):
        assert min(a, b) - 1e-12 <= v <= max(a, b) + 1e-12
    if abs(px0 - px1) < 1e-12:      # sem confundimento, ver = fazer
        assert abs(o0 - d0) < 1e-9 and abs(o1 - d1) < 1e-9


@given(st.lists(PR, min_size=2, max_size=2), PR)
def test_sem_efeito_de_x_a_quantidade_e_zero(cel_z, pz):
    x = special.logit(np.array([cel_z[0], cel_z[1], cel_z[0], cel_z[1]]))
    assert abs(proj.quantidade(x)) < 1e-12
    assert -1 <= proj.quantidade(special.logit(np.array([cel_z[0], cel_z[1], cel_z[1], cel_z[0]]))) <= 1


@given(SEMENTE)
def test_formula_de_ajuste_coincide_com_intervencao_simulada(seed):
    rng = np.random.default_rng(seed)
    n = 60_000
    obs = simular_scm(n, rng)
    a0, a1 = ajuste_empirico(obs["z"], obs["x"], obs["y"], pz=0.5)
    s1 = simular_scm(n, rng, do_x=1)["y"].mean()
    s0 = simular_scm(n, rng, do_x=0)["y"].mean()
    tol = 6 * np.sqrt(0.11 * 0.89 / n * 4)      # 6 erros-padrão (folgados) de cada lado
    assert abs(a1 - s1) < tol and abs(a0 - s0) < tol
    assert abs((a1 - a0) - (s1 - s0)) < 2 * tol


@given(SEMENTE)
def test_braco_sorteado_quebra_a_porta_dos_fundos(seed):
    # no braço sorteado X ⊥ Z: P(Z | X) = P(Z) dentro do erro amostral
    v = simular_scm(40_000, np.random.default_rng(seed), frac_loteria=1.0)
    d = v["z"][v["x"] == 1].mean() - v["z"][v["x"] == 0].mean()
    assert abs(d) < 6 * np.sqrt(0.25 / 20_000 * 2)


@given(st.lists(st.integers(1, 500), min_size=4, max_size=4))
def test_identidade_de_kleinberg(c):
    vp, fp, fn, vn = c
    p, vpp, fnr, fpr = (vp + fn) / sum(c), vp / (vp + fp), fn / (vp + fn), fp / (fp + vn)
    assert abs(kleinberg_fpr(p, vpp, fnr) - fpr) < 1e-9


@given(st.integers(3, 7), st.data())
def test_sem_pais_o_vazio_e_admissivel(n, data):
    # num DAG qualquer, se x não tem pais (foi sorteado), o conjunto vazio satisfaz a porta dos fundos
    nos = [f"v{i}" for i in range(n)]
    pais = {nos[j]: [nos[i] for i in range(j) if data.draw(st.booleans())] for j in range(n)}
    pais[nos[0]] = []
    y = nos[data.draw(st.integers(1, n - 1))]
    assert porta_dos_fundos(pais, nos[0], y, [])
    # e nenhum descendente de x é admissível
    filhos = [b for b in nos if nos[0] in pais[b]]
    if filhos:
        assert not porta_dos_fundos(pais, nos[0], y, [filhos[0]])


def test_grafo_do_lab():
    assert porta_dos_fundos(GRAFO, "X", "Y", ["Z"]) and not porta_dos_fundos(GRAFO, "X", "Y", [])
    assert d_separados(GRAFO, ["W"], ["X"], ["Z"]) and not d_separados(GRAFO, ["W"], ["X"], [])


@given(SEMENTE, CEN)
def test_lote_cumpre_contrato_e_e_deterministico(seed, cen):
    a = proj.gerar_lote(4, np.random.default_rng(seed), cen, n=150)
    b = proj.gerar_lote(4, np.random.default_rng(seed), cen, n=150)
    assert a == b
    c = proj.contrato()
    loja = Loja.__new__(Loja)
    assert all(Loja.validar_linha(loja, r, c) is None for r in a)
    assert len({r["pessoa"] for r in a}) == len(a)


@given(SEMENTE)
def test_suficientes_contam_todas_as_pessoas(seed):
    v = simular_scm(500, np.random.default_rng(seed))
    k, n = contagens(v["z"], v["x"], v["y"])
    assert n.sum() == 500 and k.sum() == v["y"].sum() and np.all(k <= n)
    suf = proj.suficientes(v)
    assert np.isfinite(proj.logpost(proj.x_inicial(suf), suf, {"a": 20.0, "b": 150.0}))
