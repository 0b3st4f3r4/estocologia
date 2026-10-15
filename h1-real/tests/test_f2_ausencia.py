"""K = 2 do H1 real: o mecanismo da ausência no contrato de dados do F2 (ver ../DECLARACAO_F2_AUSENCIA.md)."""

from portoes import MECANISMOS_DA_AUSENCIA, checar_ausencia
from rodar import projetos


def _contrato(nome):
    return projetos()[nome]().contrato()


def test_os_tres_contratos_declaram_a_ausencia():
    for nome in ("a", "b", "c"):
        c = _contrato(nome)
        assert checar_ausencia(c) == [], nome
        assert c["ausencia"]["mecanismo"] in MECANISMOS_DA_AUSENCIA
        assert c["ausencia"]["sensibilidade"] == {"feito": False, "metodos": []}
        assert c["ausencia"]["nota"].strip()


def test_contrato_sem_o_campo_e_recusado():
    c = _contrato("a")
    c.pop("ausencia")
    assert "contrato sem o campo ausencia (mecanismo da ausência não declarado)" in checar_ausencia(c)


def test_mecanismo_invalido_e_recusado():
    c = _contrato("a")
    c["ausencia"]["mecanismo"] = "CMAR"
    assert any("mecanismo inválido" in f for f in checar_ausencia(c))


def test_coluna_fora_do_contrato_e_recusada():
    c = _contrato("c")
    c["ausencia"]["colunas"] = ["perda", "coluna_que_nao_existe"]
    assert any("fora do contrato" in f for f in checar_ausencia(c))


def test_mnar_sem_sensibilidade_e_recusado_e_com_ela_passa():
    c = _contrato("b")
    c["ausencia"]["mecanismo"] = "MNAR"
    assert any("análise de sensibilidade" in f for f in checar_ausencia(c))
    c["ausencia"]["sensibilidade"] = {"feito": True, "metodos": ["Meng: razão de chances do desfecho dado observado e não observado"]}
    assert checar_ausencia(c) == []


def test_sensibilidade_incoerente_e_recusada():
    c = _contrato("b")
    c["ausencia"]["sensibilidade"] = {"feito": False, "metodos": ["método declarado sem o feito"]}
    assert any("feito = False" in f for f in checar_ausencia(c))


def test_sensibilidade_em_tipo_errado_e_recusada():
    c = _contrato("b")
    c["ausencia"]["sensibilidade"] = {"feito": "sim", "metodos": []}
    assert any("sensibilidade tem de ser" in f for f in checar_ausencia(c))
