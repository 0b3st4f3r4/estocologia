"""Fluxo ciclones: trajetórias do Atlântico (NOAA HURDAT2, subconjunto dplyr::storms), 1975–2022.

Pergunta (F7): este sistema vai intensificar rápido, isto é, ganhar 30 kt ou mais de vento máximo nas
próximas 24 h? A previsão é emitida a cada observação, 3 h depois do instante observado (a latência
do boletim é uma suposição declarada).

O rótulo é calculado pelo próprio fluxo, com as observações da tempestade já recebidas, no instante
t + 24 h + 3 h. Se a tempestade acaba antes, o rótulo é 0: não há como intensificar sem existir. Exigir
que ela sobreviva até t + 24 h para emitir seria selecionar pelo futuro.

Modelo (F4): logística reajustada a cada 1º de janeiro, com todas as previsões já resolvidas. As
features são as gravadas na emissão.

Reserva (F1): taxa-base por faixa de vento nos 10 anos anteriores.
"""

from __future__ import annotations

from collections import defaultdict

import numpy as np
import pandas as pd

from .base import FluxoBase, logistica, segundos, sigmoide, ts

H = 3600
LAT = 3 * H


class Ciclones(FluxoBase):
    nome, area, tipo = "ciclones", "clima", "binario"
    linhas_por_grupo = {"obs": 2, "canario": 2}
    sensores = {"vento": {"n_janela": 400, "n_ref": 2000},
                "residuo_padronizado": {"n_janela": 400, "n_ref": 2000}}
    disjuntor = {"janela": 1500, "margem": 0.03, "k_on": 3, "k_off": 3, "intervalo": 100}
    corte = ts("2015-01-01")
    inicio = ts("1979-01-01")
    LIMIAR = 0.20
    TREINO_ANOS = 20
    cabecalho = {
        "processo_gerador": "ciclones tropicais: intensificação governada por temperatura do mar, cisalhamento e estrutura interna",
        "nao_linearidade": "intensificação rápida é um regime raro e de cauda; depende da intensidade atual de forma não monótona",
        "unidade_e_margem": "a tempestade é a unidade independente (efeito de desenho 22,8 no H2); a observação de 6 h é pseudorréplica",
        "fisica_do_ruido": "variabilidade interna do ciclone mais ambiente não observado (sem dados de oceano nesta base)",
        "escada_causal": "degrau 1; o alerta não muda a tempestade",
    }
    contrato = {
        "pergunta": "P(vento máximo sobe ≥ 30 kt nas próximas 24 h)",
        "fonte": "dplyr::storms (NOAA HURDAT2), via Rdatasets",
        "unidade": "observação de 6 h; tempestade como bloco",
        "pode_influir": False, "degrau_pearl": 1, "tem_titulares": False,
        "decisao": {"acao": "emitir alerta de intensificação rápida", "regra": "p > 0,20 = C/L (custo de alerta falso / perda de não alertar)"},
        "escopo": "latitude e vento dentro do suporte do treino",
        "disponibilidade": {"obs": "3 h depois do instante observado"},
    }

    def eventos(self, dados):
        d = pd.read_csv(dados / "h2" / "dplyr_storms.csv")
        d["sid"] = d["name"] + d["year"].astype(str)
        t = pd.to_datetime(dict(year=d.year, month=d.month, day=d.day, hour=d.hour), utc=True)
        d["t_obs"] = segundos(t)
        d = d.drop_duplicates(["sid", "t_obs"], keep="last").sort_values(["sid", "t_obs"]).reset_index(drop=True)
        d["id"] = "c" + d["sid"] + "_" + d["t_obs"].astype(str)
        obs = pd.DataFrame({"id": d["id"], "sid": d["sid"], "t_obs": d["t_obs"], "lat": d["lat"].astype(float),
                            "lon": d["long"].astype(float), "wind": d["wind"].astype(float), "pressure": d["pressure"].astype(float),
                            "status": d["status"], "mes": d["month"], "t_disp": d["t_obs"] + LAT})
        # rótulo, só para o canário e a auditoria (o fluxo recalcula o seu a partir do estado)
        y = np.zeros(len(d), int)
        for sid, g in d.groupby("sid", sort=False):
            tt, ww = g["t_obs"].to_numpy(), g["wind"].to_numpy()
            for k, idx in enumerate(g.index):
                fut = (tt > tt[k]) & (tt <= tt[k] + 24 * H)
                y[idx] = int(fut.any() and ww[fut].max() >= ww[k] + 30)
        self._rot = pd.DataFrame({"id": d["id"], "y": y, "t_disp": d["t_obs"] + 24 * H + LAT})
        self.st: dict = defaultdict(list)          # sid -> [(t_obs, wind)]
        self.novas: list = []
        self.feats: dict = {}
        self.t_emis: dict = {}
        self.pend: dict = {}                        # id -> (sid, t_obs, wind, pm, faixa)
        self.y_res: dict = {}
        self.beta = None
        self.ano_ajuste = None
        self.hist: list = []                        # (t_emissao, faixa, y)
        self.vento_buf, self.res_buf = [], []
        self.marcos = np.sort(np.unique(np.r_[obs["t_disp"].to_numpy(), self._rot["t_disp"].to_numpy()]))
        return {"obs": obs}

    def tabela_rotulos(self, ev):
        return self._rot

    def ticks(self, ev):
        return self.marcos

    def consumir(self, tabela, df, agora):
        for r in df.itertuples(index=False):
            self.st[r.sid].append((r.t_obs, r.wind))
            self.novas.append(r)
            self.vento_buf.append(r.wind)

    def sinal_deriva(self, tabela, df):
        out = {"vento": self.vento_buf}
        self.vento_buf = []
        if self.res_buf:
            out["residuo_padronizado"], self.res_buf = self.res_buf, []
        return out

    @staticmethod
    def _faixa(w):
        return int(np.searchsorted([34, 64, 96], w, side="right"))

    def _x(self, r):
        hist = [(t, w) for t, w in self.st[r.sid] if t <= r.t_obs]
        def dv(h):
            ant = [w for t, w in hist if t <= r.t_obs - h * H]
            return r.wind - ant[-1] if ant else 0.0
        st = str(r.status)
        x = [1.0, r.wind / 100, (r.wind / 100) ** 2, (1013 - (r.pressure if np.isfinite(r.pressure) else 1005)) / 50,
             dv(6) / 20, dv(12) / 20, r.lat / 30, (r.lat / 30) ** 2, r.lon / 80, float(len(hist) <= 2)]
        x += [float(r.mes == m) for m in (7, 8, 9, 10, 11)]
        x += [float(st == s) for s in ("tropical storm", "hurricane", "extratropical", "subtropical storm", "other low")]
        return np.nan_to_num(np.array(x, float))

    def ajustar(self, agora):
        lim = agora - self.TREINO_ANOS * 365 * 86400       # ciclo de vida: a climatologia muda (e a base mudou de critério)
        ids = [i for i in self.feats if i in self.y_res and self.t_emis[i] >= lim]
        if len(ids) < 1000:
            return
        X = np.array([self.feats[i] for i in ids])
        y = np.array([self.y_res[i] for i in ids], float)
        self.beta = logistica(X, y, lam=1e-3)
        lats = X[:, 6] * 30
        self.sup_lat = (float(lats.min()), float(lats.max()))
        self.ano_ajuste = pd.Timestamp(int(agora), unit="s").year

    def precisa_ajuste(self, agora):
        return self.ano_ajuste is None or pd.Timestamp(int(agora), unit="s").year != self.ano_ajuste

    def _reserva(self, faixa, agora):
        jan = [y for t, f, y in self.hist if f == faixa and t >= agora - 10 * 365 * 86400]
        todos = [y for t, f, y in self.hist if t >= agora - 10 * 365 * 86400]
        g = np.mean(todos) if todos else 0.05
        return (sum(jan) + 20 * g) / (len(jan) + 20)

    def emitir(self, agora):
        out = []
        for r in self.novas:
            x = self._x(r)
            self.feats[r.id] = x
            self.t_emis[r.id] = agora
            f = self._faixa(r.wind)
            pr = self._reserva(f, agora)
            pm = float(np.clip(sigmoide(x @ self.beta), 0.01, 0.99)) if self.beta is not None else pr
            esc = self.beta is not None and self.sup_lat[0] <= r.lat <= self.sup_lat[1]
            self.pend[r.id] = (r.sid, r.t_obs, r.wind, pm, f)
            out.append({"id": r.id, "t_alvo": int(r.t_obs + 24 * H), "modelo": pm, "reserva": pr, "escopo_ok": bool(esc)})
        self.novas = []
        return out

    def resolver(self, agora):
        out = []
        for i in [i for i, v in self.pend.items() if v[1] + 24 * H + LAT <= agora]:
            sid, t0, w0, pm, f = self.pend.pop(i)
            fut = [w for t, w in self.st[sid] if t0 < t <= t0 + 24 * H]
            y = int(bool(fut) and max(fut) >= w0 + 30)
            self.y_res[i] = y
            self.hist.append((t0 + LAT, f, y))
            self.res_buf.append((y - pm) / np.sqrt(pm * (1 - pm)))
            out.append((i, y, int(t0 + 24 * H + LAT)))
        return out

    def decidir(self, p):
        return bool(p > self.LIMIAR)
