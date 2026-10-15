"""R4: fluxo prisoes_ri (Rhode Island, 2005–2015) com recalibração de Platt online.

Fonte da especificação: h3/CRITERIOS_H3d.md, h3/CRITERIOS.md, h3/README.md. Escolhas onde a especificação cala
(inclusive a ausência do disjuntor, cujos parâmetros não estão publicados): DECLARACAO.md desta pasta.
"""
import os
import sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
import pandas as pd
from comum import DADOS, gravar, logit_ajustar, sigm

d = pd.read_csv(os.path.join(DADOS, "h3c", "backup_rhode_island_traffic_stops_2005_2015.csv"))
d = d[d.stop_outcome.notna()].copy()
d["t"] = pd.to_datetime(d.stop_date + " " + d.stop_time, format="%Y-%m-%d %H:%M")
d = d.sort_values("t", kind="mergesort").reset_index(drop=True)
d["y"] = (d.stop_outcome == "Arrest Driver").astype(float)
d["t_rot"] = d.t + pd.Timedelta(days=1)
d["ano"] = d.t.dt.year
d["hora"] = d.t.dt.hour
d["faixa"] = np.digitize(d.hora, [6, 12, 18])
d["dow"] = d.t.dt.dayofweek
d["sexo"] = d.driver_gender.fillna("NA")
VIOL = sorted(d.violation_raw.dropna().unique())


def matriz(sub, viols, mu, sd):
    idade = sub.driver_age.to_numpy(float)
    z = np.where(np.isnan(idade), 0.0, (idade - mu) / sd)
    cols = [np.ones(len(sub)), z, z ** 2,
            (sub.sexo == "F").to_numpy(float), (sub.sexo == "NA").to_numpy(float)]
    cols += [(sub.violation_raw == v).to_numpy(float) for v in viols[1:]]
    cols += [(sub.faixa == f).to_numpy(float) for f in (1, 2, 3)]
    cols += [(sub.dow == k).to_numpy(float) for k in range(1, 7)]
    return np.column_stack(cols)


inicio = pd.Timestamp("2006-01-01")
corte = pd.Timestamp("2013-01-01")
fim = d.t.max() + pd.Timedelta(hours=6)
ticks = pd.date_range(inicio, fim, freq="6h")

t_ns = d.t.to_numpy()
trot_ns = d.t_rot.to_numpy()
ordem_rot = np.argsort(trot_ns, kind="mergesort")
trot_ord = trot_ns[ordem_rot]
y = d.y.to_numpy()

p_cru = np.full(len(d), np.nan)
p_of = np.full(len(d), np.nan)
p_clim = np.full(len(d), np.nan)
origem = np.array([""] * len(d), dtype=object)
a, b, eta = 0.0, 1.0, 0.002
ja_rot = 0                      # quantos rótulos (em ordem de disponibilidade) já foram consumidos
soma_y_disp, n_disp = 0.0, 0
ano_modelo = None
modelo = None
logit_cru = np.full(len(d), np.nan)
# rótulos de antes de 2006 também alimentam a climatologia, mas não o Platt (o Platt começa em 2006-01-01)
for k, t0 in enumerate(ticks):
    t1 = t0 + pd.Timedelta(hours=6)
    t0n = t0.to_datetime64()
    # 1) rótulos disponíveis até t0: climatologia e Platt
    novo = np.searchsorted(trot_ord, t0n, side="right")
    for j in ordem_rot[ja_rot:novo]:
        soma_y_disp += y[j]
        n_disp += 1
        if not np.isnan(logit_cru[j]):
            pp = sigm(a + b * logit_cru[j])
            gerr = y[j] - pp
            a, b = a + eta * gerr, b + eta * gerr * logit_cru[j]
    ja_rot = novo
    # 2) reajuste em 1º de janeiro, só com rótulos disponíveis
    if t0.year != ano_modelo:
        ano_modelo = t0.year
        disp = ordem_rot[:ja_rot]
        tr = d.iloc[np.sort(disp)]
        tr = tr[tr.ano < ano_modelo]
        viols = [v for v in tr.violation_raw.value_counts().index]
        idade = tr.driver_age.to_numpy(float)
        mu, sd = np.nanmean(idade), np.nanstd(idade)
        Xtr = matriz(tr, viols, mu, sd)
        beta = logit_ajustar(Xtr, tr.y.to_numpy(), lam=1e-4)
        reserva = float(d.y[d.ano == ano_modelo - 1].mean())
        modelo = (set(viols), viols, mu, sd, beta, reserva)
    # 3) emissões no tick
    i0, i1 = np.searchsorted(t_ns, [t0n, t1.to_datetime64()], side="left")
    if i1 <= i0:
        continue
    sub = d.iloc[i0:i1]
    escopo, viols, mu, sd, beta, reserva = modelo
    lg = matriz(sub, viols, mu, sd) @ beta
    pc = sigm(lg)
    dentro = sub.violation_raw.isin(escopo).to_numpy()
    logit_cru[i0:i1] = np.where(dentro, lg, np.nan)
    p_cru[i0:i1] = np.where(dentro, pc, reserva)
    p_of[i0:i1] = np.where(dentro, sigm(a + b * lg), reserva)
    origem[i0:i1] = np.where(dentro, "modelo_platt", "reserva_escopo")
    p_clim[i0:i1] = soma_y_disp / n_disp


def metricas(p, yy, clim):
    o = np.argsort(p, kind="mergesort")
    caixas = np.array_split(o, 10)
    ece = sum(len(c) / len(p) * abs(p[c].mean() - yy[c].mean()) for c in caixas)
    bs = np.mean((p - yy) ** 2)
    bc = np.mean((clim - yy) ** 2)
    return {"citl_abs": float(abs(p.mean() - yy.mean())), "ece": float(ece), "habilidade": float(1 - bs / bc)}


m = (d.t >= corte).to_numpy()
yc = y[m]
ybar = float(yc.mean())
of = metricas(p_of[m], yc, p_clim[m])
cru = metricas(p_cru[m], yc, p_clim[m])
crit = {"a": of["citl_abs"] <= 0.02 and of["ece"] <= 0.03 and of["habilidade"] > 0,
        "b": of["citl_abs"] <= 0.2 * ybar and of["ece"] <= 0.3 * ybar}
res = {"n_cofre": int(m.sum()), "ybar_cofre": ybar, "oficial": of, "cru": cru,
       "oficial_rel": {"citl_sobre_ybar": of["citl_abs"] / ybar, "ece_sobre_ybar": of["ece"] / ybar},
       "cru_rel": {"citl_sobre_ybar": cru["citl_abs"] / ybar, "ece_sobre_ybar": cru["ece"] / ybar},
       "platt_final": [float(a), float(b)], "frac_reserva_cofre": float(np.mean(origem[m] != "modelo_platt")),
       "criterio": crit, "passa": bool(crit["a"] and crit["b"]),
       "n_total_com_desfecho": int(len(d)), "categorias_violacao": VIOL}
gravar("r4_ri.json", res)
print(res)
