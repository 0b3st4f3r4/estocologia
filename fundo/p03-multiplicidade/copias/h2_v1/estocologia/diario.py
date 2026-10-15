"""Diário de Previsões do enxame (PLAN.md §4): livro-razão de probabilidades, Brier e capital de Kelly.

Toda previsão fica em [1%, 99%], nunca 0 ou 100%. Cada uma registra a fonte/filtro (M4), se o
sistema pode influir no resultado (M11), se é flutuação ou causa (M10), quantos fatores a
justificam (M12) e o degrau da escada de Pearl (M13). Ao resolver, cada previsor é pontuado por
Brier contra a climatologia e por um capital de Kelly: aposta contra uma banca que cota a
taxa-base, de modo que ln(capital) é a vantagem logarítmica sobre a climatologia.
"""

from __future__ import annotations

import json
import math
from dataclasses import asdict, dataclass
from pathlib import Path

PISO, TETO = 0.01, 0.99


@dataclass
class Previsao:
    id: str
    projeto: str
    previsor: str
    evento: str
    prob: float
    taxa_base: float
    fonte_filtro: str
    pode_influir: bool
    flutuacao_ou_causa: str
    n_fatores: int
    degrau_pearl: int
    resultado: int | None = None

    def __post_init__(self):
        if not PISO <= self.prob <= TETO:
            raise ValueError(f"{self.id}: probabilidade {self.prob} fora de [1%, 99%]")
        if self.degrau_pearl not in (1, 2, 3):
            raise ValueError(f"{self.id}: degrau de Pearl inválido")


class Diario:
    def __init__(self, caminho: Path):
        self.caminho = Path(caminho)
        self.itens: dict[str, Previsao] = {}

    def registrar(self, p: Previsao) -> None:
        if p.id in self.itens:
            raise ValueError(f"previsão {p.id} já registrada (pré-registro é imutável)")
        self.itens[p.id] = p

    def resolver(self, id_: str, resultado: int) -> None:
        p = self.itens[id_]
        if p.resultado is not None:
            raise ValueError(f"{id_} já resolvida")
        p.resultado = int(resultado)

    def placar(self, projeto: str | None = None) -> dict:
        """Brier, habilidade e capital por previsor; com `projeto`, só as previsões daquele projeto."""
        por: dict[str, dict] = {}
        for p in self.itens.values():
            if p.resultado is None or (projeto is not None and p.projeto != projeto):
                continue
            s = por.setdefault(p.previsor, {"n": 0, "brier": 0.0, "brier_clima": 0.0, "ln_capital": 0.0})
            y = p.resultado
            s["n"] += 1
            s["brier"] += (p.prob - y) ** 2
            s["brier_clima"] += (p.taxa_base - y) ** 2
            # Kelly contra a banca que cota a taxa-base: capital × p/q se ocorre, × (1−p)/(1−q) se não
            s["ln_capital"] += math.log(p.prob / p.taxa_base) if y else math.log((1 - p.prob) / (1 - p.taxa_base))
        for s in por.values():
            s["brier"] /= s["n"]
            s["brier_clima"] /= s["n"]
            s["habilidade"] = 1 - s["brier"] / s["brier_clima"] if s["brier_clima"] > 0 else 0.0
            # regra do orquestrador: quem perde para a climatologia ou tem capital em queda perde peso
            s["peso"] = 0.0 if (s["habilidade"] < 0 or s["ln_capital"] < 0) else 1.0
        return por

    def salvar(self) -> None:
        self.caminho.parent.mkdir(parents=True, exist_ok=True)
        self.caminho.write_text(json.dumps([asdict(p) for p in self.itens.values()], indent=1, ensure_ascii=False))
