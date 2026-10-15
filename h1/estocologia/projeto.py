"""Interface que um projeto de pesquisa implementa para rodar no ciclo de 6 estágios.

Os agentes F1–F8 e o orquestrador M15 são genéricos; o projeto só declara o seu mundo, o seu
modelo e a sua decisão. `dados` é sempre um dicionário {coluna: np.ndarray}, lido do
armazenamento colunar pelo agent-f2-data com projeção explícita.
"""

from __future__ import annotations

from abc import ABC, abstractmethod

import numpy as np


class Projeto(ABC):
    nome: str                 # identificador curto (a, b, c)
    titulo: str
    alegacao: str             # a alegação empírica sob exame, entre aspas
    cabecalho: dict           # as 5 perguntas do payload (AGENTS.md §IV)
    periodos: tuple = (1, 2, 3, 4)   # o último fica no cofre out-of-time do agent-f3-lifecycle
    unidade: str              # nome da coluna da unidade independente
    colunas_modelo: tuple     # projeção que os agentes de modelagem podem ler
    quase_ids: tuple = ()
    custo_inferencia: float = 1e-6   # custo, em unidades de utilidade, de rodar a inferência pesada

    # ---------- F7 · Tradução, elicitação e escopo ----------
    @abstractmethod
    def traducao(self) -> dict: ...                # pergunta substantiva, glossário (10 termos), fontes
    @abstractmethod
    def elicitacao(self) -> dict: ...              # especialistas (quartis por pessoa), quantidade, nota
    @abstractmethod
    def termo_escopo(self) -> dict: ...

    # ---------- F2 · Dados ----------
    @abstractmethod
    def contrato(self) -> dict: ...                # {colunas: {nome: {tipo, min, max, nulo}}, chave, particao}
    @abstractmethod
    def gerar_lote(self, periodo: int, rng, cenario: str = "mundo") -> list[dict]: ...
    def generalizar(self, linha: dict) -> dict:    # F5: generalização para k-anonimato
        return linha

    # ---------- F1/F4 · Modelo ----------
    dim: int
    @abstractmethod
    def suficientes(self, dados: dict) -> dict: ...
    @abstractmethod
    def logpost(self, x: np.ndarray, suf: dict, priori: dict) -> float: ...   # x irrestrito (logits)
    @abstractmethod
    def amostra_priori(self, priori: dict, rng) -> np.ndarray: ...            # x irrestrito
    @abstractmethod
    def simular_suf(self, x: np.ndarray, rng) -> dict: ...                    # dados pequenos para SBC
    @abstractmethod
    def quantidade(self, x: np.ndarray) -> float: ...                         # a quantidade de interesse
    @abstractmethod
    def x_inicial(self, suf: dict) -> np.ndarray: ...

    @abstractmethod
    def nulo(self, dados: dict, rng) -> dict: ...  # F1/M5: o modelo nulo e o teste severo
    @abstractmethod
    def ppc(self, dados: dict, amostras_x: np.ndarray, rng) -> dict: ...      # checagem preditiva (M15)
    @abstractmethod
    def resultado_unidade(self, dados: dict) -> np.ndarray: ...               # um escore por unidade (conforme)

    # ---------- F3 · Previsões out-of-time ----------
    # O cofre é dividido pelo agent-f3-lifecycle: o projeto só vê as "entradas" das unidades do
    # cofre para prever, e o F3 abre os desfechos depois de registrar as previsões.
    @abstractmethod
    def entradas_cofre(self, cofre: dict) -> dict: ...   # {"ids": [...], "deriva": np.ndarray, ...}
    @abstractmethod
    def previsoes(self, treino: dict, entradas: dict, amostras_x: np.ndarray) -> list[dict]: ...
    # cada item: {id, evento, taxa_base, probs: {previsor: p}, meta: {fonte_filtro, pode_influir,
    #             flutuacao_ou_causa, n_fatores, degrau_pearl}}
    @abstractmethod
    def desfechos(self, cofre: dict) -> dict: ...        # {id: 0/1}

    # ---------- F5 / F6 ----------
    @abstractmethod
    def estatistica_publicada(self, dados: dict) -> tuple[int, int, str]: ...  # (k, n, descrição)
    @abstractmethod
    def justica(self, ctx: dict) -> dict: ...

    # ---------- M15 · Lentes e crítica ----------
    @abstractmethod
    def lentes(self, ctx: dict) -> dict: ...
    @abstractmethod
    def critica(self, ctx: dict) -> dict: ...      # modelo rival, parâmetros importados, performatividade, premissas

    # ---------- F8 · Decisão ----------
    @abstractmethod
    def acoes(self) -> list: ...
    @abstractmethod
    def utilidade(self, acao, theta: float) -> float: ...
    @abstractmethod
    def decisao(self, ctx: dict) -> dict: ...      # C/L, frequências naturais, HOPs (amostras), regra
    @abstractmethod
    def disjuntor(self, dados: dict, ctx: dict) -> dict: ...   # {"disparou": bool, "motivo": str}
    @abstractmethod
    def priori_atipica(self) -> dict: ...          # MB7: cenário atípico do domínio
