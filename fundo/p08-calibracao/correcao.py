"""(c) A correção dos voos posta à prova (DECLARACAO.md, item c).

- c3: réplica fora da linha da recalibração do intercepto (validada contra o δ do diário de 2023) aplicada a 2013.
- c1/c2: G2 do cofre e tabela por trimestre (e por mês, informativa) dos replays novos e do diário de 2023.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

import numpy as np
import pandas as pd

from comum import AQUI, REPLAYS, C, cofre, diario, resultados

ETA = 0.002
FUSO = {"voos": "America/New_York", "voos23": "America/New_York", "voos17": "America/New_York",
        "voospnw14": "America/Los_Angeles", "voos23e1": "America/New_York", "voos23e2": "America/New_York"}


def sig(z):
    return 1 / (1 + np.exp(-np.clip(z, -30, 30)))


def replica_delta(d: pd.DataFrame, p_cru: np.ndarray, delta0: float = 0.0):
    """Recursão δ ← δ + η (y − p') na ordem do motor: no tick T resolvem-se os rótulos com t_rotulo ≤ T, depois emite-se."""
    te, tr = d["t_emissao"].to_numpy(), d["t_rotulo"].to_numpy()
    y = d["y"].to_numpy(float)
    ok = np.isfinite(p_cru)
    n = len(d)
    p_rec = np.full(n, np.nan)
    delta_em = np.full(n, np.nan)
    ordem_e = np.argsort(te, kind="stable")
    ordem_r = np.argsort(np.where(np.isfinite(tr), tr, np.inf), kind="stable")
    ticks, ini = np.unique(te[ordem_e], return_index=True)
    fim = np.r_[ini[1:], n]
    delta, jr = delta0, 0
    for T, a, b in zip(ticks, ini, fim):
        while jr < n and np.isfinite(tr[ordem_r[jr]]) and tr[ordem_r[jr]] <= T:
            i = ordem_r[jr]
            if ok[i] and np.isfinite(y[i]):
                delta += ETA * (y[i] - p_rec[i])
            jr += 1
        idx = ordem_e[a:b]
        pm = np.clip(p_cru[idx], 1e-4, 1 - 1e-4)
        p_rec[idx] = np.clip(sig(np.log(pm / (1 - pm)) + delta), 0.01, 0.99)
        delta_em[idx] = delta
    return p_rec, delta_em


def oficial(d, p_modelo):
    return np.where(d["origem"] == "modelo", p_modelo, d["reserva"]).astype(float)


def metricas(d_full: pd.DataFrame, mask: np.ndarray, p_of: np.ndarray) -> dict:
    clima = C.climatologia_movel(d_full)[mask]
    sub = d_full[mask].assign(modelo=p_of[mask], origem="modelo")
    m = C.binario(sub, clima)
    m["passa_G2"] = bool(abs(m["citl"]) <= 0.02 and m["ece"] <= 0.03 and m["habilidade_vs_climatologia"] > 0)
    return {k: m[k] for k in ("n", "p_medio", "y_medio", "citl", "ece", "habilidade_vs_climatologia", "passa_G2")}


def por_periodo(d: pd.DataFrame, p_of, p_cru_of, fuso, freq):
    r = d[d["y"].notna()].copy()
    idx = d["y"].notna().to_numpy()
    r["p_of"], r["p_cru"] = p_of[idx], p_cru_of[idx]
    loc = pd.to_datetime(r["t_emissao"], unit="s", utc=True).dt.tz_convert(fuso)
    r["per"] = loc.dt.to_period(freq).astype(str)
    out = []
    for per, g in r.groupby("per"):
        cru, of = g["p_cru"].mean() - g["y"].mean(), g["p_of"].mean() - g["y"].mean()
        out.append({"periodo": per, "n": len(g), "y_medio": g["y"].mean(), "citl_cru": cru, "citl_oficial": of,
                    "ece_oficial": C.ece(g["p_of"].to_numpy(), g["y"].to_numpy(float)),
                    "desvio_como_2013": abs(cru) >= 0.02, "corrigido_ok": abs(of) <= 0.02})
    return out


def avaliar_com_cru(nome, d, corte):
    """Diário com modelo_cru (fluxos com a correção): oficial e cru lado a lado."""
    p_cru = d["modelo_cru"].astype(float).to_numpy()
    p_cru = np.where(np.isfinite(p_cru), p_cru, d["modelo"].to_numpy(float))
    p_of = C.oficial_binario(d)
    p_cru_of = oficial(d, p_cru)
    mask = (d["y"].notna() & (d["t_emissao"] >= corte)).to_numpy()
    return {"cofre_oficial": metricas(d, mask, p_of), "cofre_cru": metricas(d, mask, p_cru_of),
            "trimestres": por_periodo(d, p_of, p_cru_of, FUSO[nome], "Q"),
            "meses": por_periodo(d, p_of, p_cru_of, FUSO[nome], "M"),
            "delta_min_max": [float(d["delta"].min()), float(d["delta"].max())]}


def main():
    out = {}
    # c3: validação da réplica no diário de 2023
    d23 = diario("voos23")
    cru = d23["modelo_cru"].astype(float).to_numpy()
    tem = np.isfinite(cru)
    p_rec, de = replica_delta(d23, np.where(tem, cru, np.nan), delta0=float(d23.loc[tem, "delta"].iloc[0]))
    dif_delta = np.abs(de[tem] - d23.loc[tem, "delta"].to_numpy())
    dif_p = np.abs(p_rec[tem] - d23.loc[tem, "modelo"].to_numpy())
    out["validacao_replica_2023"] = {"n": int(tem.sum()), "delta_dif_max": float(dif_delta.max()),
                                     "delta_dif_media": float(dif_delta.mean()), "p_dif_max": float(dif_p.max()),
                                     "p_dif_media": float(dif_p.mean())}
    # c3: retrospectiva de 2013
    d13 = diario("voos")
    r13 = resultados("voos")
    p13 = d13["modelo"].to_numpy(float)
    pr13, de13 = replica_delta(d13, p13, 0.0)
    mask = (d13["y"].notna() & (d13["t_emissao"] >= r13["corte"])).to_numpy()
    out["retro_2013"] = {"cofre_original": metricas(d13, mask, C.oficial_binario(d13)),
                         "cofre_com_correcao_fora_da_linha": metricas(d13, mask, oficial(d13, pr13)),
                         "delta_min_max": [float(np.nanmin(de13)), float(np.nanmax(de13))],
                         "meses": por_periodo(d13, oficial(d13, pr13), C.oficial_binario(d13), "America/New_York", "M")}
    # c1 e c2: 2023 original e replays novos
    out["voos23"] = avaliar_com_cru("voos23", d23, resultados("voos23")["corte"])
    for nome in ("voos17", "voospnw14", "voos23e1", "voos23e2"):
        p = REPLAYS / nome / "diario.parquet"
        if not p.exists():
            print("falta", nome)
            continue
        r = json.loads((REPLAYS / nome / "resultados.json").read_text())
        out[nome] = avaliar_com_cru(nome, pd.read_parquet(p), r["corte"])
        out[nome]["estresse"] = r.get("estresse")
        out[nome]["contagens"] = r["contagens"]
        out[nome]["deriva"] = [(e["sensor"], e["evento"], e["t"]) for e in r["eventos_operacao"] if e["tipo"] == "deriva"]
    (AQUI / "saida" / "correcao.json").write_text(json.dumps(out, indent=1, ensure_ascii=False, default=float))
    for k, v in out.items():
        if "cofre_oficial" in v:
            print(k, "oficial", v["cofre_oficial"], "\n   cru", v["cofre_cru"])
            for t in v["trimestres"]:
                print("   ", {kk: (round(vv, 4) if isinstance(vv, float) else vv) for kk, vv in t.items()})
        else:
            print(k, v if k != "retro_2013" else {kk: vv for kk, vv in v.items() if kk != "meses"})


if __name__ == "__main__":
    main()
