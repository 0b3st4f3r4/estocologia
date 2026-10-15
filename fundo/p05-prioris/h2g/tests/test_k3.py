"""H2d: testes da regra de saturação K = 3 (ver ../DECLARACAO_K3.md)."""

import numpy as np

from estocologia.icp import acerto_1nn, icp, saturado_k3


def test_x_discreto_que_determina_o_ambiente_e_saturado():
    e = np.repeat(np.arange(4), 100)
    x = np.array([0.0, 0.25, 0.5, 0.95])[e]
    assert acerto_1nn([x[:, None]], e) == 1.0
    assert saturado_k3([x[:, None]], e)


def test_x_continuo_em_faixas_disjuntas_e_saturado():
    rng = np.random.default_rng(3)
    e = np.repeat(np.arange(3), 300)
    x = rng.uniform(0, 1, e.size) + 2.0 * e          # cada ambiente numa faixa própria
    assert saturado_k3([x[:, None]], e)


def test_x_sobreposto_ou_independente_nao_e_saturado():
    rng = np.random.default_rng(4)
    e = np.repeat(np.arange(3), 400)
    x = rng.normal(0.5 * e, 1.0)                      # o ambiente desloca X, mas as faixas se sobrepõem
    assert not saturado_k3([x[:, None]], e)
    d = rng.integers(0, 3, e.size).astype(float)      # discreto independente do ambiente
    assert not saturado_k3([d[:, None]], e)
    assert not saturado_k3([], e)                     # S = ∅
    # empates exatos: célula mista conta a fração de vizinhos no mesmo ambiente
    e2 = np.array([0, 0, 1, 1])
    assert acerto_1nn([np.zeros((4, 1))], e2) == 1 / 3


def test_icp_exclui_saturados_da_familia():
    # x é constante no ambiente (como a coparticipação do RAND): {x} é aceito trivialmente, mas é saturado.
    # Os outros conjuntos são rejeitados; com a regra, nada não saturado é aceito e Ŝ = ∅.
    rng = np.random.default_rng(5)
    e = np.repeat(np.arange(3), 1500)
    x = np.array([0.0, 0.25, 0.95])[e]
    y = 2.0 * x + rng.standard_t(3, e.size)
    filho = y + rng.normal(1.0 * e, 1.0)
    blocos = {"x": x[:, None], "filho": filho[:, None], "ruido": rng.normal(size=e.size)[:, None]}
    sem = icp(blocos, y, e, "gaussiana", regra_saturacao=False)
    com = icp(blocos, y, e, "gaussiana", regra_saturacao=True)
    assert sem["S_hat"] == ["x"]
    assert com["S_hat"] == [] and com["modelo_rejeitado"]
    assert all("x" not in S for S in com["aceitos"])
    assert com["regra_saturacao"]["aceitos_saturados"] >= 1
    assert com["regra_saturacao"]["S_hat_sem_a_regra"] == ["x"]


def test_icp_sem_saturacao_nao_muda_nada():
    # o SCM do H2 (x1 → y → x2, intervenções contínuas que se sobrepõem): a regra não remove nenhum conjunto
    rng = np.random.default_rng(11)
    n = 1500
    e = rng.integers(0, 3, n)
    x1 = rng.normal(e, 1)
    y = 1.5 * x1 + rng.normal(size=n)
    x2 = y + rng.normal(2.0 * e, 1)
    b = {"x1": x1, "x2": x2, "x3": rng.normal(size=n)}
    com = icp(b, y, e, "gaussiana")
    sem = icp(b, y, e, "gaussiana", regra_saturacao=False)
    assert com["regra_saturacao"]["saturados_testados"] == 0
    assert com["S_hat"] == sem["S_hat"] and com["aceitos"] == sem["aceitos"]
