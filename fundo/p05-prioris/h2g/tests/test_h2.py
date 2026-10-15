"""Portão 1 do H2: propriedades do núcleo (ICP, d-separação, amostrador, armazém, SHELF), com Hypothesis."""

import numpy as np
import pandas as pd
import pytest
from hypothesis import given, settings, strategies as st

from estocologia.amostrador import amostrar
from estocologia.armazem import Armazem, Recusa
from estocologia.dominio import ajustar_normal
from estocologia.dsep import d_separados, descendentes, porta_dos_fundos
from estocologia.glm import ajustar
from estocologia.icp import icp, p_invariancia

sementes = st.integers(0, 2**32 - 1)


def test_icp_contem_so_pais_numa_scm_com_intervencoes_em_x():
    # x1 → y → x2; os ambientes deslocam x1 e x2, nunca y dado x1. A garantia é probabilística:
    # P(Ŝ ⊆ pais) ≥ 1 − α; em 60 sementes, a fração de violações fica abaixo de 2α.
    viol = 0
    for seed in range(60):
        rng = np.random.default_rng(seed)
        n = 1500
        e = rng.integers(0, 3, n)
        x1 = rng.normal(e, 1)
        y = 1.5 * x1 + rng.normal(size=n)
        x2 = y + rng.normal(2.0 * e, 1)
        r = icp({"x1": x1, "x2": x2, "x3": rng.normal(size=n)}, y, e, "gaussiana")
        viol += not set(r["S_hat"]) <= {"x1"}
    assert viol / 60 <= 0.10


@given(sementes)
@settings(max_examples=25)
def test_teste_de_invariancia_e_valido_sob_h0(seed):
    # sob H0 o p-valor não é sistematicamente pequeno (ver calibração completa no README)
    rng = np.random.default_rng(seed)
    n = 600
    e = rng.integers(0, 2, n)
    x = rng.normal(e, 1)
    y = (rng.random(n) < 1 / (1 + np.exp(-(x - 0.3)))).astype(float)
    p = p_invariancia(np.column_stack([np.ones(n), x]), y, e, "binomial")
    assert 0 <= p <= 1


def test_calibracao_do_teste_de_invariancia():
    ps = []
    for seed in range(300):
        rng = np.random.default_rng(seed)
        n = 800
        e = rng.integers(0, 3, n)
        x = rng.normal(e, 1)
        y = 0.8 * x + rng.normal(size=n)
        ps.append(p_invariancia(np.column_stack([np.ones(n), x]), y, e, "gaussiana"))
    assert np.mean(np.array(ps) < 0.05) < 0.09


def test_porta_dos_fundos_casos_classicos():
    A = [("Z", "X"), ("Z", "Y"), ("X", "Y"), ("X", "M"), ("M", "Y"), ("X", "C"), ("Y", "C")]
    assert porta_dos_fundos(A, "X", "Y", ["Z"])["valido"]
    assert not porta_dos_fundos(A, "X", "Y", [])["valido"]          # confundidor aberto
    assert not porta_dos_fundos(A, "X", "Y", ["Z", "M"])["valido"]  # mediador é descendente
    assert not porta_dos_fundos(A, "X", "Y", ["Z", "C"])["valido"]  # colisor é descendente
    assert d_separados([("a", "c"), ("b", "c")], "a", "b", [])
    assert not d_separados([("a", "c"), ("b", "c")], "a", "b", ["c"])
    assert not d_separados([("a", "c"), ("b", "c"), ("c", "d")], "a", "b", ["d"])   # descendente do colisor abre


@given(st.lists(st.tuples(st.integers(0, 6), st.integers(0, 6)), min_size=1, max_size=15))
def test_d_separacao_e_simetrica(arestas):
    A = [(f"v{a}", f"v{b}") for a, b in arestas if a < b]   # a < b garante DAG
    if not A:
        return
    nos = sorted({n for e in A for n in e})
    x, y = nos[0], nos[-1]
    assert d_separados(A, x, y, []) == d_separados(A, y, x, [])
    assert x not in descendentes(A, x)


def test_amostrador_recupera_gaussiana_correlacionada():
    rng = np.random.default_rng(3)
    C = np.array([[1.0, 0.95, 0.0], [0.95, 1.0, 0.3], [0.0, 0.3, 2.0]])
    P = np.linalg.inv(C)
    m = np.array([1.0, -2.0, 0.5])
    r = amostrar(lambda x: -0.5 * (x - m) @ P @ (x - m), np.zeros(3), rng)
    A = r["amostras"].reshape(-1, 3)
    assert r["rhat_max"] < 1.01 and r["ess_min"] > 400
    assert np.allclose(A.mean(0), m, atol=0.08) and np.allclose(np.cov(A.T), C, atol=0.15)


@given(sementes)
@settings(max_examples=20)
def test_logistica_converge_ao_mv(seed):
    rng = np.random.default_rng(seed)
    n = 400
    X = np.column_stack([np.ones(n), rng.normal(size=n)])
    y = (rng.random(n) < 1 / (1 + np.exp(-(0.3 + 0.8 * X[:, 1])))).astype(float)
    f = ajustar(X, y, "binomial")
    grad = X.T @ (y - f["mu"])
    assert np.max(np.abs(grad)) < 1e-4


@given(st.floats(-5, 5), st.floats(0.05, 3))
def test_shelf_normal_recupera_quartis(m, s):
    from scipy import stats
    q = {f"q{int(p * 100):02d}": m + s * stats.norm.ppf(p) for p in (0.05, 0.25, 0.5, 0.75, 0.95)}
    r = ajustar_normal(q)
    assert abs(r["media"] - m) < 1e-5 and abs(r["dp"] - s) < 1e-5


def test_armazem_recusa_e_contabiliza(tmp_path):
    arm = Armazem(tmp_path, orcamento_por_agente=10_000)
    df = pd.DataFrame({"y": np.arange(100.0), "x": np.arange(100.0), "z": np.arange(100.0), "ambiente": ["a"] * 50 + ["b"] * 50})
    arm.ingerir("d", df, cofre=["b"])
    with pytest.raises(Recusa):
        arm.ler("d", ["*"], "agent-f2-data")
    with pytest.raises(Recusa):                       # cofre antes do pré-registro
        arm.ler("d", ["y"], "agent-f2-data")
    t = arm.ler("d", ["y", "x"], "agent-f2-data", excluir=["b"])
    assert list(t.columns) == ["y", "x"] and len(t) == 50
    reg = arm.registro[-1]
    assert reg["explain_projecao"] and set(reg["explain_projecao"]) <= {"y", "x"} and reg["bytes_varridos"] < reg["bytes_tabela"]
    with pytest.raises(Recusa):
        arm.destrancar("d", "0" * 64, "agent-f4-frontier")   # só o F3 abre
    arm.destrancar("d", "a" * 64, "agent-f3-lifecycle")
    assert len(arm.ler("d", ["y"], "agent-f3-lifecycle", ambientes=["b"])) == 50
    arm2 = Armazem(tmp_path, orcamento_por_agente=1)
    arm2.cofres["d"] = set()
    with pytest.raises(Recusa):
        arm2.ler("d", ["y"], "agent-f2-data")         # estouro de orçamento


def test_icp_gl_pelo_posto_com_x_constante_no_ambiente():
    # H2b: X constante dentro de cada ambiente (a intervenção fixa X). Sob H0 (y = 2x + ruído, ambientes só mudam x)
    # o teste não pode rejeitar demais; com y deslocado por ambiente fora de x, tem de rejeitar.
    rej_h0, rej_h1 = 0, 0
    for seed in range(200):
        rng = np.random.default_rng(seed)
        e = np.repeat(np.arange(4), 300)
        x = np.array([0.0, 0.25, 0.5, 0.95])[e]
        X = np.column_stack([np.ones(e.size), x])
        y0 = 2 * x + rng.normal(size=e.size)
        y1 = y0 + np.array([0, 0.3, -0.2, 0.1])[e]
        rej_h0 += p_invariancia(X, y0, e, "gaussiana") < 0.05
        rej_h1 += p_invariancia(X, y1, e, "gaussiana") < 0.05
    assert rej_h0 / 200 < 0.09
    assert rej_h1 / 200 > 0.8
