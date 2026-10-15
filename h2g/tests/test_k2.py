"""H2c: testes do teste de invariância K = 2 (resíduos não paramétricos; ver ../h2b/DECLARACAO_K2.md)."""

import numpy as np

from calibracao import gerar, taxas
from estocologia.icp import _saturado, expandir, icp, p_invariancia_np


def test_expansao_discreto_vira_fator_e_continuo_vira_spline():
    rng = np.random.default_rng(0)
    d = rng.integers(0, 4, 500).astype(float)
    B, disc = expandir(d[:, None])
    assert disc and B.shape[1] == 3
    c = rng.normal(size=500)
    B, disc = expandir(c[:, None])
    assert not disc and B.shape[1] == 3          # linear + 2 termos da spline restrita (4 nós)
    cat = np.column_stack([(d == 1), (d == 2)]).astype(float)
    B, disc = expandir(cat)
    assert disc and B.shape[1] == 2


def test_calibracao_h0_x_discreto_cauda_pesada_n_grande():
    # critério declarado: rejeição a 5% ≤ 0,09 em ≥ 200 sementes, X discreto, ruído t3 / log-normal, n ≥ 4 000
    for c in ("discreto_t3", "discreto_lognormal", "discreto_hetero_t3"):
        r = taxas(c, 200, n=4500)
        assert r["rejeicao_K2"] <= 0.09, r
    r = taxas("discreto_t3", 200, n=15_000)
    assert r["rejeicao_K2"] <= 0.09, r


def test_calibracao_h0_x_continuo_nao_linear():
    r = taxas("cont_t3", 200, n=4500)
    assert r["rejeicao_K2"] <= 0.09, r


def test_poder_quando_o_ambiente_desloca_y_dado_x():
    for c in ("H1_locacao", "H1_escala"):
        r = taxas(c, 200, n=4500)
        assert r["rejeicao_K2"] > 0.8, r


def test_cobertura_por_frequencia_scm_com_intervencoes_em_x():
    # o teste do H2: x1 → y → x2; ambientes deslocam x1 e x2, nunca y dado x1. P(Ŝ ⊆ pais) ≥ 1 − α;
    # em 60 sementes, a fração de violações fica ≤ 2α.
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


def test_cobertura_por_frequencia_x_discreto_cauda_pesada():
    # mesma garantia com pai discreto de resposta côncava, ruído t3 e um filho que o ambiente desloca
    viol, nao_vazio = 0, 0
    for seed in range(60):
        rng = np.random.default_rng(500 + seed)
        y, e, x = gerar("discreto_t3", rng, 3000)
        filho = y + rng.normal(1.0 * e, 1)
        r = icp({"x": x, "filho": filho, "ruido": rng.normal(size=y.size)}, y, e, "gaussiana")
        viol += not set(r["S_hat"]) <= {"x"}
        nao_vazio += r["S_hat"] == ["x"]
    assert viol / 60 <= 0.10
    assert nao_vazio / 60 > 0.5            # e o teste tem poder para achar o pai


def test_saturacao_detecta_x_que_determina_o_ambiente():
    e = np.repeat(np.arange(4), 100)
    x = np.array([0.0, 0.25, 0.5, 0.95])[e]
    B, _ = expandir(x[:, None])
    assert _saturado([B], e)
    rng = np.random.default_rng(1)
    B2, _ = expandir(rng.integers(0, 3, 400).astype(float)[:, None])
    assert not _saturado([B2], e)
    # com X constante no ambiente, Y ⊥ E | X vale por construção: nem um deslocamento por ambiente é rejeitado
    y = 2 * x + np.array([0, 0.5, -0.5, 0.3])[e] + rng.normal(size=e.size)
    assert p_invariancia_np(np.hstack([np.ones((400, 1)), B]), y, e) > 0.05
