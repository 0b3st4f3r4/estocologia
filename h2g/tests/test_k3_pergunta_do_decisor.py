"""K = 3 do H2g: a pergunta do decisor (ver ../DECLARACAO_K3_pergunta_do_decisor.md).

Erro de tipo III (livro, seção 14.6): a pergunta que o modelo responde tem de ser a que o decisor fez. O teste
recusa pacote sem o campo e pacote com texto genérico, e confere que os 18 pacotes do H2g/ preenchem a regra.
"""

import copy
from pathlib import Path

import yaml

from estocologia import dominio as D

RAIZ = Path(__file__).resolve().parents[1]


def _base():
    """Pacote sintético mínimo, válido exceto pelo campo em teste."""
    return {
        "nome": "x", "area": "A", "titulo": "T", "pergunta": "P", "alegacao": "a",
        "glossario": {str(i): "termo" for i in range(10)}, "fontes": ["f"], "arquivos": ["a"],
        "familia": "gaussiana", "desfecho": "d", "chave": "x1", "candidatos": {"x1": {"tipo": "num"}},
        "icp_candidatos": ["x1"], "ajuste": [], "dag": [["x1", "y"], ["y", "x1"]],
        "ambientes": {"cofre": ["c"]}, "unidade": "u", "cabecalho": {k: "v" for k in D.CABECALHO},
        "shelf": {"quartis": {q: 1 for q in ("q05", "q25", "q50", "q75", "q95")}, "fonte": "f"},
        "decisao": {"acao": "usar a alavanca A", "beneficio_por_unidade": 1, "custo": 1, "custo_pesquisa": 0.1},
        "termo_escopo": {"vale_para": "a", "nao_vale_para": "b"},
    }


def test_sem_o_campo_e_recusado():
    assert "falta pergunta_do_decisor" in D.validar(_base())


def test_texto_generico_e_recusado():
    # sem as etiquetas: não nomeia quem decide nem a ação que muda
    m = copy.deepcopy(_base())
    m["pergunta_do_decisor"] = ("O decisor quer decidir melhor a partir dos dados e responder à pergunta que importa "
                                "para o modelo e para os dados, sem lugar-comum nenhum aqui?")
    assert D.validar(m)
    # com as etiquetas, mas quem decide é um lugar-comum
    m = copy.deepcopy(_base())
    m["pergunta_do_decisor"] = ("Quem decide: quem decide a dose. Ação que a decisão muda: usar a alavanca A em todo "
                                "o lote. A pergunta do decisor é: a dose cheia compensa em todo o lote?")
    assert any("genérica" in e for e in D.validar(m))
    # com quem decide nomeado, mas sem copiar a ação do campo decisao
    m = copy.deepcopy(_base())
    m["pergunta_do_decisor"] = ("Quem decide: o produtor do lote. Ação que a decisão muda: aplicar dose cheia em todo "
                                "o lote. A pergunta do decisor é: a dose cheia compensa em todo o lote?")
    assert any("não copia a ação" in e for e in D.validar(m))


def test_texto_declarado_passa():
    m = copy.deepcopy(_base())
    m["pergunta_do_decisor"] = ("Quem decide: o produtor do lote. Ação que a decisão muda: usar a alavanca A em todo o "
                                "lote. A pergunta do decisor é: a dose cheia compensa em todo o lote?")
    assert D.validar(m) == []


def test_os_18_pacotes_declaram_a_pergunta_do_decisor():
    pastas = sorted(p for p in (RAIZ / "dominios").iterdir() if (p / "pacote.yaml").exists())
    assert len(pastas) == 18
    for pasta in pastas:
        meta = yaml.safe_load((pasta / "pacote.yaml").read_text())
        culpados = [e for e in D.validar(meta) if "pergunta_do_decisor" in e]
        assert culpados == [], f"{pasta.name}: {culpados}"
