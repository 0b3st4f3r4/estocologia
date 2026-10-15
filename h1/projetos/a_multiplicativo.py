"""Projeto A · Aditivo vs. multiplicativo (Crononomia, M7/M8; Lab 8, a moeda de Peters)."""

from __future__ import annotations

import numpy as np
from scipy import special, stats

from estocologia import estat
from estocologia.projeto import Projeto

CARA, COROA = 1.5, 0.6
T = 100                       # rodadas por investidor nos dados
N_INV = 400                   # investidores por coorte (período)
UFS = ["AC", "AL", "AM", "AP", "BA", "CE", "DF", "ES", "GO", "MA", "MG", "MS", "MT", "PA", "PB", "PE",
       "PI", "PR", "RJ", "RN", "RO", "RR", "RS", "SC", "SE", "SP", "TO"]
REGIAO = {**{u: "N" for u in ("AC", "AM", "AP", "PA", "RO", "RR", "TO")}, **{u: "NE" for u in ("AL", "BA", "CE", "MA", "PB", "PE", "PI", "RN", "SE")},
          **{u: "CO" for u in ("DF", "GO", "MS", "MT")}, **{u: "SE" for u in ("ES", "MG", "RJ", "SP")}, **{u: "S" for u in ("PR", "RS", "SC")}}
FAIXAS = ["18-29", "30-44", "45-59", "60+"]
P_DERIVA = 0.40
LIMIAR_K = int(np.floor(T * np.log(1 / COROA) / np.log(CARA / COROA))) + 1   # menor nº de caras com riqueza final > 1


def g(f, p):
    """Taxa de crescimento temporal por rodada, E[ln(1 + f R)], apostando a fração f."""
    return p * np.log1p(0.5 * f) + (1 - p) * np.log1p(-0.4 * f)


def kelly(p):
    """f* = argmax g; fechado: f* = p/0,4 − (1−p)/0,5, truncado em [0, 1]."""
    return float(np.clip(p / 0.4 - (1 - p) / 0.5, 0, 1))


def ln_riqueza(k, t):
    return k * np.log(CARA) + (t - k) * np.log(COROA)


class ProjetoA(Projeto):
    nome = "a"
    titulo = "Aditivo vs. multiplicativo: a moeda de Peters"
    alegacao = ('"Este ativo paga +50% ou −40% com chance igual. O retorno esperado é +5% por rodada; '
                'logo, aplicar toda a riqueza nele é um bom investimento."')
    unidade = "investidor"
    colunas_modelo = ("investidor", "rodada", "cara")
    quase_ids = ("faixa_etaria", "uf")
    dim = 1
    custo_inferencia = 1e-6

    cabecalho = {
        "processo_gerador": "Mundo (L1): moeda i.i.d. com P(cara)=p; riqueza multiplicada por 1,5 ou 0,6 a cada rodada. "
                            "A alegação falha no Modelo (L2): lê a esperança de conjunto como o destino de cada investidor (L3).",
        "nao_linearidade": "A riqueza é produto de fatores; ln é côncava: E[ln M] = −0,053 < 0 < ln E[M] = 0,049 (Jensen). "
                           "Dinâmica multiplicativa: reportar mediana, quantis e eCDF da riqueza, nunca só a média.",
        "unidade_e_margem": "Unidade independente: o investidor (a trajetória), não a rodada. Margens por investidor; "
                            "as rodadas de um investidor compartilham a mesma riqueza acumulada.",
        "fisica_do_ruido": "Ruído de saltos (Bernoulli), não gaussiano. Estado quase absorvente: riqueza → 0 com probabilidade 1 "
                           "na aposta total. Agente passivo: a fração apostada não altera a moeda.",
        "escada_causal": "Degrau 2: a decisão é uma intervenção do(f); simulada diretamente. Sem confundidor: a moeda é exógena.",
    }

    # ---------------- F7 ----------------
    def traducao(self):
        return {
            "dominio": "finanças pessoais, investimento de varejo",
            "pergunta_substantiva": "Um investidor que aplica toda a riqueza, rodada após rodada, nesse ativo tende a enriquecer?",
            "unidade_independente": "o investidor ao longo do tempo (trajetória), não a rodada isolada",
            "glossario": {"retorno esperado": "média aritmética do retorno por rodada sobre muitos investidores",
                          "taxa de crescimento": "E[ln(fator)], o que uma trajetória experimenta no longo prazo",
                          "rebalanceamento": "repor a fração f da riqueza no ativo a cada rodada",
                          "alavancagem": "f > 1", "drawdown": "queda em relação ao máximo anterior",
                          "fração de Kelly": "f que maximiza a taxa de crescimento", "ruína": "riqueza que tende a zero",
                          "coorte": "grupo de investidores que começa no mesmo período",
                          "prospecto": "documento de oferta do ativo", "perfil de risco": "tolerância declarada do investidor"},
            "fontes_primarias": ["Peters, O. (2019). The ergodicity problem in economics. Nature Physics 15, 1216–1221.",
                                 "Kelly, J. L. (1956). A new interpretation of information rate. Bell System Technical Journal 35(4).",
                                 "Curso, Lab 8 (A média que ninguém vive)"],
        }

    def elicitacao(self):
        return {
            "quantidade": "p, a probabilidade de cara (alta de 50%) por rodada",
            "especialistas": [{"nome": "E1 (simulado: analista de risco)", "q25": 0.47, "q50": 0.50, "q75": 0.53},
                              {"nome": "E2 (simulado: estatístico)", "q25": 0.48, "q50": 0.50, "q75": 0.52},
                              {"nome": "E3 (simulado: gestor)", "q25": 0.45, "q50": 0.50, "q75": 0.55}],
            "protocolo": "SHELF v4: quartis individuais sem ancoragem, discussão, consenso RIO, ajuste Beta, validação facial",
            "nota": "ELICITAÇÃO SIMULADA: nenhum especialista humano foi consultado; os quartis representam a leitura do prospecto.",
        }

    def priori_atipica(self):
        return {"quantidade": "p sob prospecto otimista", "especialistas": [
            {"nome": "E4 (simulado: vendedor do ativo)", "q25": 0.55, "q50": 0.60, "q75": 0.65}]}

    def termo_escopo(self):
        return {
            "vale_para": "fator por rodada em {1,5; 0,6}, rodadas independentes, rebalanceamento sem custo, horizonte de 100 a 1 000 rodadas",
            "nao_vale_para": "custos de transação, impostos, saques, dependência entre rodadas, pagamentos que mudam com o tempo",
            "regra_de_decisao": "aplicar a fração de Kelly estimada, não a riqueza toda",
            "circuit_breaker": "se o limite superior de 97,5% de p numa janela recente cair abaixo de 0,444 (onde f* = 0), parar de aplicar",
            "assinatura": "sil (pendente)",
        }

    # ---------------- F2 ----------------
    def contrato(self):
        return {
            "nome": "rodadas_peters", "versao": "1.0",
            "colunas": {
                "investidor": {"tipo": "str"}, "periodo": {"tipo": "int", "min": 1, "max": 99},
                "rodada": {"tipo": "int", "min": 1, "max": T}, "cara": {"tipo": "int", "valores": [0, 1]},
                "faixa_etaria": {"tipo": "str", "valores": FAIXAS}, "uf": {"tipo": "str", "valores": UFS},
            },
            "chave": ["investidor", "rodada"], "particao": "periodo",
            "grao": "uma linha por investidor por rodada",
        }

    def gerar_lote(self, periodo, rng, cenario="mundo"):
        n = N_INV
        if cenario == "patologico":   # rodadas com memória: P(repetir o resultado anterior) = 0,75
            flips = np.empty((n, T), dtype=int)
            flips[:, 0] = rng.random(n) < 0.5
            for t in range(1, T):
                fica = rng.random(n) < 0.75
                flips[:, t] = np.where(fica, flips[:, t - 1], 1 - flips[:, t - 1])
        else:
            p = P_DERIVA if cenario == "deriva" else 0.5
            flips = (rng.random((n, T)) < p).astype(int)
        faixa = rng.choice(FAIXAS, n)
        uf = rng.choice(UFS, n)
        linhas = []
        for i in range(n):
            iid = f"A{periodo:02d}-{i:04d}"
            for t in range(T):
                linhas.append({"investidor": iid, "periodo": periodo, "rodada": t + 1, "cara": int(flips[i, t]),
                               "faixa_etaria": str(faixa[i]), "uf": str(uf[i])})
        return linhas

    def generalizar(self, linha):
        return {**linha, "uf": REGIAO[linha["uf"]]}

    # ---------------- modelo ----------------
    def _matriz(self, dados):
        flips = np.asarray(dados["cara"], dtype=int)
        return flips.reshape(-1, T)        # dados ordenados por (investidor, rodada)

    def suficientes(self, dados):
        f = np.asarray(dados["cara"], dtype=int)
        return {"k": int(f.sum()), "n": int(f.size)}

    def logpost(self, x, suf, priori):
        x = float(np.ravel(x)[0])
        lp, l1p = -np.logaddexp(0, -x), -np.logaddexp(0, x)
        return (suf["k"] + priori["a"]) * lp + (suf["n"] - suf["k"] + priori["b"]) * l1p   # inclui o jacobiano p(1−p)

    def amostra_priori(self, priori, rng):
        return np.array([special.logit(rng.beta(priori["a"], priori["b"]))])

    def simular_suf(self, x, rng):
        n = 500
        return {"k": int(rng.binomial(n, special.expit(x[0]))), "n": n}

    def quantidade(self, x):
        return float(special.expit(np.ravel(x)[0]))

    def x_inicial(self, suf):
        return np.array([special.logit((suf["k"] + 0.5) / (suf["n"] + 1))])

    def nulo(self, dados, rng):
        """Teste severo: no modelo da alegação (aditivo), que fração dos investidores terminaria acima do início?"""
        m = self._matriz(dados)
        k = m.sum(1)
        obs = float(np.mean(ln_riqueza(k, T) > 0))
        p = k.sum() / m.size
        sims = np.array([np.mean(0.5 * kk - 0.4 * (T - kk) > 0) for kk in rng.binomial(T, p, (2000, k.size))])
        return {"estatistica": "fração de investidores acima da riqueza inicial em 100 rodadas",
                "observado_multiplicativo": obs, "nulo_aditivo_media": float(sims.mean()), "nulo_aditivo_q01": float(np.quantile(sims, 0.01)),
                "p_valor": float(np.mean(sims <= obs)), "n_investidores": int(k.size),
                "leitura": "o mundo multiplicativo nunca produziria o que o aditivo promete, e vice-versa: a alegação usa o modelo errado"}

    def ppc(self, dados, amostras_x, rng):
        """Checagem preditiva: autocorrelação de 1ª ordem das rodadas dentro de cada investidor."""
        m = self._matriz(dados)

        def ac(mm):
            a, b = mm[:, :-1], mm[:, 1:]
            return float(np.mean((a - a.mean()) * (b - b.mean())) / mm.var())

        obs = ac(m)
        ps = special.expit(amostras_x[rng.integers(0, len(amostras_x), 300), 0])
        rep = np.array([ac((rng.random(m.shape) < p).astype(int)) for p in ps])
        pv = float(2 * min(np.mean(rep >= obs), np.mean(rep <= obs)))
        return {"estatistica": "autocorrelação lag-1 das rodadas", "observado": obs, "replicas_q025_q975": np.quantile(rep, [0.025, 0.975]).tolist(), "p_valor": pv}

    def resultado_unidade(self, dados):
        k = self._matriz(dados).sum(1)
        return ln_riqueza(k, T) / T      # g_T de cada investidor (aposta total)

    # ---------------- F3 ----------------
    def entradas_cofre(self, cofre):
        m = self._matriz(cofre)
        ids = np.asarray(cofre["investidor"]).reshape(-1, T)[:, 0]
        k50 = m[:, :50].sum(1)
        return {"ids": ids.tolist(), "k50": k50, "deriva": k50 / 50}

    def previsoes(self, treino, entradas, amostras_x):
        ps = special.expit(amostras_x[:: max(1, len(amostras_x) // 400), 0])
        k_tr = self._matriz(treino).sum(1)
        base = float(np.mean(ln_riqueza(k_tr, T) > 0))
        out = []
        for iid, k50 in zip(entradas["ids"], entradas["k50"]):
            falta = LIMIAR_K - int(k50)
            p_mod = float(np.mean(stats.binom.sf(falta - 1, 50, ps)))       # P(caras restantes ≥ falta)
            r50 = 0.5 * k50 - 0.4 * (50 - k50)                                 # leitura aditiva da alegação
            p_ale = float(stats.norm.sf(0, loc=r50 + 50 * 0.05, scale=np.sqrt(50) * 0.45))
            out.append({"id": iid, "evento": "termina 100 rodadas acima da riqueza inicial (aposta total)", "taxa_base": base,
                        "probs": {"agent-f4-frontier": p_mod, "alegacao-aditiva": p_ale},
                        "meta": {"fonte_filtro": "coorte do período 4, sem filtro de sobrevivência", "pode_influir": False,
                                 "flutuacao_ou_causa": "flutuação (moeda i.i.d.)", "n_fatores": 1, "degrau_pearl": 1}})
        return out

    def desfechos(self, cofre):
        m = self._matriz(cofre)
        ids = np.asarray(cofre["investidor"]).reshape(-1, T)[:, 0]
        return {i: int(ln_riqueza(k, T) > 0) for i, k in zip(ids, m.sum(1))}

    # ---------------- F5 / F6 ----------------
    def estatistica_publicada(self, dados):
        k = self._matriz(dados).sum(1)
        return int(np.sum(ln_riqueza(k, T) > 0)), int(k.size), "fração de investidores acima da riqueza inicial"

    def _simular_estrategias(self, p, rng, n=10_000, t=1000):
        ks = rng.binomial(t, p, n)
        out = {}
        for nome, f in (("aposta total (f = 1)", 1.0), (f"Kelly (f = {kelly(p):.2f})", kelly(p)), ("meio Kelly", kelly(p) / 2), ("não aplicar", 0.0)):
            lnw = ks * np.log1p(0.5 * f) + (t - ks) * np.log1p(-0.4 * f)
            out[nome] = lnw
        return out

    def justica(self, ctx):
        rng = np.random.default_rng(ctx["semente"] + 6)
        p_hat = float(np.mean(ctx["treino"]["cara"]))      # estimativa dos dados de treino, não o p verdadeiro
        sim = self._simular_estrategias(p_hat, rng)
        tot = sim["aposta total (f = 1)"]
        w = np.exp(tot - tot.max())
        top1 = float(np.sort(w)[::-1][: w.size // 100].sum() / w.sum())
        kel = sim[[k for k in sim if k.startswith("Kelly")][0]]
        return {
            "trilema_da_justica": "não se aplica: não há classificador nem grupos protegidos com decisões distintas",
            "quem_paga_o_erro": {"aposta_total": {"fatia_da_riqueza_do_1pct_mais_rico": top1, "frac_abaixo_de_1pct_da_riqueza_inicial": float(np.mean(tot < np.log(0.01))),
                                                  "mediana_riqueza_final": float(np.exp(np.median(tot)))},
                                 "kelly": {"frac_abaixo_da_riqueza_inicial": float(np.mean(kel < 0)), "mediana_riqueza_final": float(np.exp(np.median(kel)))}},
            "inversao_metrica": {"metrica_original": "riqueza média do conjunto (escolhe aposta total)",
                                 "metrica_invertida": "riqueza mediana / taxa temporal (escolhe Kelly)",
                                 "ganham": f"o investidor típico: mediana de {np.exp(np.median(tot)):.2e} para {np.exp(np.median(kel)):.1f} vezes a riqueza inicial",
                                 "perdem": f"o 1% mais sortudo, que sob aposta total concentraria {top1:.0%} de toda a riqueza"},
            "assimetria_de_poder": "quem vende o ativo vê a média do conjunto (comissões sobre o volume); quem compra vive uma trajetória",
        }

    # ---------------- M15 ----------------
    def lentes(self, ctx):
        q = ctx["f4"]["_q"]
        p = float(np.median(q))
        m = self._matriz(ctx["treino"])
        k = m.sum(1)
        x100 = np.exp(ln_riqueza(k, T))
        # Morfonomia: invariância entre ambientes (períodos de treino) e MDL i.i.d. vs Markov
        n_por = k.size // len(self.periodos[:-1])
        tab = np.array([[m[i * n_por:(i + 1) * n_por].sum(), m[i * n_por:(i + 1) * n_por].size - m[i * n_por:(i + 1) * n_por].sum()] for i in range(len(self.periodos) - 1)])
        chi2, p_icp, _, _ = stats.chi2_contingency(tab)
        bic = self._bic(m)
        placar = ctx["f3"]["placar"]
        return {
            "Crononomia": {"ln_E_M": float(np.log(p * CARA + (1 - p) * COROA)), "E_ln_M": float(p * np.log(CARA) + (1 - p) * np.log(COROA)),
                           "mediana_g_T_dados": float(np.median(ln_riqueza(k, T) / T)), "ergodico": False,
                           "leitura": "a esperança de conjunto cresce, a taxa temporal decresce: a média não é vivida por ninguém"},
            "Escalonomia": {"media_x100": float(x100.mean()), "mediana_x100": float(np.median(x100)),
                            "fatia_top1pct": float(np.sort(x100)[::-1][: x100.size // 100].sum() / x100.sum()),
                            "hill_alpha_k25_k50_k100": [estat.hill(x100, kk) for kk in (25, 50, 100)],
                            "leitura": "cauda lognormal (Hill instável com k): a média amostral existe mas é dominada por poucos"},
            "Fluxonomia": {"informacao_mutua_bits": 0.0, "laco_performativo": False,
                           "leitura": "o investidor não tem sinal sobre a moeda (I = 0 bits) e a fração f não altera a moeda"},
            "Morfonomia": {"icp_homogeneidade_entre_periodos_p": float(p_icp), "invariante": bool(p_icp > 0.01), **bic},
            "Crisonomia": {"placar": placar, "evpi_posterior_por_rodada": estat.evpi(q[::10], self.utilidade, self.acoes())["evpi"],
                           "leitura": "com a posterior atual, nenhuma rodada extra de dados mudaria a fração escolhida: coletar mais é curiosidade sem uso"},
        }

    def _bic(self, m):
        a, b = m[:, :-1].ravel(), m[:, 1:].ravel()
        n = b.size
        p = b.mean()
        ll_iid = n * (p * np.log(p) + (1 - p) * np.log(1 - p))
        ll_mk = 0.0
        for s in (0, 1):
            bb = b[a == s]
            q = bb.mean()
            ll_mk += bb.size * (q * np.log(q) + (1 - q) * np.log(1 - q))
        return {"bic_iid": float(-2 * ll_iid + np.log(n)), "bic_markov1": float(-2 * ll_mk + 2 * np.log(n)),
                "mdl_prefere": "i.i.d." if -2 * ll_iid + np.log(n) <= -2 * ll_mk + 2 * np.log(n) else "Markov(1)"}

    def critica(self, ctx):
        bic = self._bic(self._matriz(ctx["treino"]))
        return {"rival": {"modelo": "Markov de 1ª ordem nas rodadas", **bic},
                "performatividade": "nenhuma: a moeda é exógena; com e sem a ação do(f) a distribuição das rodadas é a mesma por construção",
                "premissas": ["pagamentos +50%/−40% conhecidos e fixos (importados do prospecto, não estimados)",
                              "rodadas independentes entre investidores (sem choque comum de mercado)",
                              "sem custos de transação no rebalanceamento"]}

    # ---------------- F8 ----------------
    def acoes(self):
        return [0.0, 0.1, 0.25, 0.5, 1.0]

    def utilidade(self, acao, theta):
        return float(g(acao, theta))

    def decisao(self, ctx):
        q = ctx["f4"]["_q"]
        fs = np.linspace(0, 1, 201)
        eg = np.array([np.mean(g(f, q[::20])) for f in fs])
        f_star = float(fs[np.argmax(eg)])
        rng = np.random.default_rng(ctx["semente"] + 8)
        p_draw = rng.choice(q, 10_000)
        t = 1000
        ks = rng.binomial(t, p_draw)
        est = {"aposta total": 1.0, f"Kelly ({f_star:.2f})": f_star, "meio Kelly": f_star / 2, "não aplicar": 0.0}
        res, hops = {}, {}
        for nome, f in est.items():
            lnw = ks * np.log1p(0.5 * f) + (t - ks) * np.log1p(-0.4 * f)
            res[nome] = {"mediana_riqueza": float(np.exp(np.median(lnw))), "de_100_acima_do_inicio": round(100 * float(np.mean(lnw > 0))),
                         "de_100_perdem_mais_da_metade": round(100 * float(np.mean(lnw < np.log(0.5)))),
                         "media_riqueza_conjunto": float(np.mean(np.exp(np.minimum(lnw, 700))))}
            hops[nome] = lnw / np.log(10)
        p_E = res["aposta total"]["de_100_perdem_mais_da_metade"] / 100
        return {
            "f_kelly_posterior": f_star,
            "estrategias_1000_rodadas": res,
            "arvore_CL": {"evento": "perder mais da metade da riqueza em 1 000 rodadas", "acao_protetora": "aplicar a fração de Kelly em vez da riqueza toda",
                          "P_evento_aposta_total": p_E, "regra": "proteger se P(evento) > C/L",
                          "limiar_robusto": f"a decisão só se inverteria se C/L > {p_E:.2f}"},
            "frequencias_naturais": (f"De cada 100 investidores que aplicam tudo por 1 000 rodadas, {res['aposta total']['de_100_acima_do_inicio']} terminam acima do que tinham "
                                     f"e {res['aposta total']['de_100_perdem_mais_da_metade']} perdem mais da metade. Com a fração de Kelly, "
                                     f"{res[f'Kelly ({f_star:.2f})']['de_100_acima_do_inicio']} de cada 100 terminam acima."),
            "classe_de_referencia": "investidores que rebalanceiam a cada rodada nesse ativo, sem custos",
            "circuit_breaker": self.termo_escopo()["circuit_breaker"],
            "hops_titulo": "Riqueza após 1 000 rodadas", "hops_rotulo": "log10(riqueza final / inicial)", "_hops": hops,
        }

    def disjuntor(self, dados, ctx):
        f = np.asarray(dados["cara"], dtype=int)
        janela = f[-20_000:]
        k, n = int(janela.sum()), int(janela.size)
        sup = float(stats.beta.ppf(0.975, 1 + k, 1 + n - k))
        return {"disparou": sup < 0.444, "p_sup_975": sup, "limiar": 0.444, "janela": n,
                "motivo": "p abaixo do ponto em que f* = 0: parar de aplicar" if sup < 0.444 else "dentro do escopo"}
