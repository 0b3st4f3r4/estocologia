"""Projeto B · Ver não é fazer, em dados reais (M13): o treinamento do NSW e os controles do CPS (LaLonde, 1986).

A alegação sob exame é a clássica que o LaLonde testou: uma comparação observacional entre os treinados
do NSW e controles do CPS, ingênua ou ajustada por covariáveis, recupera o efeito causal do treinamento.

Dados (todos reais; ver ../dados-reais/FONTES.md e SHA256SUMS):
    nsw_experimental.csv   amostra Dehejia–Wahba do NSW: 185 tratados e 260 controles SORTEADOS
    cps_observational.csv  15 992 controles observacionais do CPS

Desenho dos períodos (o framework exige 4; o último vai para o cofre do agent-f3-lifecycle):
    períodos 1–3  partições aleatórias (semente fixa PARTICAO_SEMENTE) da amostra observacional,
                  185 tratados do NSW + 15 992 controles do CPS. Não são coortes no tempo.
    período 4     os 260 controles sorteados do NSW: o padrão-ouro experimental. O cofre é FORA DA AMOSTRA
                  POR DESENHO, NÃO FORA DO TEMPO: são os mesmos anos (1974–78), outra fonte, outro mecanismo
                  de seleção. O F3 checa se o modelo observacional prevê o emprego em 1978 de quem foi
                  sorteado para não receber o treinamento, isto é, o contrafactual dos tratados.

Desfecho: binário, empregado em 1978 (re78 > 0). Justificativa em `cabecalho["nao_linearidade"]` e em
`lentes()["Escalonomia"]`: mantém a maquinaria RR/ΔR, a priori Beta elicitável, o diário de probabilidades
(Brier/Kelly) e a predição conforme; perde a escala de renda, que é relatada à parte (em US$) na conferência.

Ajuste: ponderação por escore de propensão para o efeito nos tratados (ATT), com a especificação de
Dehejia e Wahba (idade, idade², educação, educação², negro, hispânico, casado, sem diploma, re74, re75,
desempregado em 74 e 75, re74², re75²). As células do modelo são (braço × negro): a parte tratada é contagem
inteira; a parte controle é a média ponderada dos controles do CPS, convertida em pseudo-contagens pelo
tamanho efetivo de Kish (aproximação de verossimilhança ponderada, declarada em `critica`).

Linhas sintéticas: só nos lotes da banca (MB2: período 99; MB3: "deriva"; MB4: "patologico") e em períodos
fora de 1–4. São reamostragens ou perturbações declaradas de linhas reais e levam `sintetico = 1` e a
linha real de origem em `origem`.
"""

from __future__ import annotations

import csv
import functools
import hashlib
import itertools
from pathlib import Path

import numpy as np
from scipy import special, stats

from estocologia import estat
from estocologia.projeto import Projeto

DADOS = Path(__file__).resolve().parents[4] / "dados-reais"
ARQ_NSW, ARQ_CPS = "nsw_experimental.csv", "cps_observational.csv"
PARTICAO_SEMENTE = 1986        # semente fixa da partição aleatória da amostra observacional (ano do LaLonde)
N_TRATADOS = 185
W_NEGRO = 156 / 185            # composição racial dos tratados do NSW: a população-alvo do ATT (conferida em teste)
CUSTO = 0.05                   # F8: custo por participante, em pontos de emprego-equivalente (ILUSTRATIVO, não vem de dado)
LIMIAR_RISCO = 0.25            # F6: priorizar quem tem risco previsto de desemprego em 1978 > 25% (≈ o dobro da taxa do CPS)
FRAC_FORA_MAX = 0.20           # disjuntor: fração máxima do lote fora do suporte comum do escore de propensão
DESVIO_NEGRO_MAX = 0.25        # disjuntor: desvio máximo da fração de negros em relação à população-alvo
ATT_US_LITERATURA = 1794.0     # Dehejia e Wahba (1999): ATT experimental em renda de 1978, US$ (citado de memória; conferido nos dados)

# Índices dos 4 parâmetros amostrados pelo MCMC: logit P(Y = 1 | braço x, negro g)
CELULAS = ((0, 1), (0, 0), (1, 1), (1, 0))     # (x, negro)
I_BASAL = 0                                     # P(emprego 78 | sem treinamento, negro): o parâmetro elicitado
N_SBC = (76, 760, 156, 29)                      # tamanhos típicos das células (pseudo-contagens de Kish nos controles)

# Grafo assumido pela alegação ajustada (Dehejia–Wahba): D = demografia, R74/R75 = renda pré-tratamento.
GRAFO = {"D": [], "R74": ["D"], "R75": ["D", "R74"], "X": ["D", "R74", "R75"], "Y": ["X", "D", "R74", "R75"]}
# O placebo desloca o relógio um período (Imbens, 2015): pseudo-tratamento decidido com (D, R74), pseudo-desfecho R75.
GRAFO_PLACEBO = {"D": [], "R74": ["D"], "X": ["D", "R74"], "R75": ["D", "R74"]}
GRAFO_PLACEBO_U = {"D": [], "U": [], "R74": ["D"], "X": ["D", "R74", "U"], "R75": ["D", "R74", "U"]}   # U: choque transitório não medido


# =====================================================================================
# Carga dos dados reais (com conferência das somas SHA-256)
# =====================================================================================
def _sha_esperado(nome):
    for ln in (DADOS / "SHA256SUMS").read_text().splitlines():
        partes = ln.split()
        if len(partes) == 2 and partes[1] == nome:
            return partes[0]
    return None


@functools.lru_cache(maxsize=1)
def carregar():
    """Lê os dois CSV reais uma vez. Devolve {"nsw": cols, "cps": cols, "sha": {...}}; cols é {coluna: np.ndarray}."""
    out, sha = {}, {}
    for chave, arq in (("nsw", ARQ_NSW), ("cps", ARQ_CPS)):
        corpo = (DADOS / arq).read_bytes()
        h = hashlib.sha256(corpo).hexdigest()
        sha[arq] = {"sha256": h, "confere_SHA256SUMS": h == _sha_esperado(arq)}
        linhas = list(csv.DictReader(corpo.decode().splitlines()))
        cols = {"rownames": np.array([int(r["rownames"]) for r in linhas])}
        for c in ("treat", "age", "educ", "black", "hisp", "marr", "nodegree"):
            cols[c] = np.array([int(float(r[c])) for r in linhas])
        for c in ("re74", "re75", "re78"):
            cols[c] = np.array([float(r[c]) for r in linhas])
        out[chave] = cols
    out["sha"] = sha
    return out


def _linha_real(fonte_cols, i, fonte, prefixo):
    c = fonte_cols
    negro, hisp = int(c["black"][i]), int(c["hisp"][i])
    rid = f"{prefixo}-{int(c['rownames'][i]):05d}"
    return {"pessoa": rid, "periodo": 0, "fonte": fonte, "sintetico": 0, "origem": rid, "treat": int(c["treat"][i]),
            "age": int(c["age"][i]), "educ": int(c["educ"][i]), "black": negro, "hisp": hisp,
            "raca": "negro" if negro else ("hispanico" if hisp else "outro"), "marr": int(c["marr"][i]),
            "nodegree": int(c["nodegree"][i]), "re74": float(c["re74"][i]), "re75": float(c["re75"][i]),
            "re78": float(c["re78"][i]), "y": int(c["re78"][i] > 0)}


@functools.lru_cache(maxsize=1)
def linhas_reais():
    """As linhas reais: tratados e controles sorteados do NSW e controles do CPS, com id estável (rownames)."""
    d = carregar()
    nsw, cps = d["nsw"], d["cps"]
    trat = tuple(_linha_real(nsw, i, "NSW-tratado", "NSW") for i in np.flatnonzero(nsw["treat"] == 1))
    ctrl = tuple(_linha_real(nsw, i, "NSW-controle", "NSW") for i in np.flatnonzero(nsw["treat"] == 0))
    cpsl = tuple(_linha_real(cps, i, "CPS", "CPS") for i in range(cps["treat"].size))
    return {"tratados": trat, "controles_nsw": ctrl, "cps": cpsl, "observacional": trat + cpsl}


@functools.lru_cache(maxsize=1)
def particao():
    """Partição aleatória e fixa da amostra observacional em 3 períodos (tratados estratificados: ~62 por período)."""
    obs = linhas_reais()["observacional"]
    rr = np.random.default_rng(PARTICAO_SEMENTE)
    per = np.empty(len(obs), int)
    for m in (np.array([r["treat"] == 1 for r in obs]), np.array([r["treat"] == 0 for r in obs])):
        idx = np.flatnonzero(m)
        rr.shuffle(idx)
        per[idx] = 1 + np.arange(idx.size) % 3
    return per


# =====================================================================================
# Estimadores (funções puras, testadas em tests/test_b.py)
# =====================================================================================
def desenho(d, placebo=False):
    """Matriz de covariáveis de Dehejia–Wahba (rendas em milhares de US$). placebo=True tira re75 e u75,
    que viram pseudo-desfecho."""
    a, e = np.asarray(d["age"], float), np.asarray(d["educ"], float)
    r74, r75 = np.asarray(d["re74"], float) / 1000, np.asarray(d["re75"], float) / 1000
    cols = [np.ones_like(a), (a - 30) / 10, ((a - 30) / 10) ** 2, (e - 11) / 3, ((e - 11) / 3) ** 2,
            np.asarray(d["black"], float), np.asarray(d["hisp"], float), np.asarray(d["marr"], float),
            np.asarray(d["nodegree"], float), r74 / 10, (r74 == 0) * 1.0, (r74 / 10) ** 2]
    if not placebo:
        cols += [r75 / 10, (r75 == 0) * 1.0, (r75 / 10) ** 2]
    return np.column_stack(cols)


def ajustar_logit(X, y, l2=1e-3, iters=80):
    """Regressão logística por Newton–Raphson (IRLS) com crista l2 fora do intercepto. Devolve (β, hessiana)."""
    X, y = np.asarray(X, float), np.asarray(y, float)
    b = np.zeros(X.shape[1])
    P = l2 * np.eye(X.shape[1])
    P[0, 0] = 0.0
    H = np.eye(X.shape[1])
    for _ in range(iters):
        p = special.expit(X @ b)
        H = X.T @ (X * (p * (1 - p))[:, None]) + P + 1e-12 * np.eye(X.shape[1])
        passo = np.linalg.solve(H, X.T @ (y - p) - P @ b)
        if not np.all(np.isfinite(passo)):
            break
        b = b + passo
        if np.max(np.abs(passo)) < 1e-9:
            break
    p = special.expit(X @ b)
    H = X.T @ (X * (p * (1 - p))[:, None]) + P + 1e-12 * np.eye(X.shape[1])
    return b, H


def pesos_att(e):
    """Pesos de ATT para os controles: chances do escore de propensão, e / (1 − e)."""
    e = np.clip(np.asarray(e, float), 1e-12, 1 - 1e-12)
    return e / (1 - e)


def ess_kish(w):
    """Tamanho efetivo de Kish, (Σw)² / Σw²: entre 1 e n, igual a n com pesos iguais."""
    w = np.asarray(w, float)
    return float(w.sum() ** 2 / np.sum(w ** 2)) if w.size and np.sum(w ** 2) > 0 else 0.0


def att_ipw(y, t, e):
    """ATT por ponderação: média dos tratados menos a média dos controles pesada por e/(1−e).
    Devolve (att, p1, p0_ponderado, ess_controles)."""
    y, t = np.asarray(y, float), np.asarray(t, int)
    w = pesos_att(e)
    m1, m0 = t == 1, t == 0
    p1 = float(y[m1].mean())
    p0 = float(np.sum(w[m0] * y[m0]) / np.sum(w[m0]))
    return p1 - p0, p1, p0, ess_kish(w[m0])


def dif_proporcoes(y1, y0):
    """Diferença de médias com IC de Wald a 95% (desfecho binário ou contínuo; variâncias separadas)."""
    y1, y0 = np.asarray(y1, float), np.asarray(y0, float)
    d = float(y1.mean() - y0.mean())
    se = float(np.sqrt(y1.var(ddof=1) / y1.size + y0.var(ddof=1) / y0.size))
    return {"estimativa": d, "IC95": [d - 1.96 * se, d + 1.96 * se], "ep": se, "n1": int(y1.size), "n0": int(y0.size)}


def contagens(y, t, negro, e):
    """k[i], n[i] das 4 células de CELULAS. Tratados: contagens inteiras. Controles: média ponderada por
    e/(1−e) dentro do grupo, com n = tamanho efetivo de Kish (pseudo-contagens)."""
    y, t, negro = np.asarray(y, float), np.asarray(t, int), np.asarray(negro, int)
    w = pesos_att(e)
    k, n = np.zeros(4), np.zeros(4)
    for i, (xx, gg) in enumerate(CELULAS):
        m = (t == xx) & (negro == gg)
        if not m.any():
            continue
        if xx == 1:
            k[i], n[i] = y[m].sum(), m.sum()
        else:
            n[i] = ess_kish(w[m])
            k[i] = n[i] * float(np.sum(w[m] * y[m]) / np.sum(w[m]))
    return k, n


def padronizar(p_celulas, w_negro=W_NEGRO):
    """P(Y | do(x)) nos tratados, padronizado pela composição racial-alvo: Σ_g P(Y | x, g) P(g | tratado)."""
    p0n, p0o, p1n, p1o = p_celulas
    return w_negro * p0n + (1 - w_negro) * p0o, w_negro * p1n + (1 - w_negro) * p1o


def estatistica_placebo(w_out, x, e):
    """Placebo: 'efeito' do treinamento sobre um desfecho pré-tratamento, pela mesma ponderação de ATT.
    Aceita x vetor (n,) ou matriz de réplicas (R, n), com e (n,) ou (R, n)."""
    w_out = np.asarray(w_out, float)
    x = np.asarray(x, float)
    w = pesos_att(e)
    n1 = x.sum(-1)
    m1 = (x * w_out).sum(-1) / np.maximum(n1, 1)
    w0 = (1 - x) * w
    m0 = (w0 * w_out).sum(-1) / np.maximum(w0.sum(-1), 1e-300)
    return m1 - m0


def kleinberg_fpr(p, vpp, fnr):
    """Identidade de Chouldechova (2017): FPR = p/(1−p) · (1−VPP)/VPP · (1−FNR), para qualquer classificador."""
    return p / (1 - p) * (1 - vpp) / vpp * (1 - fnr)


def simular_teste(n, rng, efeito=0.8, confund=1.5, cenario="mundo"):
    """Mundo SINTÉTICO só para testes e SBC: D ~ N(0, 1), X ~ B(expit(−2 + confund·D)), Y ~ B(expit(0,5 + efeito·X − D)).
    No cenário 'patologico' um U não medido entra em X e em Y. Devolve também o ATT verdadeiro."""
    d = rng.standard_normal(n)
    u = rng.standard_normal(n) if cenario == "patologico" else np.zeros(n)
    x = (rng.random(n) < special.expit(-2 + confund * d + 1.5 * u)).astype(int)
    eta = 0.5 - d - 1.5 * u
    y = (rng.random(n) < special.expit(eta + efeito * x)).astype(int)
    att = float(np.mean(special.expit(eta[x == 1] + efeito) - special.expit(eta[x == 1])))
    return {"d": d, "x": x, "y": y, "u": u, "att": att}


# ---------- grafos (reimplementação das ferramentas do Lab 13) ----------
def ancestrais(pais, nos):
    vistos, pilha = set(), list(nos)
    while pilha:
        a = pilha.pop()
        if a not in vistos:
            vistos.add(a)
            pilha += pais.get(a, [])
    return vistos


def descendentes(pais, no):
    filhos = {a: [b for b in pais if a in pais[b]] for a in pais}
    vistos, pilha = set(), list(filhos[no])
    while pilha:
        a = pilha.pop()
        if a not in vistos:
            vistos.add(a)
            pilha += filhos[a]
    return vistos


def d_separados(pais, X, Y, Z):
    """Critério do grafo moral ancestral (Lauritzen et al., 1990)."""
    X, Y, Z = set(X), set(Y), set(Z)
    A = ancestrais(pais, X | Y | Z)
    viz = {a: set() for a in A}
    for b in A:
        ps = [p for p in pais.get(b, []) if p in A]
        for p in ps:
            viz[p].add(b)
            viz[b].add(p)
        for p, q in itertools.combinations(ps, 2):
            viz[p].add(q)
            viz[q].add(p)
    vistos, pilha = set(), [a for a in X if a not in Z]
    while pilha:
        a = pilha.pop()
        if a in Y:
            return False
        if a not in vistos:
            vistos.add(a)
            pilha += [b for b in viz[a] if b not in Z and b not in vistos]
    return True


def porta_dos_fundos(pais, x, y, Z):
    """Pearl (1993): nenhum nó de Z descende de x, e Z d-separa x de y no grafo sem as setas que saem de x."""
    if set(Z) & descendentes(pais, x):
        return False
    sem_saida = {no: [p for p in ps if p != x] for no, ps in pais.items()}
    return d_separados(sem_saida, [x], [y], Z)


def _faixa_idade(a):
    return "16-24" if a < 25 else ("25-34" if a < 35 else "35-55")


def _faixa_educ(e):
    return "0-11" if e <= 11 else ("12" if e == 12 else "13+")


# =====================================================================================
class ProjetoB(Projeto):
    nome = "b"
    dados_simulados = False
    origem_dos_dados = "dados-reais/nsw_experimental.csv + cps_observational.csv (Rdatasets causaldata)"
    titulo = "Ver não é fazer, em dados reais: o treinamento do NSW contra os controles do CPS"
    alegacao = ('"Comparar os treinados do NSW com trabalhadores do CPS, crua ou ajustando por covariáveis, '
                'recupera o efeito causal do treinamento sobre o emprego."')
    unidade = "pessoa"
    colunas_modelo = ("pessoa", "periodo", "treat", "age", "educ", "black", "hisp", "raca", "marr", "nodegree",
                      "re74", "re75", "re78", "y")
    quase_ids = ("age", "educ", "raca", "marr")
    dim = 4
    custo_inferencia = 1e-6

    cabecalho = {
        "processo_gerador": "Mundo (L1): homens elegíveis ao NSW (1976–77) sorteados para treinamento ou não; trabalhadores do CPS "
                            "que nunca foram elegíveis. A alegação (L2) troca o contrafactual 'tratado sem treinamento' pelo CPS, "
                            "cru ou reponderado por covariáveis, e conclui (L3) um efeito causal. O braço sorteado do NSW é o juiz.",
        "nao_linearidade": "Desfecho binário (empregado em 1978, re78 > 0), por escolha: a renda é inflada de zeros (35% no NSW) e "
                           "assimétrica, a média em US$ é dominada por poucos; o binário mantém RR/ΔR, a priori Beta elicitável e o "
                           "diário de probabilidades. A renda em US$ é relatada à parte. Riscos aditivos por célula; o ajuste "
                           "por pesos e/(1−e) é fortemente não linear no escore de propensão.",
        "unidade_e_margem": "Unidade independente: a pessoa. Períodos 1–3 são partições ALEATÓRIAS da amostra observacional "
                            "(185 tratados + 15 992 CPS), não coortes. O período 4 (cofre) são os 260 controles sorteados do NSW: "
                            "fora da amostra por desenho, NÃO fora do tempo (mesmos anos).",
        "fisica_do_ruido": "Bernoulli por pessoa, limitada; sem caudas no desfecho. O que domina a incerteza não é o acaso amostral, e "
                           "sim (i) o tamanho efetivo pequeno dos controles reponderados (~100 de 15 992) e (ii) a premissa não "
                           "testável de seleção só em observáveis, que o placebo com re75 questiona.",
        "escada_causal": "A alegação quer o degrau 2 (efeito de fazer o treinamento) a partir de dados do degrau 1 (CPS). O "
                         "ajuste identifica o degrau 2 só sob seleção em observáveis; o cofre experimental confere o degrau 2 "
                         "fora da amostra.",
    }

    # ---------------- F7 ----------------
    def traducao(self):
        sha = carregar()["sha"]
        return {
            "dominio": "economia do trabalho, avaliação de programas de treinamento",
            "pergunta_substantiva": "Se um homem elegível ao NSW receber o treinamento, a chance de ele estar empregado em 1978 sobe? "
                                    "E uma comparação com o CPS teria dado a mesma resposta que o sorteio?",
            "unidade_independente": "a pessoa (homem elegível ao NSW, ou trabalhador amostrado pelo CPS)",
            "glossario": {"ATT": "efeito médio do treinamento nos que foram treinados",
                          "escore de propensão": "P(treinado | covariáveis); resume as covariáveis para o ajuste",
                          "ponderação ATT": "controles pesados por e/(1−e) para imitar a distribuição dos tratados",
                          "tamanho efetivo de Kish": "(Σw)²/Σw²: quantas observações iguais os pesos valem",
                          "suporte comum": "região de covariáveis em que há tratados e controles",
                          "queda de Ashenfelter": "a renda dos futuros treinados cai logo antes do programa",
                          "placebo": "desfecho pré-tratamento (re75): o treinamento não pode causá-lo",
                          "controle sorteado": "elegível ao NSW que o sorteio deixou sem treinamento: o contrafactual experimental",
                          "seleção em observáveis": "premissa de que, dadas as covariáveis, ser treinado não depende do desfecho potencial",
                          "fora da amostra por desenho": "avaliação em outra fonte dos mesmos anos, não no futuro"},
            "fontes_primarias": ["LaLonde, R. (1986). Evaluating the econometric evaluations of training programs. AER 76(4), 604–620.",
                                 "Dehejia, R. e Wahba, S. (1999). Causal effects in nonexperimental studies. JASA 94(448), 1053–1062.",
                                 "Smith, J. e Todd, P. (2005). Does matching overcome LaLonde's critique? J. Econometrics 125, 305–353.",
                                 "Imbens, G. (2015). Matching methods in practice: three examples. J. Human Resources 50(2), 373–419.",
                                 "Ashenfelter, O. (1978). Estimating the effect of training programs on earnings. REStat 60(1), 47–57.",
                                 "(referências citadas de memória; conferir)"],
            "dados": {"origem": "Rdatasets causaldata::nsw_mixtape e causaldata::cps_mixtape (ver dados-reais/FONTES.md)", "sha256": sha},
        }

    def elicitacao(self):
        return {
            "quantidade": "P(empregado em 1978 | sem treinamento) para um homem negro elegível ao NSW",
            "especialistas": [{"nome": "E1 (simulado: economista do trabalho)", "q25": 0.45, "q50": 0.55, "q75": 0.65},
                              {"nome": "E2 (simulado: assistente social de programa de reinserção)", "q25": 0.35, "q50": 0.45, "q75": 0.55},
                              {"nome": "E3 (simulado: gestor de agência pública de emprego)", "q25": 0.50, "q50": 0.60, "q75": 0.70}],
            "protocolo": "SHELF v4: quartis individuais sem ancoragem, discussão, consenso RIO, ajuste Beta, validação facial",
            "entra_no_parametro": "logit P(Y | sem treinamento, negro); os outros três riscos recebem Beta(1, 1)",
            "nota": "ELICITAÇÃO SIMULADA: nenhum especialista humano foi consultado, e o painel foi escrito por quem já conhece a "
                    "literatura do LaLonde; não é independente dos dados. A priori resultante é fraca (≈ 10 pseudo-observações).",
        }

    def priori_atipica(self):
        return {"quantidade": "emprego sem treinamento, na leitura de quem defende a expansão do programa", "especialistas": [
            {"nome": "E4 (simulado: dirigente do programa)", "q25": 0.20, "q50": 0.30, "q75": 0.40}],
            "nota": "ELICITAÇÃO SIMULADA: o dirigente subestima o emprego de quem não é treinado, o que infla o efeito."}

    def termo_escopo(self):
        return {
            "vale_para": "homens elegíveis ao NSW em 1976–77 (desempregados de longa duração, ex-detentos, ex-usuários de drogas, "
                         "evadidos escolares) de 10 cidades dos EUA, composição racial dos tratados (84% negros), desfecho "
                         "emprego no ano civil de 1978, comparação com o CPS de 1976 sob seleção em observáveis",
            "nao_vale_para": "renda em US$ como desfecho principal, outros programas ou décadas, mulheres (AFDC), a população geral "
                             "do CPS (fora do suporte comum: 72% dos controles do CPS têm escore abaixo do menor tratado)",
            "regra_de_decisao": "financiar o treinamento se o aumento causal de emprego superar C/L; a evidência observacional só "
                                "sustenta a decisão se o placebo e o cofre experimental não a desmentirem",
            "circuit_breaker": f"parar de usar a estimativa se, num lote, mais de {FRAC_FORA_MAX:.0%} das pessoas estiverem fora do suporte "
                               f"comum do escore de propensão, se a fração de negros se afastar mais de {DESVIO_NEGRO_MAX} da população-alvo "
                               f"({W_NEGRO:.2f}), ou se o placebo (re75) num lote com tratados der |z| > 4",
            "cofre": "o período 4 são os controles sorteados do NSW: fora da amostra por desenho, não fora do tempo",
            "assinatura": "sil (pendente)",
        }

    # ---------------- F2 ----------------
    def contrato(self):
        return {
            "nome": "nsw_cps_lalonde", "versao": "1.0",
            "colunas": {
                "pessoa": {"tipo": "str"},
                "periodo": {"tipo": "int", "min": 1, "max": 99},
                "fonte": {"tipo": "str", "valores": ["NSW-tratado", "NSW-controle", "CPS"]},
                "sintetico": {"tipo": "int", "valores": [0, 1]},
                "origem": {"tipo": "str"},
                "treat": {"tipo": "int", "valores": [0, 1]},
                "age": {"tipo": "int", "min": 16, "max": 55},
                "educ": {"tipo": "int", "min": 0, "max": 18},
                "black": {"tipo": "int", "valores": [0, 1]},
                "hisp": {"tipo": "int", "valores": [0, 1]},
                "raca": {"tipo": "str", "valores": ["negro", "hispanico", "outro"]},
                "marr": {"tipo": "int", "valores": [0, 1]},
                "nodegree": {"tipo": "int", "valores": [0, 1]},
                "re74": {"tipo": "float", "min": 0.0, "max": 100_000.0},
                "re75": {"tipo": "float", "min": 0.0, "max": 100_000.0},
                "re78": {"tipo": "float", "min": 0.0, "max": 100_000.0},
                "y": {"tipo": "int", "valores": [0, 1]},
            },
            "chave": ["pessoa"], "particao": "periodo",
            "grao": "uma linha por pessoa; rendas anuais em US$ correntes; y = 1 se re78 > 0",
            "proveniencia": "linhas reais têm sintetico = 0 e origem = a própria pessoa; reamostragens/perturbações têm sintetico = 1",
        }

    def gerar_lote(self, periodo, rng, cenario="mundo", n=None):
        """Períodos 1–3: partição real da amostra observacional; período 4: controles sorteados do NSW (reais).
        Outros períodos e os cenários da banca são SINTÉTICOS, construídos a partir de linhas reais (rng):
          mundo, período fora de 1–4 (MB2 usa 99): reamostragem com reposição da amostra observacional;
          deriva (MB3/MB8): reamostragem só do CPS, 'o programa aberto à população geral';
          patologico (MB4): reamostragem da amostra observacional com os tratados forçados a re75 = 0,
            uma seleção pelo choque de 1975 que nenhuma covariável explica (viola o placebo)."""
        R = linhas_reais()
        if cenario == "mundo" and periodo in (1, 2, 3):
            per = particao()
            base = [r for r, p in zip(R["observacional"], per) if p == periodo]
            base = base if n is None else base[:n]
            return [{**r, "periodo": int(periodo)} for r in base]
        if cenario == "mundo" and periodo == self.periodos[-1]:
            base = list(R["controles_nsw"]) if n is None else list(R["controles_nsw"])[:n]
            return [{**r, "periodo": int(periodo)} for r in base]
        if cenario == "deriva":
            fonte, n = R["cps"], (2000 if n is None else n)
        elif cenario == "patologico":
            fonte, n = R["observacional"], (len(R["observacional"]) if n is None else n)
        else:
            fonte, n = R["observacional"], (2000 if n is None else n)
        idx = rng.integers(0, len(fonte), n)
        if cenario == "patologico":   # garante tratados no lote mesmo quando ele é pequeno
            trat = np.array([fonte[i]["treat"] == 1 for i in range(len(fonte))])
            idx[: min(n, 40)] = rng.choice(np.flatnonzero(trat), min(n, 40))
        out = []
        for j, i in enumerate(idx.tolist()):
            r = dict(fonte[i])
            r.update({"pessoa": f"B{periodo:02d}-{cenario[:3]}-{j:05d}", "periodo": int(periodo), "sintetico": 1, "origem": fonte[i]["pessoa"]})
            if cenario == "patologico" and r["treat"] == 1:
                r["re75"] = 0.0
            out.append(r)
        return out

    def generalizar(self, linha):
        return {**linha, "age": _faixa_idade(int(linha["age"])), "educ": _faixa_educ(int(linha["educ"]))}

    # ---------------- modelo ----------------
    # Modelo colapsado, como no projeto B simulado: 4 logits independentes, P(Y | x, g) nas células (braço × negro).
    # A verossimilhança é Bernoulli nos tratados e pseudo-Bernoulli (pesos de Kish) nos controles reponderados;
    # as prioris são independentes, então a posterior fatora e o Metropolis do framework mistura bem com d = 4.
    # O escore de propensão é estimado por máxima verossimilhança (IRLS) e entra como fixo; a incerteza dele só
    # aparece pelo tamanho efetivo de Kish (limitação declarada).
    def _propensao(self, dados, placebo=False):
        X = desenho(dados, placebo)
        b, H = ajustar_logit(X, np.asarray(dados["treat"], float))
        return special.expit(X @ b), b, H

    def suficientes(self, dados):
        t, y, negro = (np.asarray(dados[c], int) for c in ("treat", "y", "black"))
        e, b, _ = self._propensao(dados)
        k, n = contagens(y, t, negro, e)
        att, p1, p0, ess = att_ipw(y, t, e)
        return {"k": k.tolist(), "n": n.tolist(), "beta_prop": b.tolist(),
                "e_min_tratados": float(e[t == 1].min()) if (t == 1).any() else 0.0,
                "n_tratados": int((t == 1).sum()), "n_controles": int((t == 0).sum()), "ess_controles": ess,
                "att_ipw_pontual": att, "p_tratados": p1, "p_controles_ponderado": p0,
                "frac_controles_fora_suporte": float(np.mean(e[t == 0] < e[t == 1].min())) if (t == 1).any() else 1.0}

    def _priori_vetor(self, priori):
        a, b = np.ones(4), np.ones(4)
        a[I_BASAL], b[I_BASAL] = priori["a"], priori["b"]
        return a, b

    def logpost(self, x, suf, priori):
        x = np.asarray(x, float)
        a, b = self._priori_vetor(priori)
        k, n = np.asarray(suf["k"], float), np.asarray(suf["n"], float)
        lp, l1p = -np.logaddexp(0, -x), -np.logaddexp(0, x)
        return float(np.sum((k + a) * lp + (n - k + b) * l1p))   # inclui o jacobiano p(1 − p)

    def amostra_priori(self, priori, rng):
        a, b = self._priori_vetor(priori)
        return special.logit(np.clip(rng.beta(a, b), 1e-9, 1 - 1e-9))

    def simular_suf(self, x, rng):
        p = special.expit(np.asarray(x, float))
        k = np.array([rng.binomial(nn, pp) for nn, pp in zip(N_SBC, p)])
        return {"k": k.tolist(), "n": list(N_SBC)}

    def quantidade(self, x):
        """ATT em pontos de emprego: P(Y | do(treino), tratados) − P(Y | do(sem treino), tratados), padronizado pela raça."""
        d0, d1 = padronizar(special.expit(np.asarray(x, float)))
        return float(d1 - d0)

    def x_inicial(self, suf):
        return special.logit((np.asarray(suf["k"], float) + 0.5) / (np.asarray(suf["n"], float) + 1))

    # ---------- F1 / M5: modelo nulo e teste severo ----------
    def nulo(self, dados, rng):
        """Mundo nulo: o treinamento é inerte. O emprego de 1978 de TODOS (tratados inclusive) é sorteado do modelo de
        desfecho dos controles, P(Y | covariáveis) logístico com a especificação de Dehejia–Wahba, mantendo quem foi
        tratado e os pesos. Pergunta: o ATT ajustado observado se distingue do que um tratamento inerte produziria
        com esse mesmo estimador? E o contraste ingênuo?"""
        t, y = np.asarray(dados["treat"], int), np.asarray(dados["y"], float)
        X = desenho(dados)
        e = special.expit(X @ ajustar_logit(X, t)[0])
        by = ajustar_logit(X[t == 0], y[t == 0])[0]
        p_nulo = special.expit(X @ by)
        w0 = pesos_att(e)[t == 0]
        m1, m0 = t == 1, t == 0
        B, ing, aj = 2000, [], []
        for _ in range(0, B, 100):
            Yr = (rng.random((100, y.size), dtype=np.float32) < p_nulo[None, :].astype(np.float32))
            a1 = Yr[:, m1].mean(1)
            ing.append(a1 - Yr[:, m0].mean(1))
            aj.append(a1 - (Yr[:, m0].astype(float) @ w0) / w0.sum())
        ing, aj = np.concatenate(ing), np.concatenate(aj)
        obs_ing = float(y[m1].mean() - y[m0].mean())
        obs_aj = att_ipw(y, t, e)[0]
        p_aj = float(np.mean(aj >= obs_aj))
        return {"estatistica": "ATT ajustado por ponderação de propensão (pontos de emprego em 1978)",
                "nulo": "treinamento inerte: Y ~ Bernoulli(P(Y | covariáveis)) ajustado nos controles, para tratados e controles; "
                        "mesmo tratamento, mesmos pesos",
                "observado": obs_aj, "nulo_media": float(aj.mean()), "nulo_q025_q975": np.quantile(aj, [0.025, 0.975]).tolist(),
                "p_valor_unilateral": p_aj, "severidade_da_alegacao": 1 - p_aj, "severo": bool(1 - p_aj > 0.95),
                "teste_ingenuo": {"estatistica": "diferença crua NSW-tratado − CPS", "observado": obs_ing, "nulo_media": float(ing.mean()),
                                  "nulo_q025_q975": np.quantile(ing, [0.025, 0.975]).tolist(),
                                  "P_nulo_ingenuo_menor_ou_igual_ao_observado": float(np.mean(ing <= obs_ing))},
                "n_pessoas": int(t.size), "n_tratados": int(m1.sum()),
                "leitura": f"Com o treinamento inerte, a composição sozinha já daria um contraste ingênuo de ≈ {ing.mean():+.3f} "
                           f"[{np.quantile(ing, 0.025):+.3f}; {np.quantile(ing, 0.975):+.3f}]; o observado, {obs_ing:+.3f}, "
                           "é um 'dano' que é só a diferença entre NSW e CPS: a comparação ingênua mede populações, não o programa. O ATT "
                           f"ajustado observado, {obs_aj:+.3f}, fica {'fora' if p_aj < 0.025 else 'dentro'} da faixa de 95% do nulo "
                           f"[{np.quantile(aj, 0.025):+.3f}; {np.quantile(aj, 0.975):+.3f}] (p unilateral = {p_aj:.3f}). Este teste "
                           "é condicional ao modelo de desfecho e à seleção em observáveis: ele não enxerga confundidor não medido."}

    # ---------- PPC (M15 / MB4) ----------
    def ppc(self, dados, amostras_x, rng):
        """Checagem preditiva do placebo: o treinamento não pode causar o emprego de 1975 (re75 > 0).

        O modelo é saturado nas células (braço × negro) e reproduz qualquer tabela; nada nelas o falsearia. O que
        pode falhar é a premissa de seleção em observáveis. Implicação testável sob estacionaridade (Imbens, 2015):
        deslocando o relógio um período, o 'efeito' ajustado por (demografia, re74) sobre o pseudo-desfecho re75
        deve ser zero (d-separação em GRAFO_PLACEBO; falha em GRAFO_PLACEBO_U, com choque transitório U).
        Ressalva: o placebo testa a premissa vizinha um período atrás, não a premissa principal; se a seleção depende
        de re75 por si (queda de Ashenfelter observada), o placebo falha mesmo que o ajuste principal valha.

        Réplicas: X_rep ~ Bernoulli(e_placebo) com os coeficientes do escore placebo sorteados da aproximação de
        Laplace da posterior (β̂, H⁻¹); W = emprego em 1975 fixo; a estatística é a do placebo (ponderação ATT)."""
        t = np.asarray(dados["treat"], int)
        W = (np.asarray(dados["re75"], float) > 0).astype(float)
        X = desenho(dados, placebo=True)
        b, H = ajustar_logit(X, t)
        e = special.expit(X @ b)
        obs = float(estatistica_placebo(W, t, e))
        R = 300
        betas = rng.multivariate_normal(b, np.linalg.inv(H), R)
        rep = []
        for j in range(0, R, 50):     # em blocos, para não alocar R × n de uma vez
            er = special.expit(betas[j:j + 50] @ X.T)
            xr = (rng.random(er.shape) < er).astype(np.float32)
            rep.append(estatistica_placebo(W, xr, er))
        rep = np.concatenate(rep)
        pv = float(min(1.0, 2 * min(np.mean(rep >= obs), np.mean(rep <= obs))))
        return {"estatistica": "placebo: P(empregado 75 | tratado) − P(empregado 75 | controle) com pesos ATT de um escore sem re75",
                "implicacao_testada": "R75 ⊥ X | D, R74 (estacionaridade da seleção, Imbens 2015)", "observado": obs,
                "replicas_q025_q975": np.quantile(rep, [0.025, 0.975]).tolist(), "p_valor": pv,
                "z_padronizado": float((obs - rep.mean()) / rep.std()), "n_tratados": int(t.sum())}

    def resultado_unidade(self, dados):
        # o desfecho da pessoa (0/1). Para um desfecho binário, o conjunto conforme a 90% é {0, 1} sempre que a taxa-base
        # fica entre 5% e 95%: a cobertura é garantida e o conjunto não é informativo (dito em `critica`).
        return np.asarray(dados["y"], float)

    # ---------------- F3 ----------------
    _COVS = ("age", "educ", "black", "hisp", "marr", "nodegree", "re74", "re75")

    def entradas_cofre(self, cofre):
        return {"ids": np.asarray(cofre["pessoa"]).tolist(), "negro": np.asarray(cofre["black"], int),
                "covariaveis": {c: np.asarray(cofre[c]) for c in self._COVS},
                "deriva": np.asarray(cofre["re75"], float)}

    def previsoes(self, treino, entradas, amostras_x):
        p = special.expit(amostras_x[:: max(1, len(amostras_x) // 2000)]).mean(0)     # preditiva posterior por célula
        p0 = {1: float(p[0]), 0: float(p[1])}
        t, y = np.asarray(treino["treat"], int), np.asarray(treino["y"], float)
        ingenua = float(y[t == 0].mean())          # o contrafactual implícito da comparação crua: a taxa do CPS
        Xt = desenho(treino)
        by = ajustar_logit(Xt[t == 0], y[t == 0])[0]
        greg = special.expit(desenho(entradas["covariaveis"]) @ by)
        base = float(y.mean())
        out = []
        for iid, g, pr in zip(entradas["ids"], entradas["negro"].tolist(), greg.tolist()):
            out.append({"id": iid, "evento": "empregado em 1978 (re78 > 0), sem treinamento", "taxa_base": base,
                        "probs": {"agent-f4-frontier": p0[g], "alegacao-ingenua": ingenua, "ajuste-regressao-logistica": float(pr)},
                        "meta": {"fonte_filtro": "controles sorteados do NSW (cofre fora da amostra por desenho, não fora do tempo)",
                                 "pode_influir": False, "flutuacao_ou_causa": "causa: o contrafactual sem treinamento previsto pelo ajuste observacional",
                                 "n_fatores": 9, "degrau_pearl": 2}})
        self._cofre_previsoes = {"p0": p0, "ingenua": ingenua, "base": base, "regressao": dict(zip(entradas["ids"], greg.tolist()))}
        return out

    def desfechos(self, cofre):
        y = np.asarray(cofre["y"], int)
        # chamado pelo F3 depois do pré-registro: guarda o cofre aberto para as lentes (degrau 2 fora da amostra)
        self._cofre = {c: np.asarray(cofre[c]) for c in ("pessoa", "y", "re74", "re75", "re78", *self._COVS)}
        return dict(zip(np.asarray(cofre["pessoa"]).tolist(), y.tolist()))

    # ---------------- F5 / F6 ----------------
    def estatistica_publicada(self, dados):
        t, y = np.asarray(dados["treat"], int), np.asarray(dados["y"], int)
        return int(y[t == 1].sum()), int((t == 1).sum()), "fração dos treinados do NSW empregados em 1978 (dados reais)"

    def justica(self, ctx):
        d = ctx["treino"]
        per = np.asarray(d["periodo"], int)
        raca = np.asarray(d["raca"]).astype(str)
        u74, u75 = (np.asarray(d["re74"], float) == 0).astype(int), (np.asarray(d["re75"], float) == 0).astype(int)
        nd = np.asarray(d["nodegree"], int)
        des = 1 - np.asarray(d["y"], int)          # o evento a prever: desempregado em 1978
        ajuste_ = per < max(per)                   # escore ajustado nos períodos 1–2, avaliado no 3
        aval = ~ajuste_
        esc = np.full(des.size, des[ajuste_].mean())
        for rr_, a, b_, c in itertools.product(("negro", "hispanico", "outro"), (0, 1), (0, 1), (0, 1)):
            m = (raca == rr_) & (u74 == a) & (u75 == b_) & (nd == c)
            if (m & ajuste_).any():
                esc[m] = des[m & ajuste_].mean()
        yv, ev, sv = des[aval].astype(bool), esc[aval], raca[aval]
        sinal = ev > LIMIAR_RISCO
        taxas = estat.taxas_por_grupo(yv, sinal, sv, ev)
        grupos = list(taxas)
        calib = {}
        for g in grupos:
            m = sv == g
            f = lambda a: float(a.mean()) if a.size else float("nan")
            calib[g] = {"sinalizados_previsto": f(ev[m & sinal]), "sinalizados_observado": f(yv[m & sinal]),
                        "nao_sinalizados_previsto": f(ev[m & ~sinal]), "nao_sinalizados_observado": f(yv[m & ~sinal])}
        p = {g: taxas[g]["taxa_base"][0] for g in grupos}
        vpp = {g: taxas[g]["vpp"][0] for g in grupos}
        fnr = {g: taxas[g]["fnr"][0] for g in grupos}
        fpr = {g: taxas[g]["fpr"][0] for g in grupos}
        fnr_comum = float(np.nanmean(list(fnr.values())))
        fpr_implicito = {g: float(kleinberg_fpr(p[g], vpp[g], fnr_comum)) for g in grupos}
        # inversão da métrica: igualdade de oportunidade (FNR igual ao do grupo de menor FNR), sinalizando ao acaso uma
        # fração r_g da célula de risco logo abaixo do limiar em cada grupo de FNR maior (Hardt et al., 2016)
        g_ref = min(grupos, key=lambda g: fnr[g])
        rr = np.random.default_rng(ctx["semente"] + 6)
        sinal2 = sinal.copy()
        frac = {}
        for g in grupos:
            mg = sv == g
            abaixo = mg & ~sinal
            if g == g_ref or not abaixo.any() or not (yv & mg).any():
                frac[g] = 0.0
                continue
            prox = abaixo & (ev == ev[abaixo].max())
            ganho = (yv & prox).sum() / (yv & mg).sum()
            frac[g] = float(np.clip((fnr[g] - fnr[g_ref]) / ganho, 0, 1)) if ganho > 0 else 0.0
            sinal2 |= prox & (rr.random(sinal.size) < frac[g])
        taxas2 = estat.taxas_por_grupo(yv, sinal2, sv, ev)
        novos = sinal2 & ~sinal
        ganham = "; ".join(f"{g}: {int((novos & yv & (sv == g)).sum())} desempregados a mais entram na prioridade (de {int((yv & (sv == g)).sum())}), "
                           f"FNR {fnr[g]:.2f} → {taxas2[g]['fnr'][0]:.2f}" for g in grupos if frac[g] > 0)
        perdem = "; ".join(f"{g}: {int((novos & ~yv & (sv == g)).sum())} falsos positivos a mais (de {int((~yv & (sv == g)).sum())} empregados), "
                           f"VPP {vpp[g]:.3f} → {taxas2[g]['vpp'][0]:.3f}" for g in grupos if frac[g] > 0)
        perdem = (perdem + "; e o sinal deixa de significar o mesmo risco entre os grupos (perde-se a calibração); "
                  "as vagas extras saem do orçamento do programa, isto é, de quem estava na fila") if perdem else ""
        return {
            "classificador": f"priorizar para o treinamento quem tem risco previsto de desemprego em 1978 > {LIMIAR_RISCO:.0%} "
                             "(escore = frequência na célula raça × desempregado em 74 × em 75 × sem diploma; ajustado nos períodos 1–2, "
                             "avaliado no período 3 da amostra observacional real, quase toda do CPS)",
            "grupos": "raça declarada (negro, hispânico, outro), com taxas-base reais diferentes",
            "taxas_por_grupo_IC95": taxas,
            "calibracao_por_grupo": calib,
            "trilema_da_justica": {
                "aplica_se": True,
                "identidade": "FPR = p/(1−p) · (1−VPP)/VPP · (1−FNR) (Chouldechova, 2017; Kleinberg et al., 2016)",
                "taxa_base": p, "vpp": vpp, "fnr": fnr, "fpr": fpr,
                "fpr_implicito_se_fnr_igual": {"fnr_comum": fnr_comum, **fpr_implicito},
                "leitura": "O escore inclui a raça e é calibrado por grupo, mas as taxas-base de desemprego diferem ("
                           + ", ".join(f"{g} {p[g]:.3f}" for g in grupos) + "). Com a VPP fixada pela calibração e um FNR comum de "
                           f"{fnr_comum:.2f}, os FPR teriam de ser " + ", ".join(f"{fpr_implicito[g]:.3f} ({g})" for g in grupos)
                           + ": diferentes. Calibração e paridade de FPR e FNR não valem juntas com taxas-base distintas.",
            },
            "quem_paga_o_erro": {g: f"FNR {fnr[g]:.2f}: de cada 100 {g}s que ficarão desempregados, {100 * fnr[g]:.0f} ficam fora da prioridade"
                                 for g in grupos},
            "inversao_metrica": {
                "metrica_original": "calibração (mesmo limiar de risco absoluto para todos os grupos)",
                "metrica_invertida": f"igualdade de oportunidade (FNR igual ao de '{g_ref}'): sinalizar ao acaso "
                                     + ", ".join(f"{frac[g]:.0%} da célula logo abaixo do limiar em '{g}'" for g in grupos if frac[g] > 0),
                "fnr_depois": {g: taxas2[g]["fnr"][0] for g in taxas2}, "fpr_depois": {g: taxas2[g]["fpr"][0] for g in taxas2},
                "vpp_depois": {g: taxas2[g]["vpp"][0] for g in taxas2},
                "fnr_alvo": fnr[g_ref],
                "igualdade_atingida": {g: bool(abs(taxas2[g]["fnr"][0] - fnr[g_ref]) < 0.02) for g in taxas2},
                "nota": "o pós-processamento só mexe na célula de risco logo abaixo do limiar; se ela não basta (fração 100%) ou não "
                        "tem desempregados, a igualdade de FNR fica parcial, e atingi-la exige descer o limiar em mais células",
                "ganham": ganham, "perdem": perdem,
            },
            "assimetria_de_poder": "quem avalia o programa com dados baratos (CPS) decide o orçamento; quem é elegível ao NSW arca com "
                                   "o erro do contrafactual, que o sorteio mostra ser outra população",
            "dados_reais": True,
        }

    # ---------------- M15 ----------------
    def _ipw_resumo(self, dados):
        t, y = np.asarray(dados["treat"], int), np.asarray(dados["y"], float)
        e = self._propensao(dados)[0]
        att, p1, p0, ess = att_ipw(y, t, e)
        var = p1 * (1 - p1) / max((t == 1).sum(), 1) + p0 * (1 - p0) / max(ess, 1)
        return att, var

    def _renda(self, treino, rng, B=200):
        """ATT em US$ (re78): ingênuo, ajustado (ponderação, bootstrap com reestimação do escore) e experimental."""
        t = np.asarray(treino["treat"], int)
        r78 = np.asarray(treino["re78"], float)
        cof = getattr(self, "_cofre", None)
        out = {"ingenuo": dif_proporcoes(r78[t == 1], r78[t == 0])}
        X = desenho(treino)

        def aj(idx):
            tt, rr_, XX = t[idx], r78[idx], X[idx]
            e = special.expit(XX @ ajustar_logit(XX, tt)[0])
            w = pesos_att(e)[tt == 0]
            return float(rr_[tt == 1].mean() - np.sum(w * rr_[tt == 0]) / w.sum())
        ponto = aj(np.arange(t.size))
        i1, i0 = np.flatnonzero(t == 1), np.flatnonzero(t == 0)
        boot = np.array([aj(np.concatenate([rng.choice(i1, i1.size), rng.choice(i0, i0.size)])) for _ in range(B)])
        out["ajustado_ponderacao"] = {"estimativa": ponto, "IC95_bootstrap": np.quantile(boot, [0.025, 0.975]).tolist(), "B": B}
        if cof is not None:
            out["experimental"] = dif_proporcoes(r78[t == 1], cof["re78"].astype(float))
            out["literatura"] = {"experimental_US$": ATT_US_LITERATURA, "confere": bool(abs(out["experimental"]["estimativa"] - ATT_US_LITERATURA) < 1.0)}
        return out

    def _benchmark(self, ctx):
        """Ingênuo vs ajustado vs experimental (o cofre aberto), placebo nos dois mundos e o teste do cofre."""
        tr = ctx["treino"]
        cof = getattr(self, "_cofre", None)
        t, y = np.asarray(tr["treat"], int), np.asarray(tr["y"], float)
        q = ctx["f4"]["_q"]
        res = {"ingenuo": dif_proporcoes(y[t == 1], y[t == 0]),
               "ajustado_posterior": {"mediana": float(np.median(q)), "IC90": [float(np.quantile(q, 0.05)), float(np.quantile(q, 0.95))],
                                      "IC95": [float(np.quantile(q, 0.025)), float(np.quantile(q, 0.975))],
                                      "pontual_ponderacao": ctx["suf"]["att_ipw_pontual"], "ess_controles": ctx["suf"]["ess_controles"]}}
        Xt = desenho(tr)
        by = ajustar_logit(Xt[t == 0], y[t == 0])[0]
        res["ajustado_regressao_logistica"] = {"estimativa": float(y[t == 1].mean() - special.expit(Xt[t == 1] @ by).mean())}
        if cof is None:
            return res
        yc = cof["y"].astype(float)
        exp = dif_proporcoes(y[t == 1], yc)
        res["experimental"] = exp
        res["ajustado_menos_experimental"] = float(np.median(q) - exp["estimativa"])
        res["P_posterior_ajustado_dentro_IC95_experimental"] = float(np.mean((q >= exp["IC95"][0]) & (q <= exp["IC95"][1])))
        res["ingenuo_menos_experimental"] = float(res["ingenuo"]["estimativa"] - exp["estimativa"])
        # placebo (re75 > 0): no experimento o sorteio garante zero em média; no observacional ajustado, não
        w_t = (np.asarray(tr["re75"], float) > 0).astype(float)
        res["placebo_experimental"] = dif_proporcoes(w_t[t == 1], (cof["re75"].astype(float) > 0).astype(float))
        res["placebo_experimental_renda_US$"] = dif_proporcoes(np.asarray(tr["re75"], float)[t == 1], cof["re75"].astype(float))
        Xp = desenho(tr, placebo=True)
        ep = special.expit(Xp @ ajustar_logit(Xp, t)[0])
        res["placebo_observacional_ajustado"] = float(estatistica_placebo(w_t, t, ep))
        r75 = np.asarray(tr["re75"], float)
        wp = pesos_att(ep)[t == 0]
        res["placebo_observacional_ajustado_renda_US$"] = float(r75[t == 1].mean() - np.sum(wp * r75[t == 0]) / wp.sum())
        # o teste do cofre: o contrafactual observacional prevê o emprego dos controles sorteados?
        am = special.expit(ctx["f4"]["_amostras"])
        neg = cof["black"].astype(int)
        pc = self._cofre_previsoes
        calib = {}
        for g, i, nome in ((1, 0, "negro"), (0, 1, "nao_negro")):
            m = neg == g
            obs = float(yc[m].mean())
            post = am[:, i]
            se = float(np.sqrt(post.var() + obs * (1 - obs) / m.sum()))
            calib[nome] = {"n": int(m.sum()), "previsto_posterior": float(post.mean()), "observado": obs,
                           "z": float((post.mean() - obs) / se)}
        prev_tot = float(np.mean([pc["p0"][g] for g in neg.tolist()]))
        b = lambda pp: float(np.mean((np.asarray(pp) - yc) ** 2))
        res["cofre_calibracao_no_grosso"] = {"previsto_medio": prev_tot, "observado": float(yc.mean()), "por_grupo": calib,
                                             "ingenua_prevista": pc["ingenua"]}
        res["cofre_brier"] = {"agent-f4-frontier": b([pc["p0"][g] for g in neg.tolist()]), "alegacao-ingenua": b(np.full(yc.size, pc["ingenua"])),
                              "ajuste-regressao-logistica": b([pc["regressao"][i] for i in cof["pessoa"].tolist()]),
                              "climatologia": b(np.full(yc.size, pc["base"]))}
        return res

    def lentes(self, ctx):
        tr = ctx["treino"]
        q = ctx["f4"]["_q"]
        t, y = np.asarray(tr["treat"], int), np.asarray(tr["y"], int)
        per = np.asarray(tr["periodo"], int)
        rng = np.random.default_rng(ctx["semente"] + 4)
        bench = self._benchmark(ctx)
        renda = self._renda(tr, rng)
        cof = getattr(self, "_cofre", None)
        # Morfonomia: ATT ajustado por partição (homogeneidade esperada: partições aleatórias) e transporte do controle
        att_p, var_p = [], []
        for pp in sorted(set(per.tolist())):
            m = per == pp
            a, v = self._ipw_resumo({c: np.asarray(tr[c])[m] for c in tr})
            att_p.append(a)
            var_p.append(v)
        att_p, wts = np.array(att_p), 1 / np.array(var_p)
        Q = float(np.sum(wts * (att_p - np.sum(wts * att_p) / wts.sum()) ** 2))
        p_Q = float(stats.chi2.sf(Q, len(att_p) - 1))
        transp = {}
        if cof is not None:
            suf = ctx["suf"]
            for g, i, nome in ((1, 0, "negro"), (0, 1, "nao_negro")):
                kc, nc = suf["k"][i], suf["n"][i]
                m = cof["black"].astype(int) == g
                ke, ne = float(cof["y"][m].sum()), float(m.sum())
                tab = np.array([[kc, nc - kc], [ke, ne - ke]])
                c2, pv, _, _ = stats.chi2_contingency(tab, correction=False)
                transp[nome] = {"cps_ponderado": kc / nc, "nsw_sorteado": ke / ne, "ess_cps": nc, "n_nsw": int(ne), "p_invariancia": float(pv)}
        # Fluxonomia: informação mútua X–Y (bits) vista no observacional e no experimento
        def mi(xx, yy):
            pxy = np.array([[np.mean((xx == a) & (yy == b)) for b in (0, 1)] for a in (0, 1)])
            px_, py_ = pxy.sum(1, keepdims=True), pxy.sum(0, keepdims=True)
            with np.errstate(divide="ignore", invalid="ignore"):
                return float(np.nansum(pxy * np.log2(pxy / (px_ * py_))))
        mi_exp = None
        if cof is not None:
            xe = np.concatenate([np.ones((t == 1).sum(), int), np.zeros(cof["y"].size, int)])
            ye = np.concatenate([y[t == 1], cof["y"].astype(int)])
            mi_exp = mi(xe, ye)
        # Crononomia: a queda de Ashenfelter (renda e emprego por ano, por grupo)
        e = special.expit(desenho(tr) @ np.asarray(ctx["suf"]["beta_prop"]))
        w = pesos_att(e)
        def perfil(m, pesos=None):
            pesos = np.ones(m.sum()) if pesos is None else pesos
            return {f"{c}_US$": float(np.average(np.asarray(tr[c], float)[m], weights=pesos)) for c in ("re74", "re75", "re78")} | \
                   {f"empregado_{c[2:]}": float(np.average((np.asarray(tr[c], float)[m] > 0), weights=pesos)) for c in ("re74", "re75", "re78")}
        crono = {"NSW_tratados": perfil(t == 1), "CPS_cru": perfil(t == 0), "CPS_ponderado_ATT": perfil(t == 0, w[t == 0])}
        if cof is not None:
            crono["NSW_controles_sorteados"] = {f"{c}_US$": float(cof[c].astype(float).mean()) for c in ("re74", "re75", "re78")} | \
                                               {f"empregado_{c[2:]}": float((cof[c].astype(float) > 0).mean()) for c in ("re74", "re75", "re78")}
        # Escalonomia: o que o desfecho binário esconde
        r78 = np.asarray(tr["re78"], float)
        def esc(v):
            s = np.sort(v)[::-1]
            return {"frac_zeros": float(np.mean(v == 0)), "media": float(v.mean()), "mediana": float(np.median(v)),
                    "q90": float(np.quantile(v, 0.9)), "fatia_dos_5%_maiores": float(s[: max(1, v.size // 20)].sum() / max(s.sum(), 1e-9))}
        d0, d1 = padronizar(special.expit(ctx["f4"]["_amostras"]).T)
        return {
            "conferencia": {"ingenuo_vs_ajustado_vs_experimental_emprego": bench, "renda_US$": renda,
                            "dados": carregar()["sha"]},
            "Morfonomia": {
                "att_ajustado_por_particao": att_p.tolist(), "cochran_Q": Q, "p_homogeneidade": p_Q, "invariante_entre_particoes": bool(p_Q > 0.01),
                "nota_particoes": "partições aleatórias da mesma amostra: a homogeneidade é esperada por construção e não testa estabilidade no tempo",
                "transporte_controle_ajustado_para_sorteado": transp,
                "d_separacao": {"R75_indep_X_dado_D_R74 (placebo, sem U)": d_separados(GRAFO_PLACEBO, ["R75"], ["X"], ["D", "R74"]),
                                "R75_indep_X_dado_D_R74 (com choque U)": d_separados(GRAFO_PLACEBO_U, ["R75"], ["X"], ["D", "R74"])},
                "porta_dos_fundos": {"{D, R74, R75} admissível": porta_dos_fundos(GRAFO, "X", "Y", ["D", "R74", "R75"]),
                                     "{} admissível": porta_dos_fundos(GRAFO, "X", "Y", []),
                                     "{D, R74} admissível": porta_dos_fundos(GRAFO, "X", "Y", ["D", "R74"])},
                "leitura": "a forma que deveria se transportar é P(Y | sem treino, covariáveis): do CPS reponderado para os controles "
                           "sorteados. O grafo de Dehejia–Wahba admite {D, R74, R75}; a implicação testável vizinha (placebo) vale "
                           "sem choque não medido e falha com ele.",
            },
            "Fluxonomia": {"informacao_mutua_X_Y_observacional_bits": mi(t, y), "informacao_mutua_X_Y_experimental_bits": mi_exp,
                           "laco_performativo": True,
                           "leitura": "no dado observacional, saber se a pessoa foi treinada informa sobre o emprego quase só porque "
                                      "distingue duas populações (NSW vs. CPS), com sinal oposto ao experimental. Se avaliações "
                                      "observacionais virarem regra de financiamento, os programas passam a escolher participantes "
                                      "com histórico melhor (seleção de nata), e o viés da avaliação muda de sinal."},
            "Crononomia": {"queda_de_ashenfelter": crono, "ergodico": "não se aplica de modo relevante",
                           "leitura": "a renda dos elegíveis ao NSW cai de 1974 para 1975 nos dois braços sorteados: é o critério de "
                                      "elegibilidade (desemprego recente), e não efeito. No CPS ponderado a trajetória pré-tratamento "
                                      "imita a dos tratados porque re74 e re75 estão no escore; o que o escore não vê é a queda depois "
                                      "de 1975, até a entrada no programa. O cofre não é futuro: é o mesmo 1978, outra fonte."},
            "Escalonomia": {"renda_1978_tratados": esc(r78[t == 1]), "renda_1978_CPS": esc(r78[t == 0]),
                            "risco_absoluto_do_treino_por_100": float(100 * np.median(d1)), "risco_absoluto_sem_treino_por_100": float(100 * np.median(d0)),
                            "rr_causal_mediana": float(np.median(d1 / d0)), "att_mediana": float(np.median(q)),
                            "leitura": "a renda tem massa em zero e cauda longa (os 5% maiores levam uma fatia grande do total); a média em "
                                       "US$ do ATT é sensível a poucos. O binário perde a intensidade (horas, salário), mas é robusto; "
                                       "por isso a renda vai na conferência e não no modelo."},
            "Crisonomia": {"placar": ctx["f3"]["placar"], "evpi_posterior": estat.evpi(q[::10], self.utilidade, self.acoes())["evpi"],
                           "degrau_2_fora_da_amostra": {k: bench[k] for k in ("experimental", "cofre_calibracao_no_grosso", "cofre_brier") if k in bench},
                           "leitura": "o cofre é o juiz: os controles sorteados dizem qual era o emprego dos tratados sem treinamento. "
                                      "O contrafactual ingênuo (taxa do CPS) erra por muito; o ajustado é comparado no mesmo placar."},
        }

    def _bic_rival(self, tr):
        t, y = np.asarray(tr["treat"], float), np.asarray(tr["y"], float)
        n = y.size

        def ll(X):
            b = ajustar_logit(X, y, l2=0.0)[0]
            p = np.clip(special.expit(X @ b), 1e-12, 1 - 1e-12)
            return float(np.sum(y * np.log(p) + (1 - y) * np.log(1 - p))), X.shape[1]
        (l1, k1), (l2_, k2) = ll(np.column_stack([np.ones(n), t])), ll(np.column_stack([desenho(tr), t]))
        b1, b2 = -2 * l1 + k1 * np.log(n), -2 * l2_ + k2 * np.log(n)
        return {"bic_so_X (ingenuo)": b1, "bic_X_e_covariaveis": b2, "mdl_prefere": "X e covariáveis" if b2 < b1 else "só X"}

    def critica(self, ctx):
        tr = ctx["treino"]
        suf = ctx["suf"]
        return {"rival": {"modelo": "o ingênuo: P(Y | X) sem covariáveis", **self._bic_rival(tr),
                          "leitura": "o rival prevê pior (MDL) e responde à pergunta errada; o modelo ajustado prevê melhor, mas prever "
                                     "bem no degrau 1 não garante o degrau 2: só o cofre sorteado confere"},
                "degrau_2_fora_da_amostra": {k: v for k, v in self._benchmark(ctx).items() if k.startswith(("experimental", "cofre", "placebo"))},
                "performatividade": "sim: se agências passarem a ser julgadas por comparações observacionais, escolhem participantes "
                                    "com melhor histórico, e a comparação deixa de medir o programa (ver Fluxonomia)",
                "premissas": ["seleção só em observáveis dado (D, re74, re75): não testável; o placebo com re75 testa a premissa "
                              "vizinha um período atrás (estacionaridade), não a principal",
                              "renda medida do mesmo jeito no NSW (entrevistas) e no CPS (questionário anual): não verificável aqui; "
                              "LaLonde e Smith & Todd apontam a diferença como fonte de viés",
                              "mercado de trabalho local: o NSW vem de 10 cidades, o CPS é nacional; a geografia não está nas covariáveis",
                              f"suporte comum: {suf['frac_controles_fora_suporte']:.0%} dos controles do CPS têm escore abaixo do menor "
                              f"tratado; o ajuste vive de ~{suf['ess_controles']:.0f} controles efetivos (Kish)",
                              "verossimilhança ponderada por pseudo-contagens de Kish: aproximação; o escore de propensão entra fixo "
                              "(sua incerteza não se propaga ao MCMC, só ao bootstrap da renda e ao PPC)",
                              f"padronização pela composição racial dos tratados (P(negro) = {W_NEGRO:.3f}), fixada no termo de escopo",
                              "desfecho binário: perde a intensidade da renda (relatada à parte, em US$)",
                              "o conjunto conforme de um desfecho binário a 90% é {0, 1}: a cobertura é trivial e não informativa",
                              "o cofre é fora da amostra por desenho, não fora do tempo; a deriva do F3 entre o período 3 (CPS) e o 4 "
                              "(NSW) é a diferença de populações que a alegação precisa atravessar, não envelhecimento do modelo",
                              "painel SHELF simulado e escrito por quem conhece o resultado experimental"]}

    # ---------------- F8 ----------------
    def acoes(self):
        return ["financiar o treinamento", "não financiar"]

    def utilidade(self, acao, theta):
        """Em pontos de emprego por participante: financiar troca θ (o ATT causal) pelo custo C/L."""
        return float(theta - CUSTO) if acao == "financiar o treinamento" else 0.0

    def decisao(self, ctx):
        q = ctx["f4"]["_q"]
        p = special.expit(ctx["f4"]["_amostras"])
        d0, d1 = padronizar(p.T)
        tr = ctx["treino"]
        t, y = np.asarray(tr["treat"], int), np.asarray(tr["y"], float)
        ing = float(y[t == 0].mean())
        rng = np.random.default_rng(ctx["semente"] + 8)
        i = rng.integers(0, q.size, 5000)
        hops = {"treinar, do(X=1)": 100 * d1[i], "não treinar, do(X=0), ajustado": 100 * d0[i],
                "contrafactual ingênuo (taxa do CPS)": np.full(i.size, 100 * ing)}
        P_pos = float(np.mean(q > 0))
        P_vale = float(np.mean(q > CUSTO))
        m1, m0 = (round(100 * float(np.median(v))) for v in (d1, d0))
        return {
            "recomendacao": "financiar o treinamento" if P_vale > 0.5 else "não financiar com base só na evidência observacional",
            "efeito_causal_att": {"mediana": float(np.median(q)), "q05": float(np.quantile(q, 0.05)), "q95": float(np.quantile(q, 0.95))},
            "rr_causal_mediana": float(np.median(d1 / d0)),
            "arvore_CL": {"evento": "emprego em 1978", "acao": "financiar o treinamento",
                          "C": f"custo por participante, {CUSTO} ponto de emprego-equivalente (ILUSTRATIVO)", "L": "1 emprego",
                          "P_treinamento_aumenta_emprego": P_pos, "P_aumento_supera_C_L": P_vale,
                          "regra": "financiar se o ATT causal > C/L"},
            "frequencias_naturais": (f"De cada 100 homens elegíveis ao NSW que recebem o treinamento, cerca de {m1} estarão empregados em "
                                     f"1978; sem treinamento, cerca de {m0}. A comparação ingênua diria que, sem treinamento, "
                                     f"{round(100 * ing)} estariam empregados, porque compara com outra população."),
            "classe_de_referencia": "homens elegíveis ao NSW, 1976–77, 10 cidades dos EUA, 84% negros",
            "circuit_breaker": self.termo_escopo()["circuit_breaker"],
            "hops_titulo": "Empregados em 1978 por 100 elegíveis", "hops_rotulo": "empregados por 100", "_hops": hops,
        }

    def disjuntor(self, dados, ctx):
        e = special.expit(desenho(dados) @ np.asarray(ctx["suf"]["beta_prop"]))
        fora = float(np.mean(e < ctx["suf"]["e_min_tratados"]))
        f_neg = float(np.mean(np.asarray(dados["black"], int)))
        t = np.asarray(dados["treat"], int)
        motivos = []
        if fora > FRAC_FORA_MAX:
            motivos.append(f"{fora:.0%} do lote fora do suporte comum do escore de propensão")
        if abs(f_neg - W_NEGRO) > DESVIO_NEGRO_MAX:
            motivos.append(f"fração de negros {f_neg:.2f} longe da população-alvo {W_NEGRO:.2f}")
        z = None
        if (t == 1).sum() >= 30 and (t == 0).sum() >= 30:
            z = self.ppc(dados, ctx["f4"]["_amostras"], np.random.default_rng(5))["z_padronizado"]
            if abs(z) > 4:
                motivos.append(f"placebo re75 acusa seleção não explicada (z = {z:.1f})")
        return {"disparou": bool(motivos), "frac_fora_suporte": fora, "frac_negros": f_neg, "placebo_z": z,
                "motivo": "; ".join(motivos) if motivos else "dentro do escopo"}
