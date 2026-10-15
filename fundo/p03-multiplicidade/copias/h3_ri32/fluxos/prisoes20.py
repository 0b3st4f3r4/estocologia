"""Fluxo prisoes20 (H3c): prisões em Toronto, 2020–2021 (Toronto Police Service, RBDC-ARR-TBL-001).

Declarado em CRITERIOS_H3c.md antes de rodar; o hash do arquivo entra na especificação (CRITERIOS_SHA256).

Pergunta (F7): a pessoa presa vai ser registrada na delegacia (booked), em vez de liberada no local? Mesmo uso do
fluxo `prisoes`: medir o processo para auditoria, não substituí-lo. O modelo é cego à cor percebida; o F6 audita.

A base só traz ano e trimestre. O instante dentro do trimestre é sintético e determinístico (linhas espalhadas
uniformemente na ordem do ObjectId). O rótulo fica conhecido um dia depois. O modelo é reajustado no início de cada
trimestre, só com trimestres anteriores, e a previsão oficial passa pela recalibração online do intercepto do H3b.
"""

from __future__ import annotations

from pathlib import Path

import numpy as np
import pandas as pd

from estocologia.motor import sha_arquivo

from .base import FluxoBase, logistica, sigmoide, ts

DIA = 86400
TRIM = {"Jan-Mar": 1, "Apr-June": 4, "July-Sept": 7, "Oct-Dec": 10}
IDADE = {"Aged 17 years and younger": "<=17", "Aged 17 years and under": "<=17", "Aged 18 to 24 years": "18-24",
         "Aged 25 to 34 years": "25-34", "Aged 35 to 44 years": "35-44", "Aged 45 to 54 years": "45-54",
         "Aged 55 to 64 years": "55-64", "Aged 65 years and older": "65+", "Aged 65 and older": "65+"}
ACOES = ["Actions_at_arrest___Concealed_i", "Actions_at_arrest___Combative__", "Actions_at_arrest___Resisted__d",
         "Actions_at_arrest___Mental_inst", "Actions_at_arrest___Assaulted_o", "Actions_at_arrest___Cooperative"]
ARQ = "toronto_arrests_strip_searches_2020_2021.csv"


def _ini_trim(ano: int, mes: int) -> int:
    return ts(f"{ano}-{mes:02d}-01")


class Prisoes20(FluxoBase):
    nome, area, tipo = "prisoes20", "direito", "binario"
    linhas_por_grupo = {"detencao": 32, "liberacao": 32, "canario": 32}
    sensores = {"anteriores": {"n_janela": 150, "n_ref": 600},
                "residuo_padronizado": {"n_janela": 150, "n_ref": 600}}
    disjuntor = {"janela": 400, "margem": 0.03, "k_on": 3, "k_off": 3, "intervalo": 50}
    corte = ts("2021-07-01")
    inicio = ts("2020-04-01")
    LIMIAR = 0.30
    ETA = 0.002
    LAM = 1e-3
    CRITERIOS_SHA256 = sha_arquivo(Path(__file__).resolve().parents[1] / "CRITERIOS_H3c.md")
    cabecalho = {
        "processo_gerador": "decisão policial de registrar na delegacia ou liberar no local, sob regras e discricionariedade",
        "nao_linearidade": "a ocorrência e o comportamento na prisão dominam; reincidência com efeito não linear",
        "unidade_e_margem": "a prisão; a mesma pessoa aparece em várias linhas (37 mil pessoas em 65 mil prisões)",
        "fisica_do_ruido": "discricionariedade do agente e da divisão; defeito de formulário em Booked (corrigido pela regra declarada)",
        "escada_causal": "degrau 1; a diferença por cor percebida é associação, não efeito identificado",
    }
    contrato = {
        "pergunta": "P(registro na delegacia em 24 h) de cada pessoa presa",
        "fonte": "Toronto Police Service RBDC-ARR-TBL-001 (2020–2021), espelho GitHub; ver dados-reais/h3c/FONTES.md",
        "unidade": "prisão",
        "pode_influir": False, "degrau_pearl": 1, "tem_titulares": True,
        "quase_identificadores": ["cor", "ano", "trimestre", "idade", "sexo", "divisao"],
        "atributo_protegido": "cor",
        "decisao": {"acao": "marcar o caso para revisão", "regra": "p(registro) > 0,30"},
        "escopo": "categoria da ocorrência e divisão vistas no treino",
        "disponibilidade": {"detencao": "no instante (sintético) da prisão", "liberacao": "1 dia depois"},
    }

    def eventos(self, dados):
        d = pd.read_csv(Path(dados) / "h3c" / ARQ).sort_values("ObjectId", kind="stable").reset_index(drop=True)
        d["mes"] = d["Arrest_Month"].map(TRIM)
        t = np.zeros(len(d), "int64")
        for (ano, mes), g in d.groupby(["Arrest_Year", "mes"]):
            ini = _ini_trim(int(ano), int(mes))
            fim = _ini_trim(int(ano) + (mes == 10), 1 if mes == 10 else int(mes) + 3)
            t[g.index] = ini + 3600 + ((np.arange(len(g)) + 0.5) / len(g) * (fim - ini - 2 * DIA)).astype("int64")
        d["id"] = "a" + d["ObjectId"].astype(str)
        det = pd.DataFrame({
            "id": d["id"], "pessoa": d["PersonID"].astype("int64"), "cor": d["Perceived_Race"].fillna("ausente").astype(str),
            "ano": d["Arrest_Year"].astype(int), "trimestre": d["mes"].astype(int),
            "sexo": d["Sex"].fillna("U").astype(str), "idade": d["Age_group__at_arrest_"].map(IDADE).fillna("ausente"),
            "ocorrencia": d["Occurrence_Category"].fillna("ausente").astype(str),
            "divisao": d["ArrestLocDiv"].astype(str),
            **{f"a{k}": d[c].astype(int) for k, c in enumerate(ACOES)},
            "t_disp": t})
        y = ((d["Booked"] == 1) | (d["StripSearch"] == 1)).astype(int)        # regra declarada (defeito do formulário)
        lib = pd.DataFrame({"id": d["id"], "y": y, "t_disp": t + DIA})
        self._rot = lib
        self.det: dict = {}
        self.novos: list = []
        self.y: dict = {}
        self.t_y: dict = {}
        self.feats: dict = {}
        self.pend: dict = {}
        self.hist: dict = {}            # pessoa -> lista de (ano, trimestre) das prisões já consumidas
        self.beta = None
        self.trim = None
        self.cols = None
        self.ant_buf, self.res_buf = [], []
        self.delta = 0.0
        self.p_oficial: dict = {}
        self.deltas: list = []
        self.inicios = [_ini_trim(a, m) for a in (2020, 2021, 2022) for m in (1, 4, 7, 10)]
        # relógio a cada 6 h (um tick por evento, como no fluxo antigo, daria 130 mil leituras por tabela)
        grade = np.arange(_ini_trim(2020, 1), _ini_trim(2022, 1) + 2 * DIA, 6 * 3600, dtype="int64")
        self.marcos = np.sort(np.unique(np.r_[grade, self.inicios]))
        return {"detencao": det, "liberacao": lib}

    def tabela_rotulos(self, ev):
        return self._rot

    def ticks(self, ev):
        return self.marcos

    def consumir(self, tabela, df, agora):
        if tabela == "detencao":
            for r in df.itertuples(index=False):
                ant = sum(1 for q in self.hist.get(r.pessoa, []) if q < (r.ano, r.trimestre))
                self.hist.setdefault(r.pessoa, []).append((r.ano, r.trimestre))
                self.det[r.id] = (r, ant)
                self.novos.append(r.id)
                self.ant_buf.append(float(ant))
        else:
            for i, y, t in zip(df["id"], df["y"], df["t_disp"]):
                self.y[i], self.t_y[i] = int(y), int(t)

    def sinal_deriva(self, tabela, df):
        out = {}
        if self.ant_buf:
            out["anteriores"], self.ant_buf = self.ant_buf, []
        if self.res_buf:
            out["residuo_padronizado"], self.res_buf = self.res_buf, []
        return out

    @staticmethod
    def _bruto(r, ant) -> dict:
        a = min(ant, 5) / 5
        return {"sexo=" + r.sexo: 1.0, "idade=" + r.idade: 1.0, "ocorrencia=" + r.ocorrencia: 1.0,
                "divisao=" + r.divisao: 1.0, **{f"a{k}": float(getattr(r, f"a{k}")) for k in range(6)},
                "anteriores": a, "anteriores2": a * a}

    def _x(self, b: dict) -> np.ndarray:
        return np.r_[1.0, [b.get(c, 0.0) for c in self.cols]]

    def ajustar(self, agora):
        a = pd.Timestamp(int(agora), unit="s")
        atual = (a.year, (a.month - 1) // 3 * 3 + 1)
        ids = [i for i in self.det if i in self.y and (self.det[i][0].ano, self.det[i][0].trimestre) < atual]
        if len(ids) < 300:
            return
        brutos = {i: self._bruto(*self.det[i]) for i in ids}
        cats = sorted({k for b in brutos.values() for k in b})
        # categorias de referência (fora da matriz): as mais frequentes de cada variável
        ref = []
        for pref in ("sexo=", "idade=", "ocorrencia=", "divisao="):
            cont = pd.Series([k for b in brutos.values() for k in b if k.startswith(pref)]).value_counts()
            ref.append(cont.index[0])
        self.cols = [c for c in cats if c not in ref]
        self.vistos = {k for k in cats if k.startswith(("ocorrencia=", "divisao="))}
        X = np.array([self._x(brutos[i]) for i in ids])
        y = np.array([self.y[i] for i in ids], float)
        self.beta = logistica(X, y, lam=self.LAM)
        ult = max((self.det[i][0].ano, self.det[i][0].trimestre) for i in ids)
        sel = [(self.det[i][0].ano, self.det[i][0].trimestre) == ult for i in ids]
        self.base = float(y[sel].mean())
        self.trim = atual

    def precisa_ajuste(self, agora):
        return int(agora) in self.inicios and self._atual(agora) != self.trim

    @staticmethod
    def _atual(agora):
        a = pd.Timestamp(int(agora), unit="s")
        return (a.year, (a.month - 1) // 3 * 3 + 1)

    def emitir(self, agora):
        out = []
        for i in self.novos:
            r, ant = self.det[i]
            pr = self.base if self.beta is not None else 0.5
            if self.beta is not None:
                b = self._bruto(r, ant)
                pm = float(np.clip(sigmoide(self._x(b) @ self.beta), 0.01, 0.99))
                esc = ("ocorrencia=" + r.ocorrencia) in self.vistos and ("divisao=" + r.divisao) in self.vistos
                pc = float(np.clip(pm, 1e-4, 1 - 1e-4))
                po = float(np.clip(sigmoide(np.log(pc / (1 - pc)) + self.delta), 0.01, 0.99))
                extra = {"grupo": r.cor, "ano": int(r.ano), "trimestre": int(r.trimestre), "modelo_cru": pm, "delta": self.delta}
            else:
                po, esc = pr, False
                extra = {"grupo": r.cor, "ano": int(r.ano), "trimestre": int(r.trimestre), "modelo_cru": pr, "delta": self.delta}
            self.pend[i] = po
            self.p_oficial[i] = po
            out.append({"id": i, "t_alvo": int(r.t_disp + DIA), "modelo": po, "reserva": pr, "escopo_ok": bool(esc),
                        "extra": extra})
        self.novos = []
        return out

    def resolver(self, agora):
        out = []
        for i in [i for i in self.pend if i in self.y]:
            pm = self.pend.pop(i)
            self.res_buf.append((self.y[i] - pm) / np.sqrt(pm * (1 - pm)))
            out.append((i, self.y[i], self.t_y[i]))
            p = self.p_oficial.pop(i, None)
            if p is not None and self.beta is not None:
                self.delta += self.ETA * (self.y[i] - p)
        self.deltas.append(self.delta)
        return out

    def decidir(self, p):
        return bool(p > self.LIMIAR)
