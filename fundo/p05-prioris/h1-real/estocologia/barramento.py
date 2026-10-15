"""Barramento Estruturado de Fichas (Arquitetura §3; AGENTS.md §I.1 e §IV).

Os agentes não conversam em prosa: cada um publica fichas tipadas num livro-razão append-only,
encadeado por hash (a trilha imutável de auditoria que a F5 exige). Toda ficha abre com o
cabeçalho obrigatório das 5 perguntas.
"""

from __future__ import annotations

import hashlib
import json
from dataclasses import asdict, dataclass, field
from pathlib import Path

AGENTES = {
    "agent-f1-code", "agent-f2-data", "agent-f3-lifecycle", "agent-f4-frontier",
    "agent-f5-governance", "agent-f6-fairness", "agent-f7-domain", "agent-f8-decision",
    "agent-m15-orchestrator",
}
CABECALHO = ("processo_gerador", "nao_linearidade", "unidade_e_margem", "fisica_do_ruido", "escada_causal")
ESTAGIOS = {1: "traducao", 2: "ingestao_etica", 3: "simulacao_fronteira", 4: "lentes", 5: "banca", 6: "decisao"}


class FichaInvalida(ValueError):
    pass


def _canon(obj) -> str:
    return json.dumps(obj, sort_keys=True, ensure_ascii=False, default=float)


@dataclass
class Ficha:
    projeto: str
    estagio: int
    agente: str
    tipo: str
    cabecalho: dict
    payload: dict
    pais: list = field(default_factory=list)  # hashes das fichas lidas
    hash: str = ""
    anterior: str = ""

    def validar(self) -> None:
        if self.agente not in AGENTES:
            raise FichaInvalida(f"agente desconhecido: {self.agente}")
        if self.estagio not in ESTAGIOS:
            raise FichaInvalida(f"estágio inválido: {self.estagio}")
        faltam = [k for k in CABECALHO if not str(self.cabecalho.get(k, "")).strip()]
        if faltam:
            raise FichaInvalida(f"{self.tipo}: cabeçalho sem {faltam}")
        if not isinstance(self.payload, dict) or not self.payload:
            raise FichaInvalida(f"{self.tipo}: payload vazio")
        _canon(self.payload)  # precisa ser serializável

    def corpo(self) -> dict:
        d = asdict(self)
        d.pop("hash")
        return d


class Barramento:
    """Livro-razão JSONL. Cada linha guarda o hash da anterior; adulterar uma quebra a cadeia."""

    def __init__(self, caminho: Path):
        self.caminho = Path(caminho)
        self.caminho.parent.mkdir(parents=True, exist_ok=True)
        self.fichas: list[Ficha] = []
        if self.caminho.exists():
            for linha in self.caminho.read_text().splitlines():
                self.fichas.append(Ficha(**json.loads(linha)))

    @property
    def ultimo(self) -> str:
        return self.fichas[-1].hash if self.fichas else "0" * 64

    def publicar(self, ficha: Ficha) -> Ficha:
        ficha.validar()
        ficha.anterior = self.ultimo
        ficha.hash = hashlib.sha256(_canon(ficha.corpo()).encode()).hexdigest()
        self.fichas.append(ficha)
        with self.caminho.open("a") as f:
            f.write(_canon(asdict(ficha)) + "\n")
        return ficha

    def ler(self, projeto: str, tipo: str) -> Ficha:
        """A ficha mais recente de um tipo, num projeto (é o que um agente consome)."""
        for f in reversed(self.fichas):
            if f.projeto == projeto and f.tipo == tipo:
                return f
        raise KeyError(f"nenhuma ficha {tipo} em {projeto}")

    def verificar(self) -> list[str]:
        """Recalcula a cadeia; devolve as quebras encontradas."""
        erros, ant = [], "0" * 64
        for i, f in enumerate(self.fichas):
            if f.anterior != ant:
                erros.append(f"linha {i}: elo anterior não confere")
            h = hashlib.sha256(_canon(f.corpo()).encode()).hexdigest()
            if h != f.hash:
                erros.append(f"linha {i} ({f.tipo}): conteúdo adulterado")
            ant = f.hash
        return erros
