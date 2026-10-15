"""Laço performativo da central (G4, lente M11/M15): a previsão escala, a escala muda o que se observa.

Nenhuma base real registra as ações do sistema. Por isso o laço é declarado: as chegadas reais do
bank_calls são as primeiras tentativas exógenas, e a resposta do mundo à ação segue as suposições
abaixo, que são do F7.
- A escala de cada intervalo é o Erlang C sobre a previsão (AHT = 240 s, meta de 80% em 20 s, como
  no H0).
- A desistência segue o M/M/c+M com paciência exponencial de média 180 s:
  - quando a carga A = λ·AHT é menor que c: P(desistir) = P_espera(c, A) · θ/(θ + (c − A)/AHT), com θ = 1/180;
  - quando A ≥ c: a fração fluida 1 − c/A, somada à desistência de quem espera.
- Quem desiste rediscam com probabilidade r: 60% no intervalo seguinte e 40% no outro.
- A central registra as chegadas observadas (primeiras tentativas + rediscagens) e as desistências.
  Ela não identifica o chamador, porque não há ANI na base.

A escala é a mesma nos dois: o Erlang C sobre a previsão da carga observada, que é o que a central
precisa atender. Os dois previsores de DEMANDA usam o mesmo método: a média do mesmo intervalo nas 4 últimas ocorrências do mesmo
dia da semana, aplicada a uma série.
- O previsor ingênuo usa a série observada.
- O previsor controlado usa a série corrigida, observada − r̂ · (desistências defasadas). O r̂ é
  estimado pela exploração.

A exploração sorteia 10% dos intervalos e multiplica a escala deles por 0,7 ou por 1,3. Como o
sorteio independe da demanda, a regressão das chegadas seguintes no déficit sorteado identifica o
efeito da ação, do(escala), no degrau 2 de Pearl.
"""

from __future__ import annotations

import math

import numpy as np
import pandas as pd
from scipy import stats

AHT, PAC, META_SL, META_T = 240.0, 180.0, 0.80, 20.0
DIV = (0.6, 0.4)


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


def escala(lam: float) -> int:
    a = lam * AHT
    c = max(1, int(math.floor(a)) + 1)
    while 1 - _pc(c, a) * math.exp(-(c - a) * META_T / AHT) < META_SL:
        c += 1
    return c


def p_desistir(c: int, lam: float) -> float:
    a = lam * AHT
    th = 1 / PAC
    if a < c:
        return _pc(c, a) * th / (th + (c - a) / AHT)
    return min(0.95, 1 - c / a + (c / a) * th / (th + 1 / AHT))


def previsao(serie: np.ndarray, dow: np.ndarray, d: int) -> np.ndarray | None:
    mesmos = [k for k in range(d) if dow[k] == dow[d]][-4:]
    if len(mesmos) < 2:
        return None
    return serie[mesmos].mean(0)


def simular(chegadas: np.ndarray, dow: np.ndarray, r: float, controlado: bool, rng, explorar: float = 0.10,
            r_hat: float | None = None, aquec: int = 25) -> dict:
    """chegadas: dias × intervalos de primeiras tentativas (contagens reais). Devolve as séries."""
    D, J = chegadas.shape
    obs = np.zeros((D, J))
    aband = np.zeros((D, J))
    c_mat = np.zeros((D, J), int)
    expl = np.zeros((D, J))
    prev = np.full((D, J), np.nan)
    corr = np.zeros((D, J))
    for d in range(D):
        if d < aquec:
            f = chegadas[max(0, d - 5):d].mean(0) if d else chegadas[0]
        else:
            # a escala sempre serve a carga observada (primeiras tentativas + rediscagens); o que muda
            # entre os previsores é a estimativa da DEMANDA exógena, que é o que se aprende do mundo
            f = previsao(obs, dow, d)
        prev[d] = previsao(corr, dow, d) if (controlado and d >= aquec) else f
        entra = np.zeros(J + 2)
        for j in range(J):
            c = escala(max(f[j], 0) / 300.0)
            if d >= aquec and rng.random() < explorar:
                m = 0.7 if rng.random() < 0.5 else 1.3
                c = max(1, int(round(c * m)))
                expl[d, j] = m - 1
            c_mat[d, j] = c
            n = chegadas[d, j] + entra[j]
            obs[d, j] = n
            pa = p_desistir(c, n / 300.0)
            ab = rng.binomial(int(n), pa)
            aband[d, j] = ab
            red = rng.binomial(ab, r) if r > 0 else 0
            k1 = rng.binomial(red, DIV[0])
            entra[j + 1] += k1
            entra[j + 2] += red - k1
        if controlado and r_hat is not None:
            lag1 = np.r_[0, aband[d, :-1]]
            lag2 = np.r_[0, 0, aband[d, :-2]]
            corr[d] = np.maximum(obs[d] - r_hat * (DIV[0] * lag1 + DIV[1] * lag2), 0)
        else:
            corr[d] = obs[d]
    return {"obs": obs, "aband": aband, "c": c_mat, "expl": expl, "prev": prev, "corr": corr}


def detectar(sim: dict) -> dict:
    """Sensor M11: Δ log(chegadas observadas em j+1 / previsão) contra o choque sorteado de escala em j.
    Também estima r̂ por variável instrumental: o choque sorteado move as desistências de j, e as
    chegadas de j+1 respondem a elas só pela rediscagem."""
    obs, prev, expl, ab = sim["obs"], sim["prev"], sim["expl"], sim["aband"]
    d_ok = np.isfinite(prev).all(1)
    y = np.log((obs[:, 1:] + 1) / (prev[:, 1:] + 1))[d_ok]
    z = expl[:, :-1][d_ok]
    a = ab[:, :-1][d_ok]
    n_next = obs[:, 1:][d_ok]
    s = z != 0
    yb, zb = y[s], z[s]
    X = np.column_stack([np.ones(zb.size), zb])
    b, res, *_ = np.linalg.lstsq(X, yb, rcond=None)
    e = yb - X @ b
    se = np.sqrt(e @ e / (zb.size - 2) * np.linalg.inv(X.T @ X)[1, 1])
    t = b[1] / se
    p = float(2 * stats.t.sf(abs(t), zb.size - 2))
    # r̂ por mínimos quadrados em dois estágios, na equação estrutural do intervalo:
    #   excesso_j = obs_j − previsão_j = (choque de demanda) + r · R_j,  R_j = 0,6·a_{j−1} + 0,4·a_{j−2}
    # R_j é endógeno (dia cheio → mais desistência e mais excesso); os choques sorteados de escala em
    # j−1 e j−2 são instrumentos válidos: mexem nas desistências e só por elas chegam a obs_j. As
    # rediscagens em cascata já estão dentro de a_{j−1}, a_{j−2}. Tudo centrado no dia (efeito fixo).
    exc = (obs - prev)[d_ok]
    ab_ = ab[d_ok]
    ex_ = expl[d_ok]
    R = DIV[0] * ab_[:, 1:-1] + DIV[1] * ab_[:, :-2]
    Y = exc[:, 2:]
    Z = np.stack([ex_[:, 1:-1], ex_[:, :-2]], -1)

    def centrar(m):
        return m - m.mean(1, keepdims=True)

    def tsls(Yd, Rd, Zd):
        y_, r_ = centrar(Yd).ravel(), centrar(Rd).ravel()
        z_ = np.column_stack([centrar(Zd[..., k]).ravel() for k in range(Zd.shape[-1])])
        g = np.linalg.lstsq(z_, r_, rcond=None)[0]
        rh = z_ @ g
        return float(rh @ y_ / (rh @ r_))
    r_hat = tsls(Y, R, Z)
    rng = np.random.default_rng(7)
    linhas = np.arange(Y.shape[0])
    boot = []
    for _ in range(300):
        b_ = rng.choice(linhas, linhas.size)              # reamostra dias (o dia é a unidade)
        boot.append(tsls(Y[b_], R[b_], Z[b_]))
    ic = [float(np.quantile(boot, 0.05)), float(np.quantile(boot, 0.95))]
    return {"coef_choque": float(b[1]), "t": float(t), "p": p, "n_choques": int(zb.size), "r_hat": max(0.0, min(1.0, r_hat)), "r_hat_ic90_bootstrap_por_dia": ic}


def avaliar(chegadas: np.ndarray, sim: dict, dias: slice) -> dict:
    prev, verdade = sim["prev"][dias], chegadas[dias]
    ok = np.isfinite(prev).all(1)
    pv, vv = prev[ok], verdade[ok]
    return {"vies_relativo_vs_demanda_exogena": float(pv.sum() / vv.sum() - 1),
            "vies_relativo_vs_observado": float(pv.sum() / sim["obs"][dias][ok].sum() - 1),
            "atendentes_hora_por_dia": float(sim["c"][dias][ok].sum() / ok.sum() / 12),
            "desistencias_por_dia": float(sim["aband"][dias][ok].sum() / ok.sum())}


def rodar(caminho_csv, semente: int = 2027) -> dict:
    d = pd.read_csv(caminho_csv)
    t = pd.to_datetime(d["DateTime"], utc=True)
    dia = t.dt.normalize()
    tab = d.assign(dia=dia, j=((t - dia).dt.total_seconds() // 300 - 84).astype(int)).pivot(index="dia", columns="j", values="Calls")
    chegadas = tab.to_numpy(float)
    dow = tab.index.dayofweek.to_numpy()
    aval = slice(60, None)
    out = {"suposicoes": {"AHT_s": AHT, "paciencia_media_s": PAC, "meta": "80% em 20 s", "divisao_rediscagem": DIV,
                          "exploracao": "10% dos intervalos, escala × 0,7 ou × 1,3", "dias": int(chegadas.shape[0]),
                          "avaliacao": "dias 60 em diante"}}
    out["historico"] = ("v1 (descartada): a escala do previsor controlado saía da demanda corrigida, sem as rediscagens; "
                        "a central passava a subescalar, as desistências cresciam em espiral e o r̂ de um intervalo só "
                        "(0,55) deixava viés de 6,6%. v2: a escala serve a carga observada; a demanda exógena é estimada "
                        "à parte; r̂ usa a massa dos dois intervalos seguintes (deu 0,70 e viés de −4,8%). v3: a mesma razão de Wald com controle pré-choque (diferença das diferenças), porque o choque de demanda do dia, comum aos vizinhos, dominava a variância (deu 0,77, IC 0,68–0,88: a razão contava a cascata de rediscagens como efeito direto). v4: 2SLS na equação estrutural do intervalo, com as desistências que alimentam o intervalo como regressor e os choques sorteados como instrumentos.")
    for r in (0.0, 0.6):
        rng = np.random.default_rng(semente)
        ing = simular(chegadas, dow, r, False, rng)
        det = detectar(ing)
        rng = np.random.default_rng(semente + 1)
        ctl = simular(chegadas, dow, r, True, rng, r_hat=det["r_hat"])
        out[f"r={r}"] = {"deteccao_M11": det, "ingenuo": avaliar(chegadas, ing, aval), "controlado": avaliar(chegadas, ctl, aval)}
    return out
