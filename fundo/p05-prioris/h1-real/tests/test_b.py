"""Portão 1 do Projeto B (dados reais, NSW × CPS): invariantes dos estimadores de ajuste, do placebo e dos lotes (Hypothesis)."""

import numpy as np
from hypothesis import given, strategies as st
from scipy import special

from estocologia.agentes import Loja
from projetos.b_real import (GRAFO, GRAFO_PLACEBO, GRAFO_PLACEBO_U, W_NEGRO, ProjetoB, ajustar_logit, att_ipw, carregar,
                             contagens, d_separados, ess_kish, estatistica_placebo, kleinberg_fpr, linhas_reais,
                             padronizar, particao, pesos_att, porta_dos_fundos, simular_teste)

PR = st.floats(0.02, 0.98)
SEMENTE = st.integers(0, 2**32 - 1)
CEN = st.sampled_from(["mundo", "patologico", "deriva"])
PER = st.sampled_from([1, 2, 3, 4, 97, 98, 99])
proj = ProjetoB()


def test_dados_reais_conferem():
    # as somas SHA-256 batem com dados-reais/SHA256SUMS e as contagens são as do Dehejia–Wahba e do CPS
    d = carregar()
    assert all(v["confere_SHA256SUMS"] for v in d["sha"].values())
    R = linhas_reais()
    assert len(R["tratados"]) == 185 and len(R["controles_nsw"]) == 260 and len(R["cps"]) == 15_992
    assert abs(W_NEGRO - np.mean([r["black"] for r in R["tratados"]])) < 1e-12
    # ATT experimental em renda de 1978: US$ 1 794 (Dehejia e Wahba, 1999)
    att = np.mean([r["re78"] for r in R["tratados"]]) - np.mean([r["re78"] for r in R["controles_nsw"]])
    assert abs(att - 1794) < 1


def test_particao_e_fixa_e_estratificada():
    per = particao()
    trat = np.array([r["treat"] for r in linhas_reais()["observacional"]])
    assert set(per.tolist()) == {1, 2, 3}
    assert all(abs((trat[per == p] == 1).sum() - 185 / 3) <= 1 for p in (1, 2, 3))
    assert np.array_equal(per, particao())


@given(st.lists(PR, min_size=4, max_size=4), st.floats(0.0, 1.0))
def test_padronizacao_e_mistura(cel, w):
    d0, d1 = padronizar(cel, w)
    assert min(cel[0], cel[1]) - 1e-12 <= d0 <= max(cel[0], cel[1]) + 1e-12
    assert min(cel[2], cel[3]) - 1e-12 <= d1 <= max(cel[2], cel[3]) + 1e-12


@given(st.lists(PR, min_size=2, max_size=2))
def test_sem_efeito_a_quantidade_e_zero(cel_g):
    x = special.logit(np.array([cel_g[0], cel_g[1], cel_g[0], cel_g[1]]))
    assert abs(proj.quantidade(x)) < 1e-12
    assert -1 <= proj.quantidade(special.logit(np.array([cel_g[0], cel_g[1], cel_g[1], cel_g[0]]))) <= 1


@given(st.lists(st.floats(1e-6, 1e6), min_size=1, max_size=60), st.floats(1e-3, 1e3))
def test_ess_kish_limites_e_escala(w, c):
    w = np.array(w)
    e = ess_kish(w)
    assert 1 - 1e-9 <= e <= w.size + 1e-9
    assert abs(ess_kish(c * w) - e) < 1e-6 * max(e, 1)
    assert abs(ess_kish(np.ones(w.size)) - w.size) < 1e-9


@given(SEMENTE, st.floats(0.05, 0.95))
def test_propensao_constante_devolve_o_contraste_cru(seed, pe):
    # com e constante os pesos são iguais: o ATT ponderado é a diferença crua de médias
    rng = np.random.default_rng(seed)
    t = (rng.random(300) < 0.4).astype(int)
    t[:2] = [0, 1]
    y = (rng.random(300) < 0.6).astype(int)
    att, p1, p0, ess = att_ipw(y, t, np.full(300, pe))
    assert abs(att - (y[t == 1].mean() - y[t == 0].mean())) < 1e-12
    assert abs(ess - (t == 0).sum()) < 1e-9


@given(SEMENTE)
def test_ponderacao_recupera_o_att_no_mundo_sintetico(seed):
    # no mundo sintético com seleção só em observáveis, a ponderação pelo escore estimado recupera o ATT verdadeiro;
    # a diferença crua, não
    rng = np.random.default_rng(seed)
    v = simular_teste(20_000, rng)
    X = np.column_stack([np.ones(v["d"].size), v["d"]])
    e = special.expit(X @ ajustar_logit(X, v["x"])[0])
    att, _, _, ess = att_ipw(v["y"], v["x"], e)
    assert abs(att - v["att"]) < 0.06
    assert abs((v["y"][v["x"] == 1].mean() - v["y"][v["x"] == 0].mean()) - v["att"]) > 0.1


@given(SEMENTE)
def test_logit_recupera_coeficientes(seed):
    rng = np.random.default_rng(seed)
    b = rng.uniform(-1, 1, 3)
    X = np.column_stack([np.ones(20_000), rng.standard_normal((20_000, 2))])
    y = (rng.random(20_000) < special.expit(X @ b)).astype(int)
    bh = ajustar_logit(X, y, l2=0.0)[0]
    assert np.max(np.abs(bh - b)) < 0.15


@given(SEMENTE)
def test_placebo_centrado_em_zero_sem_selecao_no_pseudo_desfecho(seed):
    # se o pseudo-desfecho W não depende de X dado D, a estatística do placebo fica perto de zero
    rng = np.random.default_rng(seed)
    v = simular_teste(20_000, rng)
    W = (rng.random(v["d"].size) < special.expit(0.3 - v["d"])).astype(int)
    X = np.column_stack([np.ones(v["d"].size), v["d"]])
    e = special.expit(X @ ajustar_logit(X, v["x"])[0])
    assert abs(estatistica_placebo(W, v["x"], e)) < 0.06
    # com um U não medido em X e em W, a estatística se afasta de zero
    p = simular_teste(20_000, rng, cenario="patologico")
    Wp = (rng.random(p["d"].size) < special.expit(0.3 - p["d"] - 1.5 * p["u"])).astype(int)
    Xp = np.column_stack([np.ones(p["d"].size), p["d"]])
    ep = special.expit(Xp @ ajustar_logit(Xp, p["x"])[0])
    assert estatistica_placebo(Wp, p["x"], ep) < -0.06


@given(SEMENTE)
def test_placebo_vetorizado_igual_ao_escalar(seed):
    rng = np.random.default_rng(seed)
    W = (rng.random(50) < 0.5).astype(float)
    e = rng.uniform(0.05, 0.95, (3, 50))
    x = (rng.random((3, 50)) < e).astype(float)
    x[:, 0], x[:, 1] = 1, 0
    v = estatistica_placebo(W, x, e)
    assert np.allclose(v, [estatistica_placebo(W, x[i], e[i]) for i in range(3)])


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
    filhos = [b for b in nos if nos[0] in pais[b]]
    if filhos:
        assert not porta_dos_fundos(pais, nos[0], y, [filhos[0]])


def test_grafos_do_projeto():
    assert porta_dos_fundos(GRAFO, "X", "Y", ["D", "R74", "R75"]) and not porta_dos_fundos(GRAFO, "X", "Y", [])
    assert not porta_dos_fundos(GRAFO, "X", "Y", ["D", "R74"])
    assert d_separados(GRAFO_PLACEBO, ["R75"], ["X"], ["D", "R74"])
    assert not d_separados(GRAFO_PLACEBO_U, ["R75"], ["X"], ["D", "R74"])


@given(SEMENTE, CEN, PER)
def test_lote_cumpre_contrato_e_e_deterministico(seed, cen, per):
    a = proj.gerar_lote(per, np.random.default_rng(seed), cen, n=150)
    b = proj.gerar_lote(per, np.random.default_rng(seed), cen, n=150)
    assert a == b and len(a) == 150
    c = proj.contrato()
    loja = Loja.__new__(Loja)
    assert all(Loja.validar_linha(loja, r, c) is None for r in a)
    assert len({r["pessoa"] for r in a}) == len(a)
    assert all(r["y"] == int(r["re78"] > 0) for r in a if not (cen == "patologico"))
    # proveniência: linhas reais só nos períodos 1–4 do mundo; o resto é sintético e declarado
    real = cen == "mundo" and per in (1, 2, 3, 4)
    assert all(r["sintetico"] == (0 if real else 1) for r in a)
    if cen == "patologico":
        assert all(r["re75"] == 0.0 for r in a if r["treat"] == 1)


def test_cofre_sao_os_controles_sorteados():
    v = proj.gerar_lote(4, np.random.default_rng(0))
    assert len(v) == 260 and all(r["fonte"] == "NSW-controle" and r["treat"] == 0 for r in v)
    treino = {r["pessoa"] for p in (1, 2, 3) for r in proj.gerar_lote(p, np.random.default_rng(0))}
    assert not treino & {r["pessoa"] for r in v}


@given(SEMENTE)
def test_suficientes_e_logpost(seed):
    lote = proj.gerar_lote(99, np.random.default_rng(seed), n=1500)
    cols = list(proj.contrato()["colunas"])
    d = {c: np.array([r[c] for r in lote]) for c in cols}
    k, n = contagens(d["y"], d["treat"], d["black"], np.full(len(lote), 0.3))
    assert np.all(k <= n + 1e-9) and np.all(k >= 0)
    assert abs(n[2] + n[3] - d["treat"].sum()) < 1e-9
    suf = proj.suficientes(d)
    assert np.isfinite(proj.logpost(proj.x_inicial(suf), suf, {"a": 5.0, "b": 4.0}))


@given(st.integers(16, 55), st.integers(0, 18))
def test_generalizacao_k_anonimato(idade, educ):
    r = proj.generalizar({"age": idade, "educ": educ, "raca": "negro", "marr": 0})
    assert r["age"] in ("16-24", "25-34", "35-55") and r["educ"] in ("0-11", "12", "13+")


def test_pesos_att_positivos():
    e = np.linspace(0.001, 0.999, 50)
    w = pesos_att(e)
    assert np.all(w > 0) and np.all(np.diff(w) > 0)
