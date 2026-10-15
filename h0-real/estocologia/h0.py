"""O experimento do H0 em domínio real: a alegação de escala contra as chegadas reais.

Cada bloco usa o seu próprio fluxo de sorteios (SeedSequence.spawn), para que mudar um bloco
não perturbe os outros. Só o tempo de atendimento e a paciência são sorteados; a demanda do
mundo "real" é a observada.
"""

from __future__ import annotations

import numpy as np
from scipy import stats

from estocologia.central import (AHT, BIN, CORTE, META_SL, META_T, PACIENCIA, POR_GRUPO, carregar, conforme,
                                 dia, dispersao, erro_ingenuo, escala, escala_do_perfil, espera_media_erlang,
                                 grupos, media_por_chamada, nivel_servico)

DIAS_NULO = 400            # dias simulados no mundo da alegação para o modelo nulo
Z_MARGEM = 1.2816          # quantil 90% da normal: margem de sobredispersão da regra R2
SEMANA = ["seg", "ter", "qua", "qui", "sex"]


def agente_horas(c: np.ndarray, n_bins: int) -> float:
    dur = np.bincount(grupos(n_bins)) * BIN / 3600
    return float((c * dur).sum())


def rodar_mundo(contagens: np.ndarray, escalas, rng, aht=AHT, dist="exp", paciencia=PACIENCIA) -> dict:
    """Simula os dias dados (linhas de contagens), cada um com a sua escala. Resume na unidade dia."""
    es, ds, ab = [], [], []
    for i, x in enumerate(contagens):
        c = escalas[i] if isinstance(escalas, list) else escalas
        e, a = dia(x, c, rng, aht, dist, paciencia)
        es.append(e)
        ab.append(a)
        ds.append(np.full(e.size, i))
    e, d, a = np.concatenate(es), np.concatenate(ds), np.concatenate(ab)
    sl_dia = np.array([np.mean(x <= META_T) for x in es])
    m, ep = media_por_chamada(e, d)
    return {
        "chamadas": int(e.size), "dias": len(es),
        "espera_media_s": m, "ep_dia": ep, "ep_chamada": erro_ingenuo(e),
        "mediana_s": float(np.median(e)), "p90_s": float(np.quantile(e, 0.9)),
        "frac_acima_60s": float(np.mean(e > 60)),
        "nivel_servico": float(np.mean(e <= META_T)),
        "frac_dias_abaixo_meta": float(np.mean(sl_dia < META_SL)),
        "pior_dia_sl": float(sl_dia.min()),
        "desistencia": float(a.mean()),
        "_sl_dia": sl_dia, "_espera_dia": np.array([x.mean() for x in es]),
    }


def publico(r: dict) -> dict:
    return {k: v for k, v in r.items() if not k.startswith("_")}


def executar(semente: int = 2003, rapido: bool = False) -> dict:
    """Roda o H0 real inteiro. rapido=True usa 3 dias do cofre (só para o teste de determinismo)."""
    ss = np.random.SeedSequence(semente).spawn(8)
    rng = [np.random.default_rng(s) for s in ss]
    datas, C = carregar()
    nb = C.shape[1]
    tr = np.array([d < CORTE for d in datas])
    ds = np.array([d.weekday() for d in datas])
    Ct, Cv, dsv = C[tr], C[~tr], ds[~tr]
    if rapido:
        Cv, dsv = Cv[:3], dsv[:3]
    r: dict = {"semente": semente, "aht_s": AHT, "paciencia_s": PACIENCIA, "meta": f"{META_SL:.0%} em {META_T:.0f} s",
               "dias_treino": int(tr.sum()), "dias_cofre": int(Cv.shape[0])}

    # --- 1. A demanda real (F2): sobredispersão e dia da semana ---
    tot = C.sum(1)
    tot_t = Ct.sum(1)
    sem = np.array([tot_t[ds[tr] == w].mean() for w in range(5)])
    resid = tot_t / sem[ds[tr]]
    disp = dispersao(tot_t)
    n = tot_t.size
    r["demanda"] = {
        "chamadas_por_dia_media": float(tot.mean()), "dp_entre_dias": float(tot.std(ddof=1)),
        "dp_poisson_previsto": float(np.sqrt(tot.mean())),
        "dispersao_treino": disp,
        # sob Poisson com taxa fixa, (n−1)·D ~ χ²(n−1)
        "p_poisson": float(stats.chi2.sf((n - 1) * disp, n - 1)),
        "media_por_dia_da_semana": dict(zip(SEMANA, sem.round(0).tolist())),
        "cv_residual_apos_dia_da_semana": float(resid.std(ddof=1)),
        "pico_por_hora": float(Ct.mean(0).max() * 12), "vale_por_hora": float(Ct.mean(0)[:-1].min() * 12),
    }

    # --- 2. A alegação (L2): Erlang C sobre o perfil médio do treino ---
    perfil = Ct.mean(0)
    c0 = escala_do_perfil(perfil)
    g = grupos(nb)
    lam_g = np.bincount(g, weights=perfil) / (np.bincount(g) * BIN)
    peso = np.bincount(g, weights=perfil)
    sl_f = np.array([nivel_servico(int(c), l) for c, l in zip(c0, lam_g)])
    w_f = np.array([espera_media_erlang(int(c), l) for c, l in zip(c0, lam_g)])
    r["alegacao"] = {
        "texto": ("Escalando cada meia hora pelo Erlang C sobre a taxa média do histórico, a central atende "
                  f"{np.average(sl_f, weights=peso):.0%} das chamadas em até 20 s, com espera média de "
                  f"{np.average(w_f, weights=peso):.0f} s, todo dia."),
        "nivel_servico_formula": float(np.average(sl_f, weights=peso)),
        "espera_media_formula_s": float(np.average(w_f, weights=peso)),
        "agente_horas_dia": agente_horas(c0, nb), "escala_pico": int(c0.max()),
    }

    # --- 3. Jensen dentro da própria fórmula: nível de serviço do dia médio vs média dos dias ---
    lam_dias = np.array([np.bincount(g, weights=x) / (np.bincount(g) * BIN) for x in Cv])
    sl_dias = np.array([[nivel_servico(int(c), l) for c, l in zip(c0, ld)] for ld in lam_dias])
    peso_d = np.array([np.bincount(g, weights=x) for x in Cv])
    sl_dia_formula = (sl_dias * peso_d).sum(1) / peso_d.sum(1)
    r["jensen_formula"] = {
        "sl_no_dia_medio": float(np.average(sl_f, weights=peso)),
        "media_dos_dias": float(sl_dia_formula.mean()),
        "frac_dias_formula_abaixo_meta": float(np.mean(sl_dia_formula < META_SL)),
        "nota": "a mesma fórmula da alegação, aplicada à taxa de cada dia do cofre em vez da taxa média",
    }

    # --- 4. Os três mundos no cofre (jul–out), com a escala da alegação ---
    forma = perfil / perfil.sum()
    mundos = {
        "alegacao": rng[0].poisson(np.tile(perfil, (Cv.shape[0], 1))),
        "so_variacao_entre_dias": rng[0].poisson(np.outer(Cv.sum(1), forma)),
        "real": Cv,
    }
    r["mundos"] = {}
    for pac, rot in ((PACIENCIA, "com_desistencia"), (np.inf, "sem_desistencia")):
        r["mundos"][rot] = {k: publico(rodar_mundo(x, c0, rng[1], paciencia=pac)) for k, x in mundos.items()}
    base = rodar_mundo(Cv, c0, rng[2])

    # --- 5. Modelo nulo e teste severo (Φ3): dias do mundo da alegação ---
    nulo = rodar_mundo(rng[3].poisson(np.tile(perfil, (DIAS_NULO if not rapido else 6, 1))), c0, rng[3])
    sl_n, w_n = nulo["_sl_dia"], nulo["_espera_dia"]
    m = Cv.shape[0]
    boot = np.array([np.mean(rng[4].choice(sl_n, m) < META_SL) for _ in range(4000)])
    difs = []
    for _ in range(2000):
        a = rng[4].permutation(sl_n.size) < sl_n.size // 2
        difs.append(w_n[a].mean() - w_n[~a].mean())
    difs = np.array(difs)
    obs_frac = base["frac_dias_abaixo_meta"]
    obs_dif = float(base["_espera_dia"].mean() - w_n.mean())
    r["teste_severo"] = {
        "dias_nulo": int(sl_n.size),
        "frac_dias_abaixo_meta_nulo": float(np.mean(sl_n < META_SL)),
        "frac_dias_abaixo_meta_real": obs_frac,
        "p_frac": float(np.mean(boot >= obs_frac)),
        "nulo_frac_q99": float(np.quantile(boot, 0.99)),
        "diferenca_espera_dia_s": obs_dif,
        "nulo_dp_metades": float(difs.std(ddof=1)),
        "ep_dia_nulo_previsto": float(np.sqrt(2) * w_n.std(ddof=1) / np.sqrt(sl_n.size / 2)),
        "z": float(obs_dif / (w_n.std(ddof=1) * np.sqrt(1 / sl_n.size + 1 / m))),
    }
    # pseudorreplicação: o ep por chamada prevê um nulo muito mais estreito que o observado
    r["teste_severo"]["ep_chamada_nulo_previsto"] = float(np.sqrt(2) * nulo["ep_chamada"] * np.sqrt(2))

    # --- 6. Fora do tempo (F3): intervalo conforme do total diário, ajustado no treino ---
    meio = n // 2
    sem1 = np.array([tot_t[:meio][ds[tr][:meio] == w].mean() for w in range(5)])
    calib = tot_t[meio:] / sem1[ds[tr][meio:]]
    lo, hi = conforme(calib, 0.1)
    tot_v = Cv.sum(1)
    pv = sem1[dsv]
    r["fora_do_tempo"] = {
        "intervalo_90_razao": [lo, hi],
        "cobertura_cofre": float(np.mean((tot_v >= lo * pv) & (tot_v <= hi * pv))),
        "deriva_ks": float(stats.ks_2samp(resid, tot_v / sem[dsv]).statistic),
        "deriva_p": float(stats.ks_2samp(resid, tot_v / sem[dsv]).pvalue),
        "media_treino": float(tot_t.mean()), "media_cofre": float(tot_v.mean()),
    }

    # --- 7. A decisão (L3): quatro regras de escala no mundo real do cofre ---
    perfis_sem = np.array([Ct[ds[tr] == w].mean(0) for w in range(5)])
    cv = r["demanda"]["cv_residual_apos_dia_da_semana"]
    esc_sem = [escala_do_perfil(perfis_sem[w]) for w in range(5)]
    esc_marg = [escala_do_perfil(perfis_sem[w], fator=1 + Z_MARGEM * cv) for w in range(5)]
    ah_marg = np.mean([agente_horas(esc_marg[w], nb) for w in dsv])
    extra = 0
    while agente_horas(c0 + extra, nb) < ah_marg:
        extra += 1
    regras = {
        "R0_alegacao_perfil_medio": c0,
        "R1_perfil_do_dia_da_semana": [esc_sem[w] for w in dsv],
        "R2_dia_da_semana_mais_margem_p90": [esc_marg[w] for w in dsv],
        f"R3_perfil_medio_mais_{extra}_atendentes": c0 + extra,
    }
    r["regras"] = {}
    for k, esc in regras.items():
        res = publico(rodar_mundo(Cv, esc, rng[5]))
        ah = np.mean([agente_horas(e, nb) for e in esc]) if isinstance(esc, list) else agente_horas(esc, nb)
        res["agente_horas_dia"] = float(ah)
        r["regras"][k] = res

    # --- 8. Sensibilidade às suposições do F7 (atendimento e paciência) ---
    r["sensibilidade"] = {}
    for aht, dist, pac in ((180.0, "exp", PACIENCIA), (300.0, "exp", PACIENCIA), (AHT, "lognormal", PACIENCIA),
                           (AHT, "exp", 60.0), (AHT, "exp", 600.0)):
        c = escala_do_perfil(perfil, aht)
        rot = f"aht={aht:.0f}s {dist} paciencia={pac:.0f}s"
        r["sensibilidade"][rot] = {
            k: {kk: v for kk, v in publico(rodar_mundo(x, c, rng[6], aht, dist, pac)).items()
                if kk in ("nivel_servico", "frac_dias_abaixo_meta", "espera_media_s", "desistencia")}
            for k, x in (("alegacao", mundos["alegacao"]), ("real", Cv))
        }
    r["por_dia"] = {"sl_real_R0": base["_sl_dia"].round(4).tolist(), "sl_nulo": sl_n.round(4).tolist(),
                    "sl_formula_por_dia": sl_dia_formula.round(4).tolist(),
                    "datas_cofre": [str(d) for d in np.array(datas)[~tr][:Cv.shape[0]]], "totais_cofre": tot_v.tolist()}
    return r
