"""Fluxo energia: demanda elétrica de Victoria (AEMO), meia-hora a meia-hora, 2012–2014.

Pergunta (F7): quanta demanda haverá em cada meia-hora de amanhã, com que incerteza, para contratar a
reserva? A previsão é emitida às 00:00 UTC para as 48 meias-horas do dia UTC seguinte (horizonte de
24 a 48 h). A temperatura do instante-alvo NÃO é conhecida na emissão; só a observada até ali. Usar a
temperatura do próprio instante seria o vazamento clássico desta base, que ela traz na mesma linha.
"""

from __future__ import annotations

import numpy as np
import pandas as pd

from estocologia.monitor import CalibradorQuantis

from .base import FluxoBase, ridge, segundos, ts

DIA = 86400
MEIA = 1800


class Energia(FluxoBase):
    nome, area, tipo = "energia", "energia", "quantis"
    niveis = np.array([0.05, 0.5, 0.95])
    linhas_por_grupo = {"demanda": 48, "temperatura": 48, "calendario": 4096}
    sensores = {"temperatura_anomalia": {"n_janela": 336, "n_ref": 1344},
                "residuo_padronizado": {"n_janela": 336, "n_ref": 1344}}
    disjuntor = {"janela": 48 * 14, "margem": 0.05, "k_on": 3, "k_off": 3, "intervalo": 48}
    corte = ts("2014-01-01")
    inicio = ts("2012-03-01")
    TREINO_DIAS = 365
    REAJUSTE_DIAS = 7
    LIMIAR_RESERVA_MW = 7000.0
    cabecalho = {
        "processo_gerador": "demanda agregada de Victoria: ciclo diário e semanal, mais carga térmica (ar-condicionado no calor, aquecimento no frio)",
        "nao_linearidade": "resposta em U à temperatura; picos de calor extremo fora do suporte",
        "unidade_e_margem": "a meia-hora; margem por dia (48 previsões emitidas juntas, erros correlacionados)",
        "fisica_do_ruido": "erro dominado pela temperatura de amanhã, desconhecida na emissão",
        "escada_causal": "degrau 1 (associação); a decisão de reserva não altera a demanda",
    }
    contrato = {
        "pergunta": "demanda de cada meia-hora de amanhã, para contratar reserva",
        "fonte": "tsibbledata::vic_elec (AEMO), via Rdatasets",
        "unidade": "meia-hora; dia como bloco",
        "pode_influir": False, "degrau_pearl": 1, "tem_titulares": False,
        "decisao": {"acao": "acionar reserva extra na meia-hora", "regra": "q95 > 7000 MW (C/L: perda de déficit 19 vezes o custo da reserva)"},
        "escopo": "temperatura máxima das últimas 24 h dentro do suporte do treino",
        "disponibilidade": {"demanda": "fim da meia-hora", "temperatura": "fim da meia-hora", "calendario": "publicado antes do início"},
    }

    def eventos(self, dados):
        d = pd.read_csv(dados / "h2" / "tsibbledata_vic_elec.csv")
        T = segundos(d["Time"])
        self.T0 = int(T.min())
        dem = pd.DataFrame({"T": T, "D": d["Demand"].astype(float), "t_disp": T + MEIA})
        tem = pd.DataFrame({"T": T, "temp": d["Temperature"].astype(float), "t_disp": T + MEIA})
        cal = d.groupby("Date", as_index=False)["Holiday"].first()
        cal = pd.DataFrame({"data": cal["Date"], "feriado": cal["Holiday"].astype(str).str.upper().eq("TRUE").astype(int),
                            "t_disp": self.T0 - 2 * DIA})
        n = int((T.max() - self.T0) // MEIA) + 1
        self.D = np.full(n, np.nan)
        self.temp = np.full(n, np.nan)
        self.feriado: dict = {}
        self.pend: dict = {}
        self.res_buf: list = []
        self.anom_buf: list = []
        self.cal_m = CalibradorQuantis(self.niveis)
        self.cal_r = CalibradorQuantis(self.niveis)
        self.beta = None
        self.ult_ajuste = -10**12
        self.fim = int(T.max())
        return {"demanda": dem, "temperatura": tem, "calendario": cal}

    def ticks(self, ev):
        return np.arange(self.T0 - self.T0 % DIA + DIA, self.fim - DIA, DIA)

    def consumir(self, tabela, df, agora):
        if tabela == "calendario":
            self.feriado.update(zip(df["data"], df["feriado"]))
            return
        k = ((df["T"].to_numpy() - self.T0) // MEIA).astype(int)
        if tabela == "demanda":
            self.D[k] = df["D"].to_numpy()
        else:
            v = df["temp"].to_numpy()
            self.temp[k] = v
            for kk, vv in zip(k, v):
                passado = self.temp[kk - 48 * 30:kk:48] if kk >= 48 * 30 else np.array([])
                if passado.size and np.isfinite(passado).sum() >= 20:
                    self.anom_buf.append(vv - np.nanmean(passado))

    def sinal_deriva(self, tabela, df):
        out = {}
        if tabela == "temperatura" and self.anom_buf:
            out["temperatura_anomalia"], self.anom_buf = self.anom_buf, []
        if tabela == "demanda" and self.res_buf:
            out["residuo_padronizado"], self.res_buf = self.res_buf, []
        return out

    # ---------- features na emissão ----------
    def _dia_local(self, T):
        loc = pd.to_datetime(np.asarray(T), unit="s", utc=True).tz_convert("Australia/Melbourne")
        return loc.strftime("%Y-%m-%d"), loc.dayofweek.to_numpy()

    def _feat(self, ks: np.ndarray, emissao: np.ndarray) -> np.ndarray:
        """Features de cada alvo k como seriam vistas na emissão (só passado da emissão)."""
        T = self.T0 + ks * MEIA
        ke = ((emissao - self.T0) // MEIA).astype(int)            # primeira meia-hora não observada na emissão
        janelas = ke[:, None] - np.arange(1, 49)[None, :]
        tj = self.temp[np.clip(janelas, 0, None)]
        tmean = np.nanmean(tj, axis=1)
        tmax = np.nanmax(tj, axis=1)
        lag7 = self.D[ks - 336]
        lag2 = self.D[ks - 96]
        datas, dow = self._dia_local(T)
        fer = np.array([self.feriado.get(x, 0) for x in datas])
        slot = ks % 48
        X = [np.ones(len(ks)), lag7, lag2, np.maximum(tmax - 22, 0), np.maximum(tmax - 30, 0), np.maximum(16 - tmean, 0), fer]
        X += [(slot == s).astype(float) for s in range(1, 48)]
        X += [(dow == j).astype(float) for j in (5, 6)]
        X += [np.maximum(tmax - 22, 0) * (np.abs(slot - 10) < 8)]      # calor pesa mais na tarde local (UTC 01–09h)
        return np.column_stack(X), tmax

    def ajustar(self, agora):
        k_fim = (agora - self.T0) // MEIA            # alvos com demanda já observada
        k_ini = max(336 + 96, k_fim - 48 * self.TREINO_DIAS)
        ks = np.arange(k_ini, k_fim)
        ks = ks[np.isfinite(self.D[ks])]
        if ks.size < 48 * 30:
            return
        T = self.T0 + ks * MEIA
        emis = (T // DIA) * DIA - DIA
        X, tmax = self._feat(ks, emis)
        ok = np.isfinite(X).all(1)
        X, y, sl = X[ok], self.D[ks][ok], (ks % 48)[ok]
        self.beta = ridge(X, y, 1e-4)
        r = y - X @ self.beta
        self.s_slot = np.array([np.std(r[sl == s]) if (sl == s).sum() > 5 else np.std(r) for s in range(48)])
        self.tmax_sup = (np.quantile(tmax[ok], 0.001), np.quantile(tmax[ok], 0.999))
        recent = ks[-48 * 56:]
        e = self.D[recent] - self.D[recent - 336]
        self.s_res = float(np.nanstd(e))
        self.ult_ajuste = agora

    def precisa_ajuste(self, agora):
        return agora - self.ult_ajuste >= self.REAJUSTE_DIAS * DIA and agora >= self.inicio - 7 * DIA

    def emitir(self, agora):
        if self.beta is None:
            self.ajustar(agora)
            if self.beta is None:
                return []
        ks = np.arange((agora + DIA - self.T0) // MEIA, (agora + 2 * DIA - self.T0) // MEIA)
        ks = ks[ks < len(self.D)]
        X, tmax = self._feat(ks, np.full(ks.size, agora))
        m = X @ self.beta
        s = self.s_slot[ks % 48]
        mr = self.D[ks - 336]
        sr = np.full(ks.size, self.s_res)
        bad = ~np.isfinite(m)
        m[bad], s[bad] = mr[bad], sr[bad]
        qm, qr = self.cal_m.quantis(m, s), self.cal_r.quantis(mr, sr)
        out = []
        for i, k in enumerate(ks):
            id_ = f"e{k}"
            self.pend[k] = (id_, m[i], s[i], qm[i], mr[i], sr[i], qr[i])
            esc = bool(self.tmax_sup[0] <= tmax[i] <= self.tmax_sup[1]) and not bad[i]
            out.append({"id": id_, "t_alvo": int(self.T0 + k * MEIA), "modelo": qm[i], "reserva": qr[i], "escopo_ok": esc})
        return out

    def resolver(self, agora):
        out = []
        for k in [k for k in self.pend if np.isfinite(self.D[k])]:
            id_, m, s, qm, mr, sr, qr = self.pend.pop(k)
            y = self.D[k]
            self.cal_m.atualizar(y, m, s, qm)
            self.cal_r.atualizar(y, mr, sr, qr)
            self.res_buf.append((y - m) / s)
            out.append((id_, y, int(self.T0 + k * MEIA + MEIA)))
        return out

    def decidir(self, q):
        return bool(q[2] > self.LIMIAR_RESERVA_MW)
