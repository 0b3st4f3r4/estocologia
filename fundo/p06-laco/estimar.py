"""Ponto de fundo 6: mede o laço desistência → nova chamada no AnonymousBank 1999, como declarado em DECLARACAO.md.

Uso: python3 -I estimar.py  (escreve resultados.json ao lado)
"""
from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

import numpy as np
import pandas as pd

AQUI = Path(__file__).resolve().parent
sys.path.insert(0, str(AQUI))
from carregar import carregar  # noqa: E402

DIA = 86400
JANELAS = {"15min": 900, "60min": 3600}
N_BOOT = 1000
SEMENTE = 2026
R_H3, R_HAT_H3, R_HAT_IC_H3, VIES_H3 = 0.6, 0.5632133268561669, None, 0.29124674633711956


def preparar():
    d = carregar()
    d = d[d["outcome"].isin(["AGENT", "HANG", "PHANTOM"])].copy()
    d["type"] = d["type"].astype(str).str.strip()
    d = d.dropna(subset=["vru_entry_s", "q_start_s", "q_exit_s", "ser_exit_s"])
    d0 = d["date"].min()
    d["dia"] = (d["date"] - d0).dt.days.astype(int)
    base = d["dia"].to_numpy() * DIA
    cid = d["customer_id"].astype(str)
    d["id_ok"] = (cid != "0") & ~cid.str.contains("E", regex=False) & (cid != "nan")
    d["fila"] = d["q_start_s"] > 0
    # instantes absolutos; uma saída antes da entrada é passagem de meia-noite
    qs = base + d["q_start_s"].to_numpy()
    qe = base + d["q_exit_s"].to_numpy()
    qe = np.where(d["fila"] & (qe < qs), qe + DIA, qe)
    ve = base + d["vru_entry_s"].to_numpy()
    se = base + d["ser_exit_s"].to_numpy()
    se = np.where((d["outcome"] == "AGENT") & (se < qe) & (d["ser_exit_s"] > 0), se + DIA, se)
    d["qs"], d["qe"], d["ve"], d["se"] = qs, qe, ve, se
    return d


def fila_no_tempo(d):
    """Número na fila em cada segundo (todas as chamadas, identificadas ou não, exceto TT não importa: todas ocupam a fila)."""
    f = d[d["fila"] & (d["qe"] > d["qs"])]
    T = int(d["qe"].max()) + 2 * DIA
    dif = np.zeros(T + 1, np.int32)
    np.add.at(dif, f["qs"].to_numpy(int), 1)
    np.add.at(dif, f["qe"].to_numpy(int), -1)
    q = np.cumsum(dif)                       # q[t] = #{q_start ≤ t < q_exit}
    cum = np.concatenate([[0], np.cumsum(q, dtype=np.int64)])   # integral de q em [0, t)
    return q, cum


def montar(d):
    q, cum = fila_no_tempo(d)
    idx = d[d["fila"] & d["id_ok"] & d["outcome"].isin(["AGENT", "HANG"]) & (d["type"] != "TT")].copy()
    idx["D"] = (idx["outcome"] == "HANG").astype(float)
    proprio = (idx["qe"] > idx["qs"]).astype(int)
    idx["Z"] = q[idx["qs"].to_numpy(int)] - proprio.to_numpy()
    idx["fim"] = np.where(idx["D"] == 1, idx["qe"], np.maximum(idx["se"], idx["qe"]))
    # pool de novas chamadas: identificadas, não TT, não PHANTOM
    pool = d[d["id_ok"] & (d["type"] != "TT") & (d["outcome"] != "PHANTOM")]
    por_cli = {c: np.sort(g["ve"].to_numpy()) for c, g in pool.groupby("customer_id")}
    fim = idx["fim"].to_numpy()
    cli = idx["customer_id"].to_numpy()
    meia_noite = (np.floor(fim / DIA) + 1) * DIA
    Y = {k: np.zeros(len(idx)) for k in [*JANELAS, "mesmo_dia"]}
    for i in range(len(idx)):
        v = por_cli.get(cli[i])
        if v is None:
            continue
        a = np.searchsorted(v, fim[i], side="right")
        for k, w in JANELAS.items():
            Y[k][i] = float(np.searchsorted(v, fim[i] + w, side="right") > a)
        Y["mesmo_dia"][i] = float(np.searchsorted(v, meia_noite[i], side="right") > a)
    for k in Y:
        idx["Y_" + k] = Y[k]
    # controle da robustez 1: fila média de OUTROS clientes nos 15 min depois do fim
    a, b = fim.astype(int), fim.astype(int) + 900
    integ = (cum[b] - cum[a]).astype(float)
    fq = d[d["fila"] & (d["qe"] > d["qs"]) & d["id_ok"]]
    proprias = {c: (g["qs"].to_numpy(), g["qe"].to_numpy()) for c, g in fq.groupby("customer_id")}
    for i in range(len(idx)):
        p = proprias.get(cli[i])
        if p is None:
            continue
        sobre = np.clip(np.minimum(p[1], b[i]) - np.maximum(p[0], a[i]), 0, None).sum()
        integ[i] -= sobre
    idx["X_fila_depois"] = integ / 900.0
    idx["hora"] = (idx["q_start_s"] // 3600).astype(int)
    idx["prio"] = (idx["priority"] == 2).astype(int)
    idx["celula"] = idx["dia"].astype(str) + "_" + idx["hora"].astype(str) + "_" + idx["prio"].astype(str) + "_" + idx["type"]
    return idx


def centrar(idx, cols):
    g = idx.groupby("celula")[cols].transform("mean")
    return idx[cols].to_numpy() - g.to_numpy()


def somas_por_dia(dias, M):
    """Produtos cruzados por dia: S[d] = Σ_i m_i m_iᵀ."""
    ud, inv = np.unique(dias, return_inverse=True)
    k = M.shape[1]
    S = np.zeros((ud.size, k, k))
    for a in range(k):
        for b in range(a, k):
            S[:, a, b] = np.bincount(inv, M[:, a] * M[:, b], minlength=ud.size)
            S[:, b, a] = S[:, a, b]
    return S, inv, ud.size


def tsls(S, iy, idd, iz, ix=None):
    """2SLS exatamente identificado a partir de produtos cruzados somados; ix = controle exógeno (FWL)."""
    if ix is None:
        return S[iz, iy] / S[iz, idd]
    def res(u, v):  # <ũ, ṽ> com ũ = u − x·(x'u/x'x)
        return S[u, v] - S[u, ix] * S[ix, v] / S[ix, ix]
    return res(iz, iy) / res(iz, idd)


def estimar(idx, Wk, Zcol="Z", controle=False):
    cols = ["Y_" + Wk, "D", Zcol] + (["X_fila_depois"] if controle else [])
    M = centrar(idx, cols)
    S, inv, nd = somas_por_dia(idx["dia"].to_numpy(), M)
    ix = 3 if controle else None
    tot = S.sum(0)
    beta = tsls(tot, 0, 1, 2, ix)
    ols = tot[1, 0] / tot[1, 1] if not controle else (tot[1, 0] - tot[1, 3] * tot[3, 0] / tot[3, 3]) / (tot[1, 1] - tot[1, 3] ** 2 / tot[3, 3])
    # F do primeiro estágio, robusto por dia (sem controle: z, d centrados)
    zc, dc = M[:, 2], M[:, 1]
    if controle:
        x = M[:, 3]
        zc = zc - x * (x @ zc) / (x @ x)
        dc = dc - x * (x @ dc) / (x @ x)
    pi = (zc @ dc) / (zc @ zc)
    u = dc - pi * zc
    sc = np.bincount(inv, zc * u, minlength=nd)
    F = pi ** 2 / ((sc @ sc) / (zc @ zc) ** 2)
    rng = np.random.default_rng(SEMENTE)
    bb, bo = [], []
    for _ in range(N_BOOT):
        w = np.bincount(rng.integers(0, nd, nd), minlength=nd).astype(float)
        Sb = np.tensordot(w, S, 1)
        bb.append(tsls(Sb, 0, 1, 2, ix))
        bo.append(Sb[1, 0] / Sb[1, 1] if not controle else (Sb[1, 0] - Sb[1, 3] * Sb[3, 0] / Sb[3, 3]) / (Sb[1, 1] - Sb[1, 3] ** 2 / Sb[3, 3]))
    q = lambda a: [float(np.quantile(a, 0.05)), float(np.quantile(a, 0.95))]
    return {"r_IV": float(beta), "ic90": q(bb), "F_primeiro_estagio": float(F), "pi_primeiro_estagio": float(pi),
            "MQO_passivo": float(ols), "ic90_MQO": q(bo), "n": int(len(idx)), "dias": int(nd)}


def veredito(e):
    lo, hi = e["ic90"]
    if e["F_primeiro_estagio"] < 10:
        return "indecidido (instrumento fraco)"
    if lo <= 0:
        return "não confere"
    ordem = 0.2 <= e["r_IV"] <= 1.8
    if ordem:
        return "o laço do H3 tem o sinal e a ordem do medido"
    return "o laço do H3 superestima o medido" if e["r_IV"] < 0.2 else "o laço do H3 subestima o medido"


def main():
    d = preparar()
    idx = montar(d)
    idx["logZ"] = np.log1p(idx["Z"])
    out = {"declaracao_sha256": hashlib.sha256((AQUI / "DECLARACAO.md").read_bytes()).hexdigest(),
           "h3": {"r_declarado": R_H3, "r_hat_2SLS_simulado": R_HAT_H3, "vies_ingenuo_vs_exogena": VIES_H3}}
    out["amostra"] = {"chamadas_indice": int(len(idx)), "desistentes": int(idx["D"].sum()),
                      "clientes": int(idx["customer_id"].nunique()), "dias": int(idx["dia"].nunique()),
                      "Z_media": float(idx["Z"].mean()), "Z_q50": float(idx["Z"].median()), "Z_q90": float(idx["Z"].quantile(0.9))}
    cru = {}
    for k in [*JANELAS, "mesmo_dia"]:
        cru[k] = {"P(Y|desistiu)": float(idx.loc[idx.D == 1, "Y_" + k].mean()),
                  "P(Y|atendido)": float(idx.loc[idx.D == 0, "Y_" + k].mean())}
    out["taxas_cruas"] = cru
    out["principal_15min"] = principal = estimar(idx, "15min")
    principal["0,6_no_ic90"] = bool(principal["ic90"][0] <= R_H3 <= principal["ic90"][1])
    principal["razao_0,6_sobre_r_IV"] = float(R_H3 / principal["r_IV"]) if principal["r_IV"] != 0 else None
    principal["veredito"] = veredito(principal)
    out["robustez1_controle_fila_depois"] = r1 = estimar(idx, "15min", controle=True)
    out["robustez2_logZ"] = estimar(idx, "15min", Zcol="logZ")
    lim = (0.2, 1.8)
    dentro = lambda x: lim[0] <= x <= lim[1]
    out["fragil"] = bool((np.sign(r1["r_IV"]) != np.sign(principal["r_IV"])) or (dentro(r1["r_IV"]) != dentro(principal["r_IV"])))
    out["secundarias"] = {"60min": estimar(idx, "60min"), "mesmo_dia": estimar(idx, "mesmo_dia")}
    # secundária 1: fração das novas chamadas em 60 min que chega em até 15 min, entre desistentes
    p15, p60 = cru["15min"]["P(Y|desistiu)"], cru["60min"]["P(Y|desistiu)"]
    out["sec1_fracao_15_de_60_desistentes"] = float(p15 / p60)
    # secundária 2: excesso de chegadas observadas causado pelo laço
    nt = d[d["type"] != "TT"]
    desist = int(((nt["outcome"] == "HANG") & nt["fila"]).sum())
    pedidos = int((nt["fila"] & nt["outcome"].isin(["AGENT", "HANG"])).sum() + ((nt["outcome"] == "AGENT") & ~nt["fila"]).sum())
    r = principal["r_IV"]
    out["sec2_excesso"] = {"desistencias_na_fila": desist, "pedidos_de_atendente": pedidos,
                           "excesso_relativo": float(r * desist / (pedidos - r * desist)),
                           "excesso_relativo_ic90": [float(x * desist / (pedidos - x * desist)) for x in principal["ic90"]],
                           "h3_vies_ingenuo": VIES_H3}
    (AQUI / "resultados.json").write_text(json.dumps(out, indent=1, ensure_ascii=False))
    print(json.dumps(out, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
