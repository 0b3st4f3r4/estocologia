"""Projeto A (dados reais) · Aditivo vs. multiplicativo no Dow Jones, 1885–2023 (Crononomia, M7/M8; Lab 8).

A moeda de Peters do H1 simulado é trocada pelo fechamento diário real do Dow Jones Industrial
Average (Rdatasets `stevedata::DJIA`, só preço, sem dividendos). A alegação examinada é a falácia
multiplicativa no índice real: ler o retorno aritmético médio por dia como o destino de um
investidor alavancado que segura por décadas.

A unidade independente é o ANO-CALENDÁRIO (bloco de ~250–300 pregões), não o dia: os dias de um
mesmo ano compartilham regime de volatilidade (aglomeração), e é o ano que entra na verossimilhança,
nas margens, no conforme e nas previsões do cofre.
"""

from __future__ import annotations

import csv
import os
from functools import lru_cache
from pathlib import Path

import numpy as np
from scipy import special, stats

from estocologia import estat
from estocologia.projeto import Projeto

# ---------------- dados e períodos ----------------
DADOS = Path(os.environ.get("ESTOCOLOGIA_DADOS", Path(__file__).resolve().parents[4] / "dados-reais"))
ARQ_DJIA = DADOS / "djia_1885_2023.csv"
ARQ_CRSP = DADOS / "crsp_day_1989_1998.csv"
# Cronológicos; o último é o cofre out-of-time (F3). Cortes em quebras institucionais conhecidas:
# 1929 fecha a era pré-SEC (crash de outubro dentro do período 1); 1930–1969 cobre a Depressão,
# a guerra e o pós-guerra até o fim de Bretton Woods; 1970–1999 é o regime de câmbio flutuante e
# de juros altos até a bolha das ponto-com; 2000–2023 (cofre) é a era eletrônica (decimalização 2001).
PERIODOS = {1: (1885, 1929), 2: (1930, 1969), 3: (1970, 1999), 4: (2000, 2023)}
ANO_SINTETICO = 3000          # rótulo de ano para lotes reamostrados/perturbados (nunca colide com ano real)
ALAVANCAGEM_ALEGADA = 3.0     # o "3×" da alegação
HORIZONTE_ANOS = 20           # horizonte fixo das coortes ("segura por décadas")
ESCALA_DERIVA = 3.0           # MB3/MB8: σ diário triplicado (~3%/dia, o regime de out/1929 e out/2008)
JANELA_DISJUNTOR = 63         # pregões (um trimestre) para a variância recente do disjuntor
LS = (0.0, 0.5, 1.0, 1.5, 2.0, 3.0)


@lru_cache(maxsize=1)
def _serie():
    """Lê o CSV uma vez. Devolve datas (str), fechamentos e, por pregão a partir do 2º, o retorno simples."""
    with open(ARQ_DJIA, newline="") as fh:
        r = list(csv.DictReader(fh))
    datas = np.array([x["date"] for x in r])
    v = np.array([float(x["value"]) for x in r])
    d64 = datas.astype("datetime64[D]")
    return {"data": datas[1:], "fechamento": v[1:], "retorno": v[1:] / v[:-1] - 1,
            "dias_corridos": (d64[1:] - d64[:-1]).astype(int), "ano": np.array([int(s[:4]) for s in datas[1:]])}


# ---------------- o mundo, em funções puras (testadas no Hypothesis) ----------------
def ln_fator(r, L):
    """ln(1 + L r) por pregão; −∞ quando 1 + L r ≤ 0 (ruína: estado absorvente, a riqueza fica em zero)."""
    y = 1 + float(L) * np.asarray(r, dtype=float)
    out = np.full(y.shape, -np.inf)
    np.log(y, out=out, where=y > 0)
    return out


def g_emp(L, r):
    """Taxa temporal por pregão, E[ln(1 + L r)], estimada pela média nos retornos reais."""
    return float(np.mean(ln_fator(r, L)))


def g_quad(L, alfa, V):
    """Aproximação de 2ª ordem por ano: L α − L² V / 2 (α = deriva aritmética, V = variância realizada)."""
    return L * alfa - L * L * V / 2


def kelly_quad(alfa, V):
    """L* = argmax g_quad = α / V."""
    return alfa / V


def l_ruina(r):
    """Menor alavancagem que zera a riqueza no pior pregão: 1 / |min r| (∞ se nunca houve queda)."""
    m = float(np.min(r))
    return np.inf if m >= 0 else -1.0 / m


def por_ano(ano, x):
    """Soma de x por ano, para dados ordenados por (ano, dia). Devolve (anos, somas)."""
    ano = np.asarray(ano)
    u, ini = np.unique(ano, return_index=True)
    ordem = np.argsort(ini)
    u, ini = u[ordem], ini[ordem]
    return u, np.add.reduceat(np.asarray(x, dtype=float), ini) if ini.size else np.array([])


def gama_de(p_ano, s):
    """Média anual de ln(riqueza) com P(ano de alta) = p_ano e desvio s: γ = s Φ⁻¹(p_ano)."""
    return s * special.ndtri(p_ano)


def coortes(lnf, inicios, fins):
    """ln da riqueza final de cada coorte que compra no fechamento anterior a `inicio` e segura até `fim`."""
    C = np.concatenate([[0.0], np.cumsum(lnf)])
    return C[np.asarray(fins) + 1] - C[np.asarray(inicios)]


def inicios_mensais(datas, horizonte_anos):
    """Coortes que começam no 1º pregão de cada mês e terminam no último pregão antes de +horizonte anos."""
    d = np.asarray(datas).astype("datetime64[D]")
    mes = d.astype("datetime64[M]")
    ini = np.flatnonzero(np.r_[True, mes[1:] != mes[:-1]])
    alvo = (mes[ini] + np.timedelta64(12 * horizonte_anos, "M")).astype("datetime64[D]") + (d[ini] - mes[ini].astype("datetime64[D]"))
    ok = alvo <= d[-1]
    ini, alvo = ini[ok], alvo[ok]
    fim = np.searchsorted(d, alvo, side="right") - 1
    return ini, fim


def _ac1(x):
    x = np.asarray(x, float) - np.mean(x)
    return float(np.sum(x[:-1] * x[1:]) / np.sum(x * x))


class ProjetoA(Projeto):
    nome = "a"
    dados_simulados = False
    origem_dos_dados = "dados-reais/djia_1885_2023.csv (Rdatasets stevedata::DJIA)"
    tem_titulares = False
    titulo = "Aditivo vs. multiplicativo no Dow Jones real (1885–2023)"
    alegacao = ('"O Dow rende em média μ por dia; alavancando 3× (rebalanceado diariamente) o investidor ganha 3μ, '
                'e quem segura por décadas sempre sai ganhando."')
    periodos = (1, 2, 3, 4)
    unidade = "ano"
    colunas_modelo = ("data", "ano", "dia", "dias_corridos", "retorno")
    quase_ids = ()            # índice público agregado: não há titulares nem quase-identificadores
    dim = 4                   # (logit p_ano, ln s, ln V̄, ln κ)
    custo_inferencia = 1e-6

    cabecalho = {
        "processo_gerador": "Mundo (L1): fechamentos diários reais do DJIA (só preço). Modelo (L2): por ano, ln-crescimento G ~ N(γ, s²) e "
                            "variância realizada V com ln V ~ N(ln V̄ − κ²/2, κ²), anos i.i.d. A alegação falha no Modelo: lê a deriva "
                            "aritmética (L·μ) como o destino de cada trajetória alavancada (L3).",
        "nao_linearidade": "ln(1 + L r) é côncava em L: E[ln(1 + L r)] ≈ L μ − L² σ²/2 (arrasto de volatilidade) < L μ. Com L ≥ 1/|r_min| "
                           "um único pregão zera a riqueza. Reportar mediana, quantis e frações de coortes, nunca só a média.",
        "unidade_e_margem": "Unidade independente: o ano-calendário (bloco de pregões), não o dia — os dias de um ano compartilham regime de "
                            "volatilidade. Margens, conforme e previsões são por ano; coortes sobrepostas de 20 anos valem ~n_anos/20 "
                            "observações independentes, não o número de coortes.",
        "fisica_do_ruido": "Ruído de caudas pesadas e aglomerado no tempo (1929–32, 1987, 2008, 2020), não gaussiano por dia. Estado "
                           "absorvente para L ≥ 1/|r_min|. Pregões ausentes (fechamento da bolsa de jul–dez/1914, 135 dias corridos) "
                           "impedem rebalancear. Agente passivo: o investidor individual não move o índice.",
        "escada_causal": "Degrau 2: a decisão é do(L), aplicada aos retornos reais sob a premissa de agente passivo (sem impacto de preço). "
                         "Sem confundidor para o investidor individual; no agregado, alavancagem de margem realimenta quedas (1929, 1987).",
    }

    # ---------------- F7 ----------------
    def traducao(self):
        return {
            "dominio": "finanças pessoais, investimento de varejo em índice de ações e produtos alavancados",
            "pergunta_substantiva": "Um investidor que segura o Dow alavancado 3×, rebalanceando todo dia, por 20 anos, tende a terminar acima do que "
                                    "começou e a ganhar 3 vezes o que ganha quem segura 1×?",
            "unidade_independente": "o ano-calendário de pregões (bloco), não o pregão isolado nem a coorte sobreposta",
            "glossario": {"retorno médio diário": "média aritmética de r = P_t/P_{t−1} − 1 sobre os pregões (média de conjunto por dia)",
                          "taxa de crescimento temporal": "E[ln(1 + L r)], o que uma trajetória experimenta no longo prazo",
                          "arrasto de volatilidade": "diferença L μ − E[ln(1 + L r)] ≈ L² σ²/2",
                          "alavancagem L": "exposição de L vezes a riqueza ao índice, reposta a cada fechamento",
                          "rebalanceamento diário": "repor a exposição L × riqueza a cada pregão (como um ETF alavancado)",
                          "fração de Kelly L*": "alavancagem que maximiza a taxa temporal; acima dela a alavancagem destrói crescimento",
                          "ruína": "um pregão com 1 + L r ≤ 0: riqueza zero, estado absorvente",
                          "coorte": "investidores que começam na mesma data real e seguram pelo mesmo horizonte",
                          "drawdown": "queda em relação ao máximo anterior",
                          "índice só de preço": "sem reinvestimento de dividendos (subestima o retorno total em ~3–4% a.a. no século XX)"},
            "fontes_primarias": ["Peters, O. (2019). The ergodicity problem in economics. Nature Physics 15, 1216–1221.",
                                 "Kelly, J. L. (1956). A new interpretation of information rate. Bell System Technical Journal 35(4).",
                                 "Dados: Rdatasets stevedata::DJIA (fechamento diário 1885-02-16 a 2023-11-21); conferência: Ecdat::CRSPday 1989–1998.",
                                 "Curso, Lab 8 (A média que ninguém vive)"],
        }

    def elicitacao(self):
        return {
            "quantidade": "p_ano, a probabilidade de o Dow (só preço) fechar um ano-calendário acima do início",
            "especialistas": [{"nome": "E1 (simulado: planejador financeiro)", "q25": 0.56, "q50": 0.62, "q75": 0.68},
                              {"nome": "E2 (simulado: historiador econômico)", "q25": 0.52, "q50": 0.60, "q75": 0.66},
                              {"nome": "E3 (simulado: gestor de fundo de índice)", "q25": 0.58, "q50": 0.65, "q75": 0.71}],
            "protocolo": "SHELF v4: quartis individuais sem ancoragem, discussão, consenso RIO, ajuste Beta, validação facial",
            "prioris_importadas": {"ln s": "N(ln 0,20; 0,5²)", "ln V̄": "N(ln 0,03; 1²)", "ln κ": "N(ln 0,5; 0,5²)",
                                   "nota": "não elicitadas: declaradas pelo autor, largas, na escala de um índice de ações anual"},
            "nota": "PAINEL SIMULADO: nenhum especialista humano foi consultado; os quartis são uma leitura de manual de "
                    "investimento (≈ 2 anos de alta em cada 3), não uma elicitação real.",
        }

    def priori_atipica(self):
        return {"quantidade": "p_ano sob o cenário de década perdida (Nikkei pós-1990, Dow 1929–1954)", "especialistas": [
            {"nome": "E4 (simulado: investidor japonês que viveu 1990–2012)", "q25": 0.40, "q50": 0.47, "q75": 0.54}]}

    def termo_escopo(self):
        return {
            "vale_para": "índice amplo de ações dos EUA, só preço, rebalanceamento diário no fechamento, L entre 0 e 3, horizonte de 1 a 30 anos",
            "nao_vale_para": "custo de financiamento da alavancagem (taxa de juros ausente nos dados), dividendos, impostos, custos de transação, "
                             "chamadas de margem intradiárias, ações individuais, índices de outros países (sobrevivência do mercado dos EUA)",
            "regra_de_decisao": "alavancar no máximo até a fração de Kelly estimada por ano-unidade, nunca 3× por leitura da média aritmética",
            "circuit_breaker": f"se, com a variância realizada dos últimos {JANELA_DISJUNTOR} pregões, o L* do quantil 97,5% da posterior cair "
                               "abaixo de 1, desalavancar (L ≤ 1) até a variância recente voltar",
            "assinatura": "sil (pendente)",
        }

    # ---------------- F2 ----------------
    def contrato(self):
        return {
            "nome": "djia_pregoes", "versao": "1.0",
            "colunas": {
                "data": {"tipo": "str"}, "periodo": {"tipo": "int", "min": 1, "max": 99},
                "ano": {"tipo": "int", "min": 1885, "max": 9999}, "dia": {"tipo": "int", "min": 1, "max": 400},
                "dias_corridos": {"tipo": "int", "min": 1, "max": 400}, "fechamento": {"tipo": "float", "min": 0.0},
                "origem": {"tipo": "str", "valores": ["real", "reamostrado", "perturbado"]},
                "retorno": {"tipo": "float", "min": -1.0, "max": 1.0},
            },
            "chave": ["ano", "dia"], "particao": "periodo",
            "grao": "uma linha por pregão; 'ano' ≥ 3000 marca lote sintético (reamostrado ou perturbado), com 'data' real de origem",
        }

    def _linhas(self, periodo, idx, origem, anos=None, retornos=None):
        s = _serie()
        anos = s["ano"][idx] if anos is None else np.asarray(anos)
        ret = s["retorno"][idx] if retornos is None else np.asarray(retornos)
        fech = s["fechamento"][idx]
        if retornos is not None:              # fechamento coerente com o retorno perturbado
            base = s["fechamento"][idx[0]] / (1 + s["retorno"][idx[0]])
            fech = base * np.cumprod(1 + ret)
        out, dia, ant = [], 0, None
        for j, i in enumerate(idx):
            a = int(anos[j])
            dia = dia + 1 if a == ant else 1
            ant = a
            out.append({"data": str(s["data"][i]), "periodo": int(periodo), "ano": a, "dia": dia,
                        "dias_corridos": int(s["dias_corridos"][i]), "fechamento": float(fech[j]),
                        "origem": origem, "retorno": float(ret[j])})
        return out

    def _idx_anos(self, a0, a1):
        ano = _serie()["ano"]
        return np.flatnonzero((ano >= a0) & (ano <= a1))

    def gerar_lote(self, periodo, rng, cenario="mundo"):
        s = _serie()
        a_tr0, a_tr1 = PERIODOS[1][0], PERIODOS[self.periodos[-2]][1]
        if cenario == "deriva":
            # PERTURBAÇÃO DECLARADA: pregões reais do cofre com retornos multiplicados por 3 (σ ~3%/dia, regime de crise)
            idx = self._idx_anos(*PERIODOS[self.periodos[-1]])
            return self._linhas(periodo, idx, "perturbado", retornos=np.clip(ESCALA_DERIVA * s["retorno"][idx], -0.99, 0.99))
        if cenario == "patologico":
            # PERTURBAÇÃO DECLARADA: anos reais de treino reordenados do pior para o melhor ln-crescimento
            # (anos sintéticos 3000+j): tendência perfeita entre unidades, que o modelo i.i.d. por ano proíbe
            idx = self._idx_anos(a_tr0, a_tr1)
            u, G = por_ano(s["ano"][idx], np.log1p(s["retorno"][idx]))
            ordem = u[np.argsort(G, kind="stable")]
            idx2, anos = [], []
            for j, a in enumerate(ordem):
                ii = idx[s["ano"][idx] == a]
                idx2.append(ii)
                anos.append(np.full(ii.size, ANO_SINTETICO + j))
            return self._linhas(periodo, np.concatenate(idx2), "perturbado", anos=np.concatenate(anos))
        if periodo in PERIODOS:
            return self._linhas(periodo, self._idx_anos(*PERIODOS[periodo]), "real")
        # período desconhecido (MB2): REAMOSTRAGEM de 5 anos reais de treino, com reposição, por bloco de ano
        sorteados = rng.integers(a_tr0, a_tr1 + 1, 5)
        idx2, anos = [], []
        for j, a in enumerate(sorteados):
            ii = self._idx_anos(int(a), int(a))
            idx2.append(ii)
            anos.append(np.full(ii.size, ANO_SINTETICO + j))
        return self._linhas(periodo, np.concatenate(idx2), "reamostrado", anos=np.concatenate(anos))

    def generalizar(self, linha):
        return linha

    # ---------------- modelo ----------------
    def _anuais(self, dados, L=1.0):
        """Por ano: ln-crescimento a alavancagem L e variância realizada V = Σ r²."""
        r = np.asarray(dados["retorno"], dtype=float)
        u, G = por_ano(dados["ano"], ln_fator(r, L))
        _, V = por_ano(dados["ano"], r * r)
        return u, G, V

    def suficientes(self, dados):
        _, G, V = self._anuais(dados)
        lv = np.log(V)
        return {"n": int(G.size), "sG": float(G.sum()), "sG2": float((G * G).sum()), "sL": float(lv.sum()), "sL2": float((lv * lv).sum())}

    @staticmethod
    def _par(x):
        z, w, v, u = np.ravel(x)[:4]
        p, s, Vb, k = special.expit(z), np.exp(w), np.exp(v), np.exp(u)
        return p, s, Vb, k, gama_de(p, s)

    def logpost(self, x, suf, priori):
        z, w, v, u = np.ravel(x)[:4]
        if abs(z) > 30 or abs(w) > 10 or abs(v) > 15 or abs(u) > 10:
            return -np.inf
        p, s, Vb, k, gam = self._par(x)
        n = suf["n"]
        ll = -n * w - (suf["sG2"] - 2 * gam * suf["sG"] + n * gam * gam) / (2 * s * s)
        m = v - k * k / 2
        ll += -n * u - (suf["sL2"] - 2 * m * suf["sL"] + n * m * m) / (2 * k * k)
        lp = -np.logaddexp(0, -z) * priori["a"] - np.logaddexp(0, z) * priori["b"]      # Beta em p, com jacobiano p(1−p)
        lp += -0.5 * ((w - np.log(0.2)) / 0.5) ** 2 - 0.5 * ((v - np.log(0.03)) / 1.0) ** 2 - 0.5 * ((u - np.log(0.5)) / 0.5) ** 2
        return float(ll + lp)

    def amostra_priori(self, priori, rng):
        return np.array([special.logit(rng.beta(priori["a"], priori["b"])), rng.normal(np.log(0.2), 0.5),
                         rng.normal(np.log(0.03), 1.0), rng.normal(np.log(0.5), 0.5)])

    def simular_suf(self, x, rng):
        n = 60
        p, s, Vb, k, gam = self._par(x)
        G = rng.normal(gam, s, n)
        lv = rng.normal(np.log(Vb) - k * k / 2, k, n)
        return {"n": n, "sG": float(G.sum()), "sG2": float((G * G).sum()), "sL": float(lv.sum()), "sL2": float((lv * lv).sum())}

    def quantidade(self, x):
        """L* de Kelly por ano-unidade: (γ + V̄/2) / V̄, a alavancagem que maximiza a taxa temporal."""
        p, s, Vb, k, gam = self._par(x)
        return float(kelly_quad(gam + Vb / 2, Vb))

    def x_inicial(self, suf):
        n = suf["n"]
        mG = suf["sG"] / n
        sG = np.sqrt(max(suf["sG2"] / n - mG * mG, 1e-6))
        mL = suf["sL"] / n
        sL = np.sqrt(max(suf["sL2"] / n - mL * mL, 1e-6))
        p = np.clip(stats.norm.cdf(mG / sG), 0.01, 0.99)
        return np.array([special.logit(p), np.log(sG), mL + sL * sL / 2, np.log(sL)])

    def _coortes_ano(self, G, h=HORIZONTE_ANOS):
        """ln da riqueza de coortes que começam em cada ano-unidade e seguram h anos (somas móveis de G)."""
        C = np.concatenate([[0.0], np.cumsum(G)])
        return C[h:] - C[:-h]

    def nulo(self, dados, rng):
        """Teste severo: no mundo da alegação (ganha L·μ, sem arrasto), que fração das coortes de 20 anos a 3×
        terminaria acima do início? Nulo = anos reais reamostrados (unidade = ano) com crescimento 3·Σr."""
        r = np.asarray(dados["retorno"], float)
        _, G3 = por_ano(dados["ano"], ln_fator(r, ALAVANCAGEM_ALEGADA))
        _, A = por_ano(dados["ano"], r)
        _, G1 = por_ano(dados["ano"], np.log1p(r))
        obs3 = float(np.mean(self._coortes_ano(G3) > 0))
        obs1 = float(np.mean(self._coortes_ano(G1) > 0))
        n = A.size
        sims = np.array([np.mean(self._coortes_ano(ALAVANCAGEM_ALEGADA * A[rng.integers(0, n, n)]) > 0) for _ in range(2000)])
        sims_mediana = np.array([np.median(self._coortes_ano(ALAVANCAGEM_ALEGADA * A[rng.integers(0, n, n)])) for _ in range(500)])
        return {"estatistica": f"fração de coortes (início em cada ano, horizonte {HORIZONTE_ANOS} anos) acima da riqueza inicial, L = 3",
                "observado_multiplicativo_L3": obs3, "observado_L1": obs1, "alegacao_literal": 1.0,
                "nulo_aditivo_media": float(sims.mean()), "nulo_aditivo_q01": float(np.quantile(sims, 0.01)),
                "p_valor": float(np.mean(sims <= obs3)),
                "ln_riqueza_mediana_20a_L3": {"observado": float(np.median(self._coortes_ano(G3))), "nulo_aditivo_media": float(sims_mediana.mean())},
                "n_coortes": int(n - HORIZONTE_ANOS + 1), "n_anos": int(n), "n_efetivo_independente": round(n / HORIZONTE_ANOS, 1),
                "leitura": "a alegação aritmética prevê quase todas as coortes 3× acima do início; a fração real é o que o "
                           "arrasto de volatilidade deixa. Coortes sobrepostas: só ~n_anos/20 blocos independentes."}

    def ppc(self, dados, amostras_x, rng):
        """Checagem preditiva com duas estatísticas declaradas a priori, por ano-unidade: autocorrelação lag-1 do
        ln-crescimento anual (independência das unidades) e o pior ano (cauda esquerda). p combinado por Bonferroni."""
        _, G, _ = self._anuais(dados)
        obs_ac, obs_min = _ac1(G), float(G.min())
        idx = rng.integers(0, len(amostras_x), 400)
        rac, rmin = [], []
        for x in amostras_x[idx]:
            p, s, Vb, k, gam = self._par(x)
            Gr = rng.normal(gam, s, G.size)
            rac.append(_ac1(Gr))
            rmin.append(Gr.min())
        rac, rmin = np.array(rac), np.array(rmin)
        p_ac = float(2 * min(np.mean(rac >= obs_ac), np.mean(rac <= obs_ac)))
        p_min = float(2 * min(np.mean(rmin >= obs_min), np.mean(rmin <= obs_min)))
        return {"estatistica": "min(1, 2·min(p_autocorrelação_lag1, p_pior_ano)) do ln-crescimento anual",
                "autocorrelacao_lag1": {"observado": obs_ac, "replicas_q025_q975": np.quantile(rac, [0.025, 0.975]).tolist(), "p": p_ac},
                "pior_ano": {"observado": obs_min, "replicas_q025_q975": np.quantile(rmin, [0.025, 0.975]).tolist(), "p": p_min},
                "observado": obs_ac, "p_valor": float(min(1.0, 2 * min(p_ac, p_min))), "n_anos": int(G.size)}

    def resultado_unidade(self, dados):
        """Escore por ano-unidade: ln-crescimento anual do investidor 3× rebalanceado diariamente."""
        return self._anuais(dados, ALAVANCAGEM_ALEGADA)[1]

    # ---------------- F3 ----------------
    def entradas_cofre(self, cofre):
        """Por ano do cofre: só a 1ª metade dos pregões (ln-crescimento 3× até o meio do ano) e a volatilidade
        realizada do ano (para o monitor de deriva). A 2ª metade fica fechada até o F3 abrir os desfechos."""
        ano = np.asarray(cofre["ano"])
        r = np.asarray(cofre["retorno"], float)
        u, ini = np.unique(ano, return_index=True)
        o = np.argsort(ini)
        u, ini = u[o], ini[o]
        fim = np.r_[ini[1:], ano.size]
        g_meio, frac, vol = [], [], []
        for a, b in zip(ini, fim):
            h = (b - a) // 2
            g_meio.append(ln_fator(r[a:a + h], ALAVANCAGEM_ALEGADA).sum())
            frac.append((b - a - h) / (b - a))
            vol.append(np.sqrt(np.sum(r[a:b] ** 2)))
        return {"ids": [str(int(x)) for x in u], "g_meio": np.array(g_meio), "frac_restante": np.array(frac), "deriva": np.array(vol)}

    def previsoes(self, treino, entradas, amostras_x):
        xs = amostras_x[:: max(1, len(amostras_x) // 400)]
        L = ALAVANCAGEM_ALEGADA
        par = np.array([self._par(x) for x in xs])               # p, s, V̄, κ, γ
        s, Vb, gam = par[:, 1], par[:, 2], par[:, 4]
        alfa = gam + Vb / 2
        g_L = g_quad(L, alfa, Vb)                                # taxa temporal anual a 3× (com arrasto)
        g_ale = L * alfa                                         # leitura da alegação: 3μ, sem arrasto
        _, G3, _ = self._anuais(treino, L)
        base = float(np.mean(G3 > 0))
        out = []
        for iid, gm, f in zip(entradas["ids"], entradas["g_meio"], entradas["frac_restante"]):
            p_mod = float(np.mean(stats.norm.sf(-gm, loc=f * g_L, scale=np.sqrt(f) * L * s)))
            p_ale = float(np.mean(stats.norm.sf(-gm, loc=f * g_ale, scale=np.sqrt(f) * L * s)))
            out.append({"id": iid, "evento": "o investidor 3× rebalanceado diariamente termina o ano acima do início", "taxa_base": base,
                        "probs": {"agent-f4-frontier": p_mod, "alegacao-aritmetica": p_ale},
                        "meta": {"fonte_filtro": "anos 2000–2023 do DJIA (cofre), só preço; índice com troca de componentes (viés de sobrevivência)",
                                 "pode_influir": False, "flutuacao_ou_causa": "flutuação (retornos do índice)", "n_fatores": 2, "degrau_pearl": 2}})
        return out

    def desfechos(self, cofre):
        u, G3, _ = self._anuais(cofre, ALAVANCAGEM_ALEGADA)
        return {str(int(a)): int(g > 0) for a, g in zip(u, G3)}

    # ---------------- F5 / F6 ----------------
    def estatistica_publicada(self, dados):
        _, G, _ = self._anuais(dados)
        return (int(np.sum(G > 0)), int(G.size),
                "fração de anos de alta do DJIA (dado público agregado, sem titulares: k-anonimato e LGPD não se aplicam a pessoas aqui)")

    def _coortes_mensais(self, dados, Ls=LS):
        ini, fim = inicios_mensais(dados["data"], HORIZONTE_ANOS)
        r = np.asarray(dados["retorno"], float)
        return {L: coortes(ln_fator(r, L), ini, fim) for L in Ls}, np.asarray(dados["data"])[ini]

    def justica(self, ctx):
        tr = ctx["treino"]
        co, datas = self._coortes_mensais(tr)
        l3, l1 = co[3.0], co[1.0]
        lk = co[2.0]
        w3 = np.exp(l3)
        top10 = float(np.sort(w3)[::-1][: w3.size // 10].sum() / w3.sum())
        piores = datas[np.argsort(l3)[:3]].tolist()
        return {
            "trilema_da_justica": "não se aplica: não há classificador nem grupos protegidos; as 'pessoas' são coortes de datas de entrada",
            "quem_paga_o_erro": {
                "alavancagem_3x": {"frac_coortes_abaixo_do_inicio_20a": float(np.mean(l3 < 0)), "mediana_riqueza_20a": float(np.exp(np.median(l3))),
                                   "fatia_da_riqueza_somada_dos_10pct_melhores_inicios": top10, "piores_datas_de_entrada": piores},
                "alavancagem_2x": {"frac_coortes_abaixo_do_inicio_20a": float(np.mean(lk < 0)), "mediana_riqueza_20a": float(np.exp(np.median(lk)))},
                "sem_alavancagem": {"frac_coortes_abaixo_do_inicio_20a": float(np.mean(l1 < 0)), "mediana_riqueza_20a": float(np.exp(np.median(l1)))},
                "leitura": "quem paga é quem entra perto de um pico (1906, 1929, 1965–1973): o acaso da data de entrada, não a escolha do investidor",
            },
            "inversao_metrica": {"metrica_original": "riqueza média do conjunto de coortes (média aritmética, a que vende o 3×)",
                                 "metrica_invertida": "riqueza mediana / taxa temporal por coorte (escolhe L perto de Kelly)",
                                 "ganham": f"a coorte típica: mediana em 20 anos de {np.exp(np.median(l3)):.2f}× (3×) para {np.exp(np.median(lk)):.2f}× (2×) "
                                           f"e {np.exp(np.median(l1)):.2f}× (1×) a riqueza inicial; quem entrou num pico deixa de ser arruinado",
                                 "perdem": f"as coortes mais sortudas (entrada em fundos de mercado como 1932 e 1942), cujos 10% melhores inícios somam "
                                           f"{top10:.0%} de toda a riqueza 3× — e os emissores de produtos alavancados, cujas taxas crescem com o volume vendido"},
            "assimetria_de_poder": "o emissor do produto 3× e o corretor veem a média do conjunto (taxas sobre o patrimônio de muitos); "
                                   "o cliente vive uma única trajetória, com uma única data de entrada",
            "nota_dados": "dado público agregado de um índice; não há atributos pessoais, logo nenhuma auditoria por grupo protegido foi inventada",
        }

    # ---------------- M15 ----------------
    def _eventos(self, tr):
        r = np.asarray(tr["retorno"], float)
        d = np.asarray(tr["data"])
        dc = None
        ev = {}
        i87 = np.flatnonzero(d == "1987-10-19")
        if i87.size:
            x = r[i87[0]]
            ev["1987-10-19"] = {"retorno": float(x), **{f"perda_L{L:g}": float(-L * x) for L in (1, 2, 3)}, "L_ruina": float(-1 / x)}
        a, b = np.searchsorted(d, "1929-09-03"), np.searchsorted(d, "1932-07-08", side="right")
        if b > a:
            seg = r[a:b]
            ev["1929-09-03_a_1932-07-08"] = {f"riqueza_final_L{L:g}": float(np.exp(ln_fator(seg, L).sum())) for L in (1, 2, 3)}
            ev["1929-09-03_a_1932-07-08"]["pregoes"] = int(seg.size)
        if "dias_corridos" in tr:
            dc = np.asarray(tr["dias_corridos"])
        i14 = np.flatnonzero(d == "1914-12-12")
        if i14.size:
            ev["fechamento_1914"] = {"ultimo_pregao": str(d[i14[0] - 1]), "reabertura": "1914-12-12", "retorno_na_reabertura": float(r[i14[0]]),
                                     "dias_sem_pregao": int(dc[i14[0]]) if dc is not None else None,
                                     "leitura": "4,5 meses sem poder rebalancear nem sair; nesta série a reabertura é +4,4% (a perda foi de liquidez, não de preço)"}
        return ev

    def lentes(self, ctx):
        tr = ctx["treino"]
        q = ctx["f4"]["_q"]
        r = np.asarray(tr["retorno"], float)
        u, G1, V = self._anuais(tr)
        dias_ano = r.size / u.size
        mu = float(r.mean())
        grade = np.round(np.arange(0, 4.41, 0.01), 2)
        gs = np.array([g_emp(L, r) for L in grade])
        L_emp = float(grade[np.argmax(gs)])
        co, _ = self._coortes_mensais(tr)
        coort = {f"L{L:g}": {"frac_acima_do_inicio": float(np.mean(v > 0)), "media_riqueza_conjunto": float(np.mean(np.exp(v))),
                             "mediana_riqueza": float(np.exp(np.median(v))), "taxa_temporal_mediana_ao_ano": float(np.median(v) / HORIZONTE_ANOS)}
                 for L, v in co.items()}
        # Morfonomia: invariância entre os períodos de treino
        per = [(a, b) for p, (a, b) in PERIODOS.items() if p in self.periodos[:-1]]
        Gp = [G1[(u >= a) & (u <= b)] for a, b in per]
        rp = [r[(np.asarray(tr["ano"]) >= a) & (np.asarray(tr["ano"]) <= b)] for a, b in per]
        kw = stats.kruskal(*Gp)
        lev = stats.levene(*[V[(u >= a) & (u <= b)] for a, b in per])
        # Fluxonomia: informação mútua entre o sinal de ontem e o de hoje
        s0, s1 = (r[:-1] > 0).astype(int), (r[1:] > 0).astype(int)
        tab = np.array([[np.sum((s0 == i) & (s1 == j)) for j in (0, 1)] for i in (0, 1)], float) / s0.size
        mi = float(np.nansum(tab * np.log2(tab / np.outer(tab.sum(1), tab.sum(0)))))
        perdas = -r[r < 0]
        return {
            "Crononomia": {"mu_aritmetico_diario": mu, "alegacao_3mu_diario": 3 * mu,
                           "g_emp_diario": {f"L{L:g}": g_emp(L, r) for L in (0.5, 1, 1.5, 2, 3, 4)},
                           "g_emp_ao_ano": {f"L{L:g}": g_emp(L, r) * dias_ano for L in (1, 2, 3)},
                           "arrasto_L3_ao_ano": (3 * mu - g_emp(3, r)) * dias_ano,
                           "kelly_empirico_diario": L_emp, "kelly_posterior_ano_unidade": {"q05": float(np.quantile(q, 0.05)), "q50": float(np.median(q)), "q95": float(np.quantile(q, 0.95))},
                           "L_ruina_pior_pregao": l_ruina(r), "coortes_20_anos_inicio_mensal": coort,
                           "n_coortes": int(co[1.0].size), "n_efetivo_independente": round(u.size / HORIZONTE_ANOS, 1),
                           "eventos_absorventes": self._eventos(tr), "ergodico": False,
                           "leitura": "a média de conjunto cresce com L (3μ); a taxa temporal cresce só até L* e depois cai; "
                                      "as coortes sobrepostas não são 1 141 provas independentes, são ~6 blocos de 20 anos"},
            "Escalonomia": {"curtose_excesso_diaria": float(stats.kurtosis(r)), "hill_alpha_perdas_k50_k100_k200": [estat.hill(perdas, k) for k in (50, 100, 200)],
                            "pior_pregao": float(r.min()), "pior_ano_ln": float(G1.min()), "ano_do_pior": int(u[np.argmin(G1)]),
                            "leitura": "cauda de perdas com α de Hill ~3–4: variância finita, curtose alta; um pregão de 1987 vale 26 desvios-padrão diários"},
            "Fluxonomia": {"informacao_mutua_sinal_ontem_hoje_bits": mi, "laco_performativo_individual": False,
                           "laco_performativo_agregado": "sim, fora do modelo: chamadas de margem (1929) e seguro de portfólio (1987) vendem na queda e a ampliam",
                           "leitura": "o sinal de ontem quase não informa o de hoje; o investidor individual não move o índice"},
            "Morfonomia": {"kruskal_G_entre_periodos_p": float(kw.pvalue), "levene_V_entre_periodos_p": float(lev.pvalue),
                           "kelly_quad_por_periodo": [float(kelly_quad(np.mean(x), np.mean(x * x))) for x in rp],
                           "invariante": bool(kw.pvalue > 0.01 and lev.pvalue > 0.01), **self._rival(G1)},
            "Crisonomia": {"placar": ctx["f3"]["placar"], "evpi_posterior": estat.evpi(q[::10], self.utilidade, self.acoes())["evpi"],
                           "leitura": "o EVPI posterior em unidades de V̄ mede quanto a incerteza sobre L* ainda custa na escolha de L"},
        }

    def _rival(self, G):
        n = G.size
        mu, sd = G.mean(), G.std()
        ll_n = float(np.sum(stats.norm.logpdf(G, mu, sd)))
        df, lo, sc = stats.t.fit(G)
        ll_t = float(np.sum(stats.t.logpdf(G, df, lo, sc)))
        bn, bt = -2 * ll_n + 2 * np.log(n), -2 * ll_t + 3 * np.log(n)
        return {"bic_normal_anual": float(bn), "bic_t_anual": float(bt), "t_graus_liberdade": float(df), "mdl_prefere": "normal" if bn <= bt else "t de Student"}

    def _crsp(self, tr):
        """Conferência opcional: DJIA vs. índice CRSP (Ecdat::CRSPday), 1989–1998, nos pregões comuns."""
        if not ARQ_CRSP.exists():
            return {"disponivel": False}
        with open(ARQ_CRSP, newline="") as fh:
            c = {f"{int(x['year']):04d}-{int(x['month']):02d}-{int(x['day']):02d}": float(x["crsp"]) for x in csv.DictReader(fh)}
        d = np.asarray(tr["data"])
        r = np.asarray(tr["retorno"], float)
        m = np.array([x in c for x in d])
        if m.sum() < 100:
            return {"disponivel": False}
        rd, rc = r[m], np.array([c[x] for x in d[m]])
        return {"disponivel": True, "pregoes_comuns": int(m.sum()), "correlacao_diaria": float(np.corrcoef(rd, rc)[0, 1]),
                "kelly_quad_djia": float(kelly_quad(rd.mean(), np.mean(rd * rd))), "kelly_quad_crsp": float(kelly_quad(rc.mean(), np.mean(rc * rc))),
                "nota": "o CRSP inclui dividendos e é ponderado por valor; serve para checar a ordem de grandeza de μ e σ, não para substituir o DJIA"}

    def critica(self, ctx):
        _, G1, _ = self._anuais(ctx["treino"])
        return {"rival": {"modelo": "t de Student para o ln-crescimento anual (caudas pesadas por ano)", **self._rival(G1)},
                "performatividade": "nula para o investidor individual (agente passivo); no agregado, produtos alavancados que rebalanceiam no "
                                    "fechamento compram na alta e vendem na queda, e esse laço não está nos dados nem no modelo",
                "premissas": ["índice só de preço: sem dividendos (subestima μ em ~3–4% a.a.)",
                              "alavancagem sem custo de financiamento (não há taxa de juros nos dados): favorece a alegação; os dois vieses "
                              "se compensam só se dividendos ≈ juros, o que não é garantido",
                              "rebalanceamento exatamente no fechamento, sem custos, sem chamada de margem intradiária",
                              "anos-unidade i.i.d.: ignora regimes de vários anos (1929–1954, 1966–1982)",
                              "1885 e 2023 são anos parciais; 1914 tem 135 dias sem pregão tratados como um único retorno",
                              "composição do DJIA trocada ao longo do tempo (viés de sobrevivência) e mercado dos EUA é um sobrevivente entre países"],
                "conferencia_crsp": self._crsp(ctx["treino"])}

    # ---------------- F8 ----------------
    def acoes(self):
        return list(LS)

    def utilidade(self, acao, theta):
        """Taxa temporal anual em unidades de V̄: L·θ − L²/2 com θ = L* (só a ordem entre ações importa)."""
        return float(acao * theta - acao * acao / 2)

    def decisao(self, ctx):
        q = ctx["f4"]["_q"]
        Lg = np.linspace(0, 4, 401)
        eu = np.array([np.mean(L * q[::20] - L * L / 2) for L in Lg])
        L_star = float(Lg[np.argmax(eu)])
        rng = np.random.default_rng(ctx["semente"] + 8)
        tr = ctx["treino"]
        est = {"3× (alegação)": 3.0, f"Kelly ({L_star:.2f}×)": L_star, f"meio Kelly ({L_star / 2:.2f}×)": L_star / 2, "1× (sem alavancagem)": 1.0}
        anuais = {nome: self._anuais(tr, L)[1] for nome, L in est.items()}
        n = next(iter(anuais.values())).size
        sorteio = rng.integers(0, n, (10_000, HORIZONTE_ANOS))       # bootstrap por ano-unidade
        res, hops = {}, {}
        for nome, G in anuais.items():
            lnw = G[sorteio].sum(1)
            res[nome] = {"mediana_riqueza": float(np.exp(np.median(lnw))), "de_100_acima_do_inicio": round(100 * float(np.mean(lnw > 0))),
                         "de_100_perdem_mais_da_metade": round(100 * float(np.mean(lnw < np.log(0.5)))),
                         "media_riqueza_conjunto": float(np.mean(np.exp(np.minimum(lnw, 700))))}
            hops[nome] = lnw / np.log(10)
        k3 = "3× (alegação)"
        kk = f"Kelly ({L_star:.2f}×)"
        p_E = res[k3]["de_100_perdem_mais_da_metade"] / 100
        return {
            "L_kelly_posterior": L_star,
            "estrategias_20_anos": res,
            "arvore_CL": {"evento": "perder mais da metade da riqueza em 20 anos", "acao_protetora": f"alavancar no máximo {L_star:.2f}× (Kelly) em vez de 3×",
                          "P_evento_3x": p_E, "regra": "proteger se P(evento) > C/L", "limiar_robusto": f"a decisão só se inverteria se C/L > {p_E:.2f}"},
            "frequencias_naturais": (f"De cada 100 investidores que seguram o Dow 3× por 20 anos (anos reais de 1885–1999 sorteados), "
                                     f"{res[k3]['de_100_acima_do_inicio']} terminam acima do que tinham e {res[k3]['de_100_perdem_mais_da_metade']} perdem mais da metade. "
                                     f"Com {L_star:.2f}×, {res[kk]['de_100_acima_do_inicio']} de cada 100 terminam acima; sem alavancagem, "
                                     f"{res['1× (sem alavancagem)']['de_100_acima_do_inicio']}."),
            "classe_de_referencia": "investidores que rebalanceiam diariamente no DJIA só-preço, sem custo de financiamento, horizonte de 20 anos",
            "circuit_breaker": self.termo_escopo()["circuit_breaker"],
            "hops_titulo": "Riqueza após 20 anos (anos reais reamostrados)", "hops_rotulo": "log10(riqueza final / inicial)", "_hops": hops,
        }

    def disjuntor(self, dados, ctx):
        r = np.asarray(dados["retorno"], float)
        V_rec = float(np.mean(r[-JANELA_DISJUNTOR:] ** 2))
        V_tr = float(np.mean(np.asarray(ctx["treino"]["retorno"], float) ** 2))
        L_sup = float(np.quantile(ctx["f4"]["_q"], 0.975) * V_tr / V_rec)
        return {"disparou": L_sup < 1.0, "L_kelly_recente_q975": L_sup, "limiar": 1.0, "janela_pregoes": JANELA_DISJUNTOR,
                "variancia_diaria_recente": V_rec, "variancia_diaria_treino": V_tr,
                "motivo": "variância recente leva L* abaixo de 1: desalavancar" if L_sup < 1.0 else "dentro do escopo"}
