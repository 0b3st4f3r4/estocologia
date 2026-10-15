"""Projeto C · A cauda que Kelly não viu (Escalonomia + Crononomia, M8/M9; Lab 9, Parte 6).

Uma estratégia que vende seguro ganha 1% por semana com probabilidade 0,996 e perde 50% com
probabilidade 0,004. Com alavancagem f ≥ 2, uma única perda zera o fundo (estado absorvente).
A alegação lê um histórico de 250 semanas sem perdas como prova de que o risco é desprezível.
"""

from __future__ import annotations

from functools import lru_cache

import numpy as np
from scipy import optimize, special, stats

from estocologia import estat
from estocologia.projeto import Projeto

P_VERDADEIRO = 0.004          # chance de perda por semana no mundo
GANHO, PERDA = 0.01, -0.5     # retorno semanal: +1% ou −50%
TETO = 3.0                    # teto de alavancagem do banco
HIST = 250                    # semanas de histórico (cinco anos)
FUT = 1000                    # semanas futuras observadas depois da escolha de f (Exercício 6c)
N_FUNDOS = 2000               # fundos por coorte (período)
P_DERIVA = 0.02               # cenário de deriva: a chance de perda quintuplica
P_DESPREZIVEL = 0.0004        # o "risco desprezível" da alegação: uma perda a cada ~48 anos
LN_PISO = np.log(1e-300)      # piso numérico para ln(0), o mesmo do notebook do Lab 9
G_RUINA = float(LN_PISO / FUT)   # taxa de crescimento realizada atribuída a um fundo arruinado

TIPOS = ["independente", "multimercado", "banco", "seguradora"]
REGRAS = ["kelly_kn", "meio_kelly_kn", "kelly_laplace"]
# como cada tipo de gestora escolhe a alavancagem a partir do próprio histórico
REGRA_POR_TIPO = {"independente": [0.8, 0.1, 0.1], "multimercado": [0.5, 0.3, 0.2],
                  "banco": [0.1, 0.6, 0.3], "seguradora": [0.1, 0.2, 0.7]}
UFS = ["AC", "AL", "AM", "AP", "BA", "CE", "DF", "ES", "GO", "MA", "MG", "MS", "MT", "PA", "PB", "PE",
       "PI", "PR", "RJ", "RN", "RO", "RR", "RS", "SC", "SE", "SP", "TO"]
REGIAO = {**{u: "N" for u in ("AC", "AM", "AP", "PA", "RO", "RR", "TO")}, **{u: "NE" for u in ("AL", "BA", "CE", "MA", "PB", "PE", "PI", "RN", "SE")},
          **{u: "CO" for u in ("DF", "GO", "MS", "MT")}, **{u: "SE" for u in ("ES", "MG", "RJ", "SP")}, **{u: "S" for u in ("PR", "RS", "SC")}}
ANO_MIN, ANO_MAX = 1995, 2020
F_SEM_ALAVANCAGEM = 1.0
P_DISJUNTOR = (2 - F_SEM_ALAVANCAGEM) / 102   # p em que f* = 1: alavancar deixa de compensar


# ---------------- o mundo, em funções puras (testadas no Hypothesis) ----------------
def g(f, p):
    """Taxa de crescimento temporal por semana, E[ln(1 + f R)], com o piso de ln(0) do Lab 9."""
    f = np.asarray(f, dtype=float)
    return (1 - p) * np.log1p(f * GANHO) + p * np.log(np.maximum(1 + f * PERDA, 1e-300))


def kelly(p):
    """f* = argmax g. Fechado: g'(f) = 0 ⇒ f* = 2 − 102 p (para p > 0), truncado em [0, TETO].
    Com p = 0, g cresce sem limite em f, e quem acredita em p̂ = 0 alavanca até o teto."""
    if p <= 0:
        return TETO
    return float(np.clip(2 - 102 * p, 0.0, TETO))


def laplace(k, n):
    """Regra de sucessão de Laplace: nunca dá probabilidade zero ao que ainda não aconteceu."""
    return (k + 1) / (n + 2)


@lru_cache(maxsize=None)
def alavancagem_da_regra(regra: str, k: int, n: int = HIST) -> float:
    if regra == "kelly_kn":
        return kelly(k / n)
    if regra == "meio_kelly_kn":
        return kelly(k / n) / 2
    if regra == "kelly_laplace":
        return kelly(laplace(k, n))
    raise ValueError(regra)


def ln_riqueza(f, ganhos, perdas):
    """ln da riqueza final (riqueza inicial 1). Ruína (−∞) se e só se f·PERDA ≤ −1 e houve perda:
    a riqueza que chega a zero ou abaixo fica em zero (estado absorvente)."""
    f, ganhos, perdas = np.broadcast_arrays(np.asarray(f, float), np.asarray(ganhos, float), np.asarray(perdas, float))
    fator_perda = 1 + f * PERDA
    arruinado = (fator_perda <= 0) & (perdas > 0)
    seguro = np.where(fator_perda > 0, fator_perda, 1.0)
    lnw = ganhos * np.log1p(f * GANHO) + perdas * np.log(seguro)
    return np.where(arruinado, -np.inf, lnw)


def arruina(f, perdas):
    return (1 + np.asarray(f, float) * PERDA <= 0) & (np.asarray(perdas) > 0)


def simular_regras(p, k_hist, perdas_fut, fut=FUT) -> dict:
    """Exercício 6c: cada investidor vê o próprio histórico, escolhe f̂ e investe por `fut` semanas."""
    f_star = kelly(float(np.median(p)) if np.ndim(p) else p)
    regras = {
        "Kelly verdadeiro": np.full(k_hist.size, f_star),
        "meio Kelly verdadeiro": np.full(k_hist.size, f_star / 2),
        "Kelly com k/n": np.array([alavancagem_da_regra("kelly_kn", int(k)) for k in k_hist]),
        "meio Kelly com k/n": np.array([alavancagem_da_regra("meio_kelly_kn", int(k)) for k in k_hist]),
        "Kelly com Laplace": np.array([alavancagem_da_regra("kelly_laplace", int(k)) for k in k_hist]),
    }
    out = {}
    for nome, f in regras.items():
        lnw = ln_riqueza(f, fut - perdas_fut, perdas_fut)
        w = np.exp(lnw)
        out[nome] = {"_lnw": lnw, "f_medio": float(f.mean()), "fracao_arruinada": float(np.mean(np.isinf(lnw))),
                     "mediana_lnW_por_semana": float(np.median(lnw) / fut), "media_W": float(w.mean()),
                     "mediana_W": float(np.median(w)), "de_100_arruinados": round(100 * float(np.mean(np.isinf(lnw))))}
    return out


@lru_cache(maxsize=1)
def conferencia_lab9() -> dict:
    """Refaz o Exercício 6 do Lab 9 com a mesma semente (1814) e a mesma ordem de sorteios."""
    rng = np.random.default_rng(1814)
    f_star = kelly(P_VERDADEIRO)
    n_inv = 10_000
    k_hist = rng.binomial(HIST, P_VERDADEIRO, n_inv)
    perdas = rng.random((n_inv, FUT)) < P_VERDADEIRO
    sim = simular_regras(P_VERDADEIRO, k_hist, perdas.sum(1))
    tabela_b = [{"k": k, "P_k": float(stats.binom.pmf(k, HIST, P_VERDADEIRO)), "f_kn": kelly(k / HIST), "f_laplace": kelly(laplace(k, HIST))} for k in range(5)]
    return {
        "f_estrela": round(f_star, 4), "g_estrela_por_semana": float(g(f_star, P_VERDADEIRO)), "g_meio_kelly": float(g(f_star / 2, P_VERDADEIRO)),
        "P_nenhuma_perda_250": float((1 - P_VERDADEIRO) ** HIST),
        "esperado_lab": {"f_estrela": 1.592, "g_estrela": 0.00937, "P_nenhuma_perda_250": 0.367},
        "confere": bool(abs(f_star - 1.592) < 5e-4 and abs(g(f_star, P_VERDADEIRO) - 0.00937) < 5e-6 and abs((1 - P_VERDADEIRO) ** HIST - 0.367) < 5e-4),
        "tabela_6b": tabela_b,
        "simulacao_6c": {k: {kk: vv for kk, vv in v.items() if not kk.startswith("_")} for k, v in sim.items()},
        "historicos_sem_perda": float(np.mean(k_hist == 0)), "futuros_com_perda": float(np.mean(perdas.sum(1) > 0)),
    }


def _beta_quantis(qs: dict) -> dict:
    """Ajuste Beta por mínimos quadrados em quantos quantis houver (SHELF com extremos)."""
    probs = np.array([float(k[1:]) / 100 for k in qs])
    alvo = np.array(list(qs.values()))

    def perda(lab):
        a, b = np.exp(lab)
        return np.sum((np.log(stats.beta.ppf(probs, a, b)) - np.log(alvo)) ** 2)

    r = optimize.minimize(perda, x0=np.log([1.0, 1000.0]), method="Nelder-Mead", options={"xatol": 1e-10, "fatol": 1e-14, "maxiter": 5000})
    a, b = np.exp(r.x)
    return {"a": float(a), "b": float(b), "q95": float(stats.beta.ppf(0.95, a, b)), "media": float(a / (a + b)),
            "P_p_maior_igual_0004": float(stats.beta.sf(P_VERDADEIRO, a, b))}


class ProjetoC(Projeto):
    nome = "c"
    titulo = "A cauda que Kelly não viu: vender seguro com histórico limpo"
    alegacao = ('"Este fundo nunca perdeu em 250 semanas (cinco anos); logo, o risco de perda é desprezível '
                'e vale alavancar no máximo."')
    unidade = "fundo"
    colunas_modelo = ("fundo", "regra", "semanas_hist", "perdas_hist", "alavancagem", "semanas_fut", "perdas_fut")
    quase_ids = ("tipo_gestora", "uf", "ano_fundacao")
    dim = 1
    custo_inferencia = 1e-6

    cabecalho = {
        "processo_gerador": "Mundo (L1): cada semana, +1% com probabilidade 0,996 ou −50% com 0,004, i.i.d.; o fundo aplica a "
                            "alavancagem f escolhida a partir do próprio histórico. A alegação falha no Modelo (L2): lê a frequência "
                            "observada k/n = 0 como p = 0, e a ausência de evidência como evidência de ausência (L3).",
        "nao_linearidade": "ln(1 + fR) é côncava e tem uma barreira em f = 2: com f·0,5 ≥ 1, uma única perda leva a riqueza a zero. "
                           "g(f) = 0,996 ln(1 + 0,01f) + 0,004 ln(1 − 0,5f) tem máximo em f* = 2 − 102p = 1,592. "
                           "Riqueza final é multiplicativa e de cauda pesada: reportar mediana, fração arruinada e eCDF, nunca só a média.",
        "unidade_e_margem": "Unidade independente: o fundo (o histórico de um gestor), não a semana. Margens por fundo; "
                            "as semanas de um fundo partilham a mesma riqueza acumulada e a mesma alavancagem.",
        "fisica_do_ruido": "Ruído de saltos raros (Bernoulli com p = 0,004), não gaussiano: em 250 semanas, 37% dos fundos não veem "
                           "salto nenhum. Estado absorvente: ruína. Agente passivo: a alavancagem não altera a chance de perda.",
        "escada_causal": "Degrau 2: a alavancagem é uma intervenção do(f) sobre a riqueza, simulada diretamente; a chance de perda é exógena. "
                         "A escolha de f depende do histórico (seleção pelo que se viu), não de um confundidor da perda.",
    }

    # ---------------- F7 ----------------
    def traducao(self):
        return {
            "dominio": "gestão de fundos; estratégias que vendem proteção (seguro, opções fora do dinheiro, carry)",
            "pergunta_substantiva": "Um histórico de cinco anos sem perdas é evidência suficiente de que o risco é desprezível e de que vale alavancar ao teto?",
            "unidade_independente": "o fundo (o histórico de um gestor), não a semana isolada",
            "glossario": {"histórico limpo": "nenhuma semana de perda no período observado",
                          "alavancagem": "fração f da riqueza exposta à estratégia; f > 1 é tomar emprestado",
                          "fração de Kelly": "f que maximiza a taxa de crescimento E[ln(1 + fR)]",
                          "ruína": "riqueza que chega a zero ou abaixo; estado absorvente",
                          "evento de cauda": "a perda de 50%, rara (uma vez a cada 250 semanas em média)",
                          "regra de sucessão": "p̂ = (k + 1)/(n + 2), de Laplace, que nunca estima zero",
                          "drawdown": "queda em relação ao máximo anterior",
                          "cotista": "quem aplica no fundo e arca com a perda",
                          "taxa de performance": "remuneração do gestor proporcional ao ganho acima de um índice",
                          "coorte": "conjunto de fundos que começam no mesmo período"},
            "fontes_primarias": ["Kelly, J. L. (1956). A new interpretation of information rate. Bell System Technical Journal 35(4).",
                                 "Taleb, N. N. (2020). Statistical Consequences of Fat Tails. STEM Academic Press.",
                                 "Laplace, P.-S. (1814). Essai philosophique sur les probabilités.",
                                 "Mayo, D. (2018). Statistical Inference as Severe Testing. Cambridge University Press.",
                                 "Curso, Lab 9, Parte 6 (A cauda que Kelly não viu)"],
        }

    def elicitacao(self):
        esp = [
            {"nome": "E1 (simulado: gestor de fundo de venda de volatilidade, nunca viu perda)", "q05": 0.00005, "q25": 0.0002, "q50": 0.0005, "q75": 0.0010, "q95": 0.0030},
            {"nome": "E2 (simulado: analista de risco de corretora)", "q05": 0.0001, "q25": 0.0004, "q50": 0.0010, "q75": 0.0020, "q95": 0.0060},
            {"nome": "E3 (simulado: atuário de resseguro)", "q05": 0.0002, "q25": 0.0005, "q50": 0.0012, "q75": 0.0025, "q95": 0.0080},
        ]
        cons = {k: float(np.mean([e[k] for e in esp])) for k in ("q05", "q25", "q50", "q75", "q95")}
        so_quartis = _beta_quantis({k: cons[k] for k in ("q25", "q50", "q75")})
        com_extremos = _beta_quantis(cons)
        palpite = cons["q50"]
        corrige = com_extremos["P_p_maior_igual_0004"] > so_quartis["P_p_maior_igual_0004"]
        return {
            "quantidade": "p, a probabilidade de perda de 50% numa semana",
            "especialistas": esp,
            "protocolo": "SHELF v4: quartis individuais sem ancoragem, depois extremos q05/q95 ('que valor o surpreenderia?'), "
                         "discussão, consenso RIO (média dos quantis), ajuste Beta, validação facial",
            "ancoragem": {
                "diagnostico": "especialistas que nunca viram uma perda ancoram baixo: a mediana do consenso é "
                               f"{palpite:.4f}, {P_VERDADEIRO / palpite:.1f} vezes menor que o p do mundo (0,004)",
                "palpite_unico_sem_shelf": {"p": palpite, "kelly": kelly(palpite)},
                "beta_so_quartis": so_quartis, "beta_com_extremos_q05_q95": com_extremos,
                "shelf_corrige": bool(corrige),
                "leitura": (f"perguntar pelos extremos eleva P(p ≥ 0,004) de {so_quartis['P_p_maior_igual_0004']:.1%} para "
                            f"{com_extremos['P_p_maior_igual_0004']:.1%} e o q95 de {so_quartis['q95']:.4f} para {com_extremos['q95']:.4f}: "
                            "corrige uma parte pequena da ancoragem; o p verdadeiro continua na cauda da priori, e quem corrige o resto são os dados")
                if corrige else "os extremos não mudaram a massa da priori acima de 0,004: o SHELF não corrigiu a ancoragem",
            },
            "nota": "ELICITAÇÃO SIMULADA: nenhum especialista humano foi consultado; os quantis representam a leitura típica de quem "
                    "só conhece históricos sem perda. O agent-f7 usa o consenso dos quartis para a priori Beta (deliberadamente ancorada baixo).",
        }

    def priori_atipica(self):
        return {"quantidade": "p sob o trauma de uma crise recente", "especialistas": [
            {"nome": "E4 (simulado: gestor de risco logo depois de uma crise)", "q25": 0.02, "q50": 0.04, "q75": 0.07}]}

    def termo_escopo(self):
        return {
            "vale_para": "perda de magnitude fixa (−50%) com chance semanal constante e independente entre semanas e entre fundos; "
                         "ganho de +1%; alavancagem até 3 sem custo de financiamento; horizontes de 250 a 1 000 semanas",
            "nao_vale_para": "perdas em aglomerados (contágio), magnitude de perda com cauda Pareto, choque comum a todos os fundos, "
                             "chamadas de margem antes do fim da semana, resgates em pânico, custos de financiamento da alavancagem",
            "regra_de_decisao": "alavancagem pela fração de Kelly com p estimado na coorte de fundos (ou pela regra de Laplace no próprio histórico), "
                                "nunca pelo k/n de um histórico limpo; nunca f ≥ 2",
            "circuit_breaker": f"se o limite inferior de 97,5% de p na janela recente de históricos passar de {P_DISJUNTOR:.4f} "
                               "(onde f* = 1 e alavancar deixa de compensar), desalavancar e reabrir a estimação",
            "assinatura": "sil (pendente)",
        }

    # ---------------- F2 ----------------
    def contrato(self):
        return {
            "nome": "fundos_venda_de_seguro", "versao": "1.0",
            "colunas": {
                "fundo": {"tipo": "str"}, "periodo": {"tipo": "int", "min": 1, "max": 99},
                "tipo_gestora": {"tipo": "str", "valores": TIPOS}, "uf": {"tipo": "str", "valores": UFS},
                "ano_fundacao": {"tipo": "int", "min": ANO_MIN, "max": ANO_MAX},
                "regra": {"tipo": "str", "valores": REGRAS},
                "semanas_hist": {"tipo": "int", "min": 1, "max": 5000}, "perdas_hist": {"tipo": "int", "min": 0, "max": 5000},
                "alavancagem": {"tipo": "float", "min": 0.0, "max": TETO},
                "semanas_fut": {"tipo": "int", "min": 1, "max": 5000}, "perdas_fut": {"tipo": "int", "min": 0, "max": 5000},
            },
            "chave": ["fundo"], "particao": "periodo",
            "grao": "uma linha por fundo: contagens de perdas no histórico (antes da escolha de f) e nas semanas seguintes",
        }

    def _perdas_contagio(self, n, semanas, rng):
        """Patológico: perdas em aglomerados. Base 0,002; nas 4 semanas após uma perda, 0,25."""
        cont = np.zeros((n, semanas), dtype=bool)
        desde = np.full(n, 99)
        for t in range(semanas):
            p = np.where(desde < 4, 0.25, 0.002)
            cont[:, t] = rng.random(n) < p
            desde = np.where(cont[:, t], 0, desde + 1)
        return cont

    def gerar_lote(self, periodo, rng, cenario="mundo"):
        n = N_FUNDOS
        if cenario == "patologico":
            sem = self._perdas_contagio(n, HIST + FUT, rng)
            k_hist, k_fut = sem[:, :HIST].sum(1), sem[:, HIST:].sum(1)
        else:
            p = P_DERIVA if cenario == "deriva" else P_VERDADEIRO
            k_hist, k_fut = rng.binomial(HIST, p, n), rng.binomial(FUT, p, n)
        tipo = rng.choice(len(TIPOS), n)
        u = rng.random(n)
        uf = rng.choice(UFS, n)
        ano = rng.integers(ANO_MIN, ANO_MAX + 1, n)
        linhas = []
        for i in range(n):
            t = TIPOS[tipo[i]]
            regra = REGRAS[int(np.searchsorted(np.cumsum(REGRA_POR_TIPO[t]), u[i]))]
            linhas.append({"fundo": f"C{periodo:02d}-{i:05d}", "periodo": int(periodo), "tipo_gestora": t, "uf": str(uf[i]),
                           "ano_fundacao": int(ano[i]), "regra": regra, "semanas_hist": HIST, "perdas_hist": int(k_hist[i]),
                           "alavancagem": float(alavancagem_da_regra(regra, int(k_hist[i]))), "semanas_fut": FUT, "perdas_fut": int(k_fut[i])})
        return linhas

    def generalizar(self, linha):
        a = linha["ano_fundacao"]
        faixa = "até 2004" if a <= 2004 else ("2005-2014" if a <= 2014 else "2015 em diante")
        return {**linha, "uf": REGIAO[linha["uf"]], "ano_fundacao": faixa}

    # ---------------- modelo ----------------
    @staticmethod
    def _arr(dados, c):
        return np.asarray(dados[c])

    def _ln_w(self, dados):
        f = self._arr(dados, "alavancagem").astype(float)
        m = self._arr(dados, "perdas_fut").astype(int)
        return ln_riqueza(f, self._arr(dados, "semanas_fut") - m, m)

    def suficientes(self, dados):
        k = self._arr(dados, "perdas_hist").astype(int) + self._arr(dados, "perdas_fut").astype(int)
        n = self._arr(dados, "semanas_hist").astype(int) + self._arr(dados, "semanas_fut").astype(int)
        return {"k": int(k.sum()), "n": int(n.sum())}

    def logpost(self, x, suf, priori):
        x = float(np.ravel(x)[0])
        lp, l1p = -np.logaddexp(0, -x), -np.logaddexp(0, x)
        return (suf["k"] + priori["a"]) * lp + (suf["n"] - suf["k"] + priori["b"]) * l1p   # inclui o jacobiano p(1−p)

    def amostra_priori(self, priori, rng):
        return np.array([special.logit(np.clip(rng.beta(priori["a"], priori["b"]), 1e-12, 1 - 1e-12))])

    def simular_suf(self, x, rng):
        n = 20 * HIST       # vinte históricos de cinco anos
        return {"k": int(rng.binomial(n, special.expit(x[0]))), "n": n}

    def quantidade(self, x):
        return float(special.expit(np.ravel(x)[0]))

    def x_inicial(self, suf):
        return np.array([special.logit((suf["k"] + 0.5) / (suf["n"] + 1))])

    def nulo(self, dados, rng):
        """Teste severo (Mayo): "nunca perdeu em 250 semanas" teria acusado o mundo perigoso?"""
        k = self._arr(dados, "perdas_hist").astype(int)
        m = self._arr(dados, "perdas_fut").astype(int)
        obs = float(np.mean(k == 0))
        p0_perigo = (1 - P_VERDADEIRO) ** HIST
        p0_desprez = (1 - P_DESPREZIVEL) ** HIST
        sims = rng.binomial(k.size, p0_perigo, 5000) / k.size
        severidade = 1 - p0_perigo
        semanas_95 = int(np.ceil(np.log(0.05) / np.log(1 - P_VERDADEIRO)))
        rv = p0_desprez / p0_perigo
        return {
            "estatistica": "fração de fundos sem nenhuma perda em 250 semanas (a evidência da alegação)",
            "observado": obs, "n_fundos": int(k.size),
            "nulo_mundo_perigoso_p0004": {"esperado": p0_perigo, "q01_q99": np.quantile(sims, [0.01, 0.99]).tolist(),
                                          "p_valor_bilateral": float(2 * min(np.mean(sims >= obs), np.mean(sims <= obs)))},
            "mundo_desprezivel_p00004": {"P_nenhuma_perda_250": p0_desprez},
            "severidade": {"P_acusar_perda_se_mundo_perigoso": severidade, "P_passar_mesmo_perigoso": p0_perigo,
                           "razao_de_verossimilhanca_desprezivel_vs_perigoso": rv, "bits_de_evidencia": float(np.log2(rv)),
                           "semanas_para_severidade_95": semanas_95, "anos_para_severidade_95": semanas_95 / 52,
                           "limiar_severo": 0.95, "severo": bool(severidade >= 0.95)},
            "historicos_limpos_que_perderam_depois": {"n": int(np.sum(k == 0)), "frac_com_perda_nas_1000_semanas_seguintes": float(np.mean(m[k == 0] > 0))},
            "conferencia_lab9": {kk: v for kk, v in conferencia_lab9().items() if kk not in ("simulacao_6c", "tabela_6b")},
            "leitura": (f"sob p = 0,004, {p0_perigo:.1%} dos fundos não mostram perda em 250 semanas (observado: {obs:.1%}); "
                        f"o teste 'nunca perdeu' só acusaria o perigo com probabilidade {severidade:.0%}, muito abaixo de 95%. "
                        f"Um histórico limpo é apenas {rv:.1f} vezes mais provável no mundo desprezível ({np.log2(rv):.1f} bits): "
                        f"seriam precisas {semanas_95} semanas (~{semanas_95 / 52:.0f} anos) sem perda para um teste severo"),
        }

    def ppc(self, dados, amostras_x, rng):
        """Checagem preditiva: índice de dispersão (variância/média) das perdas por fundo.
        O binomial i.i.d. prevê ≈ 1 − p; o contágio (perdas em aglomerados) infla a variância."""
        k = self._arr(dados, "perdas_hist").astype(int) + self._arr(dados, "perdas_fut").astype(int)
        n = self._arr(dados, "semanas_hist").astype(int) + self._arr(dados, "semanas_fut").astype(int)
        disp = lambda c: float(c.var() / max(c.mean(), 1e-12))
        obs = disp(k)
        ps = special.expit(amostras_x[rng.integers(0, len(amostras_x), 400), 0])
        rep = np.array([disp(rng.binomial(n, p)) for p in ps])
        pv = float(2 * min(np.mean(rep >= obs), np.mean(rep <= obs)))
        return {"estatistica": "índice de dispersão das perdas por fundo (variância/média, 1 250 semanas)", "observado": obs,
                "replicas_q025_q975": np.quantile(rep, [0.025, 0.975]).tolist(), "p_valor": pv,
                "frac_fundos_com_3_ou_mais_perdas_no_historico": float(np.mean(self._arr(dados, "perdas_hist") >= 3))}

    def resultado_unidade(self, dados):
        lnw = self._ln_w(dados)
        fut = self._arr(dados, "semanas_fut").astype(float)
        return np.where(np.isinf(lnw), G_RUINA, lnw / fut)     # taxa de crescimento realizada por semana

    # ---------------- F3 ----------------
    def entradas_cofre(self, cofre):
        k = self._arr(cofre, "perdas_hist").astype(int)
        return {"ids": [str(i) for i in self._arr(cofre, "fundo")], "perdas_hist": k,
                "semanas_hist": self._arr(cofre, "semanas_hist").astype(int),
                "alavancagem": self._arr(cofre, "alavancagem").astype(float),
                "semanas_fut": self._arr(cofre, "semanas_fut").astype(int),
                "deriva": k.astype(float)}

    def previsoes(self, treino, entradas, amostras_x):
        ps = special.expit(amostras_x[:: max(1, len(amostras_x) // 400), 0])
        base = float(np.mean(np.isinf(self._ln_w(treino))))
        out = []
        for iid, k, n, f, nf in zip(entradas["ids"], entradas["perdas_hist"], entradas["semanas_hist"], entradas["alavancagem"], entradas["semanas_fut"]):
            barreira = float(1 + f * PERDA <= 0)          # só f ≥ 2 pode ser arruinado por uma perda
            p_mod = barreira * float(np.mean(1 - (1 - ps) ** nf))
            p_kn = barreira * (1 - (1 - k / n) ** nf)
            p_lap = barreira * (1 - (1 - laplace(k, n)) ** nf)
            out.append({"id": iid, "evento": f"ruína nas próximas {nf} semanas sob a alavancagem que o fundo escolheu", "taxa_base": base,
                        "probs": {"agent-f4-frontier": p_mod, "alegacao-historico": p_kn, "laplace-historico": p_lap},
                        "meta": {"fonte_filtro": "coorte do período 4 inteira, inclusive os fundos que depois fecharam (sem viés de sobrevivência)",
                                 "pode_influir": False, "flutuacao_ou_causa": "causa: a alavancagem escolhida decide se uma perda é absorvente",
                                 "n_fatores": 2, "degrau_pearl": 2}})
        return out

    def desfechos(self, cofre):
        lnw = self._ln_w(cofre)
        return {str(i): int(r) for i, r in zip(self._arr(cofre, "fundo"), np.isinf(lnw))}

    # ---------------- F5 / F6 ----------------
    def estatistica_publicada(self, dados):
        k = self._arr(dados, "perdas_hist").astype(int)
        return int(np.sum(k == 0)), int(k.size), "fração de fundos sem nenhuma perda em 250 semanas"

    def justica(self, ctx):
        tr = ctx["treino"]
        regra = self._arr(tr, "regra").astype(str)
        k = self._arr(tr, "perdas_hist").astype(int)
        lnw = self._ln_w(tr)
        ruina = np.isinf(lnw)
        f = self._arr(tr, "alavancagem").astype(float)
        por_regra = {}
        for r in REGRAS:
            s = regra == r
            w = np.exp(lnw[s])
            por_regra[r] = {"n_fundos": int(s.sum()), "fracao_arruinada": float(ruina[s].mean()), "mediana_riqueza_final": float(np.median(w)),
                            "media_riqueza_final": float(w.mean())}
        limpos_alav = (k == 0) & (f >= 2)
        n_la = int(limpos_alav.sum())
        w_all = np.exp(lnw)
        top = np.sort(w_all)[::-1]
        top1 = float(top[: max(1, w_all.size // 100)].sum() / w_all.sum())
        sobrev = int(np.sum(limpos_alav & ~ruina))
        return {
            "trilema_da_justica": "não se aplica: não há classificador nem grupos protegidos com decisões distintas; a escolha de f é do gestor, "
                                  "e os cotistas não são selecionados por atributo pessoal. O erro recai sobre quem confiou num histórico limpo.",
            "quem_paga_o_erro": {"cotistas_de_fundos_alavancados_por_historico_limpo": {"n_fundos": n_la, "fracao_arruinada": float(ruina[limpos_alav].mean()) if n_la else 0.0},
                                 "por_regra_de_alavancagem": por_regra,
                                 "fatia_da_riqueza_total_do_1pct_mais_rico": top1},
            "inversao_metrica": {"metrica_original": "riqueza média do conjunto de fundos / histórico sem perdas (premia alavancar ao teto)",
                                 "metrica_invertida": "fração arruinada e taxa de crescimento mediana (premia Kelly com Laplace ou meio Kelly)",
                                 "ganham": f"os cotistas dos {n_la} fundos de histórico limpo que alavancaram 3×: {ruina[limpos_alav].mean():.0%} deles foram arruinados "
                                           "e deixariam de ser com f < 2",
                                 "perdem": f"os gestores com taxa de performance e os {sobrev} fundos alavancados que não viram perda em 1 000 semanas, "
                                           f"e que puxam a métrica média: o 1% mais rico dos fundos concentra {top1:.0%} de toda a riqueza"},
            "assimetria_de_poder": "o gestor recebe taxa de performance sobre os anos bons e não devolve nada na ruína (opção grátis); "
                                   "o cotista vive a trajetória inteira, inclusive o estado absorvente",
        }

    # ---------------- M15 ----------------
    def _morfonomia(self, tr):
        k = self._arr(tr, "perdas_hist").astype(int) + self._arr(tr, "perdas_fut").astype(int)
        n = self._arr(tr, "semanas_hist").astype(int) + self._arr(tr, "semanas_fut").astype(int)
        per = np.array([int(str(i)[1:3]) for i in self._arr(tr, "fundo")])
        tab = np.array([[k[per == q].sum(), n[per == q].sum() - k[per == q].sum()] for q in np.unique(per)])
        _, p_icp, _, _ = stats.chi2_contingency(tab)
        ll_bin = float(stats.binom.logpmf(k, n, k.sum() / n.sum()).sum())

        def nll(th):
            mu, phi = special.expit(th[0]), np.exp(th[1])
            return -stats.betabinom.logpmf(k, n, mu * phi, (1 - mu) * phi).sum()

        r = optimize.minimize(nll, x0=[special.logit(k.sum() / n.sum()), np.log(1e4)], method="Nelder-Mead")
        N = k.size
        bic_bin, bic_bb = -2 * ll_bin + np.log(N), 2 * r.fun + 2 * np.log(N)
        return {"icp_homogeneidade_entre_periodos_p": float(p_icp), "invariante": bool(p_icp > 0.01),
                "bic_binomial": float(bic_bin), "bic_beta_binomial": float(bic_bb),
                "mdl_prefere": "binomial (p comum)" if bic_bin <= bic_bb else "beta-binomial (p heterogêneo)"}

    def lentes(self, ctx):
        q = ctx["f4"]["_q"]
        p = float(np.median(q))
        tr = ctx["treino"]
        lab = conferencia_lab9()
        lnw = self._ln_w(tr)
        w = np.exp(lnw)
        vivos = w[w > 0]
        s = np.sort(vivos)[::-1]
        ecdf_ccdf = np.arange(1, s.size + 1) / w.size
        idx = np.unique(np.clip(np.round(np.geomspace(1, s.size, 12)).astype(int) - 1, 0, s.size - 1))
        top1 = float(np.sort(w)[::-1][: w.size // 100].sum() / w.sum())
        # Fluxonomia: informação que um histórico limpo carrega sobre "perigoso" vs "desprezível" (50/50)
        pl = np.array([(1 - P_VERDADEIRO) ** HIST, (1 - P_DESPREZIVEL) ** HIST])
        pj = np.array([[0.5 * pl[0], 0.5 * (1 - pl[0])], [0.5 * pl[1], 0.5 * (1 - pl[1])]])
        px = pj.sum(0)
        mi = float(np.sum(pj * np.log2(pj / (0.5 * px[None, :]))))
        post = ctx["priori"]["a"] + ctx["suf"]["k"], ctx["priori"]["b"] + ctx["suf"]["n"] - ctx["suf"]["k"]
        evsi = estat.evsi_beta_bernoulli(post[0], post[1], HIST, self.utilidade, self.acoes(), np.random.default_rng(ctx["semente"] + 4), m=2000)
        return {
            "Crononomia": {"f_estrela_posterior": kelly(p), "g_estrela_posterior": float(g(kelly(p), p)),
                           "conferencia_lab9": {kk: lab[kk] for kk in ("f_estrela", "g_estrela_por_semana", "P_nenhuma_perda_250", "esperado_lab", "confere")},
                           "ln_E_fator_f3": float(np.log(1 + 3 * ((1 - p) * GANHO + p * PERDA))), "E_ln_fator_f3": float(g(3.0, p)),
                           "ruina_absorvente": "com f ≥ 2, a primeira perda leva a riqueza a zero e dali não sai: a média do conjunto conta fundos que não existem mais",
                           "ergodico": False,
                           "leitura": "Kelly maximiza a taxa temporal, não a média do conjunto; com f = 3 a esperança por semana é positiva, mas a taxa temporal é negativa (ruína quase certa)"},
            "Escalonomia": {"riqueza_final_media": float(w.mean()), "riqueza_final_mediana": float(np.median(w)),
                            "fracao_arruinada": float(np.mean(w == 0)), "fatia_top1pct": top1,
                            "hill_alpha_k25_k50_k100": [estat.hill(vivos, kk) for kk in (25, 50, 100)],
                            "ecdf_loglog": [{"riqueza": float(s[i]), "P_maior_igual": float(ecdf_ccdf[i])} for i in idx],
                            "media_W_lab6c_kn_vs_kelly": [lab["simulacao_6c"]["Kelly com k/n"]["media_W"], lab["simulacao_6c"]["Kelly verdadeiro"]["media_W"]],
                            "magnitude_da_perda": "fixa em −50% neste mundo (sem cauda na perda individual); a cauda pesada está na riqueza final",
                            "leitura": "a média da riqueza é dominada por poucos fundos alavancados que não viram perda em 1 000 semanas "
                                       f"(1% mais rico = {top1:.0%} da riqueza; no Lab 9, média de {lab['simulacao_6c']['Kelly com k/n']['media_W']:.1e} com k/n "
                                       f"contra {lab['simulacao_6c']['Kelly verdadeiro']['media_W']:.1e} com Kelly verdadeiro). Hill instável entre k: a cauda é uma "
                                       "mistura (massa em 1,03^1000 dos sobreviventes alavancados), não uma lei de potência"},
            "Fluxonomia": {"informacao_mutua_historico_limpo_bits": mi,
                           "laco_performativo": False,
                           "leitura": f"um histórico de 250 semanas sem perda carrega só {mi:.2f} bits sobre 'perigoso vs desprezível'; "
                                      "a alavancagem não altera a chance de perda neste mundo (sem laço performativo), embora no mercado real muitos vendedores de seguro "
                                      "alavancados tornem as perdas correlacionadas"},
            "Morfonomia": self._morfonomia(tr),
            "Crisonomia": {"placar": ctx["f3"]["placar"], "evpi_posterior_por_semana": estat.evpi(q[::10], self.utilidade, self.acoes())["evpi"],
                           "evsi_mais_250_semanas": evsi,
                           "leitura": "pooling de fundos já identifica p; mais 250 semanas de um fundo não mudariam a fração escolhida. "
                                      "O valor da informação estava em agregar históricos, não em esperar o próprio"},
        }

    def critica(self, ctx):
        mf = self._morfonomia(ctx["treino"])
        return {"rival": {"modelo": "beta-binomial (p heterogêneo entre fundos), aproximação de contágio e de mistura de regimes",
                          **{k: v for k, v in mf.items() if k.startswith(("bic", "mdl"))}},
                "performatividade": "nenhuma no modelo: a perda é exógena e a alavancagem só decide se ela é absorvente. No mercado real, "
                                    "a recomendação de não alavancar reduziria a venda forçada em crises, que hoje torna as perdas correlacionadas",
                "premissas": ["magnitude da perda fixa em −50% (importada do Lab 9, não estimada; uma cauda Pareto na perda pioraria tudo)",
                              "semanas independentes (sem contágio) e fundos independentes (sem choque comum)",
                              "alavancagem sem custo de financiamento e sem chamada de margem intrassemanal",
                              "o ganho de +1% por semana é conhecido e fixo"]}

    # ---------------- F8 ----------------
    def acoes(self):
        return [0.0, 0.5, 0.8, 1.2, 1.59, 3.0]

    def utilidade(self, acao, theta):
        return float(g(acao, theta))

    def decisao(self, ctx):
        q = ctx["f4"]["_q"]
        fs = np.linspace(0, TETO, 601)
        eg = np.array([np.mean(g(f, q[::20])) for f in fs])
        f_star = float(fs[np.argmax(eg)])
        rng = np.random.default_rng(ctx["semente"] + 8)
        n = 10_000
        p_draw = rng.choice(q, n)
        k_hist = rng.binomial(HIST, p_draw)
        m_fut = rng.binomial(FUT, p_draw)
        sim = simular_regras(float(np.median(q)), k_hist, m_fut)
        res, hops = {}, {}
        for nome in ("Kelly verdadeiro", "Kelly com k/n", "Kelly com Laplace", "meio Kelly com k/n", "meio Kelly verdadeiro"):
            v = sim[nome]
            res[nome] = {kk: vv for kk, vv in v.items() if not kk.startswith("_")}
            res[nome]["de_100_crescem"] = round(100 * float(np.mean(v["_lnw"] > 0)))
            hops[nome] = np.where(np.isinf(v["_lnw"]), -5.0, v["_lnw"] / np.log(10))
        p_E = res["Kelly com k/n"]["fracao_arruinada"]
        # C: crescimento sacrificado ao trocar k/n por Laplace no fundo típico; L: crescimento perdido na ruína (toda a riqueza)
        C = max(res["Kelly verdadeiro"]["mediana_lnW_por_semana"] - res["Kelly com Laplace"]["mediana_lnW_por_semana"], 0.0) * FUT
        L = -LN_PISO
        return {
            "f_kelly_posterior": f_star,
            "regras_1000_semanas": res,
            "arvore_CL": {"evento": "ruína (riqueza a zero) em 1 000 semanas", "acao_protetora": "estimar p pela regra de Laplace (ou pela coorte) e nunca alavancar f ≥ 2",
                          "P_evento_kelly_kn": p_E, "P_evento_kelly_laplace": res["Kelly com Laplace"]["fracao_arruinada"],
                          "C_custo_protecao_ln": C, "L_perda_ruina_ln": L, "C_sobre_L": C / L,
                          "regra": "proteger se P(evento) > C/L", "proteger": bool(p_E > C / L),
                          "limiar_robusto": f"a decisão só se inverteria se C/L > {p_E:.2f}"},
            "frequencias_naturais": (f"De cada 100 fundos que escolhem a alavancagem pelo k/n do próprio histórico, cerca de {round(100 * float(np.mean(k_hist == 0)))} "
                                     f"veem cinco anos sem perda e alavancam 3×; {res['Kelly com k/n']['de_100_arruinados']} de cada 100 terminam arruinados em 1 000 semanas. "
                                     f"Com a regra de Laplace, {res['Kelly com Laplace']['de_100_arruinados']} de cada 100; com o Kelly da coorte, "
                                     f"{res['Kelly verdadeiro']['de_100_arruinados']}."),
            "classe_de_referencia": "fundos que vendem proteção com perda rara de 50%, alavancagem até 3, escolhida a partir de 250 semanas de histórico",
            "circuit_breaker": self.termo_escopo()["circuit_breaker"],
            "hops_titulo": "Riqueza após 1 000 semanas", "hops_rotulo": "log10(riqueza final / inicial); ruína = −5", "_hops": hops,
        }

    def disjuntor(self, dados, ctx):
        k = int(np.sum(self._arr(dados, "perdas_hist")))
        n = int(np.sum(self._arr(dados, "semanas_hist")))
        inf = float(stats.beta.ppf(0.025, 1 + k, 1 + n - k))
        disp = inf > P_DISJUNTOR
        return {"disparou": bool(disp), "p_inf_025": inf, "limiar": P_DISJUNTOR, "semanas_na_janela": n,
                "motivo": "a chance de perda subiu além do ponto em que alavancar compensa: desalavancar" if disp else "dentro do escopo"}
