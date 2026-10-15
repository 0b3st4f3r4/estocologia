"""Ponto 4, H3: as quatro versões do estimador do laço performativo da central (DECLARACAO.md).

A v4 é o código de h3/estocologia/performativo.py, importado sem mudança. As v1–v3 não existem mais no código; são
reconstruídas aqui pela descrição do campo `historico` do próprio módulo:
- v1: a escala do previsor controlado sai da previsão da demanda corrigida; r̂ = Σ z_j·exc_{j+1} / (0,6·Σ z_j·a_j);
- v2: a escala serve a carga observada (como na v4); r̂ = Σ z_j·(exc_{j+1} + exc_{j+2}) / Σ z_j·a_j;
- v3: a razão da v2 com controle pré-choque: numerador Σ z_j·[(exc_{j+1} + exc_{j+2}) − (exc_{j−1} + exc_{j−2})],
  denominador Σ z_j·(a_j − a_{j−1});
- v4: 2SLS (código atual).
O r̂ sai sempre da simulação ingênua, como em `rodar`; o critério é o G4(b) do H3: |viés| ≤ 3% com r = 0,6.

Uso:  python3 laco_versoes.py <bank_calls.csv>     → resultados/laco_versoes.json
"""

import importlib.util
import json
import sys
from pathlib import Path

import numpy as np
import pandas as pd

AQUI = Path(__file__).resolve().parent
RAIZ = AQUI.parent.parent
spec = importlib.util.spec_from_file_location("performativo_h3", RAIZ / "h3" / "estocologia" / "performativo.py")
P = importlib.util.module_from_spec(spec)
spec.loader.exec_module(P)


def simular_v1(chegadas, dow, r, controlado, rng, explorar=0.10, r_hat=None, aquec=25):
    """Cópia de P.simular com uma só diferença: no controlado, a escala sai da previsão da demanda corrigida."""
    D, J = chegadas.shape
    obs, aband = np.zeros((D, J)), np.zeros((D, J))
    c_mat, expl = np.zeros((D, J), int), np.zeros((D, J))
    prev, corr = np.full((D, J), np.nan), np.zeros((D, J))
    for d in range(D):
        if d < aquec:
            f = chegadas[max(0, d - 5):d].mean(0) if d else chegadas[0]
        else:
            f = P.previsao(corr if controlado else obs, dow, d)        # v1: o controlado escala pela demanda corrigida
        prev[d] = P.previsao(corr, dow, d) if (controlado and d >= aquec) else f
        entra = np.zeros(J + 2)
        for j in range(J):
            c = P.escala(max(f[j], 0) / 300.0)
            if d >= aquec and rng.random() < explorar:
                m = 0.7 if rng.random() < 0.5 else 1.3
                c = max(1, int(round(c * m)))
                expl[d, j] = m - 1
            c_mat[d, j] = c
            n = chegadas[d, j] + entra[j]
            obs[d, j] = n
            pa = P.p_desistir(c, n / 300.0)
            ab = rng.binomial(int(n), pa)
            aband[d, j] = ab
            red = rng.binomial(ab, r) if r > 0 else 0
            k1 = rng.binomial(red, P.DIV[0])
            entra[j + 1] += k1
            entra[j + 2] += red - k1
        if controlado and r_hat is not None:
            lag1 = np.r_[0, aband[d, :-1]]
            lag2 = np.r_[0, 0, aband[d, :-2]]
            corr[d] = np.maximum(obs[d] - r_hat * (P.DIV[0] * lag1 + P.DIV[1] * lag2), 0)
        else:
            corr[d] = obs[d]
    return {"obs": obs, "aband": aband, "c": c_mat, "expl": expl, "prev": prev, "corr": corr}


def r_wald(sim, versao):
    ok = np.isfinite(sim["prev"]).all(1)
    exc = (sim["obs"] - sim["prev"])[ok]
    a = sim["aband"][ok]
    z = sim["expl"][ok]
    J = exc.shape[1]
    if versao == "v1":
        j = np.arange(0, J - 1)
        num = np.sum(z[:, j] * exc[:, j + 1])
        den = P.DIV[0] * np.sum(z[:, j] * a[:, j])
    elif versao == "v2":
        j = np.arange(0, J - 2)
        num = np.sum(z[:, j] * (exc[:, j + 1] + exc[:, j + 2]))
        den = np.sum(z[:, j] * a[:, j])
    elif versao == "v3":
        j = np.arange(2, J - 2)
        num = np.sum(z[:, j] * ((exc[:, j + 1] + exc[:, j + 2]) - (exc[:, j - 1] + exc[:, j - 2])))
        den = np.sum(z[:, j] * (a[:, j] - a[:, j - 1]))
    else:
        raise ValueError(versao)
    return float(num / den)


def carregar(caminho):
    d = pd.read_csv(caminho)
    t = pd.to_datetime(d["DateTime"], utc=True)
    dia = t.dt.normalize()
    tab = d.assign(dia=dia, j=((t - dia).dt.total_seconds() // 300 - 84).astype(int)).pivot(index="dia", columns="j", values="Calls")
    return tab.to_numpy(float), tab.index.dayofweek.to_numpy()


def main(caminho, semente=2027):
    chegadas, dow = carregar(caminho)
    aval = slice(60, None)
    r = 0.6
    rng = np.random.default_rng(semente)
    ing = P.simular(chegadas, dow, r, False, rng)          # a mesma simulação ingênua para as quatro versões
    det = P.detectar(ing)
    out = {"r_verdadeiro": r, "ingenuo": P.avaliar(chegadas, ing, aval), "deteccao_M11": det, "versoes": {}}
    for v in ("v1", "v2", "v3", "v4"):
        r_bruto = det["r_hat"] if v == "v4" else r_wald(ing, v)
        r_hat = max(0.0, min(1.0, r_bruto))
        rng = np.random.default_rng(semente + 1)
        sim = simular_v1 if v == "v1" else P.simular
        ctl = sim(chegadas, dow, r, True, rng, r_hat=r_hat)
        av = P.avaliar(chegadas, ctl, aval)
        out["versoes"][v] = {"r_hat": r_hat, "r_hat_bruto": r_bruto, "controlado": av,
                             "G4b_passa": abs(av["vies_relativo_vs_demanda_exogena"]) <= 0.03}
        print(v, f"r̂ = {r_hat:.3f}", f"viés = {av['vies_relativo_vs_demanda_exogena']:+.4f}", "G4(b):",
              out["versoes"][v]["G4b_passa"], flush=True)
    (AQUI / "resultados" / "laco_versoes.json").write_text(json.dumps(out, indent=1, ensure_ascii=False, default=float))


if __name__ == "__main__":
    main(sys.argv[1])
