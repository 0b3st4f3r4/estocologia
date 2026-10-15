"""Fluxo prisões: detidos por porte de maconha em Toronto, 1997–2002 (Toronto Star; carData::Arrests).

Pergunta (F7): este detido vai ser liberado com intimação, em vez de levado à delegacia? O previsor
quer medir o processo, não substituí-lo. O uso declarado é a auditoria: marcar para revisão os
casos com baixa chance prevista de liberação. Por isso o F6 audita o previsor por cor da pele em
cada ano, e o F5 libera as taxas anuais com privacidade diferencial.

A base só traz o ano. O instante de cada detenção dentro do ano é sintético e determinístico: as
linhas são espalhadas uniformemente no ano, na ordem do arquivo. O rótulo passa a ser conhecido um
dia depois da detenção. O modelo é reajustado só em 1º de janeiro, com os anos anteriores, e não
recebe a cor da pele: é "cego à cor", e o F6 mostra o que isso não garante.
"""

from __future__ import annotations

import numpy as np
import pandas as pd

from .base import FluxoBase, logistica, sigmoide, ts

DIA = 86400


class Prisoes(FluxoBase):
    nome, area, tipo = "prisoes", "direito", "binario"
    linhas_por_grupo = {"detencao": 1, "liberacao": 1, "canario": 1}
    sensores = {"verificacoes": {"n_janela": 150, "n_ref": 600},
                "residuo_padronizado": {"n_janela": 150, "n_ref": 600}}
    disjuntor = {"janela": 400, "margem": 0.03, "k_on": 3, "k_off": 3, "intervalo": 50}
    corte = ts("2001-01-01")
    inicio = ts("1998-01-01")
    LIMIAR = 0.70
    cabecalho = {
        "processo_gerador": "decisão policial de liberar com intimação ou levar à delegacia, por discricionariedade sob regras",
        "nao_linearidade": "efeito do número de registros prévios (checks) é forte e não linear",
        "unidade_e_margem": "o detido; sem identificador de agente ou de divisão (agrupamento não observável)",
        "fisica_do_ruido": "discricionariedade do agente não observada",
        "escada_causal": "degrau 1; a diferença por cor é associação, não efeito causal identificado (H2: Ŝ não contém cor)",
    }
    contrato = {
        "pergunta": "P(liberação com intimação) de cada detido",
        "fonte": "carData::Arrests (Toronto Star 2002), via Rdatasets",
        "unidade": "detido",
        "pode_influir": False, "degrau_pearl": 1, "tem_titulares": True,
        "quase_identificadores": ["colour", "year", "age", "sex", "employed", "citizen"],
        "atributo_protegido": "colour",
        "decisao": {"acao": "marcar o caso para revisão", "regra": "p(liberação) < 0,70"},
        "escopo": "idade e verificações dentro do suporte do treino",
        "disponibilidade": {"detencao": "no instante (sintético) da detenção", "liberacao": "1 dia depois"},
    }

    def eventos(self, dados):
        d = pd.read_csv(dados / "h2" / "carData_Arrests.csv")
        d["id"] = "p" + d["rownames"].astype(str)
        t = np.zeros(len(d), "int64")
        for ano, g in d.groupby("year"):
            ini, fim = ts(f"{ano}-01-01"), ts(f"{ano + 1}-01-01")
            t[g.index] = ini + 3600 + ((np.arange(len(g)) + 0.5) / len(g) * (fim - ini - 2 * DIA)).astype("int64")
        d["t"] = t
        det = pd.DataFrame({"id": d["id"], "colour": d["colour"], "year": d["year"], "age": d["age"].astype(float),
                            "sex": d["sex"], "employed": d["employed"], "citizen": d["citizen"], "checks": d["checks"].astype(float),
                            "t_disp": d["t"]})
        lib = pd.DataFrame({"id": d["id"], "y": (d["released"] == "Yes").astype(int), "t_disp": d["t"] + DIA})
        self._rot = lib
        self.det: dict = {}
        self.novos: list = []
        self.y: dict = {}
        self.t_y: dict = {}
        self.feats: dict = {}
        self.pend: dict = {}
        self.beta = None
        self.ano = None
        self.chk_buf, self.res_buf = [], []
        self.marcos = np.sort(np.unique(np.r_[det["t_disp"], lib["t_disp"], [ts(f"{a}-01-01") for a in range(1997, 2004)]]))
        return {"detencao": det, "liberacao": lib}

    def tabela_rotulos(self, ev):
        return self._rot

    def ticks(self, ev):
        return self.marcos

    def consumir(self, tabela, df, agora):
        if tabela == "detencao":
            for r in df.itertuples(index=False):
                self.det[r.id] = r
                self.novos.append(r)
                self.chk_buf.append(r.checks)
        else:
            for i, y, t in zip(df["id"], df["y"], df["t_disp"]):
                self.y[i], self.t_y[i] = int(y), int(t)

    def sinal_deriva(self, tabela, df):
        out = {}
        if self.chk_buf:
            out["verificacoes"], self.chk_buf = self.chk_buf, []
        if self.res_buf:
            out["residuo_padronizado"], self.res_buf = self.res_buf, []
        return out

    @staticmethod
    def _x(r):
        return np.array([1.0, r.age / 30, (r.age / 30) ** 2, float(r.sex == "Male"), float(r.employed == "Yes"),
                         float(r.citizen == "Yes"), r.checks / 6, (r.checks / 6) ** 2])

    def ajustar(self, agora):
        ano = pd.Timestamp(int(agora), unit="s").year
        ids = [i for i in self.feats if i in self.y and self.det[i].year < ano]     # só anos anteriores, mesmo no reajuste por deriva
        if len(ids) < 300:
            return
        X = np.array([self.feats[i] for i in ids])
        y = np.array([self.y[i] for i in ids], float)
        self.beta = logistica(X, y, lam=1e-3)
        self.sup = (X[:, 1].min() * 30, X[:, 1].max() * 30, X[:, 6].max() * 6)
        self.base = float(y[[self.det[i].year == pd.Timestamp(int(agora), unit="s").year - 1 for i in ids]].mean()) \
            if any(self.det[i].year == pd.Timestamp(int(agora), unit="s").year - 1 for i in ids) else float(y.mean())
        self.ano = pd.Timestamp(int(agora), unit="s").year

    def precisa_ajuste(self, agora):
        a = pd.Timestamp(int(agora), unit="s")
        return self.ano is None or (a.year != self.ano and a.dayofyear == 1)

    def emitir(self, agora):
        out = []
        for r in self.novos:
            x = self._x(r)
            self.feats[r.id] = x
            pr = self.base if self.beta is not None else 0.8
            pm = float(np.clip(sigmoide(x @ self.beta), 0.01, 0.99)) if self.beta is not None else pr
            esc = self.beta is not None and self.sup[0] <= r.age <= self.sup[1] and r.checks <= self.sup[2]
            self.pend[r.id] = pm
            out.append({"id": r.id, "t_alvo": int(r.t_disp + DIA), "modelo": pm, "reserva": pr, "escopo_ok": bool(esc),
                        "extra": {"grupo": r.colour, "ano": int(r.year)}})
        self.novos = []
        return out

    def resolver(self, agora):
        out = []
        for i in [i for i in self.pend if i in self.y]:
            pm = self.pend.pop(i)
            self.res_buf.append((self.y[i] - pm) / np.sqrt(pm * (1 - pm)))
            out.append((i, self.y[i], self.t_y[i]))
        return out

    def decidir(self, p):
        return bool(p < self.LIMIAR)
