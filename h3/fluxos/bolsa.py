"""Fluxo bolsa: Dow Jones Industrial Average, fechamento diário, 1885–2023.

Pergunta (F7): qual a perda que o pregão de amanhã só ultrapassa com 1% de chance (VaR 99%)? A
previsão é emitida no fechamento, para o retorno logarítmico do pregão seguinte, qualquer que seja a
data dele. O calendário de pregões não é usado.

Modelo (F4): simulação histórica filtrada. Uma volatilidade EWMA (λ = 0,94, RiskMetrics) multiplica
os quantis empíricos dos retornos padronizados dos últimos 2 500 pregões, com correção conforme
adaptativa dos níveis.

Reserva (F1): simulação histórica pura, com os quantis empíricos dos últimos 250 retornos.

Lente de Crononomia: o processo é multiplicativo, e o capital cresce a E[ln(1+r)], não a E[r]. A
decisão de cortar exposição usa o VaR, não a média.
"""

from __future__ import annotations

import numpy as np
import pandas as pd

from estocologia.monitor import CalibradorQuantis

from .base import FluxoBase, segundos, ts


class Bolsa(FluxoBase):
    nome, area, tipo = "bolsa", "financas", "quantis"
    niveis = np.array([0.01, 0.05, 0.5])
    linhas_por_grupo = {"fechamento": 1}
    sensores = {"retorno_absoluto": {"n_janela": 60, "n_ref": 1000},
                "retorno_padronizado": {"n_janela": 250, "n_ref": 1000}}
    disjuntor = {"janela": 500, "margem": 0.05, "k_on": 3, "k_off": 3, "intervalo": 20}
    corte = ts("2000-01-01")
    inicio = ts("1887-01-01")
    LAMBDA = 0.94
    N_HIST = 2500
    N_RESERVA = 250
    LIMIAR_VAR = 0.03
    cabecalho = {
        "processo_gerador": "preço de uma carteira de 30 ações: retornos com aglomeração de volatilidade e caudas pesadas",
        "nao_linearidade": "multiplicativa (Crononomia): o capital cresce a E[ln], e a volatilidade muda de regime",
        "unidade_e_margem": "o pregão; os quantis condicionam na volatilidade do dia",
        "fisica_do_ruido": "caudas com α ≈ 3 a 4; choques de crise (1929, 1987, 2008, 2020)",
        "escada_causal": "degrau 1; o VaR publicado por um agente pequeno não move o índice",
    }
    contrato = {
        "pergunta": "VaR 99% do retorno do próximo pregão",
        "fonte": "stevedata::DJIA, via Rdatasets",
        "unidade": "pregão",
        "pode_influir": False, "degrau_pearl": 1, "tem_titulares": False,
        "decisao": {"acao": "cortar exposição à metade", "regra": "VaR 99% > 3% do capital"},
        "escopo": "volatilidade prevista até 1,5 vez a maior do treino",
        "disponibilidade": {"fechamento": "16:00 de Nova York (21:00 UTC) do pregão"},
    }

    def eventos(self, dados):
        d = pd.read_csv(dados / "djia_1885_2023.csv")
        T = segundos(d["date"]) + 21 * 3600
        self.t_fech = T.to_numpy()
        ev = pd.DataFrame({"i": np.arange(len(d)), "v": d["value"].astype(float), "t_disp": T})
        self.v: list = []
        self.r: list = []
        self.z: list = []
        self.s2 = None
        self.pend: dict = {}
        self.cal_m = CalibradorQuantis(self.niveis, n_max=self.N_HIST, gama=0.002, n_min=250)
        self.buf_abs, self.buf_z = [], []
        self.sig_max = 0.0
        return {"fechamento": ev}

    def ticks(self, ev):
        return self.t_fech

    def consumir(self, tabela, df, agora):
        for i, v in zip(df["i"].to_numpy(), df["v"].to_numpy()):
            if self.v:
                r = float(np.log(v / self.v[-1]))
                self.r.append(r)
                self.buf_abs.append(abs(r))
                if self.s2 is not None:
                    self.z.append(r / np.sqrt(self.s2))
                    self.buf_z.append(self.z[-1])
                self.s2 = r * r if self.s2 is None else self.LAMBDA * self.s2 + (1 - self.LAMBDA) * r * r
                self.s2 = max(self.s2, 1e-8)
            self.v.append(v)
            self.ultimo_i = int(i)

    def sinal_deriva(self, tabela, df):
        out = {"retorno_absoluto": self.buf_abs, "retorno_padronizado": self.buf_z}
        self.buf_abs, self.buf_z = [], []
        return out

    def ajustar(self, agora):
        pass

    def emitir(self, agora):
        if len(self.r) < self.N_RESERVA or self.s2 is None:
            return []
        s = float(np.sqrt(self.s2))
        if agora < self.corte:
            self.sig_max = max(self.sig_max, s)
        qm = self.cal_m.quantis(np.array([0.0]), np.array([s]))[0]
        qr = np.quantile(np.array(self.r[-self.N_RESERVA:]), self.niveis)
        id_ = f"b{self.ultimo_i + 1}"
        self.pend[self.ultimo_i + 1] = (id_, s, qm)
        esc = s <= 1.5 * self.sig_max if self.sig_max > 0 else True
        return [{"id": id_, "t_alvo": int(agora + 86400), "modelo": qm, "reserva": qr, "escopo_ok": bool(esc),
                 "extra": {"sigma": s}}]

    def resolver(self, agora):
        out = []
        for i in [i for i in self.pend if i <= self.ultimo_i]:
            id_, s, qm = self.pend.pop(i)
            y = self.r[i - 1]
            self.cal_m.atualizar(y, 0.0, s, qm)
            out.append((id_, y, int(self.t_fech[i])))
        return out

    def decidir(self, q):
        return bool(-q[0] > self.LIMIAR_VAR)
