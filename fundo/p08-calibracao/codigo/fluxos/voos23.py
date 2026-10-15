"""Fluxo voos23 (H3b): o mesmo fluxo `voos`, com cofre novo em 2023 e recalibração online do intercepto.

Declarado em CRITERIOS_H3b.md antes de rodar. O modelo, as features, o reserva, o disjuntor e os sensores são os
do fluxo `voos`. O que muda:
- os dados são os de 2023 (nycflights23), lidos pelo mesmo código através de um mapa de caminhos;
- o modelo oficial passa a ser p' = σ(logit p + δ). A cada rótulo resolvido, δ ← δ + η (y − p'), com η = 0,002.
"""

from __future__ import annotations

import numpy as np

from .base import sigmoide, ts
from .voos import Voos


class _Mapa:
    """Faz `dados / "h2" / "nycflights13_*.csv"` apontar para os arquivos de 2023 em dados-reais/h3b."""

    NOMES = {"nycflights13_flights.csv": "nycflights23_flights.csv", "nycflights13_weather.csv": "nycflights23_weather.csv"}

    def __init__(self, dados):
        self.base = dados / "h3b"

    def __truediv__(self, nome):
        return self if nome == "h2" else self.base / self.NOMES[nome]


class Voos23(Voos):
    nome = "voos23"
    corte = ts("2023-10-01")
    inicio = ts("2023-01-22")
    ETA = 0.002

    def eventos(self, dados):
        self.delta = 0.0
        self.p_oficial: dict = {}
        self.deltas: list = []
        return super().eventos(_Mapa(dados))

    def emitir(self, agora):
        out = super().emitir(agora)
        for p in out:
            if self.beta is not None:
                pm = float(np.clip(p["modelo"], 1e-4, 1 - 1e-4))
                pr = float(np.clip(sigmoide(np.log(pm / (1 - pm)) + self.delta), 0.01, 0.99))
                p["extra"] = {"modelo_cru": p["modelo"], "delta": self.delta}
                p["modelo"] = pr
            self.p_oficial[p["id"]] = p["modelo"]
        return out

    def resolver(self, agora):
        out = super().resolver(agora)
        for i, y, _ in out:
            p = self.p_oficial.pop(i, None)
            if p is not None and self.beta is not None:
                self.delta += self.ETA * (y - p)
        self.deltas.append(self.delta)
        return out
