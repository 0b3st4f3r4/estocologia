"""Fluxo voos: todas as partidas de Nova York (JFK, LGA, EWR) em 2013.

Pergunta (F7): este voo vai chegar com mais de 15 min de atraso, ou ser cancelado ou desviado? A
previsão é emitida 2 h antes da partida programada, a cada hora, para os voos que partem entre 2 e
3 h depois.

Na emissão só se sabe:
- a grade, publicada uma semana antes;
- as partidas já ocorridas, com o atraso de cada uma;
- o tempo meteorológico observado no aeroporto até a hora anterior.

O atraso do próprio voo, que a base traz na mesma linha, é o vazamento óbvio. Aqui ele só chega
quando o avião parte, e o rótulo só quando pousa.

Instantes (UTC):
- partida programada = time_hour + minuto;
- partida real = programada + dep_delay;
- chegada ≈ partida real + air_time + 20 min de táxi (suposição declarada, porque a base não traz o
  instante de pouso em UTC);
- cancelado: o rótulo vale 1 e passa a ser conhecido na partida programada;
- desviado (partiu e não tem atraso de chegada): o rótulo vale 1 e passa a ser conhecido 6 h depois
  da partida.

Modelo (F4): logística reajustada a cada dia sobre os 60 dias anteriores. As features são as
gravadas no instante de cada emissão passada, e por isso o treino herda a disciplina temporal da
emissão.
"""

from __future__ import annotations

from collections import defaultdict, deque

import numpy as np
import pandas as pd

from .base import FluxoBase, logistica, segundos, sigmoide, ts

H = 3600


class Voos(FluxoBase):
    nome, area, tipo = "voos", "logistica", "binario"
    linhas_por_grupo = {"agenda": 40, "partida": 40, "chegada": 40, "clima": 3, "canario": 40}
    sensores = {"atraso_partida": {"n_janela": 2000, "n_ref": 8000},
                "residuo_padronizado": {"n_janela": 2000, "n_ref": 8000}}
    disjuntor = {"janela": 3000, "margem": 0.03, "k_on": 3, "k_off": 3, "intervalo": 300}
    corte = ts("2013-10-01")
    inicio = ts("2013-01-22")
    TREINO_DIAS = 60
    LIMIAR = 0.35
    cabecalho = {
        "processo_gerador": "propagação de atrasos na rede: tempo meteorológico, congestionamento do aeroporto e da companhia ao longo do dia",
        "nao_linearidade": "atrasos se acumulam ao longo do dia (efeito de rede); cauda pesada nos dias de tempestade",
        "unidade_e_margem": "o voo; a margem efetiva é o dia por aeroporto (efeito de desenho 11,5 no H2)",
        "fisica_do_ruido": "choques comuns por dia e aeroporto; ruído idiossincrático do voo",
        "escada_causal": "degrau 1; o alerta ao passageiro não muda o voo",
    }
    contrato = {
        "pergunta": "P(atraso de chegada > 15 min, cancelamento ou desvio), 2 h antes da partida",
        "fonte": "nycflights13::flights e ::weather, via Rdatasets",
        "unidade": "voo; dia × aeroporto como bloco",
        "pode_influir": False, "degrau_pearl": 1, "tem_titulares": False,
        "decisao": {"acao": "remarcar conexão do passageiro", "regra": "p > 0,35 = C/L (custo da remarcação / perda da conexão)"},
        "escopo": "companhia e aeroporto vistos no treino",
        "disponibilidade": {"agenda": "7 dias antes da partida programada", "partida": "na partida real (cancelado: na programada)",
                            "chegada": "no pouso estimado", "clima": "1 h depois da observação"},
    }

    def eventos(self, dados):
        f = pd.read_csv(dados / "h2" / "nycflights13_flights.csv")
        w = pd.read_csv(dados / "h2" / "nycflights13_weather.csv")
        prog = segundos(f["time_hour"]) + f["minute"].astype(int) * 60
        f = f.assign(id=("v" + f["rownames"].astype(str)), prog=prog)
        canc = f["dep_time"].isna() | f["dep_delay"].isna()
        real = f["prog"] + f["dep_delay"].fillna(0) * 60
        desv = ~canc & f["arr_delay"].isna()
        y = (canc | desv | (f["arr_delay"] > 15)).astype(int)
        t_rot = np.where(canc, f["prog"], np.where(desv, real + 6 * H, real + (f["air_time"].fillna(0) + 20) * 60)).astype("int64")
        loc = pd.to_datetime(f["prog"], unit="s", utc=True).dt.tz_convert("America/New_York")
        agenda = pd.DataFrame({"id": f["id"], "prog": f["prog"], "carrier": f["carrier"], "origin": f["origin"], "dest": f["dest"],
                               "distance": f["distance"].astype(float), "hora": loc.dt.hour, "dow": loc.dt.dayofweek,
                               "t_disp": f["prog"] - 7 * 86400})
        partida = pd.DataFrame({"id": f["id"], "origin": f["origin"], "carrier": f["carrier"],
                                "dep_delay": f["dep_delay"].astype(float), "t_disp": np.where(canc, f["prog"], real).astype("int64")})
        chegada = pd.DataFrame({"id": f["id"], "y": y, "t_disp": t_rot})
        wt = segundos(w["time_hour"])
        clima = pd.DataFrame({"origin": w["origin"], "t_obs": wt, "visib": w["visib"].astype(float),
                              "wind": w["wind_speed"].astype(float).clip(upper=60), "precip": w["precip"].astype(float),
                              "t_disp": wt + H})
        self._rot = chegada
        self.ag: dict = {}
        self.por_hora: dict = defaultdict(list)
        self.part_orig: dict = defaultdict(deque)       # origem -> (t_disp, atrasado>15, cancelado)
        self.part_cia: dict = defaultdict(deque)
        self.clima: dict = {}
        self.y: dict = {}
        self.t_y: dict = {}
        self.feats: dict = {}                            # id -> (t_emissao, x)
        self.pend: dict = {}
        self.res_buf, self.dep_buf = [], []
        self.beta = None
        self.ult_ajuste = -10**12
        self.base: dict = defaultdict(lambda: [0, 0])    # (origem, bloco de hora) -> [soma y, n], últimos 28 dias
        self.hist_base: deque = deque()
        t0 = int(f["prog"].min())
        self.t_ini = t0 - t0 % H
        self.t_fim = int(t_rot.max())
        cias = sorted(f["carrier"].unique())
        dests = f["dest"].value_counts().index[:30].tolist()
        self.cias, self.dests, self.origens = cias, dests, ["EWR", "JFK", "LGA"]
        return {"agenda": agenda, "partida": partida, "chegada": chegada, "clima": clima}

    def tabela_rotulos(self, ev):
        return self._rot

    def ticks(self, ev):
        return np.arange(self.t_ini, self.t_fim + H, H)

    def consumir(self, tabela, df, agora):
        if tabela == "agenda":
            for r in df.itertuples(index=False):
                self.ag[r.id] = r
                self.por_hora[(r.prog // H) * H].append(r.id)
        elif tabela == "partida":
            for i, o, c, d, t in zip(df["id"], df["origin"], df["carrier"], df["dep_delay"], df["t_disp"]):
                canc = not np.isfinite(d)
                self.part_orig[o].append((t, (not canc) and d > 15, canc))
                self.part_cia[c].append((t, d if not canc else np.nan))
                if not canc:
                    self.dep_buf.append(min(d, 300.0))
        elif tabela == "clima":
            for o, v, wv, p, t in zip(df["origin"], df["visib"], df["wind"], df["precip"], df["t_obs"]):
                self.clima[o] = (v, wv, p, t)
        elif tabela == "chegada":
            for i, y, t in zip(df["id"], df["y"], df["t_disp"]):
                self.y[i] = int(y)
                self.t_y[i] = int(t)

    def sinal_deriva(self, tabela, df):
        out = {}
        if tabela == "partida" and self.dep_buf:
            out["atraso_partida"], self.dep_buf = self.dep_buf, []
        if tabela == "chegada" and self.res_buf:
            out["residuo_padronizado"], self.res_buf = self.res_buf, []
        return out

    def _estado(self, agora):
        """Agregados do passado imediato, por origem e companhia, até agora."""
        out = {}
        for o, dq in self.part_orig.items():
            while dq and dq[0][0] <= agora - 3 * H:
                dq.popleft()
            n = len(dq)
            out[("o", o)] = (sum(a for _, a, _ in dq) / n if n else 0.2, sum(c for _, _, c in dq), n)
        for c, dq in self.part_cia.items():
            while dq and dq[0][0] <= agora - 6 * H:
                dq.popleft()
            v = [d for _, d in dq if np.isfinite(d)]
            out[("c", c)] = (float(np.mean(np.clip(v, -30, 180))) if v else 10.0, len(dq) - len(v))
        return out

    def _x(self, r, est, agora):
        o_taxa, o_canc, o_n = est.get(("o", r.origin), (0.2, 0, 0))
        c_med, c_canc = est.get(("c", r.carrier), (10.0, 0))
        v, wv, p, tob = self.clima.get(r.origin, (10.0, 10.0, 0.0, 0))
        v, wv, p = (10.0 if not np.isfinite(v) else v), (10.0 if not np.isfinite(wv) else wv), (0.0 if not np.isfinite(p) else p)
        fresco = agora - tob <= 3 * H
        x = [1.0, o_taxa, np.log1p(o_canc), np.log1p(o_n), c_med / 60, np.log1p(c_canc),
             float(v < 3) * fresco, wv / 20 * fresco, float(p > 0) * fresco, r.distance / 1000]
        x += [float(r.hora == h) for h in range(6, 23)]
        x += [float(r.dow == d) for d in range(1, 7)]
        x += [float(r.carrier == c) for c in self.cias[1:]]
        x += [float(r.origin == o) for o in self.origens[1:]]
        x += [float(r.dest == d) for d in self.dests]
        return np.nan_to_num(np.array(x, dtype=float))

    def ajustar(self, agora):
        ids = [i for i, (te, _) in self.feats.items() if i in self.y and te >= agora - self.TREINO_DIAS * 86400]
        if len(ids) < 3000:
            return
        X = np.array([self.feats[i][1] for i in ids])
        y = np.array([self.y[i] for i in ids], float)
        self.beta = logistica(X, y, lam=1e-4)
        self.cias_treino = {self.ag[i].carrier for i in ids}
        self.ult_ajuste = agora
        for i in [i for i, (te, _) in self.feats.items() if te < agora - (self.TREINO_DIAS + 5) * 86400]:
            del self.feats[i]

    def precisa_ajuste(self, agora):
        return agora - self.ult_ajuste >= 86400

    def _reserva(self, r):
        b = (r.origin, r.hora // 4)
        s, n = self.base[b]
        S, N = sum(v[0] for v in self.base.values()), sum(v[1] for v in self.base.values())
        glob = S / N if N else 0.2
        return (s + 20 * glob) / (n + 20)

    def emitir(self, agora):
        alvo = agora + 2 * H
        ids = self.por_hora.pop(alvo, [])
        if not ids:
            return []
        est = self._estado(agora)
        out = []
        for i in ids:
            r = self.ag[i]
            x = self._x(r, est, agora)
            self.feats[i] = (agora, x)
            pr = self._reserva(r)
            pm = float(np.clip(sigmoide(x @ self.beta), 0.01, 0.99)) if self.beta is not None else pr
            esc = self.beta is not None and r.carrier in self.cias_treino
            self.pend[i] = (pm, (r.origin, r.hora // 4), agora)
            out.append({"id": i, "t_alvo": int(r.prog), "modelo": pm, "reserva": pr, "escopo_ok": bool(esc)})
        return out

    def resolver(self, agora):
        out = []
        for i in [i for i in self.pend if i in self.y]:
            pm, b, te = self.pend.pop(i)
            y = self.y[i]
            self.res_buf.append((y - pm) / np.sqrt(pm * (1 - pm)))
            self.base[b][0] += y
            self.base[b][1] += 1
            self.hist_base.append((agora, b, y))
            out.append((i, y, self.t_y[i]))
        while self.hist_base and self.hist_base[0][0] < agora - 28 * 86400:
            _, b, y = self.hist_base.popleft()
            self.base[b][0] -= y
            self.base[b][1] -= 1
        return out

    def decidir(self, p):
        return bool(p > self.LIMIAR)
