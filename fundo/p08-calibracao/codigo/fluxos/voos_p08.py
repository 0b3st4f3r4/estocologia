"""Fluxos do ponto 8 (fundo/p08-calibracao/DECLARACAO.md, item c). Todos herdam de Voos23 sem mudar modelo, features,
reserva, disjuntor, sensores nem a recalibração do intercepto. O que muda é só a leitura dos dados:

- Voos17: NY 2017 (jayleetx/nycflights), corte 2017-10-01, início 2017-01-22.
- VoosPNW14: Portland e Seattle 2014 (ismayc/pnwflights14), corte 2014-10-01, início 2014-01-22; aeroportos PDX e
  SEA no lugar de EWR, JFK e LGA.
- Voos23E1 / Voos23E2: o cofre de 2023 com deriva de rótulo injetada (estresse S1 degrau, S2 perfil mensal de 2013).
"""

from __future__ import annotations

import json
from pathlib import Path

import numpy as np
import pandas as pd

from .base import ts
from .voos23 import Voos23

PARAM = Path(__file__).resolve().parents[2] / "saida" / "parametros_estresse.json"
SEMENTE = 20261015


class _MapaP08:
    def __init__(self, dados, prefixo):
        self.base, self.prefixo = dados / "p08", prefixo

    def __truediv__(self, nome):
        if nome == "h2":
            return self
        return self.base / nome.replace("nycflights13", self.prefixo)


class Voos17(Voos23):
    nome = "voos17"
    corte = ts("2017-10-01")
    inicio = ts("2017-01-22")
    PREFIXO = "nyc17"

    def eventos(self, dados):
        self.delta = 0.0
        self.p_oficial, self.deltas = {}, []
        return super(Voos23, self).eventos(_MapaP08(dados, self.PREFIXO))


class VoosPNW14(Voos17):
    nome = "voospnw14"
    corte = ts("2014-10-01")
    inicio = ts("2014-01-22")
    PREFIXO = "pnw14"

    def eventos(self, dados):
        ev = super().eventos(dados)
        self.origens = ["PDX", "SEA"]
        return ev


class _Estresse(Voos23):
    MODO = ""

    def eventos(self, dados):
        ev = super().eventos(dados)
        par = json.loads(PARAM.read_text())
        ag, ch = ev["agenda"], ev["chegada"]
        assert (ag["id"].to_numpy() == ch["id"].to_numpy()).all()
        prog = ag["prog"].to_numpy()
        mes = pd.to_datetime(prog, unit="s", utc=True).tz_convert("America/New_York").month.to_numpy()
        if self.MODO == "S1":
            f = np.where(prog >= self.corte, par["S1_f"], 0.0)
        else:
            f = np.zeros(len(prog))
            for m, v in par["S2"].items():
                f[(prog >= self.corte) & (mes == int(m))] = v["f"]
        u = np.random.default_rng(SEMENTE).random(len(prog))
        troca = (ch["y"].to_numpy() == 0) & (u < f)
        ch.loc[troca, "y"] = 1
        self.trocas = int(troca.sum())
        self.estresse_info = {"modo": self.MODO, "trocados": self.trocas, "voos_no_cofre": int((prog >= self.corte).sum())}
        return ev


class Voos23E1(_Estresse):
    nome, MODO = "voos23e1", "S1"


class Voos23E2(_Estresse):
    nome, MODO = "voos23e2", "S2"
