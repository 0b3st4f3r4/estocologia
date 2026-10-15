"""Funções comuns da reimplementação independente (ponto de fundo 2).

Escrito só a partir da especificação publicada (README, declarações, pacotes YAML). Não usa nem leu o código
dos horizontes.
"""
import os
import itertools
import json
import numpy as np
from scipy import stats
from scipy.spatial import cKDTree

DADOS = os.environ.get("ESTOCOLOGIA_DADOS", "/mnt/project-files/estocologia/dados-reais")
AQUI = os.path.dirname(os.path.abspath(__file__))
SAIDA = os.path.join(AQUI, "saida")
SEMENTE = 20261015


def gravar(nome, obj):
    os.makedirs(SAIDA, exist_ok=True)
    with open(os.path.join(SAIDA, nome), "w") as f:
        json.dump(obj, f, indent=2, ensure_ascii=False, sort_keys=True)


# ---------------------------------------------------------------- regressão logística (Newton, L2 opcional)

def logit_ajustar(X, y, lam=0.0, w=None, iters=100, tol=1e-10):
    """Minimiza mean(w * logloss) + lam/2 * ||beta[1:]||^2. X já tem a coluna de 1 na posição 0."""
    n, k = X.shape
    w = np.ones(n) if w is None else w
    sw = w.sum()
    beta = np.zeros(k)
    pen = np.full(k, lam)
    pen[0] = 0.0
    for _ in range(iters):
        eta = X @ beta
        p = 1.0 / (1.0 + np.exp(-eta))
        g = X.T @ (w * (p - y)) / sw + pen * beta
        H = (X * (w * p * (1 - p))[:, None]).T @ X / sw + np.diag(pen)
        H += 1e-12 * np.eye(k)
        step = np.linalg.solve(H, g)
        # busca linear simples
        f0 = _perda(X, y, w, beta, pen)
        t = 1.0
        while t > 1e-8:
            nb = beta - t * step
            if _perda(X, y, w, nb, pen) <= f0 + 1e-14:
                break
            t /= 2
        beta = nb
        if np.max(np.abs(t * step)) < tol:
            break
    return beta


def _perda(X, y, w, beta, pen):
    eta = X @ beta
    ll = np.logaddexp(0, eta) - y * eta
    return (w * ll).sum() / w.sum() + 0.5 * (pen * beta ** 2).sum()


def sigm(z):
    return 1.0 / (1.0 + np.exp(-z))


# ---------------------------------------------------------------- ICP: teste K = 2 e regra de saturação K = 3

def rcs(x, nos):
    """Spline cúbica restrita de Harrell: devolve as k-2 colunas não lineares."""
    t = np.asarray(nos, float)
    k = len(t)
    pos = lambda v: np.maximum(v, 0.0) ** 3
    cols = []
    for j in range(k - 2):
        c = (pos(x - t[j])
             - pos(x - t[k - 2]) * (t[k - 1] - t[j]) / (t[k - 1] - t[k - 2])
             + pos(x - t[k - 1]) * (t[k - 2] - t[j]) / (t[k - 1] - t[k - 2]))
        cols.append(c / (t[k - 1] - t[0]) ** 2)
    return np.column_stack(cols) if cols else np.empty((len(x), 0))


class Candidatos:
    """Guarda, para cada candidato, a expansão K = 2 e o bloco do funil (para a saturação)."""

    def __init__(self, df, tipos):
        # df: DataFrame do treino; tipos: dict nome -> 'num' | 'cat'
        self.nomes = list(tipos)
        self.exp = {}
        self.bloco = {}
        for nome, tipo in tipos.items():
            v = df[nome]
            if tipo == "cat":
                niveis = v.value_counts().index.tolist()  # mais frequente primeiro
                ind = np.column_stack([(v == lv).to_numpy(float) for lv in niveis[1:]]) if len(niveis) > 1 \
                    else np.empty((len(v), 0))
                self.exp[nome] = ind
                self.bloco[nome] = ind
            else:
                x = v.to_numpy(float)
                dp = x.std()
                z = (x - x.mean()) / dp if dp > 0 else x * 0
                self.bloco[nome] = z[:, None]
                distintos = np.unique(x)
                if len(distintos) <= 10:
                    freq = v.value_counts().index.tolist()
                    self.exp[nome] = np.column_stack([(x == lv).astype(float) for lv in freq[1:]]) \
                        if len(freq) > 1 else np.empty((len(x), 0))
                else:
                    nos = np.unique(np.quantile(z, [0.05, 0.35, 0.65, 0.95]))
                    if len(nos) < 3:
                        self.exp[nome] = z[:, None]
                    else:
                        self.exp[nome] = np.column_stack([z, rcs(z, nos)])

    def matriz(self, S):
        n = next(iter(self.exp.values())).shape[0]
        partes = [np.ones((n, 1))] + [self.exp[s] for s in S]
        return np.column_stack(partes)

    def bloco_sat(self, S):
        if not S:
            return None
        B = np.column_stack([self.bloco[s] for s in S])
        dp = B.std(axis=0)
        B = B[:, dp > 0] / dp[dp > 0]
        return B


def p_invariancia_np(y, X, amb):
    beta, *_ = np.linalg.lstsq(X, y, rcond=None)
    r = y - X @ beta
    a = np.abs(r)
    g, *_ = np.linalg.lstsq(X, a, rcond=None)
    s = np.maximum(X @ g, 0.1 * a.mean())
    u = r / s
    grupos = [u[amb == e] for e in np.unique(amb)]
    p_loc = stats.kruskal(*grupos).pvalue
    p_esc = stats.levene(*grupos, center="median").pvalue
    return float(min(1.0, 2 * min(p_loc, p_esc)))


def acerto_1nn(B, amb):
    n = len(amb)
    if B is None or B.shape[1] == 0:
        _, cont = np.unique(amb, return_counts=True)
        mapa = dict(zip(np.unique(amb), cont))
        return float(np.mean([(mapa[e] - 1) / (n - 1) for e in amb]))
    # células exatas
    _, inv, cont = np.unique(B, axis=0, return_inverse=True, return_counts=True)
    inv = inv.ravel()
    escore = np.empty(n)
    # contagem por célula × ambiente
    codamb = np.unique(amb, return_inverse=True)[1].ravel()
    namb = codamb.max() + 1
    tab = np.zeros((cont.size, namb))
    np.add.at(tab, (inv, codamb), 1)
    multi = cont[inv] >= 2
    escore[multi] = (tab[inv[multi], codamb[multi]] - 1) / (cont[inv[multi]] - 1)
    unicos = np.where(~multi)[0]
    if unicos.size:
        arv = cKDTree(B)
        _, idx = arv.query(B[unicos], k=2)
        viz = np.where(idx[:, 0] == unicos, idx[:, 1], idx[:, 0])
        escore[unicos] = (codamb[viz] == codamb[unicos]).astype(float)
    return float(escore.mean())


def icp(y, cand, amb, alpha=0.05, limiar_sat=0.99):
    linhas = []
    for k in range(len(cand.nomes) + 1):
        for S in itertools.combinations(cand.nomes, k):
            X = cand.matriz(S)
            p = p_invariancia_np(y, X, amb)
            sat = acerto_1nn(cand.bloco_sat(S), amb) >= limiar_sat
            linhas.append({"S": list(S), "p": p, "saturado": bool(sat)})
    aceitos = [l for l in linhas if l["p"] > alpha and not l["saturado"]]
    aceitos_sat = [l for l in linhas if l["p"] > alpha and l["saturado"]]
    if aceitos:
        S_hat = set(aceitos[0]["S"])
        for l in aceitos[1:]:
            S_hat &= set(l["S"])
    else:
        S_hat = set()
    nao_sat = [l for l in linhas if not l["saturado"]]
    melhor = max(nao_sat, key=lambda l: l["p"]) if nao_sat else None
    todos_aceitos = aceitos + aceitos_sat
    S_k2 = set(todos_aceitos[0]["S"]) if todos_aceitos else set()
    for l in todos_aceitos[1:]:
        S_k2 &= set(l["S"])
    return {
        "S_hat": sorted(S_hat),
        "n_conjuntos": len(linhas),
        "aceitos_nao_saturados": len(aceitos),
        "aceitos_saturados": len(aceitos_sat),
        "saturados_testados": sum(l["saturado"] for l in linhas),
        "maior_p_nao_saturado": melhor,
        "S_hat_sem_regra": sorted(S_k2),
        "p_por_conjunto": {"|".join(l["S"]): [l["p"], l["saturado"]] for l in linhas},
    }
