"""Predição Causal Invariante (Peters, Bühlmann & Meinshausen 2016) — a lente Morfonomia do H2.

Para cada subconjunto S de preditores candidatos, testa H0_S: "a lei de Y dado X_S é a mesma em
todos os ambientes". Os pais causais de Y satisfazem H0 sob intervenções que não tocam Y; logo,
com probabilidade ≥ 1 − α, a interseção dos S aceitos está contida nos pais (Ŝ ⊆ pa(Y)).

Teste aproximado (o "approximate" do pacote InvariantCausalPrediction):
- gaussiana: F do modelo agrupado contra um modelo por ambiente, mais Levene dos resíduos (Bonferroni);
- binomial: razão de verossimilhanças do agrupado contra coeficientes por ambiente.

Cada candidato pode ocupar várias colunas (uma variável categórica vira um bloco de indicadoras).

H2c (K = 2, ver ../h2b/DECLARACAO_K2.md): na família gaussiana o teste passa a ser `p_invariancia_np`, não
paramétrico nos resíduos de UM ajuste agrupado, no espírito de Heinze-Deml, Peters & Meinshausen (2018),
"Invariant causal prediction for nonlinear models" (citada de memória, conferir). O teste K = 1 (`p_invariancia`)
fica intacto, para comparação e para a binomial.

H2d (K = 3, ver ../DECLARACAO_K3.md): regra de saturação. Um S é saturado quando o ambiente é função
determinística das colunas de S (acerto médio do vizinho mais próximo, deixando a linha de fora, ≥ 0,99). Nesse
caso Y ⊥ E | X_S vale por construção e o teste tem poder zero. Saturados saem da família: não contam como
aceitos nem como rejeitados; Ŝ é a interseção dos aceitos NÃO saturados.
"""

from __future__ import annotations

from itertools import combinations

import numpy as np
from scipy import stats
from scipy.spatial import cKDTree

from estocologia.glm import ajustar


def _matriz(blocos: dict[str, np.ndarray], S: tuple, n: int) -> tuple[np.ndarray, list]:
    cols, nomes = [np.ones((n, 1))], [("intercepto", 0)]
    for v in S:
        b = blocos[v].reshape(n, -1)
        cols.append(b)
        nomes += [(v, j) for j in range(b.shape[1])]
    return np.hstack(cols), nomes


def p_invariancia(X: np.ndarray, y: np.ndarray, amb: np.ndarray, familia: str) -> float:
    """p-valor de H0: mesmos coeficientes (e, na gaussiana, mesma variância residual) em todos os ambientes."""
    rotulos = np.unique(amb)
    m, n = rotulos.size, X.shape[0]
    # H2b (correção): os graus de liberdade contam o POSTO da matriz, não o número de colunas. Com ambientes que
    # são intervenções em X, um candidato costuma ser constante dentro do ambiente (coparticipação do plano, turma
    # pequena, estado) ou um nível categórico some de um ambiente; no H2 essas colunas colineares contavam como
    # graus de liberdade sem reduzir a soma dos quadrados, o F ficava diluído e o teste aceitava demais.
    p = int(np.linalg.matrix_rank(X))
    agr = ajustar(X, y, familia)
    ll_sep, rss_sep, gl_sep = 0.0, 0.0, 0
    for a in rotulos:
        s = amb == a
        if s.sum() <= X.shape[1]:   # ambiente pequeno demais para um ajuste próprio: entra só pelo intercepto
            Xa = np.ones((s.sum(), 1))
        else:
            Xa = X[s]
        f = ajustar(Xa, y[s], familia)
        ll_sep += f["ll"]
        gl_sep += int(np.linalg.matrix_rank(Xa))
        if familia == "gaussiana":
            rss_sep += f["rss"]
    gl = gl_sep - p
    if gl <= 0:
        return 1.0
    if familia == "gaussiana":
        F = ((agr["rss"] - rss_sep) / gl) / (rss_sep / max(n - gl_sep, 1))
        p1 = float(stats.f.sf(F, gl, max(n - gl_sep, 1)))
        res = y - agr["mu"]
        p2 = float(stats.levene(*[res[amb == a] for a in rotulos], center="median").pvalue)
        return min(1.0, 2 * min(p1, p2))
    lr = 2 * (ll_sep - agr["ll"])
    return float(stats.chi2.sf(max(lr, 0.0), gl))


# ---------------- K = 2: teste não paramétrico de resíduos ----------------
LIMIAR_DISCRETO = 10                      # numérico com ≤ 10 valores distintos no treino vira fator
QUANTIS_NOS = (0.05, 0.35, 0.65, 0.95)     # spline cúbica restrita de Harrell, 4 nós
PISO_ESCALA = 0.1                          # ŝ ≥ 0,1 · média |r|
LIMIAR_SATURACAO = 0.99                    # diagnóstico: fração de linhas em células puras (um só ambiente)


def _spline_restrita(x: np.ndarray, nos: np.ndarray) -> np.ndarray:
    """Base de spline cúbica restrita (natural) de Harrell: k nós → k − 2 colunas não lineares."""
    k = nos.size
    t_k, t_k1 = nos[-1], nos[-2]
    esc = (t_k - nos[0]) ** 2
    cub = lambda u: np.clip(u, 0, None) ** 3
    cols = [(cub(x - nos[j]) - cub(x - t_k1) * (t_k - nos[j]) / (t_k - t_k1)
             + cub(x - t_k) * (t_k1 - nos[j]) / (t_k - t_k1)) / esc for j in range(k - 2)]
    return np.column_stack(cols)


def expandir(bloco: np.ndarray) -> tuple[np.ndarray, bool]:
    """Expansão K = 2 de um candidato. Devolve (colunas, é_discreto).

    - bloco com várias colunas (categórico já em indicadoras): mantido, discreto;
    - uma coluna com ≤ LIMIAR_DISCRETO valores distintos: fator (indicadoras, sem o valor mais frequente);
    - uma coluna contínua: linear + spline cúbica restrita com nós nos QUANTIS_NOS (nós repetidos removidos;
      com < 3 nós distintos, só linear)."""
    b = np.asarray(bloco, float)
    b = b.reshape(b.shape[0], -1)
    if b.shape[1] != 1:
        return b, True
    x = b[:, 0]
    vals, cont = np.unique(x, return_counts=True)
    if vals.size <= LIMIAR_DISCRETO:
        ref = vals[np.argmax(cont)]
        outros = [v for v in vals if v != ref]
        return (np.column_stack([(x == v).astype(float) for v in outros]) if outros else np.zeros((x.size, 0))), True
    nos = np.unique(np.quantile(x, QUANTIS_NOS))
    if nos.size < 3:
        return b, False
    return np.column_stack([x, _spline_restrita(x, nos)]), False


def _ols_ajuste(X: np.ndarray, y: np.ndarray) -> np.ndarray:
    beta, *_ = np.linalg.lstsq(X, y, rcond=None)
    return X @ beta


def p_invariancia_np(X: np.ndarray, y: np.ndarray, amb: np.ndarray, detalhe: bool = False):
    """Teste K = 2. X já expandido e com intercepto. Um ajuste agrupado de Y; escala agrupada de |r|;
    u = r / ŝ; Kruskal–Wallis (locação) e Brown–Forsythe (escala) de u entre ambientes; Bonferroni (2 testes)."""
    rotulos = np.unique(amb)
    r = y - _ols_ajuste(X, y)
    a = np.abs(r)
    s = np.maximum(_ols_ajuste(X, a), PISO_ESCALA * a.mean())
    u = r / s
    grupos = [u[amb == e] for e in rotulos]
    p_loc = float(stats.kruskal(*grupos).pvalue)
    p_esc = float(stats.levene(*grupos, center="median").pvalue)
    p = min(1.0, 2 * min(p_loc, p_esc))
    if detalhe:
        return p, {"p_locacao_kw": p_loc, "p_escala_bf": p_esc}
    return p


def _saturado(discretos: list[np.ndarray], amb: np.ndarray) -> bool:
    """Diagnóstico pré-declarado: a parte discreta de S determina o ambiente (≥ 99% das linhas em células puras)."""
    if not discretos:
        return False
    D = np.hstack(discretos)
    if D.shape[1] == 0:
        return False
    _, cel = np.unique(D, axis=0, return_inverse=True)
    cel = cel.ravel()
    _, ae = np.unique(amb, return_inverse=True)
    pares = np.unique(np.column_stack([cel, ae]), axis=0)
    n_amb_por_cel = np.bincount(pares[:, 0], minlength=cel.max() + 1)
    return bool(np.mean(n_amb_por_cel[cel] == 1) >= LIMIAR_SATURACAO)


def acerto_1nn(colunas: list[np.ndarray], amb: np.ndarray) -> float:
    """K = 3: acerto médio do vizinho mais próximo (LOO) ao prever o ambiente pelas colunas de S.

    Linhas com outras linhas idênticas em S (célula com m ≥ 2): escore = fração das m − 1 outras no mesmo
    ambiente (empates exatos). Linha única em S: 1 se a linha mais próxima (euclidiana, colunas com dp 1) está no
    mesmo ambiente. S = ∅ (ou só colunas constantes): uma célula com todas as linhas."""
    n = amb.size
    _, ae = np.unique(amb, return_inverse=True)
    ae = ae.ravel()
    if n < 2:
        return 1.0
    blocos = [np.asarray(c, float).reshape(n, -1) for c in colunas]
    D = np.hstack(blocos) if blocos else np.zeros((n, 0))
    sd = D.std(0) if D.shape[1] else np.zeros(0)
    D = D[:, sd > 0] / sd[sd > 0] if D.shape[1] else D
    if D.shape[1] == 0:
        cont = np.bincount(ae).astype(float)
        return float(np.sum(cont * (cont - 1)) / (n * (n - 1)))
    _, cel = np.unique(D, axis=0, return_inverse=True)
    cel = cel.ravel()
    m = np.bincount(cel)
    par = cel * (ae.max() + 1) + ae
    m_par = np.bincount(par, minlength=(cel.max() + 1) * (ae.max() + 1))
    esc = np.empty(n)
    rep = m[cel] >= 2
    esc[rep] = (m_par[par[rep]] - 1) / (m[cel[rep]] - 1)
    unic = np.flatnonzero(~rep)
    if unic.size:
        arv = cKDTree(D)
        _, idx = arv.query(D[unic], k=2, workers=-1)
        viz = np.where(idx[:, 0] == unic, idx[:, 1], idx[:, 0])
        esc[unic] = (ae[viz] == ae[unic]).astype(float)
    return float(esc.mean())


def saturado_k3(colunas: list[np.ndarray], amb: np.ndarray) -> bool:
    """Regra K = 3: S é saturado se o ambiente é (quase) função determinística de X_S (acerto 1-NN ≥ 0,99)."""
    return acerto_1nn(colunas, amb) >= LIMIAR_SATURACAO


def icp(blocos: dict[str, np.ndarray], y: np.ndarray, amb: np.ndarray, familia: str, alpha: float = 0.05,
        max_tamanho: int | None = None, teste: str = "residuos_np", regra_saturacao: bool = True) -> dict:
    """Roda o ICP sobre todos os subconjuntos (até max_tamanho). Devolve Ŝ, os aceitos e os intervalos.

    O intervalo de confiança de cada coeficiente é a união, sobre os S aceitos, do intervalo de nível
    1 − α no ajuste agrupado (0 quando a variável não está em S), como em Peters et al. (2016, §3.2).
    """
    nomes = sorted(blocos)
    n = y.size
    k = len(nomes) if max_tamanho is None else min(max_tamanho, len(nomes))
    z = stats.norm.ppf(1 - alpha / 2)
    usar_np = teste == "residuos_np" and familia == "gaussiana"
    if teste not in ("residuos_np", "linear_k1"):
        raise ValueError(teste)
    exp = {v: expandir(blocos[v]) for v in nomes} if usar_np else {}
    testados, aceitos, saturados, aceitos_k2 = [], [], [], []
    ic: dict[str, list] = {}
    for t in range(k + 1):
        for S in combinations(nomes, t):
            X, cols = _matriz(blocos, S, n)
            # K = 3: a saturação é medida nas colunas brutas de S (as que o funil passa), para qualquer teste
            acerto = acerto_1nn([blocos[v] for v in S], amb)
            sat = acerto >= LIMIAR_SATURACAO
            sat_k2 = _saturado([exp[v][0] for v in S if exp[v][1]], amb) if usar_np else False
            if usar_np:
                Xe = np.hstack([np.ones((n, 1))] + [exp[v][0] for v in S])
                pv, det = p_invariancia_np(Xe, y, amb, detalhe=True)
            else:
                pv, det = p_invariancia(X, y, amb, familia), {}
            testados.append({"S": list(S), "p": pv, **det, "acerto_1nn": acerto, "saturado": sat,
                             "saturado_diag_k2": sat_k2})
            if pv > alpha:
                aceitos_k2.append(list(S))
                if sat:
                    saturados.append(list(S))
                if sat and regra_saturacao:
                    continue                      # fora da família: não conta como aceito
                aceitos.append(list(S))
                f = ajustar(X, y, familia)
                se = np.sqrt(np.clip(np.diag(f["cov"]), 0, None))
                for v in nomes:
                    idx = [i for i, (nm, _) in enumerate(cols) if nm == v]
                    lo = min((f["beta"][i] - z * se[i] for i in idx), default=0.0)
                    hi = max((f["beta"][i] + z * se[i] for i in idx), default=0.0)
                    if not idx:
                        lo = hi = 0.0
                    cur = ic.setdefault(v, [np.inf, -np.inf])
                    ic[v] = [min(cur[0], lo), max(cur[1], hi)]
    S_hat = sorted(set.intersection(*map(set, aceitos))) if aceitos else []
    nao_sat = [S for S in aceitos_k2 if S not in saturados]
    familia_ef = [t for t in testados if not (regra_saturacao and t["saturado"])]
    melhor = max(familia_ef or testados, key=lambda d: d["p"])
    melhor_todos = max(testados, key=lambda d: d["p"])
    return {
        "teste": "K2: resíduos não paramétricos (KW + Brown–Forsythe, Bonferroni)" if usar_np else "K1: linear gaussiano (F + Levene) / RV",
        "melhor_conjunto": {"S": melhor["S"], "p": melhor["p"]},
        "regra_saturacao": {
            "ativa": regra_saturacao, "limiar_acerto_1nn": LIMIAR_SATURACAO,
            "saturados_testados": sum(t["saturado"] for t in testados),
            "aceitos_saturados": len(saturados), "aceitos_nao_saturados": len(nao_sat),
            "aceitos_sem_a_regra": len(aceitos_k2),
            "S_hat_sem_a_regra": sorted(set.intersection(*map(set, aceitos_k2))) if aceitos_k2 else [],
            "melhor_conjunto_incluindo_saturados": {"S": melhor_todos["S"], "p": melhor_todos["p"],
                                                    "saturado": melhor_todos["saturado"]},
            "saturados_diag_k2_entre_aceitos": sum(t["saturado_diag_k2"] for t in testados if t["p"] > alpha),
            "nota": "K = 3 (DECLARACAO_K3): saturados fora da família; Ŝ = interseção dos aceitos não saturados"},
        "alpha": alpha, "candidatos": nomes, "subconjuntos_testados": len(testados), "subconjuntos_na_familia": len(familia_ef),
        "aceitos": aceitos, "S_hat": S_hat,
        "modelo_rejeitado": not aceitos,   # nenhum S invariante: heterogeneidade, não linearidade ou pai oculto
        "situacao": (("nenhum conjunto não saturado aceito" + (" (só saturados aceitos)" if saturados else "")) if not aceitos else
                     "vazio aceito: os ambientes não mexem o bastante em Y para separar conjuntos (pouco poder)" if [] in aceitos
                     else ("Ŝ não vazio" if set.intersection(*map(set, aceitos)) else "Ŝ vazio por conjuntos aceitos disjuntos")),
        "ambientes": int(np.unique(amb).size),
        "intervalos": {v: [float(a), float(b)] for v, (a, b) in ic.items()},
        "testes": testados,
    }
