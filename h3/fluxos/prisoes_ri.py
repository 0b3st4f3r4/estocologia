"""Fluxo prisoes_ri (H3d): prisão do motorista em paradas da polícia estadual de Rhode Island, 2005–2015.

Declarado em CRITERIOS_H3d.md antes de rodar; o hash do arquivo entra na especificação (CRITERIOS_SHA256).

O modelo é cego à cor; o F6 audita. Reajuste em 1º de janeiro só com anos anteriores (λ = 1e-4) e previsão
oficial pela recalibração online de Platt: p' = σ(a + b·logit p), (a, b) com um passo de gradiente da log-perda a
cada rótulo (η = 0,002; a = 0, b = 1 no início).
"""

from __future__ import annotations

from pathlib import Path

import numpy as np
import pandas as pd

from estocologia.motor import sha_arquivo

from .base import FluxoBase, logistica, segundos, sigmoide, ts

DIA = 86400
ARQ = "backup_rhode_island_traffic_stops_2005_2015.csv"


class PrisoesRI(FluxoBase):
    nome, area, tipo = "prisoes_ri", "direito", "binario"
    # H3d, depois do cofre: grupos de 4 linhas (≈ as 5 paradas por tick de 6 h); com 32 o G5 dava 6,4 varreduras.
    # Só muda a leitura em disco; o diário de previsões fica idêntico (conferido e relatado no README).
    linhas_por_grupo = {"detencao": 4, "liberacao": 4, "canario": 4}
    sensores = {"idade": {"n_janela": 150, "n_ref": 600},
                "residuo_padronizado": {"n_janela": 150, "n_ref": 600}}
    disjuntor = {"janela": 400, "margem": 0.03, "k_on": 3, "k_off": 3, "intervalo": 50}
    corte = ts("2013-01-01")
    inicio = ts("2006-01-01")
    LIMIAR = 0.10
    ETA = 0.002
    LAM = 1e-4
    CRITERIOS_SHA256 = sha_arquivo(Path(__file__).resolve().parents[1] / "CRITERIOS_H3d.md")
    cabecalho = {
        "processo_gerador": "decisão do policial de prender o motorista numa parada de trânsito",
        "nao_linearidade": "o motivo da parada domina; idade com efeito não linear; evento raro (~3%)",
        "unidade_e_margem": "a parada; sem identificador de motorista nem de policial",
        "fisica_do_ruido": "discricionariedade do policial e circunstâncias não registradas",
        "escada_causal": "degrau 1; a diferença por cor é associação, não efeito identificado",
    }
    contrato = {
        "pergunta": "P(prisão do motorista) em cada parada",
        "fonte": "Stanford Open Policing Project, Rhode Island 2005–2015 (subconjunto, espelho GitHub); ver dados-reais/h3c/FONTES.md",
        "unidade": "parada",
        "pode_influir": False, "degrau_pearl": 1, "tem_titulares": True,
        "quase_identificadores": ["cor", "ano", "sexo", "idade"],
        "atributo_protegido": "cor",
        "decisao": {"acao": "marcar o caso para revisão", "regra": "p(prisão) > 0,10"},
        "escopo": "categoria do motivo da parada vista no treino",
        "disponibilidade": {"detencao": "no instante da parada", "liberacao": "1 dia depois"},
    }

    def eventos(self, dados):
        d = pd.read_csv(Path(dados) / "h3c" / ARQ)
        d = d[d["stop_outcome"].notna() & d["driver_gender"].notna() & d["driver_age"].notna()].reset_index(drop=True)
        t = segundos(pd.to_datetime(d["stop_date"] + " " + d["stop_time"], utc=True)).to_numpy("int64")
        o = np.argsort(t, kind="stable")
        d, t = d.iloc[o].reset_index(drop=True), t[o]
        d["id"] = "s" + pd.Series(np.arange(len(d))).astype(str)
        hora = (t % DIA) // 3600
        det = pd.DataFrame({"id": d["id"], "cor": d["driver_race"].fillna("ausente").astype(str),
                            "ano": pd.to_datetime(t, unit="s").year.astype(int), "sexo": d["driver_gender"].astype(str),
                            "idade": d["driver_age"].astype(float), "motivo": d["violation_raw"].astype(str),
                            "faixa_hora": (hora // 6).astype(int), "dia_semana": ((t // DIA + 3) % 7).astype(int), "t_disp": t})
        lib = pd.DataFrame({"id": d["id"], "y": (d["stop_outcome"] == "Arrest Driver").astype(int), "t_disp": t + DIA})
        self._rot = lib
        self.det: dict = {}
        self.novos: list = []
        self.y: dict = {}
        self.t_y: dict = {}
        self.pend: dict = {}
        self.beta = None
        self.ano = None
        self.idade_buf, self.res_buf = [], []
        self.a, self.b = 0.0, 1.0
        self.logit_pend: dict = {}
        self.ab: list = []
        self.inicios = [ts(f"{a}-01-01") for a in range(2005, 2017)]
        grade = np.arange(ts("2005-01-01"), ts("2016-01-03"), 6 * 3600, dtype="int64")
        self.marcos = np.sort(np.unique(np.r_[grade, self.inicios]))
        return {"detencao": det, "liberacao": lib}

    def tabela_rotulos(self, ev):
        return self._rot

    def ticks(self, ev):
        return self.marcos

    def consumir(self, tabela, df, agora):
        if tabela == "detencao":
            for r in df.itertuples(index=False):
                self.det[r.id] = r
                self.novos.append(r.id)
                self.idade_buf.append(r.idade)
        else:
            for i, y, t in zip(df["id"], df["y"], df["t_disp"]):
                self.y[i], self.t_y[i] = int(y), int(t)

    def sinal_deriva(self, tabela, df):
        out = {}
        if self.idade_buf:
            out["idade"], self.idade_buf = self.idade_buf, []
        if self.res_buf:
            out["residuo_padronizado"], self.res_buf = self.res_buf, []
        return out

    @staticmethod
    def _bruto(r) -> dict:
        z = (r.idade - 34.0) / 12.7
        return {"sexo=" + r.sexo: 1.0, "motivo=" + r.motivo: 1.0, f"hora={r.faixa_hora}": 1.0,
                f"dia={r.dia_semana}": 1.0, "idade": z, "idade2": z * z}

    def _x(self, b: dict) -> np.ndarray:
        return np.r_[1.0, [b.get(c, 0.0) for c in self.cols]]

    def ajustar(self, agora):
        ano = pd.Timestamp(int(agora), unit="s").year
        ids = [i for i in self.det if i in self.y and self.det[i].ano < ano]
        if len(ids) < 300:
            return
        brutos = {i: self._bruto(self.det[i]) for i in ids}
        cats = sorted({k for b in brutos.values() for k in b})
        ref = []
        for pref in ("sexo=", "motivo=", "hora=", "dia="):
            cont = pd.Series([k for b in brutos.values() for k in b if k.startswith(pref)]).value_counts()
            ref.append(cont.index[0])
        self.cols = [c for c in cats if c not in ref]
        self.vistos = {k for k in cats if k.startswith("motivo=")}
        X = np.array([self._x(brutos[i]) for i in ids])
        y = np.array([self.y[i] for i in ids], float)
        self.beta = logistica(X, y, lam=self.LAM)
        ant = [self.det[i].ano == ano - 1 for i in ids]
        self.base = float(y[ant].mean()) if any(ant) else float(y.mean())
        self.ano = ano

    def precisa_ajuste(self, agora):
        return int(agora) in self.inicios and pd.Timestamp(int(agora), unit="s").year != self.ano

    def emitir(self, agora):
        out = []
        for i in self.novos:
            r = self.det[i]
            pr = self.base if self.beta is not None else 0.05
            if self.beta is not None:
                pm = float(np.clip(sigmoide(self._x(self._bruto(r)) @ self.beta), 1e-4, 1 - 1e-4))
                lg = float(np.log(pm / (1 - pm)))
                po = float(np.clip(sigmoide(self.a + self.b * lg), 0.001, 0.999))
                esc = ("motivo=" + r.motivo) in self.vistos
                self.logit_pend[i] = lg
            else:
                pm, po, esc = pr, pr, False
            self.pend[i] = po
            out.append({"id": i, "t_alvo": int(r.t_disp + DIA), "modelo": po, "reserva": pr, "escopo_ok": bool(esc),
                        "extra": {"grupo": r.cor, "ano": int(r.ano), "modelo_cru": pm, "platt_a": self.a, "platt_b": self.b}})
        self.novos = []
        return out

    def resolver(self, agora):
        out = []
        for i in [i for i in self.pend if i in self.y]:
            pm = self.pend.pop(i)
            yi = self.y[i]
            self.res_buf.append((yi - pm) / np.sqrt(pm * (1 - pm)))
            out.append((i, yi, self.t_y[i]))
            lg = self.logit_pend.pop(i, None)
            if lg is not None:
                e = yi - pm
                self.a += self.ETA * e
                self.b += self.ETA * e * lg
        self.ab.append((self.a, self.b))
        return out

    def decidir(self, p):
        return bool(p > self.LIMIAR)
