"""K = 3 do H2g: a ausência no contrato de dados (ver ../../h1-real/DECLARACAO_F2_AUSENCIA.md).

A regra de `checar_tabela` é vacuosa no material de hoje — nenhuma das 18 tabelas tem valor ausente —, e o
último teste guarda essa premissa: é ela que permite a regra entrar sem mexer em número publicado.
"""

from pathlib import Path

import numpy as np
import pandas as pd

from estocologia import dominio as D

RAIZ = Path(__file__).resolve().parents[1]
DADOS = RAIZ.parent / "dados-reais" / "h2b"


def _pacote(**meta):
    base = {"nome": "x", "candidatos": {"x1": 1}, "familia": "gaussiana", "ambientes": {"cofre": ["c"]}}
    base.update(meta)
    return D.Pacote(raiz=Path("."), meta=base, preparar=None)


def _tabela(com_ausente=False):
    # dois ambientes de treino (a, b) e o cofre (c), que o pacote sintético declara
    df = pd.DataFrame({"y": [1.0, 2.0, 3.0, 4.0], "ambiente": ["a", "b", "c", "a"], "unidade": [1, 2, 3, 4],
                       "x1": [1.0, 2.0, 3.0, 4.0]})
    if com_ausente:
        df.loc[3, "x1"] = np.nan
    return df


def test_tabela_sem_ausente_passa_sem_o_bloco():
    assert D.checar_tabela(_pacote(), _tabela()) == []


def test_nao_declarado_com_coluna_ausente_e_recusado():
    erros = D.checar_tabela(_pacote(), _tabela(com_ausente=True))
    assert any("NAO_DECLARADO" in e for e in erros)


def test_mecanismo_declarado_cobrindo_a_coluna_passa():
    p = _pacote(ausencia={"mecanismo": "MCAR", "colunas": ["x1"], "sensibilidade": {"feito": False, "metodos": []}})
    assert D.checar_tabela(p, _tabela(com_ausente=True)) == []


def test_coluna_ausente_fora_da_lista_e_recusada():
    p = _pacote(ausencia={"mecanismo": "MCAR", "colunas": [], "sensibilidade": {"feito": False, "metodos": []}})
    erros = D.checar_tabela(p, _tabela(com_ausente=True))
    assert any("fora de ausencia.colunas" in e for e in erros)


def test_mecanismo_invalido_e_recusado_mesmo_sem_ausente():
    p = _pacote(ausencia={"mecanismo": "CMAR", "colunas": [], "sensibilidade": {"feito": False, "metodos": []}})
    assert any("mecanismo inválido" in e for e in D.checar_tabela(p, _tabela()))


def test_mnar_sem_sensibilidade_e_recusado():
    p = _pacote(ausencia={"mecanismo": "MNAR", "colunas": ["x1"], "sensibilidade": {"feito": False, "metodos": []}})
    erros = D.checar_tabela(p, _tabela(com_ausente=True))
    assert any("análise de sensibilidade" in e for e in erros)


def test_nenhuma_das_dezoito_tabelas_tem_valor_ausente():
    for pasta in sorted(p for p in (RAIZ / "dominios").iterdir() if (p / "pacote.yaml").exists()):
        p = D.carregar(pasta)
        df = p.preparar(DADOS)
        ausentes = {c: int(df[c].isna().sum()) for c in df.columns if bool(df[c].isna().any())}
        assert ausentes == {}, f"{p.nome}: {ausentes}"
