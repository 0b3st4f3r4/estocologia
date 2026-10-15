"""Portão 1 do H3: propriedades do núcleo (armazém temporal, sensores, disjuntor, calibrador, métricas)."""

import numpy as np
import pandas as pd
import pytest
from hypothesis import given, settings, strategies as st

from estocologia import calibracao as C
from estocologia.armazem import Armazem, Recusa, Relogio
from estocologia.monitor import CalibradorQuantis, Disjuntor, SensorDeriva


def _arm(tmp_path, corte=5_000, grupo=8):
    rel = Relogio(0)
    arm = Armazem(tmp_path, rel, corte_cofre=corte)
    arm.ingerir("t", pd.DataFrame({"x": np.arange(100.0), "t_disp": np.arange(0, 10_000, 100)}), grupo)
    return rel, arm


@given(st.integers(0, 4_000), st.integers(1, 900))
@settings(max_examples=40)
def test_janela_devolve_exatamente_o_intervalo(desde, largura):
    import tempfile
    from pathlib import Path
    with tempfile.TemporaryDirectory() as tmp:
        rel, arm = _arm(Path(tmp))
        ate = min(desde + largura, 4_999)
        rel.avancar(ate)
        df = arm.ler("t", ["x"], "agent-f2-data", desde, ate)
        esperado = [t for t in range(0, 10_000, 100) if desde < t <= ate]
        assert sorted(df["t_disp"].tolist()) == esperado


def test_armazem_recusa_futuro_cofre_estrela_e_segunda_abertura(tmp_path):
    rel, arm = _arm(tmp_path)
    rel.avancar(1_000)
    for f in (lambda: arm.ler("t", ["x"], "a", 0, 1_001), lambda: arm.ler("t", ["*"], "a", 0, 500)):
        with pytest.raises(Recusa):
            f()
    rel.avancar(6_000)
    with pytest.raises(Recusa):
        arm.ler("t", ["x"], "agent-f4-frontier", 0, 6_000)
    with pytest.raises(Recusa):
        arm.destrancar("a" * 64, "agent-f4-frontier")
    arm.destrancar("a" * 64, "agent-f3-lifecycle")
    with pytest.raises(Recusa):
        arm.destrancar("b" * 64, "agent-f3-lifecycle")
    with pytest.raises(Recusa):
        rel.avancar(10)
    assert len(arm.ler("t", ["x"], "agent-f4-frontier", 0, 6_000)) == 60


def test_varredura_incremental_e_O_tabela(tmp_path):
    rel, arm = _arm(tmp_path, corte=10**9, grupo=1)   # grupo de linhas do tamanho do tick
    ult = -1
    for t in range(0, 10_000, 100):
        rel.avancar(t)
        arm.ler("t", ["x"], "a", ult, t)
        ult = t
    eq = arm.resumo_varredura()["por_tabela"]["t"]["varreduras_equivalentes"]
    assert eq <= 1.5


def test_sensor_de_deriva_calado_no_estacionario_e_alarma_no_deslocamento():
    rng = np.random.default_rng(1)
    s = SensorDeriva("x", n_janela=200, n_ref=1000)
    s.alimentar(rng.normal(size=1000 + 200 * 100), 0, calibrando=True)
    s.fixar_limiares()
    ev = s.alimentar(rng.normal(size=200 * 100), 1, calibrando=False)
    assert sum(e["evento"] == "ligou" for e in ev) <= 1
    ev = s.alimentar(rng.normal(0.5, 1, size=200 * 5), 2, calibrando=False)
    assert any(e["evento"] == "ligou" for e in ev)


def test_disjuntor_abre_e_fecha_com_histerese():
    d = Disjuntor(janela=50, margem=0.05, k_on=3, k_off=3)
    evs = []
    for i in range(200):
        d.alimentar(2.0, 1.0)
        e = d.verificar(i)
        evs.append(e["evento"] if e else None)
    assert "abriu" in evs and d.aberto
    for i in range(200):
        d.alimentar(0.5, 1.0)
        e = d.verificar(1000 + i)
        if e:
            evs.append(e["evento"])
    assert evs[-1] == "fechou" and not d.aberto


def test_calibrador_adaptativo_cobre_sob_cauda_pesada():
    rng = np.random.default_rng(2)
    cal = CalibradorQuantis([0.05, 0.5, 0.95])
    dentro = []
    for i in range(6000):
        q = cal.quantis(np.array([0.0]), np.array([1.0]))[0]
        y = rng.standard_t(3) * 2.0                 # escala errada de propósito
        if i > 1000:
            dentro.append(q[0] <= y <= q[2])
        cal.atualizar(y, 0.0, 1.0, q)
    assert 0.87 <= np.mean(dentro) <= 0.93


@given(st.integers(0, 2**32 - 1))
@settings(max_examples=25)
def test_climatologia_movel_nao_olha_adiante(seed):
    rng = np.random.default_rng(seed)
    n = 200
    te = np.sort(rng.integers(0, 10_000, n))
    df = pd.DataFrame({"t_emissao": te, "t_rotulo": te + rng.integers(1, 500, n), "y": rng.integers(0, 2, n).astype(float)})
    c1 = C.climatologia_movel(df)
    df2 = df.copy()
    futuro = df2["t_rotulo"] > 5_000
    df2.loc[futuro, "y"] = 1 - df2.loc[futuro, "y"]
    c2 = C.climatologia_movel(df2)
    assert np.allclose(c1[te <= 5_000], c2[te <= 5_000])


def test_kupiec_aceita_taxa_nominal_e_rejeita_o_dobro():
    rng = np.random.default_rng(3)
    assert C.kupiec(rng.random(20_000) < 0.01, 0.01)["p"] > 0.01
    assert C.kupiec(rng.random(20_000) < 0.02, 0.01)["p"] < 1e-6
