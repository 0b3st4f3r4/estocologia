"""Fluxo central: chamadas a cada 5 min na central de um banco norte-americano, mar–out de 2003.

Pergunta (F7): quantas chamadas chegam em cada intervalo do próximo dia útil, para escalar
atendentes? A previsão é emitida às 21:10, depois do último intervalo, para os 169 intervalos do
próximo dia de operação. O calendário de operação é público e conhecido antes, e é o único dado
futuro admitido.

Modelo (F4): nível do dia × perfil intradiário, os dois do passado observado.
- O nível é a média das últimas 4 ocorrências do mesmo dia da semana, corrigida pela razão entre os
  últimos 5 dias e a média do dia da semana deles.
- O perfil é a fração média de cada intervalo nas últimas 8 semanas.
- A escala é sqrt(m + (cv·m)²), com cv do erro de nível recente. O calibrador adaptativo corrige os
  quantis.

O reserva é o mesmo intervalo do mesmo dia da semana anterior.
"""

from __future__ import annotations

import math

import numpy as np
import pandas as pd

from estocologia.monitor import CalibradorQuantis

from .base import FluxoBase, segundos, ts

DIA = 86400
SLOTS = 169
AHT, META_SL, META_T = 240.0, 0.80, 20.0      # suposições do H0 (não estão na base)


def _pc(c: int, a: float) -> float:
    """Erlang C pela recursão estável de Erlang B (sem fatoriais: não estoura para cargas grandes)."""
    if a <= 0:
        return 0.0
    if c <= a:
        return 1.0
    b = 1.0
    for k in range(1, c + 1):
        b = a * b / (k + a * b)
    return b / (1 - (a / c) * (1 - b))


def erlang_escala(lam: float) -> int:
    a = lam * AHT
    c = max(1, int(math.floor(a)) + 1)
    while 1 - _pc(c, a) * math.exp(-(c - a) * META_T / AHT) < META_SL:
        c += 1
    return c


class Central(FluxoBase):
    nome, area, tipo = "central", "servicos", "quantis"
    niveis = np.array([0.05, 0.5, 0.95])
    linhas_por_grupo = {"chamadas": 169, "calendario": 4096}
    sensores = {"nivel_do_dia": {"n_janela": 5, "n_ref": 20},
                "residuo_padronizado": {"n_janela": 169 * 3, "n_ref": 169 * 10}}
    disjuntor = {"janela": 169 * 5, "margem": 0.05, "k_on": 2, "k_off": 2, "intervalo": 169}
    corte = ts("2003-09-01")
    inicio = ts("2003-04-07")
    SEMANAS_PERFIL = 8
    cabecalho = {
        "processo_gerador": "chegadas de clientes ao telefone: ciclo intradiário, dia da semana e choques de nível por dia",
        "nao_linearidade": "nível de serviço côncavo na carga (Jensen, H0); superdispersão 273 entre dias",
        "unidade_e_margem": "o dia é a unidade independente; os 169 intervalos de um dia compartilham o choque de nível",
        "fisica_do_ruido": "Poisson dentro do intervalo mais choque multiplicativo do dia (CV ~6% depois do dia da semana)",
        "escada_causal": "degrau 1 aqui; o laço performativo (escala → desistência → rediscagem) é testado à parte no G4",
    }
    contrato = {
        "pergunta": "chamadas por intervalo de 5 min no próximo dia útil, para escalar atendentes",
        "fonte": "fpp3::bank_calls (Shen & Huang 2008), via Rdatasets",
        "unidade": "dia",
        "pode_influir": True, "degrau_pearl": 1, "tem_titulares": False,
        "decisao": {"acao": "atendentes por intervalo pelo Erlang C sobre o q95", "regra": "meta de 80% em 20 s; AHT de 240 s suposto"},
        "escopo": "nível previsto dentro do suporte dos totais diários do treino",
        "disponibilidade": {"chamadas": "fim do intervalo de 5 min", "calendario": "publicado antes do início"},
    }

    def eventos(self, dados):
        d = pd.read_csv(dados / "bank_calls.csv")
        T = segundos(d["DateTime"])
        dia = (T // DIA) * DIA
        self.dias = np.sort(dia.unique())
        self.idx_dia = {int(x): i for i, x in enumerate(self.dias)}
        self.Y = np.full((len(self.dias), SLOTS), np.nan)
        self.T07 = 7 * 3600
        ch = pd.DataFrame({"T": T, "n": d["Calls"].astype(float), "t_disp": T + 300})
        cal = pd.DataFrame({"dia": self.dias.astype("int64"), "t_disp": int(self.dias[0]) - 2 * DIA})
        self.dias_conhecidos: list = []
        self.pend: dict = {}
        self.cal_m = CalibradorQuantis(self.niveis)
        self.cal_r = CalibradorQuantis(self.niveis)
        self.res_buf, self.nivel_buf = [], []
        self.cv = 0.08
        self.erros_nivel: list = []
        self.sup = None
        return {"chamadas": ch, "calendario": cal}

    def ticks(self, ev):
        return np.array([int(x) + 21 * 3600 + 600 for x in self.dias[:-1]])

    def consumir(self, tabela, df, agora):
        if tabela == "calendario":
            self.dias_conhecidos = sorted(df["dia"].astype(int).tolist())
            return
        T = df["T"].to_numpy()
        i = np.array([self.idx_dia[int(x)] for x in (T // DIA) * DIA])
        j = ((T % DIA) - self.T07) // 300
        self.Y[i, j] = df["n"].to_numpy()

    def sinal_deriva(self, tabela, df):
        out = {}
        if tabela == "chamadas":
            if self.res_buf:
                out["residuo_padronizado"], self.res_buf = self.res_buf, []
            if self.nivel_buf:
                out["nivel_do_dia"], self.nivel_buf = self.nivel_buf, []
        return out

    def _dow(self, dia):
        return pd.Timestamp(int(dia), unit="s", tz="UTC").dayofweek

    def _previsao(self, alvo_i):
        """Nível e perfil do dia alvo com os dias completos anteriores a ele (todos já observados)."""
        passados = [i for i in range(alvo_i) if np.isfinite(self.Y[i]).all()]
        if len(passados) < 10:
            return None
        tot = {i: self.Y[i].sum() for i in passados}
        dw = self._dow(self.dias[alvo_i])
        mesmos = [i for i in passados if self._dow(self.dias[i]) == dw][-4:]
        if len(mesmos) < 2:
            return None
        base = np.mean([tot[i] for i in mesmos])
        ult = passados[-5:]
        razoes = []
        for i in ult:
            mi = [tot[k] for k in passados if k < i and self._dow(self.dias[k]) == self._dow(self.dias[i])][-4:]
            if mi:
                razoes.append(tot[i] / np.mean(mi))
        nivel = base * (np.exp(np.mean(np.log(razoes))) if razoes else 1.0)
        janela = passados[-5 * self.SEMANAS_PERFIL:]
        perfil = np.mean([self.Y[i] / tot[i] for i in janela], axis=0)
        semana = [i for i in passados if self._dow(self.dias[i]) == dw]
        reserva = self.Y[semana[-1]] if semana else None
        return nivel, perfil, reserva, [tot[i] for i in passados]

    def precisa_ajuste(self, agora):
        return False

    def ajustar(self, agora):
        pass

    def emitir(self, agora):
        hoje = (agora // DIA) * DIA
        prox = [d for d in self.dias_conhecidos if d > hoje]
        if not prox:
            return []
        alvo = prox[0]
        ai = self.idx_dia[alvo]
        r = self._previsao(ai)
        if r is None:
            return []
        nivel, perfil, reserva, totais = r
        if reserva is None:
            return []
        m = nivel * perfil
        s = np.sqrt(m + (self.cv * m) ** 2)
        sr = np.sqrt(reserva + (0.15 * reserva) ** 2)
        qm, qr = self.cal_m.quantis(m, s), self.cal_r.quantis(reserva, sr)
        esc = bool(min(totais) * 0.8 <= nivel <= max(totais) * 1.2)
        out = []
        for j in range(SLOTS):
            id_ = f"c{alvo}_{j}"
            self.pend[(ai, j)] = (id_, m[j], s[j], qm[j], reserva[j], sr[j], qr[j])
            out.append({"id": id_, "t_alvo": int(alvo + self.T07 + 300 * j), "modelo": qm[j], "reserva": qr[j], "escopo_ok": esc,
                        "extra": {"nivel_previsto": float(nivel)}})
        self.pend[("nivel", ai)] = nivel
        return out

    def resolver(self, agora):
        out = []
        for chave in [k for k in self.pend if k[0] != "nivel" and np.isfinite(self.Y[k[0], k[1]])]:
            ai, j = chave
            id_, m, s, qm, mr, sr, qr = self.pend.pop(chave)
            y = self.Y[ai, j]
            self.cal_m.atualizar(y, m, s, qm)
            self.cal_r.atualizar(y, mr, sr, qr)
            self.res_buf.append((y - m) / s)
            out.append((id_, y, int(self.dias[ai] + self.T07 + 300 * j + 300)))
            if j == SLOTS - 1 and ("nivel", ai) in self.pend:
                nv = self.pend.pop(("nivel", ai))
                e = float(np.log(np.nansum(self.Y[ai]) / nv))
                self.nivel_buf.append(e)
                self.erros_nivel.append(e)
                self.cv = float(np.sqrt(np.mean(np.square(self.erros_nivel[-30:])))) if len(self.erros_nivel) >= 5 else self.cv
        return out

    def decidir(self, q):
        return erlang_escala(max(q[2], 0) / 300.0)
