"""Projeto C em domínio real · A cauda que o ressegurador não viu (Escalonomia + Crononomia, M8/M9).

Dados reais: perdas de incêndio da Copenhagen Re acima de 1 M DKK, 1980-01-03 a 1990-12-31
(2 167 sinistros em milhões de DKK de 1985; `dados-reais/danish_fire.csv`, ver FONTES.md).

Um ressegurador vende uma camada de excesso de danos por risco "100 xs 50" (paga, por sinistro,
a parte da perda entre 50 e 150 M DKK). No fim de 1988 ele olha os últimos 36 meses, não vê
nenhum sinistro na camada, conclui que o risco é desprezível e dimensiona a exposição (limite
vendido / capital) como um apostador de Kelly que acredita em p = 0: no teto. Uma perda que
consome o limite inteiro com exposição f ≥ 1/(1 − π) zera o capital (estado absorvente).

Unidade independente: o MÊS (bloco de tempo). Toda margem, todo bootstrap e toda previsão
out-of-time são por mês, nunca por sinistro: sinistros do mesmo mês partilham o mesmo capital,
a mesma exposição e, possivelmente, a mesma conjuntura (clima, inflação de sinistros).

Por que a retenção de 50 M DKK (e não outra): é um número redondo, acima de 2 160 dos 2 167
sinistros (≈ percentil 99,7), cinco vezes o limiar de 10 M DKK em que McNeil (1997) ajusta a GPD,
e é a altura em que a falácia aparece de fato nos dados de treino: 4 meses atingidos em
1980–82, 1 em 1983–85 e nenhum em 1986–88 (40 meses limpos seguidos). Declaração honesta: a
retenção foi escolhida por quem conhecia a série inteira (as perdas de 152 e 145 M DKK de
1989–90 são famosas na literatura), então o cofre não é cego para o analista; por isso a
tabela de sensibilidade a outras retenções (20 a 50) sai junto, no `nulo` e nas lentes.
"""

from __future__ import annotations

import csv
from functools import lru_cache
from pathlib import Path

import numpy as np
from scipy import optimize, special, stats

from estocologia import estat
from estocologia.projeto import Projeto

ARQUIVO = Path(__file__).resolve().parent.parent.parent / "dados-reais" / "danish_fire.csv"

RETENCAO, LIMITE = 50.0, 100.0   # camada 100 xs 50 (M DKK de 1985), por risco
U_POT = 10.0                     # limiar do POT/GPD (McNeil 1997)
CARREGAMENTO = 1.2               # prêmio de mercado = 1,2 × burning cost mensal do treino (premissa declarada)
TETO = 3.0                       # teto de exposição: limite vendido até 3 × o capital
JANELA = 36                      # meses de histórico que a alegação lê (o triênio 1986–88)
FUT = 120                        # meses futuros na simulação da decisão (dez anos)
P_DESPREZIVEL = 1 / 1200         # o "risco desprezível" da alegação: um mês atingido por século
LN_PISO = np.log(1e-300)         # piso numérico para ln(0), o mesmo do Lab 9
G_RUINA = float(LN_PISO / FUT)   # taxa de crescimento atribuída a um capital arruinado
PERIODOS_ANOS = {1: (1980, 1982), 2: (1983, 1985), 3: (1986, 1988), 4: (1989, 1990)}
ORIGENS = ["real", "reamostra_real", "sintetico_deriva", "sintetico_patologico"]
REGRAS = ["kelly_kn", "meio_kelly_kn", "kelly_laplace"]
FAIXAS = [(1.0, 10.0, "1–10"), (10.0, RETENCAO, "10–50"), (RETENCAO, np.inf, "≥50 (camada)")]
FATOR_DERIVA = 3.0               # deriva sintética: severidade × 3 (inflação de sinistros / choque de valor segurado)
MESES_SURTO = 8                  # patológico sintético: 8 meses seguidos com sinistro na camada (contágio)
N_BOOT = 300                     # réplicas do bootstrap por blocos mensais


# ---------------- os dados reais ----------------
@lru_cache(maxsize=1)
def dados_reais() -> tuple:
    """Lê o CSV uma vez. Devolve tuplas (sinistro, data, mes, ano, perda), em ordem cronológica."""
    with open(ARQUIVO, newline="") as fh:
        linhas = list(csv.DictReader(fh))
    out, seq, ultima = [], 0, None
    for r in linhas:
        data = r["Date"]
        seq = seq + 1 if data == ultima else 1
        ultima = data
        out.append((f"DK{data.replace('-', '')}-{seq:02d}", data, data[:7], int(data[:4]), float(r["Loss"])))
    return tuple(out)


def meses_do_periodo(periodo: int) -> list[str]:
    a0, a1 = PERIODOS_ANOS[periodo]
    return [f"{a}-{m:02d}" for a in range(a0, a1 + 1) for m in range(1, 13)]


def excesso_camada(x):
    """Parte da perda que cai na camada 100 xs 50, em M DKK."""
    return np.minimum(np.maximum(np.asarray(x, float) - RETENCAO, 0.0), LIMITE)


def perdas_treino() -> np.ndarray:
    return np.array([r[4] for r in dados_reais() if r[3] <= PERIODOS_ANOS[3][1]])


# ---------------- cauda: Hill e POT/GPD ----------------
def hill_ic(x, k: int) -> dict:
    """Hill (1975) com IC assintótico de 95%: α̂ (1 ± 1,96/√k)."""
    a = estat.hill(np.asarray(x, float), k)
    return {"k": int(k), "alpha": a, "ic95_assintotico": [a * (1 - 1.96 / np.sqrt(k)), a * (1 + 1.96 / np.sqrt(k))]}


def ajustar_gpd(x, u: float = U_POT) -> dict:
    """Máxima verossimilhança da GPD nos excessos sobre u (POT). ξ = 1/α."""
    e = np.asarray(x, float)
    e = e[e > u] - u
    xi, _, sig = stats.genpareto.fit(e, floc=0)
    return {"u": u, "n_exc": int(e.size), "xi": float(xi), "sigma": float(sig), "alpha": float(1 / xi) if xi > 0 else float("inf")}


def p_pot(xi: float, sig: float, lam_mes: float, u: float = U_POT, r: float = RETENCAO) -> float:
    """Chance mensal de ≥ 1 sinistro acima de r: 1 − exp(−λ_u · S_GPD(r − u)), Poisson nos excessos."""
    s = (1 + xi * (r - u) / sig) ** (-1 / xi) if xi != 0 else np.exp(-(r - u) / sig)
    return float(1 - np.exp(-lam_mes * s))


def severidade(xi: float, sig: float, u: float = U_POT, nos: int = 64) -> dict:
    """Distribuição de s = (fração do limite consumida | camada atingida), sob a GPD acima da retenção.
    Átomo em s = 1 (perda ≥ 150) mais quadratura por quantis na parte contínua."""
    sr = sig + xi * (RETENCAO - u)
    gp = stats.genpareto(xi, scale=sr)
    p_cheio = float(gp.sf(LIMITE))
    qs = (np.arange(nos) + 0.5) / nos * (1 - p_cheio)
    s = gp.ppf(qs) / LIMITE
    w = np.full(nos, (1 - p_cheio) / nos)
    return {"s": np.append(s, 1.0), "w": np.append(w, p_cheio), "p_limite_cheio": p_cheio,
            "E_s": float(np.sum(np.append(s, 1.0) * np.append(w, p_cheio)))}


@lru_cache(maxsize=1)
def mercado() -> dict:
    """Parâmetros do mundo, todos estimados só com o treino (1980–88): GPD, λ, severidade e prêmio."""
    x = perdas_treino()
    n_meses = 12 * (PERIODOS_ANOS[3][1] - PERIODOS_ANOS[1][0] + 1)
    gp = ajustar_gpd(x)
    sev = severidade(gp["xi"], gp["sigma"])
    burning = float(excesso_camada(x).sum() / LIMITE / n_meses)
    premio = CARREGAMENTO * burning
    return {"gpd": gp, "lambda_mes": gp["n_exc"] / n_meses, "sev": sev, "burning_cost_mes": burning, "premio_mes": premio,
            "rol_anual": 12 * premio, "barreira": 1 / (1 - premio), "p_pot": p_pot(gp["xi"], gp["sigma"], gp["n_exc"] / n_meses),
            "p_critico": premio / sev["E_s"], "n_meses": n_meses}


# ---------------- o mundo, em funções puras (testadas no Hypothesis) ----------------
def g(f, p, sev: dict | None = None, premio: float | None = None):
    """Taxa de crescimento temporal por mês, E[ln(1 + fπ − f·S)], S = 0 com prob. 1 − p e S ~ severidade com prob. p."""
    sev = sev or mercado()["sev"]
    pi = mercado()["premio_mes"] if premio is None else premio
    f = np.asarray(f, dtype=float)
    lim = np.log1p(f * pi)
    fat = 1 + f[..., None] * pi - f[..., None] * sev["s"]
    lperda = np.sum(sev["w"] * np.log(np.maximum(fat, 1e-300)), axis=-1)
    return (1 - p) * lim + p * lperda


@lru_cache(maxsize=4096)
def kelly(p: float) -> float:
    """f* = argmax g em [0, TETO]. Com p = 0 o prêmio é ganho certo e quem acredita em p̂ = 0 vai ao teto.
    Com p > 0 e massa em S = 1, f* fica sempre abaixo da barreira 1/(1 − π)."""
    p = float(p)
    if p <= 0:
        return TETO
    # além da barreira, um sinistro de limite cheio (massa > 0) arruína: g verdadeira é −∞ (o piso só serve para exibição)
    teto = min(TETO, mercado()["barreira"] * (1 - 1e-9))
    fs = np.linspace(0, teto, 3001)
    gs = g(fs, p)
    i = int(np.argmax(gs))
    a, b = fs[max(i - 1, 0)], fs[min(i + 1, fs.size - 1)]
    r = optimize.minimize_scalar(lambda f: -float(g(f, p)), bounds=(a, b), method="bounded", options={"xatol": 1e-7})
    return float(np.clip(r.x, 0.0, teto)) if -r.fun >= gs[i] else float(fs[i])


def laplace(k, n):
    """Regra de sucessão de Laplace: nunca dá probabilidade zero ao que ainda não aconteceu."""
    return (k + 1) / (n + 2)


def alavancagem_da_regra(regra: str, k: int, n: int = JANELA) -> float:
    if regra == "kelly_kn":
        return kelly(k / n)
    if regra == "meio_kelly_kn":
        return kelly(k / n) / 2
    if regra == "kelly_laplace":
        return kelly(laplace(k, n))
    raise ValueError(regra)


def ln_riqueza(f, s_mensal, premio: float | None = None):
    """ln do capital final (capital inicial 1) com exposição f e frações do limite consumidas por mês.
    Estado absorvente: se em algum mês 1 + fπ − f·s ≤ 0, o capital fica em zero (−∞) para sempre."""
    pi = mercado()["premio_mes"] if premio is None else premio
    f = np.asarray(f, float)
    s = np.asarray(s_mensal, float)
    fat = 1 + f[..., None] * pi - f[..., None] * s
    arruinado = np.any(fat <= 0, axis=-1)
    lnw = np.sum(np.log(np.where(fat > 0, fat, 1.0)), axis=-1)
    return np.where(arruinado, -np.inf, lnw)


def maior_sequencia(v) -> int:
    """Maior número de posições consecutivas verdadeiras."""
    melhor = atual = 0
    for b in np.asarray(v, bool):
        atual = atual + 1 if b else 0
        melhor = max(melhor, atual)
    return melhor


def bootstrap_meses(perdas, meses, fn, rng, B: int = N_BOOT) -> np.ndarray:
    """Bootstrap por blocos mensais: reamostra meses (a unidade), leva todos os sinistros de cada mês."""
    meses = np.asarray(meses)
    um = np.unique(meses)
    por_mes = {m: np.asarray(perdas, float)[meses == m] for m in um}
    out = []
    for _ in range(B):
        sel = rng.choice(um, um.size)
        out.append(fn(np.concatenate([por_mes[m] for m in sel]), um.size))
    return np.array(out)


def _beta_quantis(qs: dict, p_ref: float) -> dict:
    """Ajuste Beta por mínimos quadrados em quantos quantis houver (SHELF com extremos)."""
    probs = np.array([float(k[1:]) / 100 for k in qs])
    alvo = np.array(list(qs.values()))

    def perda(lab):
        a, b = np.exp(lab)
        return np.sum((np.log(stats.beta.ppf(probs, a, b)) - np.log(alvo)) ** 2)

    r = optimize.minimize(perda, x0=np.log([1.0, 100.0]), method="Nelder-Mead", options={"xatol": 1e-10, "fatol": 1e-14, "maxiter": 5000})
    a, b = np.exp(r.x)
    return {"a": float(a), "b": float(b), "q95": float(stats.beta.ppf(0.95, a, b)), "media": float(a / (a + b)),
            "P_p_maior_igual_p_critico": float(stats.beta.sf(p_ref, a, b))}


class ProjetoC(Projeto):
    nome = "c"
    dados_simulados = False
    origem_dos_dados = "dados-reais/danish_fire.csv (CRAN fitdistrplus::danishuni)"
    tem_titulares = False
    titulo = "A cauda que o ressegurador não viu: camada 100 xs 50 nas perdas de incêndio dinamarquesas (dados reais)"
    alegacao = ('"Nossa camada 100 xs 50 não foi tocada em 36 meses (1986–88); logo, o risco é desprezível '
                'e vale vender limite até o teto de 3 vezes o capital."')
    periodos = (1, 2, 3, 4)
    unidade = "mes"
    colunas_modelo = ("sinistro", "mes", "perda")
    quase_ids = ("data", "perda")
    dim = 1
    custo_inferencia = 1e-6

    cabecalho = {
        "processo_gerador": "Mundo (L1): sinistros reais de incêndio > 1 M DKK da Copenhagen Re, 1980–90; cada mês, a camada 100 xs 50 é "
                            "atingida se algum sinistro passa de 50 M DKK, e a fração do limite consumida segue a cauda das perdas. "
                            "A alegação falha no Modelo (L2): lê k/n = 0 numa janela de 36 meses como p = 0, e a ausência de evidência "
                            "como evidência de ausência (L3).",
        "nao_linearidade": "ln(1 + fπ − f·S) é côncava em f e tem barreira em f = 1/(1 − π) ≈ 1,015: com o limite cheio consumido, "
                           "o capital vai a zero. A cauda da perda é de potência (ξ ≈ 0,4–0,6, α ≈ 1,7–2,5): variância infinita ou quase; "
                           "reportar mediana, fração arruinada e intervalos por bootstrap, nunca só médias.",
        "unidade_e_margem": "Unidade independente: o mês (bloco de tempo), não o sinistro. Margens, bootstrap e previsões por mês; "
                            "os sinistros de um mês partilham capital, exposição e conjuntura.",
        "fisica_do_ruido": "Saltos raros e de magnitude de Pareto: em 108 meses de treino, 5 meses atingidos, um deles com perda de 263 M "
                           "(limite inteiro). A frequência de sinistros cresce ao longo da década (não estacionária). Estado absorvente: ruína.",
        "escada_causal": "Degrau 1 para a chance de perda (associação temporal, exógena ao ressegurador); degrau 2 para a exposição, uma "
                         "intervenção do(f) sobre o capital, simulada e reencenada nos meses reais.",
    }

    # ---------------- F7 ----------------
    def traducao(self):
        return {
            "dominio": "resseguro de propriedade: camadas de excesso de danos por risco (per-risk XL) sobre carteira industrial dinamarquesa",
            "pergunta_substantiva": "Três anos sem sinistro na camada 100 xs 50 são evidência suficiente de que o risco é desprezível e de que vale "
                                    "vender limite até 3 vezes o capital?",
            "unidade_independente": "o mês (bloco de tempo), não o sinistro isolado",
            "glossario": {"retenção (prioridade)": "valor da perda abaixo do qual a cedente paga sozinha (50 M DKK)",
                          "limite": "valor máximo que o ressegurador paga por sinistro na camada (100 M DKK)",
                          "camada 100 xs 50": "o ressegurador paga a parte da perda entre 50 e 150 M DKK",
                          "rate on line": "prêmio anual dividido pelo limite",
                          "burning cost": "perdas históricas da camada divididas pelo limite e pelo tempo: o preço pela experiência",
                          "cedente": "a seguradora primária que transfere o risco",
                          "exposição f": "limite vendido dividido pelo capital do ressegurador",
                          "ruína": "capital que chega a zero; estado absorvente",
                          "POT/GPD": "picos acima de um limiar modelados pela distribuição generalizada de Pareto",
                          "índice de cauda α": "expoente de potência da cauda, P(X > x) ~ x^(−α); ξ = 1/α"},
            "fontes_primarias": ["McNeil, A. J. (1997). Estimating the tails of loss severity distributions using extreme value theory. ASTIN Bulletin 27(1).",
                                 "Embrechts, P., Klüppelberg, C. & Mikosch, T. (1997). Modelling Extremal Events. Springer.",
                                 "Hill, B. M. (1975). A simple general approach to inference about the tail of a distribution. Annals of Statistics 3(5).",
                                 "Kelly, J. L. (1956). A new interpretation of information rate. Bell System Technical Journal 35(4).",
                                 "Mayo, D. (2018). Statistical Inference as Severe Testing. Cambridge University Press.",
                                 "Dados: CRAN fitdistrplus::danishuni (espelho GitHub), ver dados-reais/FONTES.md"],
        }

    def elicitacao(self):
        pc = mercado()["p_critico"]
        esp = [
            {"nome": "E1 (simulado: subscritor que vendeu a camada e viu três anos limpos)", "q05": 0.001, "q25": 0.003, "q50": 0.006, "q75": 0.010, "q95": 0.020},
            {"nome": "E2 (simulado: analista de risco da cedente, lembra de 1980–82)", "q05": 0.005, "q25": 0.012, "q50": 0.020, "q75": 0.030, "q95": 0.050},
            {"nome": "E3 (simulado: atuário que usa uma cauda de Pareto)", "q05": 0.008, "q25": 0.018, "q50": 0.030, "q75": 0.045, "q95": 0.080},
        ]
        cons = {k: float(np.mean([e[k] for e in esp])) for k in ("q05", "q25", "q50", "q75", "q95")}
        so_quartis = _beta_quantis({k: cons[k] for k in ("q25", "q50", "q75")}, pc)
        com_extremos = _beta_quantis(cons, pc)
        corrige = com_extremos["P_p_maior_igual_p_critico"] > so_quartis["P_p_maior_igual_p_critico"]
        return {
            "quantidade": "p, a probabilidade de a camada 100 xs 50 ser atingida num mês",
            "especialistas": esp,
            "protocolo": "SHELF v4: quartis individuais sem ancoragem, depois extremos q05/q95, discussão, consenso RIO (média dos quantis), "
                         "ajuste Beta, validação facial",
            "ancoragem": {
                "diagnostico": f"a mediana do consenso é {cons['q50']:.4f} por mês (um mês atingido a cada {1 / cons['q50'] / 12:.1f} anos); "
                               f"a frequência de treino é {5 / 108:.4f} e o p em que vender a camada deixa de compensar é {pc:.4f}",
                "palpite_unico_sem_shelf": {"p": cons["q50"], "kelly": kelly(cons["q50"])},
                "beta_so_quartis": so_quartis, "beta_com_extremos_q05_q95": com_extremos, "shelf_corrige": bool(corrige),
            },
            "nota": "ELICITAÇÃO SIMULADA: nenhum especialista humano foi consultado; o painel é um conjunto declarado de leituras típicas de "
                    "1988 (uma ancorada no triênio limpo, duas não). O agent-f7 usa o consenso dos quartis para a priori Beta.",
        }

    def priori_atipica(self):
        return {"quantidade": "p logo depois do incêndio de 263 M DKK (julho de 1980)", "especialistas": [
            {"nome": "E4 (simulado: subscritor em 1980, depois da maior perda da série)", "q25": 0.04, "q50": 0.07, "q75": 0.11}]}

    def termo_escopo(self):
        return {
            "vale_para": "camada 100 xs 50 por risco, sinistros de incêndio industrial dinamarqueses > 1 M DKK em valores de 1985; meses "
                         "independentes; prêmio de mercado = 1,2 × burning cost de 1980–88, ganho mês a mês; exposição até 3 × o capital",
            "nao_vale_para": "catástrofe com muitos riscos num evento (camada por evento), reinstalações, cláusula de índice, retrocessão, "
                             "inadimplência da própria cedente, outros países ou ramos, valores depois de 1990 sem nova correção",
            "regra_de_decisao": "exposição pela fração de Kelly com p estimado em todo o treino (ou Laplace na janela), nunca pelo k/n de uma "
                                "janela limpa; nunca f ≥ 1/(1 − π)",
            "circuit_breaker": f"se o limite inferior de 97,5% de p na janela recente de meses passar de {mercado()['p_critico']:.4f} "
                               "(onde f* = 0 e vender a camada deixa de compensar ao prêmio de mercado), parar de vender e reabrir a estimação",
            "assinatura": "sil (pendente)",
        }

    # ---------------- F2 ----------------
    def contrato(self):
        return {
            "nome": "sinistros_incendio_dinamarca", "versao": "1.0",
            "colunas": {
                "sinistro": {"tipo": "str"}, "periodo": {"tipo": "int", "min": 1, "max": 99},
                "data": {"tipo": "str"}, "mes": {"tipo": "str"}, "ano": {"tipo": "int", "min": 1980, "max": 1990},
                "perda": {"tipo": "float", "min": 1.0, "max": 10_000.0},
                "excesso_camada": {"tipo": "float", "min": 0.0, "max": LIMITE},
                "origem": {"tipo": "str", "valores": ORIGENS},
            },
            "chave": ["sinistro"], "particao": "periodo",
            "grao": "uma linha por sinistro real (> 1 M DKK); a unidade independente é o mês, agregado na leitura. "
                    "Linhas com origem ≠ 'real' são perturbações sintéticas declaradas (banca) ou reamostras de linhas reais",
        }

    @staticmethod
    def _linha(sid, periodo, data, perda, origem):
        return {"sinistro": sid, "periodo": int(periodo), "data": data, "mes": data[:7], "ano": int(data[:4]),
                "perda": float(perda), "excesso_camada": float(excesso_camada(perda)), "origem": origem}

    def _reais(self, periodo):
        a0, a1 = PERIODOS_ANOS[periodo]
        return [r for r in dados_reais() if a0 <= r[3] <= a1]

    def gerar_lote(self, periodo, rng, cenario="mundo"):
        base = self._reais(periodo if periodo in PERIODOS_ANOS else 3)
        if cenario == "deriva":
            # SINTÉTICO: as linhas reais do período (ou de 1986–88) com a severidade multiplicada por 3
            return [self._linha(f"S{periodo:02d}-D-{i:05d}", periodo, r[1], r[4] * FATOR_DERIVA, "sintetico_deriva") for i, r in enumerate(base)]
        if cenario == "patologico":
            # SINTÉTICO: linhas reais do período mais um surto de 8 meses seguidos, cada um com um sinistro da camada
            # sorteado entre os sinistros reais > 50 M DKK do treino (contágio que o modelo i.i.d. não prevê)
            meses = sorted({r[2] for r in base})
            ini = int(rng.integers(0, len(meses) - MESES_SURTO + 1))
            grandes = [r[4] for r in dados_reais() if r[4] > RETENCAO and r[3] <= PERIODOS_ANOS[3][1]]
            out = [self._linha(f"S{periodo:02d}-P-{i:05d}", periodo, r[1], r[4], "sintetico_patologico") for i, r in enumerate(base)]
            for j, m in enumerate(meses[ini:ini + MESES_SURTO]):
                out.append(self._linha(f"S{periodo:02d}-Q-{j:05d}", periodo, f"{m}-15", float(rng.choice(grandes)), "sintetico_patologico"))
            return out
        if periodo in PERIODOS_ANOS:
            # REAL: as linhas do CSV, sem sorteio
            return [self._linha(r[0], periodo, r[1], r[4], "real") for r in base]
        # período desconhecido (ex.: 99 da banca): REAMOSTRA com reposição de linhas reais de toda a série
        todas = dados_reais()
        idx = rng.integers(0, len(todas), 300)
        return [self._linha(f"R{periodo:02d}-{i:05d}", periodo, todas[j][1], todas[j][4], "reamostra_real") for i, j in enumerate(idx)]

    def generalizar(self, linha):
        x = float(linha["perda"])
        faixa = next(n for lo, hi, n in FAIXAS if lo <= x < hi)
        return {**linha, "data": str(linha["data"])[:4], "perda": faixa}

    # ---------------- modelo ----------------
    @staticmethod
    def _arr(dados, c):
        return np.asarray(dados[c])

    def _meses(self, dados):
        """Agrega por mês (a unidade): meses ordenados, perda máxima e fração do limite consumida."""
        mes = self._arr(dados, "mes").astype(str)
        x = self._arr(dados, "perda").astype(float)
        um, inv = np.unique(mes, return_inverse=True)
        mx = np.zeros(um.size)
        np.maximum.at(mx, inv, x)
        s = np.zeros(um.size)
        np.add.at(s, inv, excesso_camada(x) / LIMITE)
        return um, mx, s

    def suficientes(self, dados):
        _, mx, _ = self._meses(dados)
        return {"k": int(np.sum(mx > RETENCAO)), "n": int(mx.size)}

    def logpost(self, x, suf, priori):
        x = float(np.ravel(x)[0])
        lp, l1p = -np.logaddexp(0, -x), -np.logaddexp(0, x)
        return (suf["k"] + priori["a"]) * lp + (suf["n"] - suf["k"] + priori["b"]) * l1p   # inclui o jacobiano p(1−p)

    def amostra_priori(self, priori, rng):
        return np.array([special.logit(np.clip(rng.beta(priori["a"], priori["b"]), 1e-12, 1 - 1e-12))])

    def simular_suf(self, x, rng):
        n = mercado()["n_meses"]   # 108 meses, como o treino
        return {"k": int(rng.binomial(n, special.expit(x[0]))), "n": n}

    def quantidade(self, x):
        return float(special.expit(np.ravel(x)[0]))

    def x_inicial(self, suf):
        return np.array([special.logit((suf["k"] + 0.5) / (suf["n"] + 1))])

    def _cauda(self, dados, rng) -> dict:
        """Hill e POT/GPD com IC por bootstrap de meses (a unidade). Guardado por conjunto de dados."""
        x = self._arr(dados, "perda").astype(float)
        mes = self._arr(dados, "mes").astype(str)
        chave = (x.size, float(x.sum()))
        cache = self.__dict__.setdefault("_cache_cauda", {})
        if chave in cache:
            return cache[chave]
        hills = {k: hill_ic(x, k) for k in (25, 50, 100)}
        gp = ajustar_gpd(x)
        n_meses = np.unique(mes).size
        rr = np.random.default_rng(int(rng.integers(2**31)))
        bh = bootstrap_meses(x, mes, lambda v, n: estat.hill(v, 50), rr)

        def boot_gpd(v, n):
            a = ajustar_gpd(v)
            return [a["xi"], p_pot(a["xi"], a["sigma"], a["n_exc"] / n)]

        bg = bootstrap_meses(x, mes, boot_gpd, rr)
        res = {"hill": hills, "hill_k50_ic95_bootstrap_meses": np.quantile(bh, [0.025, 0.975]).tolist(),
               "gpd": gp, "gpd_xi_ic95_bootstrap_meses": np.quantile(bg[:, 0], [0.025, 0.975]).tolist(),
               "gpd_alpha_ic95_bootstrap_meses": (1 / np.quantile(bg[:, 0], [0.975, 0.025])).tolist(),
               "p_pot_mes": p_pot(gp["xi"], gp["sigma"], gp["n_exc"] / n_meses),
               "p_pot_ic95_bootstrap_meses": np.quantile(bg[:, 1], [0.025, 0.975]).tolist(),
               "n_boot": N_BOOT, "n_meses": int(n_meses), "n_sinistros": int(x.size)}
        cache[chave] = res
        return res

    def nulo(self, dados, rng):
        """Teste severo (Mayo): "a camada não foi tocada em 36 meses" teria acusado um mundo perigoso?"""
        um, mx, _ = self._meses(dados)
        hit = mx > RETENCAO
        k_jan, n_jan = int(hit[-JANELA:].sum()), JANELA
        cauda = self._cauda(dados, rng)
        p_per = cauda["p_pot_mes"]                       # "perigoso": a taxa que a própria cauda do treino implica
        pc = mercado()["p_critico"]
        p0_perigo = (1 - p_per) ** n_jan
        p0_desprez = (1 - P_DESPREZIVEL) ** n_jan
        rv = p0_desprez / p0_perigo
        meses_95 = int(np.ceil(np.log(0.05) / np.log(1 - p_per)))
        meses_95_crit = int(np.ceil(np.log(0.05) / np.log(1 - pc)))
        limpa_obs = maior_sequencia(~hit)
        sims = np.array([maior_sequencia(rng.random(hit.size) >= p_per) for _ in range(2000)])
        x = self._arr(dados, "perda").astype(float)
        anos = np.array([int(m[:4]) for m in um])
        sens = {}
        for r in (20.0, 25.0, 30.0, 40.0, 50.0):
            h = np.zeros(um.size, bool)
            mes = self._arr(dados, "mes").astype(str)
            for m in np.unique(mes[x > r]):
                h[np.searchsorted(um, m)] = True
            sens[f"retencao_{int(r)}"] = {f"periodo_{p}": int(h[(anos >= a0) & (anos <= a1)].sum()) for p, (a0, a1) in PERIODOS_ANOS.items() if p != 4}
        return {
            "estatistica": f"camada sem sinistro nos últimos {JANELA} meses do treino (a evidência da alegação)",
            "janela": {"meses": n_jan, "meses_atingidos": k_jan, "k_sobre_n": k_jan / n_jan, "laplace": laplace(k_jan, n_jan),
                       "kelly_kn": alavancagem_da_regra("kelly_kn", k_jan), "kelly_laplace": alavancagem_da_regra("kelly_laplace", k_jan)},
            "treino_inteiro": {"meses": int(hit.size), "meses_atingidos": int(hit.sum()), "frequencia": float(hit.mean()),
                               "laplace": laplace(int(hit.sum()), int(hit.size))},
            "cauda": cauda,
            "nulo_iid_p_pot": {"p_mes": p_per, "maior_sequencia_limpa_observada": limpa_obs,
                               "q05_q50_q95": np.quantile(sims, [0.05, 0.5, 0.95]).tolist(), "P_sequencia_maior_igual": float(np.mean(sims >= limpa_obs))},
            "severidade": {"p_perigoso_pot": p_per, "P_acusar_se_perigoso": 1 - p0_perigo, "P_passar_mesmo_perigoso": p0_perigo,
                           "razao_de_verossimilhanca_desprezivel_vs_perigoso": rv, "bits_de_evidencia": float(np.log2(rv)),
                           "meses_para_severidade_95": meses_95, "anos_para_severidade_95": meses_95 / 12,
                           "p_critico_decisao": pc, "meses_para_severidade_95_no_p_critico": meses_95_crit,
                           "anos_para_severidade_95_no_p_critico": meses_95_crit / 12, "limiar_severo": 0.95, "severo": bool(1 - p0_perigo >= 0.95)},
            "sensibilidade_retencao_meses_atingidos": sens,
            "leitura": (f"sob o p que a própria cauda do treino implica ({p_per:.3f} por mês), {p0_perigo:.1%} das janelas de {n_jan} meses "
                        f"não mostram sinistro na camada; o teste 'nunca tocou' acusaria o perigo com probabilidade {1 - p0_perigo:.0%} (< 95%). "
                        f"Uma janela limpa é {rv:.1f} vezes mais provável no mundo desprezível ({np.log2(rv):.1f} bits); seriam precisos "
                        f"{meses_95} meses (~{meses_95 / 12:.0f} anos) limpos para um teste severo"),
        }

    def ppc(self, dados, amostras_x, rng):
        """Checagem preditiva: maior sequência de meses consecutivos atingidos. O Bernoulli mensal i.i.d.
        quase nunca produz surtos longos; contágio (perdas em aglomerados) produz."""
        _, mx, _ = self._meses(dados)
        hit = mx > RETENCAO
        obs = maior_sequencia(hit)
        ps = special.expit(amostras_x[rng.integers(0, len(amostras_x), 400), 0])
        rep = np.array([maior_sequencia(rng.random(hit.size) < p) for p in ps])
        limpa = np.array([maior_sequencia(rng.random(hit.size) >= p) for p in ps])
        pv = float(min(1.0, 2 * min(np.mean(rep >= obs), np.mean(rep <= obs))))
        return {"estatistica": "maior sequência de meses consecutivos com sinistro na camada", "observado": obs, "n_meses": int(hit.size),
                "replicas_q025_q975": np.quantile(rep, [0.025, 0.975]).tolist(), "p_valor": pv,
                "maior_sequencia_limpa": {"observado": maior_sequencia(~hit), "replicas_q025_q975": np.quantile(limpa, [0.025, 0.975]).tolist(),
                                          "P_replica_maior_igual": float(np.mean(limpa >= maior_sequencia(~hit)))}}

    def resultado_unidade(self, dados):
        _, mx, _ = self._meses(dados)
        return np.log10(mx)          # log10 da maior perda do mês: o escore de cauda por unidade

    # ---------------- F3 ----------------
    def entradas_cofre(self, cofre):
        um, _, _ = self._meses(cofre)
        return {"ids": [str(m) for m in um], "deriva": np.log10(self._arr(cofre, "perda").astype(float))}

    def previsoes(self, treino, entradas, amostras_x):
        ps = special.expit(amostras_x[:: max(1, len(amostras_x) // 400), 0])
        _, mx, _ = self._meses(treino)
        hit = mx > RETENCAO
        base = float(hit.mean())
        k, n = int(hit[-JANELA:].sum()), JANELA
        p_mod = float(np.mean(ps))
        p_pot_ = mercado()["p_pot"]
        out = []
        for iid in entradas["ids"]:
            out.append({"id": iid, "evento": "camada 100 xs 50 atingida no mês (algum sinistro > 50 M DKK)", "taxa_base": base,
                        "probs": {"agent-f4-frontier": p_mod, "alegacao-janela-limpa": k / n, "laplace-janela": laplace(k, n), "pot-gpd-treino": p_pot_},
                        "meta": {"fonte_filtro": "todos os sinistros > 1 M DKK registrados pela Copenhagen Re em 1989–90 (série completa, sem filtro de sobrevivência)",
                                 "pode_influir": False, "flutuacao_ou_causa": "flutuação: incêndios raros, exógenos ao ressegurador",
                                 "n_fatores": 1, "degrau_pearl": 1}})
        return out

    def desfechos(self, cofre):
        um, mx, _ = self._meses(cofre)
        return {str(m): int(v > RETENCAO) for m, v in zip(um, mx)}

    # ---------------- F5 / F6 ----------------
    def estatistica_publicada(self, dados):
        s = self.suficientes(dados)
        return s["k"], s["n"], "fração de meses de 1980–88 com sinistro na camada 100 xs 50"

    def _k_anonimato_sinistros(self):
        """k-anonimato contado por sinistro (o que poderia reidentificar um segurado: data e tamanho de um grande incêndio)."""
        tr = [r for r in dados_reais() if r[3] <= PERIODOS_ANOS[3][1]]
        bruto = [{"data": r[1], "perda": round(r[4], 6)} for r in tr]
        gen = [self.generalizar({"data": r[1], "perda": r[4]}) for r in tr]
        gen_per = [{"periodo": next(p for p, (a0, a1) in PERIODOS_ANOS.items() if a0 <= r[3] <= a1), "perda": g_["perda"]} for r, g_ in zip(tr, gen)]
        gen_tri = [{"perda": g_["perda"]} for g_ in gen]
        return {"k_bruto_data_perda": estat.k_anonimato(bruto, ["data", "perda"]),
                "k_ano_faixa": estat.k_anonimato(gen, ["data", "perda"]),
                "k_periodo_faixa": estat.k_anonimato(gen_per, ["periodo", "perda"]),
                "k_so_faixa_1980_88": estat.k_anonimato(gen_tri, ["perda"]),
                "leitura": "não há pessoas nos dados; o que se pode reidentificar é um segurado industrial pelo seu grande incêndio (data e valor "
                           "saem em jornal). Os sinistros da camada só ficam com k ≥ 5 quando agregados em todo o treino; a estatística publicada "
                           "(contagem de meses atingidos) não traz tamanhos nem datas exatas"}

    def justica(self, ctx):
        tr = ctx["treino"]
        um, mx, s = self._meses(tr)
        anos = np.array([int(m[:4]) for m in um])
        x = self._arr(tr, "perda").astype(float)
        ano_sin = np.array([int(str(m)[:4]) for m in self._arr(tr, "mes")])
        por_ano = {str(a): {"sinistros": int(np.sum(ano_sin == a)), "meses_atingidos": int(np.sum((mx > RETENCAO) & (anos == a))),
                            "fracao_do_limite_consumida": float(s[anos == a].sum())} for a in np.unique(anos)}
        exc = excesso_camada(x)
        por_faixa = {}
        for lo, hi, nome in FAIXAS:
            sel = (x >= lo) & (x < hi)
            por_faixa[nome] = {"sinistros": int(sel.sum()), "fatia_da_perda_total": float(x[sel].sum() / x.sum()),
                               "fatia_da_perda_da_camada": float(exc[sel].sum() / max(exc.sum(), 1e-12))}
        f_kn = alavancagem_da_regra("kelly_kn", int(np.sum(mx[-JANELA:] > RETENCAO)))
        return {
            "trilema_da_justica": "não se aplica: não há classificador, pessoas nem grupos protegidos nos dados; a decisão (exposição) é do "
                                  "ressegurador. Os 'grupos' declarados são só anos e faixas de tamanho de sinistro, que existem nos dados.",
            "por_ano": por_ano,
            "por_faixa_de_sinistro": por_faixa,
            "governanca_real": {**self._k_anonimato_sinistros(),
                                "nota_framework": "o agent-f5 conta k-anonimato sobre uma linha por unidade (mês), não por sinistro; o cálculo por sinistro está aqui. "
                                                  "O dossiê do F5 marca dados_simulados = True por padrão do framework: aqui os dados são REAIS (só a elicitação é simulada)"},
            "quem_paga_o_erro": {"faixa_que_absorve_a_inadimplencia": "≥50 (camada)",
                                 "leitura": f"se o ressegurador vende {f_kn:.1f} × o capital em limite e é arruinado, quem fica sem receber são as cedentes "
                                            "dos maiores incêndios (faixa ≥ 50 M DKK): os sinistros pequenos nunca dependeram da camada"},
            "inversao_metrica": {"metrica_original": "meses sem sinistro na camada / lucro de prêmio no triênio (premia vender limite no teto)",
                                 "metrica_invertida": "fração arruinada e crescimento mediano do capital (premia Kelly com Laplace ou com o treino inteiro)",
                                 "ganham": "as cedentes e os segurados dos maiores incêndios (faixa ≥ 50 M DKK), que passam a ter o sinistro pago em vez de "
                                           "um ressegurador insolvente",
                                 "perdem": "os acionistas e subscritores que recebiam prêmio sobre 3 × o capital em limite nos anos limpos, e as cedentes "
                                           "que compravam capacidade barata"},
            "assimetria_de_poder": "o subscritor recebe bônus sobre os anos sem sinistro e não devolve nada na ruína (opção grátis); a cedente "
                                   "vive a trajetória inteira e só descobre a insolvência quando o grande incêndio chega",
        }

    # ---------------- M15 ----------------
    def _morfonomia(self, tr):
        um, mx, _ = self._meses(tr)
        hit = mx > RETENCAO
        anos = np.array([int(m[:4]) for m in um])
        per = np.array([next(p for p, (a0, a1) in PERIODOS_ANOS.items() if a0 <= a <= a1) for a in anos])
        tab = np.array([[hit[per == q].sum(), (~hit[per == q]).sum()] for q in np.unique(per)])
        p_icp = float(stats.fisher_exact(tab[[0, 2]])[1]) if tab.shape[0] >= 3 else float("nan")
        _, p_chi, _, _ = stats.chi2_contingency(tab)
        k = np.array([hit[anos == a].sum() for a in np.unique(anos)])
        n = np.array([(anos == a).sum() for a in np.unique(anos)])
        ll_bin = float(stats.binom.logpmf(k, n, k.sum() / n.sum()).sum())

        def nll(th):
            mu, phi = special.expit(th[0]), np.exp(th[1])
            return -stats.betabinom.logpmf(k, n, mu * phi, (1 - mu) * phi).sum()

        r = optimize.minimize(nll, x0=[special.logit(max(k.sum(), 0.5) / n.sum()), np.log(1e3)], method="Nelder-Mead")
        N = k.size
        # frequência de sinistros (todos > 1 M DKK) por mês: Poisson com tendência log-linear
        mes_sin = self._arr(tr, "mes").astype(str)
        cont = np.array([np.sum(mes_sin == m) for m in um])
        t = np.arange(um.size) / 12.0
        nllp = lambda b: -stats.poisson.logpmf(cont, np.exp(b[0] + b[1] * t)).sum()
        rp = optimize.minimize(nllp, x0=[np.log(cont.mean()), 0.0], method="Nelder-Mead")
        r0 = -stats.poisson.logpmf(cont, cont.mean()).sum()
        return {"homogeneidade_entre_periodos_qui2_p": float(p_chi), "fisher_periodo1_vs_periodo3_p": p_icp, "invariante": bool(p_chi > 0.01),
                "meses_atingidos_por_periodo": {str(q): int(tab[i, 0]) for i, q in enumerate(np.unique(per))},
                "bic_binomial_anual": float(-2 * ll_bin + np.log(N)), "bic_beta_binomial_anual": float(2 * r.fun + 2 * np.log(N)),
                "mdl_prefere": "binomial (p comum)" if -2 * ll_bin + np.log(N) <= 2 * r.fun + 2 * np.log(N) else "beta-binomial (p heterogêneo)",
                "tendencia_frequencia_sinistros": {"crescimento_anual": float(np.exp(rp.x[1]) - 1), "razao_verossimilhanca_p": float(stats.chi2.sf(2 * (r0 - rp.fun), 1)),
                                                    "leitura": "a contagem mensal de sinistros > 1 M DKK cresce ao longo da década: o mundo não é estacionário"}}

    def _replay_real(self) -> dict:
        """Reencenação em meses reais (pós-cofre: usa 1989–90, já aberto pelo F3). Cada regra escolhe f no fim de 1988."""
        reais = dados_reais()
        mes = np.array([r[2] for r in reais])
        x = np.array([r[4] for r in reais])
        um = np.array(sorted(set(mes)))
        s = np.array([excesso_camada(x[mes == m]).sum() / LIMITE for m in um])
        hit = np.array([x[mes == m].max() > RETENCAO for m in um])
        i89 = int(np.searchsorted(um, "1989-01"))
        k_jan = int(hit[i89 - JANELA:i89].sum())
        p_post = mercado()["p_pot"]
        regras = {"Kelly com k/n da janela": alavancagem_da_regra("kelly_kn", k_jan), "meio Kelly com k/n": alavancagem_da_regra("meio_kelly_kn", k_jan),
                  "Kelly com Laplace": alavancagem_da_regra("kelly_laplace", k_jan), "Kelly com treino inteiro": kelly(hit[:i89].mean()),
                  "Kelly com POT/GPD": kelly(p_post)}
        cofre = {}
        for nome, f in regras.items():
            lnw = float(ln_riqueza(f, s[i89:]))
            cofre[nome] = {"f": f, "ln_capital_1989_90": lnw, "arruinado": bool(np.isinf(lnw)),
                           "capital_final": float(np.exp(lnw)) if np.isfinite(lnw) else 0.0}
        # janelas móveis em toda a série: decisões tomadas sempre que os últimos 36 meses estavam limpos
        decis, ruinas = 0, 0
        for t in range(JANELA, um.size - 24 + 1):
            if hit[t - JANELA:t].sum() == 0:
                decis += 1
                ruinas += int(np.isinf(ln_riqueza(TETO, s[t:t + 24])))
        return {"fim_de_1988": cofre, "meses_cofre_atingidos": [str(m) for m in um[i89:][hit[i89:]]],
                "fracao_do_limite_nos_meses_atingidos": s[i89:][hit[i89:]].tolist(),
                "janelas_moveis_limpas_36m": {"decisoes": decis, "arruinadas_em_24m_com_f_teto": ruinas}}

    def lentes(self, ctx):
        q = ctx["f4"]["_q"]
        p = float(np.median(q))
        tr = ctx["treino"]
        mk = mercado()
        nul = ctx["f1"]["modelo_nulo"]
        cauda = nul["cauda"]
        replay = self._replay_real()
        x = self._arr(tr, "perda").astype(float)
        us = np.array([5, 10, 15, 20, 30])
        me = [{"u": float(u), "excesso_medio": float(np.mean(x[x > u] - u)), "n": int(np.sum(x > u))} for u in us]
        todas = np.array([r[4] for r in dados_reais()])
        p_jan = nul["janela"]
        # Fluxonomia: informação que uma janela limpa carrega sobre "perigoso (p_POT)" vs "desprezível" (50/50)
        pl = np.array([(1 - mk["p_pot"]) ** JANELA, (1 - P_DESPREZIVEL) ** JANELA])
        pj = np.array([[0.5 * pl[0], 0.5 * (1 - pl[0])], [0.5 * pl[1], 0.5 * (1 - pl[1])]])
        px = pj.sum(0)
        mi = float(np.sum(pj * np.log2(pj / (0.5 * px[None, :]))))
        post = ctx["priori"]["a"] + ctx["suf"]["k"], ctx["priori"]["b"] + ctx["suf"]["n"] - ctx["suf"]["k"]
        evsi = estat.evsi_beta_bernoulli(post[0], post[1], JANELA, self.utilidade, self.acoes(), np.random.default_rng(ctx["semente"] + 4), m=2000)
        return {
            "Crononomia": {"f_estrela_posterior": kelly(p), "g_estrela_posterior_mes": float(g(kelly(p), p)),
                           "premio_mes": mk["premio_mes"], "rol_anual": mk["rol_anual"], "barreira_f": mk["barreira"], "p_critico": mk["p_critico"],
                           "ln_E_fator_f_teto_sob_p_janela_0": float(np.log1p(TETO * mk["premio_mes"])),
                           "E_ln_fator_f_teto_sob_p_posterior": float(g(TETO, p)),
                           "replay_real_pos_cofre": replay,
                           "ruina_absorvente": "com f ≥ 1/(1 − π), um sinistro que consome o limite inteiro leva o capital a zero e dali não sai",
                           "ergodico": False,
                           "leitura": "Kelly maximiza a taxa temporal; quem lê a janela limpa como p = 0 vai ao teto e é arruinado pelo primeiro sinistro "
                                      "de limite cheio (agosto de 1989, 152 M DKK)"},
            "Escalonomia": {"hill_treino": cauda["hill"], "hill_k50_ic95_bootstrap_meses": cauda["hill_k50_ic95_bootstrap_meses"],
                            "gpd_treino": cauda["gpd"], "gpd_xi_ic95_bootstrap_meses": cauda["gpd_xi_ic95_bootstrap_meses"],
                            "gpd_alpha_ic95_bootstrap_meses": cauda["gpd_alpha_ic95_bootstrap_meses"],
                            "pos_cofre_serie_inteira": {"hill_k50": estat.hill(todas, 50), "gpd": ajustar_gpd(todas)},
                            "excesso_medio_por_limiar": me, "severidade_camada": {"P_limite_cheio_dado_atingida": mk["sev"]["p_limite_cheio"], "E_s": mk["sev"]["E_s"]},
                            "leitura": (f"Hill (k = 50) dá α = {cauda['hill'][50]['alpha']:.2f}, IC por meses {cauda['hill_k50_ic95_bootstrap_meses'][0]:.2f}–"
                                        f"{cauda['hill_k50_ic95_bootstrap_meses'][1]:.2f}; a GPD dá ξ = {cauda['gpd']['xi']:.2f} (α = {cauda['gpd']['alpha']:.2f}), IC de ξ "
                                        f"{cauda['gpd_xi_ic95_bootstrap_meses'][0]:.2f}–{cauda['gpd_xi_ic95_bootstrap_meses'][1]:.2f}. Hill é mais pesado no ponto; nove anos "
                                        "não bastam para fixar ξ (o IC quase toca a cauda exponencial), mas o excesso médio cresce com o limiar (assinatura de "
                                        "Pareto) e a série inteira, pós-cofre, empurra ξ para cima")},
            "Fluxonomia": {"informacao_mutua_janela_limpa_bits": mi, "laco_performativo": False,
                           "leitura": f"uma janela de {JANELA} meses sem sinistro na camada carrega só {mi:.2f} bits sobre 'perigoso vs desprezível'; "
                                      f"a regra de Laplace na janela dá p = {p_jan['laplace']:.3f}, não zero"},
            "Morfonomia": self._morfonomia(tr),
            "Crisonomia": {"placar": ctx["f3"]["placar"], "evpi_posterior_por_mes": estat.evpi(q[::10], self.utilidade, self.acoes())["evpi"],
                           "evsi_mais_36_meses": evsi,
                           "leitura": "o valor da informação estava em usar a série inteira (e a cauda abaixo da retenção), não em esperar a própria camada"},
        }

    def critica(self, ctx):
        tr = ctx["treino"]
        mf = self._morfonomia(tr)
        mk = mercado()
        # sensibilidade: a mesma previsão constante com priori plana (Beta(1, 1)) e com a taxa POT
        s = ctx["suf"]
        return {"rival": {"modelo": "POT/GPD (usa a cauda abaixo da retenção) e beta-binomial anual (p heterogêneo entre anos)",
                          "p_pot_treino": mk["p_pot"], "p_bernoulli_treino": s["k"] / s["n"], "p_posterior_priori_plana": (s["k"] + 1) / (s["n"] + 2),
                          **{k: v for k, v in mf.items() if k.startswith(("bic", "mdl"))}},
                "performatividade": "nenhuma na chance de incêndio; no mercado, a capacidade barata vendida por quem lê janelas limpas derruba o "
                                    "preço da camada para todos (ciclo de subscrição)",
                "premissas": ["prêmio = 1,2 × burning cost de 1980–88 (premissa de mercado declarada, não estimada)",
                              "severidade da camada pela GPD do treino com limiar 10 (McNeil 1997); ξ incerto (ver IC por bootstrap de meses)",
                              "meses independentes e p constante, embora a frequência de sinistros cresça na década",
                              "uma camada por risco, sem reinstalação nem retrocessão; valores em DKK de 1985",
                              "a retenção de 50 foi escolhida por quem conhecia a série inteira (declarado no cabeçalho do módulo)"]}

    # ---------------- F8 ----------------
    def acoes(self):
        return [0.0, 0.2, 0.4, 0.8, 1.5, 3.0]

    def utilidade(self, acao, theta):
        return float(g(acao, theta))

    def decisao(self, ctx):
        q = ctx["f4"]["_q"]
        fs = np.linspace(0, TETO, 601)
        eg = np.array([np.mean(g(f, q[::20])) for f in fs])
        f_star = float(fs[np.argmax(eg)])
        mk = mercado()
        rng = np.random.default_rng(ctx["semente"] + 8)
        n = 10_000
        p_draw = rng.choice(q, n)
        k_hist = rng.binomial(JANELA, p_draw)
        atinge = rng.random((n, FUT)) < p_draw[:, None]
        sev = mk["sev"]
        s = np.where(atinge, rng.choice(sev["s"], (n, FUT), p=sev["w"] / sev["w"].sum()), 0.0)
        regras = {"Kelly posterior": np.full(n, f_star), "meio Kelly posterior": np.full(n, f_star / 2),
                  "Kelly com k/n da janela": np.array([alavancagem_da_regra("kelly_kn", int(k)) for k in k_hist]),
                  "meio Kelly com k/n": np.array([alavancagem_da_regra("meio_kelly_kn", int(k)) for k in k_hist]),
                  "Kelly com Laplace": np.array([alavancagem_da_regra("kelly_laplace", int(k)) for k in k_hist])}
        res, hops = {}, {}
        for nome, f in regras.items():
            lnw = ln_riqueza(f, s)
            w = np.exp(lnw)
            res[nome] = {"f_medio": float(f.mean()), "fracao_arruinada": float(np.mean(np.isinf(lnw))), "mediana_lnW_por_mes": float(np.median(lnw) / FUT),
                         "media_W": float(w.mean()), "mediana_W": float(np.median(w)), "de_100_arruinados": round(100 * float(np.mean(np.isinf(lnw)))),
                         "de_100_crescem": round(100 * float(np.mean(lnw > 0)))}
            hops[nome] = np.where(np.isinf(lnw), -5.0, lnw / np.log(10))
        p_E = res["Kelly com k/n da janela"]["fracao_arruinada"]
        C = max(res["Kelly posterior"]["mediana_lnW_por_mes"] - res["Kelly com Laplace"]["mediana_lnW_por_mes"], 0.0) * FUT
        L = -LN_PISO
        replay = self._replay_real()
        return {
            "f_kelly_posterior": f_star,
            "regras_120_meses": res,
            "replay_real_1989_90": replay["fim_de_1988"],
            "arvore_CL": {"evento": "ruína (capital a zero) em 120 meses", "acao_protetora": "estimar p pela série inteira ou por Laplace e nunca f ≥ 1/(1 − π)",
                          "P_evento_kelly_kn": p_E, "P_evento_kelly_laplace": res["Kelly com Laplace"]["fracao_arruinada"],
                          "C_custo_protecao_ln": C, "L_perda_ruina_ln": L, "C_sobre_L": C / L,
                          "regra": "proteger se P(evento) > C/L", "proteger": bool(p_E > C / L),
                          "limiar_robusto": f"a decisão só se inverteria se C/L > {p_E:.2f}"},
            "frequencias_naturais": (f"De cada 100 resseguradores que dimensionam a camada pelo k/n dos últimos 36 meses, cerca de "
                                     f"{round(100 * float(np.mean(k_hist == 0)))} veem a janela limpa e vendem 3 × o capital; "
                                     f"{res['Kelly com k/n da janela']['de_100_arruinados']} de cada 100 terminam arruinados em dez anos. Com Laplace, "
                                     f"{res['Kelly com Laplace']['de_100_arruinados']}; com o Kelly posterior, {res['Kelly posterior']['de_100_arruinados']}. "
                                     f"Nos dados reais, quem vendeu 3 × no fim de 1988 foi arruinado em agosto de 1989."),
            "classe_de_referencia": "camadas per-risk de incêndio industrial com retenção ≈ percentil 99,7 dos sinistros, dimensionadas por 36 meses de histórico",
            "circuit_breaker": self.termo_escopo()["circuit_breaker"],
            "hops_titulo": "Capital após 120 meses", "hops_rotulo": "log10(capital final / inicial); ruína = −5", "_hops": hops,
        }

    def disjuntor(self, dados, ctx):
        k = self.suficientes(dados)
        inf = float(stats.beta.ppf(0.025, 1 + k["k"], 1 + k["n"] - k["k"]))
        pc = mercado()["p_critico"]
        disp = inf > pc
        return {"disparou": bool(disp), "p_inf_025": inf, "limiar": pc, "meses_na_janela": k["n"], "meses_atingidos": k["k"],
                "motivo": "a chance mensal de atingir a camada passou do ponto em que vendê-la compensa: parar e reestimar" if disp else "dentro do escopo"}
