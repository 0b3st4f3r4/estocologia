"""Projeto B · Ver não é fazer (M13; Lab 13, Parte 0): a vitamina, os hábitos e o infarto.

SCM do núcleo (o do lab, mantido exatamente):
    Z (hábitos saudáveis) ~ Bernoulli(0,5)
    X (toma vitamina)     ~ Bernoulli(0,2 + 0,5 Z)
    Y (infarto em 10 anos) ~ Bernoulli(0,12 + 0,02 X − 0,08 Z)

Fora do núcleo, o projeto registra:
    S (sexo) ~ uniforme em {F, M}, independente de Z e X, com efeito ±0,02 em Y. Como S é
      independente de (Z, X) e entra com sinais opostos e peso igual, P(Y | X, Z) marginal em S é
      exatamente a do lab: os números do núcleo continuam conferindo. S serve ao F6 (taxas-base
      diferentes por grupo) e é quase-identificador.
    W (internação por acidente em 10 anos), um controle negativo de desfecho: W ~ Bernoulli(0,06 − 0,03 Z).
      W depende dos hábitos, mas não da vitamina. O grafo assumido implica W ⊥ X | Z.
    idade e UF: quase-identificadores (F5) e a variável monitorada pela deriva (idade).
    braço: no período 4 (cofre), metade das pessoas recebe a vitamina por loteria (do(X) sorteado),
      o que permite checar o degrau 2 da escada de Pearl out-of-time.
"""

from __future__ import annotations

import itertools

import numpy as np
from scipy import special, stats

from estocologia import estat
from estocologia.projeto import Projeto

N_PESSOAS = 20_000            # pessoas por período (coorte)
P_ALVO = 0.5                  # P(Z = 1) na população-alvo (importada; ver termo de escopo)
EFEITO_SEXO = 0.02            # ± em P(Y), fora do núcleo
FRAC_LOTERIA = 0.5            # fração do período do cofre com X sorteado
CUSTO = 0.002                 # custo de tomar a vitamina 10 anos, em infartos-equivalentes por pessoa
LIMIAR_RISCO = 0.09           # F6: sinalizar para o programa de prevenção quem tem risco previsto > 9%
UFS = ["AC", "AL", "AM", "AP", "BA", "CE", "DF", "ES", "GO", "MA", "MG", "MS", "MT", "PA", "PB", "PE",
       "PI", "PR", "RJ", "RN", "RO", "RR", "RS", "SC", "SE", "SP", "TO"]
PESO_UF = np.array([0.4, 1.6, 1.9, 0.4, 6.9, 4.3, 1.4, 1.9, 3.4, 3.3, 10.0, 1.4, 1.8, 4.0, 1.9, 4.5,
                    1.6, 5.6, 8.0, 1.6, 0.8, 0.3, 5.3, 3.7, 1.1, 21.9, 0.7])
PESO_UF = PESO_UF / PESO_UF.sum()     # pesos populacionais aproximados: UFs pequenas criam células raras
REGIAO = {**{u: "N" for u in ("AC", "AM", "AP", "PA", "RO", "RR", "TO")}, **{u: "NE" for u in ("AL", "BA", "CE", "MA", "PB", "PE", "PI", "RN", "SE")},
          **{u: "CO" for u in ("DF", "GO", "MS", "MT")}, **{u: "SE" for u in ("ES", "MG", "RJ", "SP")}, **{u: "S" for u in ("PR", "RS", "SC")}}

# Índices dos 4 parâmetros amostrados pelo MCMC: logit P(Y = 1 | X = x, Z = z)
CELULAS = ((0, 0), (0, 1), (1, 0), (1, 1))     # (x, z)
I_BASAL = 0                                     # P(Y | X = 0, Z = 0): risco basal, o parâmetro elicitado

GRAFO = {"Z": [], "S": [], "X": ["Z"], "Y": ["X", "Z", "S"], "W": ["Z"]}
GRAFO_U = {"Z": [], "U": [], "S": [], "X": ["Z", "U"], "Y": ["X", "Z", "U", "S"], "W": ["Z", "U"]}


# =====================================================================================
# O SCM e o estimador de ajuste (funções puras, testadas em tests/test_b.py)
# =====================================================================================
def p_y(x, z, s=None, u=0, cenario="mundo"):
    """P(Y = 1 | X, Z[, S, U]). Com s=None devolve a marginal em S (a do núcleo do lab)."""
    x, z, u = np.asarray(x, float), np.asarray(z, float), np.asarray(u, float)
    if cenario == "patologico":   # confundidor não medido U, que também reduz o risco
        base = 0.12 + 0.02 * x - 0.06 * z - 0.03 * u
    else:
        base = 0.12 + 0.02 * x - 0.08 * z
    if s is None:
        return base
    return base + EFEITO_SEXO * (2 * np.asarray(s, float) - 1)


def p_x(z, u=0, cenario="mundo"):
    z, u = np.asarray(z, float), np.asarray(u, float)
    if cenario == "patologico":
        return 0.1 + 0.4 * z + 0.3 * u
    return 0.2 + 0.5 * z


def p_w(z, u=0, cenario="mundo"):
    z, u = np.asarray(z, float), np.asarray(u, float)
    if cenario == "patologico":
        return 0.09 - 0.03 * z - 0.05 * u
    return 0.06 - 0.03 * z


def simular_scm(n, rng, cenario="mundo", do_x=None, pz=0.5, frac_loteria=0.0):
    """Sorteia n pessoas do SCM. do_x fixa X (cirurgia); frac_loteria sorteia X ~ Bernoulli(1/2)
    para essa fração das pessoas (o braço do experimento), independentemente de Z."""
    z = (rng.random(n) < pz).astype(int)
    u = (rng.random(n) < 0.5).astype(int) if cenario == "patologico" else np.zeros(n, int)
    s = (rng.random(n) < 0.5).astype(int)
    braco = (rng.random(n) < frac_loteria).astype(int)
    if do_x is not None:
        x = np.full(n, int(do_x))
    else:
        x_obs = (rng.random(n) < p_x(z, u, cenario)).astype(int)
        x_lot = (rng.random(n) < 0.5).astype(int)
        x = np.where(braco == 1, x_lot, x_obs)
    y = (rng.random(n) < p_y(x, z, s, u, cenario)).astype(int)
    w = (rng.random(n) < p_w(z, u, cenario)).astype(int)
    return {"z": z, "x": x, "y": y, "s": s, "w": w, "u": u, "braco": braco}


def contagens(z, x, y):
    """k[i], n[i] para as 4 células (x, z) de CELULAS."""
    z, x, y = (np.asarray(a, int) for a in (z, x, y))
    k = np.array([int(np.sum(y[(x == xx) & (z == zz)])) for xx, zz in CELULAS])
    n = np.array([int(np.sum((x == xx) & (z == zz))) for xx, zz in CELULAS])
    return k, n


def ajuste(py_celulas, pz):
    """Fórmula de ajuste (porta dos fundos): P(y | do(x)) = Σ_z P(y | x, z) P(z), para x = 0 e 1."""
    p00, p01, p10, p11 = py_celulas
    return (1 - pz) * p00 + pz * p01, (1 - pz) * p10 + pz * p11


def observacional(py_celulas, pz, px0, px1):
    """P(y | x) = Σ_z P(y | x, z) P(z | x): o que um estudo observacional relata."""
    p00, p01, p10, p11 = py_celulas
    pz_x1 = pz * px1 / (pz * px1 + (1 - pz) * px0)
    pz_x0 = pz * (1 - px1) / (pz * (1 - px1) + (1 - pz) * (1 - px0))
    return (1 - pz_x0) * p00 + pz_x0 * p01, (1 - pz_x1) * p10 + pz_x1 * p11


def ajuste_empirico(z, x, y, pz=None):
    """Fórmula de ajuste com as frequências da amostra; pz=None usa P(Z) da própria amostra (como o lab)."""
    k, n = contagens(z, x, y)
    pz = float(np.mean(z)) if pz is None else pz
    return ajuste(k / np.maximum(n, 1), pz)


def rr_observacional(x, y):
    x, y = np.asarray(x, int), np.asarray(y, int)
    return float(y[x == 1].mean() / y[x == 0].mean())


def kleinberg_fpr(p, vpp, fnr):
    """Identidade de Chouldechova (2017): FPR = p/(1−p) · (1−VPP)/VPP · (1−FNR), para qualquer classificador."""
    return p / (1 - p) * (1 - vpp) / vpp * (1 - fnr)


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


# =====================================================================================
class ProjetoB(Projeto):
    nome = "b"
    titulo = "Ver não é fazer: a vitamina, os hábitos e o infarto"
    alegacao = '"Quem toma vitamina tem 21% menos infartos; logo, tomar vitamina protege."'
    unidade = "pessoa"
    colunas_modelo = ("pessoa", "periodo", "braco", "sexo", "z", "x", "y", "w")
    quase_ids = ("idade", "uf", "sexo")
    dim = 4
    custo_inferencia = 1e-6

    cabecalho = {
        "processo_gerador": "Mundo (L1): SCM Z → X, Z → Y, X → Y, com Z (hábitos) ~ B(0,5), X ~ B(0,2 + 0,5 Z), "
                            "Y ~ B(0,12 + 0,02 X − 0,08 Z). A alegação falha no Modelo (L2): lê P(y | x) como P(y | do(x)) "
                            "e conclui (L3) uma proteção que o mecanismo não tem.",
        "nao_linearidade": "Riscos aditivos dentro do estrato, mas a mistura sobre Z inverte o sinal (Simpson): "
                           "o RR bruto 0,79 e o RR causal 1,25 têm sinais de efeito opostos. Reportar risco absoluto, não só relativo.",
        "unidade_e_margem": "Unidade independente: a pessoa. Margens por pessoa; quatro coortes (períodos) de 20 000; "
                            "o período 4 fica no cofre e tem um braço sorteado.",
        "fisica_do_ruido": "Ruído de Bernoulli, limitado em [0, 1]; sem caudas. O que domina a incerteza não é o acaso "
                           "amostral (60 000 pessoas), e sim a premissa não testável de que Z bloqueia toda porta dos fundos.",
        "escada_causal": "Degrau 2 (intervenção): identificado pela porta dos fundos com {Z}; conferido out-of-time por "
                         "um braço sorteado (do(X) por loteria). A manchete está no degrau 1.",
    }

    # ---------------- F7 ----------------
    def traducao(self):
        return {
            "dominio": "epidemiologia cardiovascular, suplementação alimentar",
            "pergunta_substantiva": "Se uma pessoa passar a tomar a vitamina, o risco dela de infarto em dez anos cai?",
            "unidade_independente": "a pessoa (adulto acompanhado por dez anos)",
            "glossario": {"risco relativo": "razão entre a incidência em quem toma e em quem não toma",
                          "risco absoluto": "fração de pessoas com infarto em dez anos",
                          "confundidor": "causa comum de tomar a vitamina e de ter infarto (aqui, os hábitos saudáveis)",
                          "efeito do usuário saudável": "quem adere a suplementos já tem hábitos que protegem",
                          "porta dos fundos": "caminho X ← Z → Y que cria associação sem causação",
                          "fórmula de ajuste": "Σ_z P(y | x, z) P(z): troca observação por intervenção se Z é admissível",
                          "ensaio randomizado": "X sorteado, o que corta a seta Z → X",
                          "controle negativo": "desfecho que a vitamina não pode causar; associação com ela denuncia confundimento",
                          "positividade": "toda combinação de Z tem quem tome e quem não tome",
                          "coorte": "pessoas que entram no estudo no mesmo período"},
            "fontes_primarias": ["Pearl, J. (2009). Causality, 2ª ed., cap. 3 (critério de porta dos fundos).",
                                 "Writing Group for the WHI Investigators (2002). JAMA 288(3), 321–333 (terapia hormonal: observação vs. ensaio).",
                                 "Lipsitch, Tchetgen Tchetgen e Cohen (2010). Negative controls. Epidemiology 21(3), 383–388.",
                                 "Curso, Lab 13, Parte 0 (Ver e fazer)"],
        }

    def elicitacao(self):
        return {
            "quantidade": "risco basal de infarto em dez anos: P(Y = 1 | sem vitamina, sem hábitos saudáveis)",
            "especialistas": [{"nome": "E1 (simulado: cardiologista clínico)", "q25": 0.10, "q50": 0.12, "q75": 0.145},
                              {"nome": "E2 (simulado: epidemiologista)", "q25": 0.095, "q50": 0.115, "q75": 0.135},
                              {"nome": "E3 (simulado: médico de família)", "q25": 0.105, "q50": 0.125, "q75": 0.15}],
            "protocolo": "SHELF v4: quartis individuais sem ancoragem, discussão, consenso RIO, ajuste Beta, validação facial",
            "entra_no_parametro": "logit P(Y | X = 0, Z = 0); os outros três riscos e P(Z), P(X | Z) recebem Beta(1, 1)",
            "nota": "ELICITAÇÃO SIMULADA: nenhum especialista humano foi consultado; os quartis imitam tabelas de risco cardiovascular.",
        }

    def priori_atipica(self):
        return {"quantidade": "risco basal sob a leitura do fabricante da vitamina", "especialistas": [
            {"nome": "E4 (simulado: departamento médico do fabricante)", "q25": 0.20, "q50": 0.25, "q75": 0.30}],
            "nota": "ELICITAÇÃO SIMULADA: o fabricante superestima o risco de quem não toma, para valorizar o produto."}

    def termo_escopo(self):
        return {
            "vale_para": "adultos de 40 a 79 anos de uma população com P(hábitos saudáveis) ≈ 0,5, horizonte de dez anos, "
                         "a vitamina na dose do estudo, sob a premissa de que os hábitos Z bloqueiam toda porta dos fundos",
            "nao_vale_para": "outras doses, populações com outra mistura de hábitos (o efeito é padronizado para P(Z) = 0,5), "
                             "desfechos além do infarto, mundos com confundidor não medido (o controle negativo W monitora isso)",
            "regra_de_decisao": "recomendar a vitamina só se a redução absoluta de risco causal superar o custo C/L; "
                                "com a posterior atual, não recomendar",
            "circuit_breaker": "parar de usar a estimativa se, num lote recente, P(Z = 1) se afastar mais de 0,08 da população-alvo, "
                               "se a positividade cair (P(X | Z) < 2% ou > 98% em algum estrato), ou se o controle negativo W "
                               "ficar associado a X dentro dos estratos de Z (|z| > 4)",
            "assinatura": "sil (pendente)",
        }

    # ---------------- F2 ----------------
    def contrato(self):
        return {
            "nome": "coorte_vitamina", "versao": "1.0",
            "colunas": {
                "pessoa": {"tipo": "str"},
                "periodo": {"tipo": "int", "min": 1, "max": 99},
                "braco": {"tipo": "int", "valores": [0, 1]},
                "idade": {"tipo": "int", "min": 18, "max": 110},
                "uf": {"tipo": "str", "valores": UFS},
                "sexo": {"tipo": "str", "valores": ["F", "M"]},
                "z": {"tipo": "int", "valores": [0, 1]},
                "x": {"tipo": "int", "valores": [0, 1]},
                "y": {"tipo": "int", "valores": [0, 1]},
                "w": {"tipo": "int", "valores": [0, 1]},
            },
            "chave": ["pessoa"], "particao": "periodo",
            "grao": "uma linha por pessoa (seguimento de dez anos)",
        }

    def gerar_lote(self, periodo, rng, cenario="mundo", n=None):
        n = N_PESSOAS if n is None else n
        frac = FRAC_LOTERIA if (periodo == self.periodos[-1] and cenario == "mundo") else 0.0
        if cenario == "deriva":
            # a vitamina chega às farmácias populares: público mais velho e com menos hábitos saudáveis
            v = simular_scm(n, rng, "mundo", pz=0.25)
            idade = rng.integers(55, 90, n)
        else:
            v = simular_scm(n, rng, cenario, frac_loteria=frac)
            idade = rng.integers(40, 80, n)
        uf = rng.choice(len(UFS), n, p=PESO_UF)
        cols = {k: v[k].tolist() for k in ("z", "x", "y", "w", "s", "braco")}
        idade, uf = idade.tolist(), uf.tolist()
        return [{"pessoa": f"B{periodo:02d}-{i:05d}", "periodo": int(periodo), "braco": cols["braco"][i],
                 "idade": idade[i], "uf": UFS[uf[i]], "sexo": "M" if cols["s"][i] else "F",
                 "z": cols["z"][i], "x": cols["x"][i], "y": cols["y"][i], "w": cols["w"][i]} for i in range(n)]

    def generalizar(self, linha):
        i = int(linha["idade"])
        return {**linha, "idade": f"{10 * (i // 10)}-{10 * (i // 10) + 9}", "uf": REGIAO[linha["uf"]]}

    # ---------------- modelo ----------------
    # Modelo colapsado. Os parâmetros são P(Z), P(X | Z = 0), P(X | Z = 1) e os 4 riscos P(Y | X, Z),
    # todos Bernoulli com priori Beta. A verossimilhança fatora e as prioris são independentes, então
    # a posterior também fatora: P(Z) e P(X | Z) têm posterior Beta fechada (usada no PPC, no nulo e
    # na deriva), e o MCMC percorre só os 4 logits de risco de que a quantidade depende. Motivo
    # numérico: o Metropolis do framework (4 × 3 000, passo fixo) não passa R-hat < 1,01 com d ≥ 5
    # em cerca de 40% das sementes (testado); com d = 4 passa em todas as testadas.
    # A quantidade é o efeito padronizado para a população-alvo, P(Z = 1) = 0,5 (padronização direta),
    # e não para a mistura da amostra: o disjuntor checa que a amostra continua parecida com o alvo.
    def suficientes(self, dados):
        k, n = contagens(dados["z"], dados["x"], dados["y"])
        z, x = np.asarray(dados["z"], int), np.asarray(dados["x"], int)
        return {"k": k.tolist(), "n": n.tolist(),
                "kz": int(z.sum()), "nz": int(z.size),
                "kx": [int(x[z == 0].sum()), int(x[z == 1].sum())], "nx": [int((z == 0).sum()), int((z == 1).sum())]}

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
        v = simular_scm(2000, rng)                     # Z e X do mundo; Y dos riscos sorteados
        p = special.expit(np.asarray(x, float))
        k, n = contagens(v["z"], v["x"], v["y"])
        k = np.array([rng.binomial(nn, pp) for nn, pp in zip(n, p)])
        return {"k": k.tolist(), "n": n.tolist()}

    def quantidade(self, x):
        """Diferença de risco causal P(Y | do(X = 1)) − P(Y | do(X = 0)) pela fórmula de ajuste."""
        d0, d1 = ajuste(special.expit(np.asarray(x, float)), P_ALVO)
        return float(d1 - d0)

    def x_inicial(self, suf):
        return special.logit((np.asarray(suf["k"]) + 0.5) / (np.asarray(suf["n"]) + 1))

    def _conjugadas(self, suf, rng, m):
        """Sorteios da posterior Beta fechada de P(Z), P(X | Z = 0), P(X | Z = 1) (prioris Beta(1, 1))."""
        pz = rng.beta(1 + suf["kz"], 1 + suf["nz"] - suf["kz"], m)
        px0 = rng.beta(1 + suf["kx"][0], 1 + suf["nx"][0] - suf["kx"][0], m)
        px1 = rng.beta(1 + suf["kx"][1], 1 + suf["nx"][1] - suf["kx"][1], m)
        return pz, px0, px1

    # ---------- conferência com o lab ----------
    def conferencia(self, dados, rng=None):
        rng = np.random.default_rng(1921) if rng is None else rng
        z, x, y = (np.asarray(dados[c], int) for c in ("z", "x", "y"))
        a0, a1 = ajuste_empirico(z, x, y)
        sim1 = simular_scm(400_000, rng, do_x=1)["y"].mean()
        sim0 = simular_scm(400_000, rng, do_x=0)["y"].mean()
        return {"rr_observacional": rr_observacional(x, y), "p_y_dado_toma": float(y[x == 1].mean()), "p_y_dado_nao_toma": float(y[x == 0].mean()),
                "rr_formula_de_ajuste": float(a1 / a0), "ajuste_do1": float(a1), "ajuste_do0": float(a0),
                "rr_intervencao_simulada_400k": float(sim1 / sim0), "do1_simulado": float(sim1), "do0_simulado": float(sim0),
                "p_habitos_dado_toma": float(z[x == 1].mean()), "p_habitos_dado_nao_toma": float(z[x == 0].mean()),
                "exato_no_scm": {"rr_observacional": float(np.divide(*observacional((0.12, 0.04, 0.14, 0.06), 0.5, 0.2, 0.7)[::-1])),
                                 "rr_causal": float(np.divide(*ajuste((0.12, 0.04, 0.14, 0.06), 0.5)[::-1]))},
                "lab": "Lab 13, Exercício 0: RR observacional 0,79; do(X) 1,24; ajuste 1,26",
                "confere": bool(0.74 < y[x == 1].mean() / y[x == 0].mean() < 0.84 and 1.15 < a1 / a0 < 1.36 and 1.18 < sim1 / sim0 < 1.30)}

    # ---------- F1 / M5: modelo nulo e teste severo ----------
    def nulo(self, dados, rng):
        """Mundo nulo: a vitamina não tem efeito nenhum (P(Y | X, Z) = P(Y | Z)), com o mesmo confundimento por Z,
        ajustado aos dados. Quanto o RR observacional de 'proteção' aparece mesmo assim?"""
        z, x, y = (np.asarray(dados[c], int) for c in ("z", "x", "y"))
        N = z.size
        pz, px = z.mean(), [x[z == zz].mean() for zz in (0, 1)]
        pyz = [y[z == zz].mean() for zz in (0, 1)]
        B = 4000
        nz1 = rng.binomial(N, pz, B)
        nz = np.stack([N - nz1, nz1])
        n1 = np.stack([rng.binomial(nz[zz], px[zz]) for zz in (0, 1)])
        n0 = nz - n1
        e1 = np.stack([rng.binomial(n1[zz], pyz[zz]) for zz in (0, 1)])
        e0 = np.stack([rng.binomial(n0[zz], pyz[zz]) for zz in (0, 1)])
        rr_obs_nulo = (e1.sum(0) / n1.sum(0)) / (e0.sum(0) / n0.sum(0))
        aj1 = ((e1 / np.maximum(n1, 1)) * (nz / N)).sum(0)
        aj0 = ((e0 / np.maximum(n0, 1)) * (nz / N)).sum(0)
        rr_aj_nulo = aj1 / aj0
        obs = rr_observacional(x, y)
        a0, a1 = ajuste_empirico(z, x, y)
        # o nulo "do enunciado": coeficiente de X zerado e o resto do SCM do lab intacto
        pz_x1 = 0.5 * 0.7 / (0.5 * 0.7 + 0.5 * 0.2)
        pz_x0 = 0.5 * 0.3 / (0.5 * 0.3 + 0.5 * 0.8)
        rr_nulo_lab = ((1 - pz_x1) * 0.12 + pz_x1 * 0.04) / ((1 - pz_x0) * 0.12 + pz_x0 * 0.04)
        return {"estatistica": "RR observacional P(infarto | toma) / P(infarto | não toma)",
                "nulo": "vitamina sem efeito, P(Y | X, Z) = P(Y | Z) estimada, mesmo P(Z) e P(X | Z) dos dados",
                "observado": obs, "nulo_media": float(rr_obs_nulo.mean()), "nulo_q025_q975": np.quantile(rr_obs_nulo, [0.025, 0.975]).tolist(),
                "nulo_lab_coef_X_zerado": float(rr_nulo_lab),
                "P_nulo_RR_menor_ou_igual_ao_observado": float(np.mean(rr_obs_nulo <= obs)),
                "severidade_da_alegacao": float(np.mean(rr_obs_nulo > obs)),
                "severo": bool(np.mean(rr_obs_nulo > obs) > 0.95),
                "teste_ajustado": {"estatistica": "RR pela fórmula de ajuste", "observado": float(a1 / a0), "nulo_media": float(rr_aj_nulo.mean()),
                                   "nulo_q025_q975": np.quantile(rr_aj_nulo, [0.025, 0.975]).tolist(),
                                   "p_valor_unilateral": float(np.mean(rr_aj_nulo >= a1 / a0))},
                "n_pessoas": int(N),
                "leitura": "No mundo em que a vitamina não faz nada, o confundimento sozinho produz RR observacional "
                           f"≈ {rr_obs_nulo.mean():.2f}, ainda mais 'protetor' que o observado {obs:.2f}: o padrão da manchete "
                           "sobrevive ao nulo, logo a evidência observacional NÃO é severa para 'a vitamina protege' "
                           "(se a alegação fosse falsa, o estudo a confirmaria do mesmo jeito). O teste ajustado por Z é severo e "
                           f"aponta o contrário: RR ajustado {a1 / a0:.2f}, fora do nulo."}

    # ---------- PPC (M15 / MB4) ----------
    def ppc(self, dados, amostras_x, rng):
        """Checagem preditiva de uma implicação testável do grafo assumido: W ⊥ X | Z.

        Por que não uma checagem da conjunta (Z, X, Y): o modelo é saturado nela, e qualquer tabela 2×2×2
        é reproduzida perfeitamente; nenhuma estatística de (Z, X, Y) o falsearia. O que pode falhar é o
        grafo. O controle negativo W (internação por acidente) depende dos hábitos e não da vitamina; se
        o grafo está certo, dentro de cada estrato de Z ele é independente de X. Um confundidor não medido
        U (o cenário "patologico": 'consciência sanitária', que leva a tomar a vitamina e a ter menos
        acidentes e infartos) deixa um rastro aí, e é exatamente o que invalida a porta dos fundos com {Z}.

        Réplicas: X_rep ~ Bernoulli(P(X | Z)) com P(X | Z) sorteado da posterior conjugada, W e Z fixos.
        Estatística: diferença de taxa de W entre quem toma e quem não toma, padronizada pelos estratos de Z."""
        z, x, w = (np.asarray(dados[c], int) for c in ("z", "x", "w"))

        def T(xx):
            out = 0.0
            for zz in (0, 1):
                s = z == zz
                n1 = xx[..., s].sum(-1)
                n0 = s.sum() - n1
                w1 = (xx[..., s] * w[s]).sum(-1)
                w0 = w[s].sum() - w1
                out = out + s.mean() * (w1 / np.maximum(n1, 1) - w0 / np.maximum(n0, 1))
            return out

        obs = float(T(x))
        suf = self.suficientes(dados)
        R = 300
        _, px0, px1 = self._conjugadas(suf, rng, R)
        rep = []
        for j in range(0, R, 50):     # em blocos, para não alocar R × n de uma vez
            pxi = np.where(z[None, :] == 1, px1[j:j + 50, None], px0[j:j + 50, None])
            rep.append(np.asarray(T((rng.random(pxi.shape, dtype=np.float32) < pxi).astype(np.int8))))
        rep = np.concatenate(rep)
        pv = float(min(1.0, 2 * min(np.mean(rep >= obs), np.mean(rep <= obs))))
        return {"estatistica": "controle negativo: P(W | X=1, Z) − P(W | X=0, Z), média nos estratos de Z",
                "implicacao_testada": "W ⊥ X | Z (d-separação no grafo assumido)", "observado": obs,
                "replicas_q025_q975": np.quantile(rep, [0.025, 0.975]).tolist(), "p_valor": pv,
                "z_padronizado": float((obs - rep.mean()) / rep.std())}

    def resultado_unidade(self, dados):
        # o desfecho da pessoa (0/1). Para um desfecho binário, o conjunto conforme a 90% é {0, 1} sempre que
        # a taxa-base fica entre 5% e 95%: a cobertura é garantida e o conjunto não é informativo (dito em `critica`).
        return np.asarray(dados["y"], float)

    # ---------------- F3 ----------------
    def entradas_cofre(self, cofre):
        return {"ids": np.asarray(cofre["pessoa"]).tolist(), "z": np.asarray(cofre["z"], int), "x": np.asarray(cofre["x"], int),
                "braco": np.asarray(cofre["braco"], int), "deriva": np.asarray(cofre["idade"], float)}

    def previsoes(self, treino, entradas, amostras_x):
        p = special.expit(amostras_x[:: max(1, len(amostras_x) // 2000)]).mean(0)     # preditiva posterior por célula
        cel = {c: p[i] for i, c in enumerate(CELULAS)}
        xt, yt = np.asarray(treino["x"], int), np.asarray(treino["y"], int)
        marg = {0: float(yt[xt == 0].mean()), 1: float(yt[xt == 1].mean())}
        base = float(yt.mean())
        out = []
        for iid, z, x, b in zip(entradas["ids"], entradas["z"].tolist(), entradas["x"].tolist(), entradas["braco"].tolist()):
            out.append({"id": iid, "evento": "infarto em dez anos", "taxa_base": base,
                        "probs": {"agent-f4-frontier": float(cel[(x, z)]), "alegacao-marginal": marg[x]},
                        "meta": {"fonte_filtro": "coorte do período 4, sem filtro; metade com X sorteado" if b else "coorte do período 4, sem filtro",
                                 "pode_influir": False, "flutuacao_ou_causa": "causa (efeito de X ajustado por Z)",
                                 "n_fatores": 2, "degrau_pearl": 2 if b else 1}})
        self._cofre_previsoes = {"celulas": {f"x{a}z{c}": float(v) for (a, c), v in cel.items()}, "marginal": marg, "base": base}
        return out

    def desfechos(self, cofre):
        y = np.asarray(cofre["y"], int)
        # chamado pelo F3 depois do pré-registro: guarda o cofre aberto para as lentes (degrau 2 out-of-time)
        self._cofre = {c: np.asarray(cofre[c]) for c in ("z", "x", "y", "braco")}
        return dict(zip(np.asarray(cofre["pessoa"]).tolist(), y.tolist()))

    # ---------------- F5 / F6 ----------------
    def estatistica_publicada(self, dados):
        x, y = np.asarray(dados["x"], int), np.asarray(dados["y"], int)
        return int(y[x == 1].sum()), int((x == 1).sum()), "incidência de infarto em dez anos entre quem toma a vitamina"

    def justica(self, ctx):
        d = ctx["treino"]
        per = np.asarray(d["periodo"], int)
        z, x, y = (np.asarray(d[c], int) for c in ("z", "x", "y"))
        s = (np.asarray(d["sexo"]) == "M").astype(int)
        ajuste_ = per < max(per)      # escore ajustado nos períodos 1–2, avaliado no 3
        aval = ~ajuste_
        esc = np.zeros(y.size)
        for ss, xx, zz in itertools.product((0, 1), repeat=3):
            c = (s == ss) & (x == xx) & (z == zz)
            esc[c] = y[c & ajuste_].mean()
        yv, ev, sv = y[aval].astype(bool), esc[aval], np.where(s[aval] == 1, "M", "F")
        sinal = ev > LIMIAR_RISCO
        taxas = estat.taxas_por_grupo(yv, sinal, sv, ev)
        # calibração dentro do grupo: entre os sinalizados e os não sinalizados, risco previsto ≈ observado
        calib = {}
        for g in ("F", "M"):
            m = sv == g
            calib[g] = {"sinalizados_previsto": float(ev[m & sinal].mean()), "sinalizados_observado": float(yv[m & sinal].mean()),
                        "nao_sinalizados_previsto": float(ev[m & ~sinal].mean()), "nao_sinalizados_observado": float(yv[m & ~sinal].mean())}
        p = {g: taxas[g]["taxa_base"][0] for g in taxas}
        vpp = {g: taxas[g]["vpp"][0] for g in taxas}
        fnr = {g: taxas[g]["fnr"][0] for g in taxas}
        fpr = {g: taxas[g]["fpr"][0] for g in taxas}
        fnr_comum = float(np.mean(list(fnr.values())))
        fpr_implicito = {g: float(kleinberg_fpr(p[g], vpp[g], fnr_comum)) for g in taxas}
        # inversão da métrica: igualdade de oportunidade (FNR igual), sinalizando ao acaso uma fração r dos homens
        # da célula de risco logo abaixo do limiar (Hardt et al., 2016, pós-processamento aleatorizado)
        mM = sv == "M"
        abaixo = mM & ~sinal
        prox = abaixo & (ev == ev[abaixo].max())
        ganho_fnr = (yv & prox).sum() / (yv & mM).sum()
        r = float(np.clip((fnr["M"] - fnr["F"]) / ganho_fnr, 0, 1))
        rr = np.random.default_rng(ctx["semente"] + 6)
        sinal2 = sinal | (prox & (rr.random(sinal.size) < r))
        taxas2 = estat.taxas_por_grupo(yv, sinal2, sv, ev)
        n_inf_M, n_sem_M = int((yv & mM).sum()), int((~yv & mM).sum())
        extra_vp = int((sinal2 & ~sinal & yv).sum())
        extra_fp = int((sinal2 & ~sinal & ~yv).sum())
        return {
            "classificador": f"sinalizar para o programa de prevenção quem tem risco previsto P(Y | X, Z, sexo) > {LIMIAR_RISCO:.0%} "
                             "(escore = frequência na célula, ajustado nos períodos 1–2, avaliado no período 3)",
            "grupos": "sexo (F/M), com taxas-base diferentes por construção (±2 pontos fora do núcleo)",
            "taxas_por_grupo_IC95": taxas,
            "calibracao_por_grupo": calib,
            "trilema_da_justica": {
                "aplica_se": True,
                "identidade": "FPR = p/(1−p) · (1−VPP)/VPP · (1−FNR) (Chouldechova, 2017; Kleinberg et al., 2016)",
                "taxa_base": p, "vpp": vpp, "fnr": fnr, "fpr": fpr,
                "fpr_implicito_se_fnr_igual": {"fnr_comum": fnr_comum, **fpr_implicito},
                "leitura": "O escore é calibrado nos dois grupos (risco previsto ≈ observado em cada faixa), mas as taxas-base "
                           f"diferem ({p['F']:.3f} vs {p['M']:.3f}). Pela identidade, com VPP de cada grupo fixa pela calibração e um FNR comum "
                           f"de {fnr_comum:.2f}, os FPR teriam de ser {fpr_implicito['F']:.3f} (F) e {fpr_implicito['M']:.3f} (M): "
                           "diferentes. Calibração e paridade de FPR e FNR não valem juntas com taxas-base distintas.",
            },
            "quem_paga_o_erro": {"F": f"FNR {fnr['F']:.2f}: de cada 100 mulheres que terão infarto, {100 * fnr['F']:.0f} ficam fora do programa",
                                 "M": f"FNR {fnr['M']:.2f}: de cada 100 homens que terão infarto, {100 * fnr['M']:.0f} ficam fora do programa"},
            "inversao_metrica": {
                "metrica_original": "calibração (mesmo limiar de risco absoluto para os dois sexos)",
                "metrica_invertida": f"igualdade de oportunidade (FNR igual): sinalizar ao acaso {r:.0%} dos homens da célula logo abaixo do limiar",
                "fnr_depois": {g: taxas2[g]["fnr"][0] for g in taxas2}, "fpr_depois": {g: taxas2[g]["fpr"][0] for g in taxas2},
                "vpp_depois": {g: taxas2[g]["vpp"][0] for g in taxas2},
                "ganham": f"homens que terão infarto: {extra_vp} a mais entram no programa (de {n_inf_M}), FNR M de {fnr['M']:.2f} para {taxas2['M']['fnr'][0]:.2f}",
                "perdem": f"homens de baixo risco: {extra_fp} falsos positivos a mais (de {n_sem_M} sem infarto), e o sinal deixa de significar o "
                          f"mesmo risco nos dois sexos (VPP M {vpp['M']:.3f} → {taxas2['M']['vpp'][0]:.3f}); o programa paga o custo extra",
            },
            "assimetria_de_poder": "quem vende a vitamina lucra com a leitura observacional (degrau 1); quem toma arca com o risco causal (degrau 2)",
        }

    # ---------------- M15 ----------------
    def _icp(self, treino):
        """ICP (Peters, Bühlmann e Meinshausen, 2016): ambientes = coorte observacional (períodos 1–3) e braço
        sorteado do cofre (período 4). Um conjunto S é aceito se P(Y | S) é igual nos dois ambientes."""
        cof = getattr(self, "_cofre", None)
        if cof is None:
            return {"disponivel": False}
        lot = cof["braco"] == 1
        amb = [{c: np.asarray(treino[c], int) for c in ("z", "x", "y")}, {c: cof[c][lot].astype(int) for c in ("z", "x", "y")}]
        res = {}
        for nomeS, S in (("{}", ()), ("{X}", ("x",)), ("{Z}", ("z",)), ("{X, Z}", ("x", "z"))):
            tabs = []
            chis, gl = 0.0, 0
            for vals in itertools.product((0, 1), repeat=len(S)):
                tab = []
                for e in amb:
                    m = np.ones(e["y"].size, bool)
                    for c, v in zip(S, vals):
                        m &= e[c] == v
                    tab.append([int(e["y"][m].sum()), int(m.sum() - e["y"][m].sum())])
                c2, _, g, _ = stats.chi2_contingency(np.array(tab), correction=False)
                chis += c2
                gl += g
                tabs.append(tab)
            pv = float(stats.chi2.sf(chis, gl))
            res[nomeS] = {"p_invariancia": pv, "aceito": pv > 0.01}
        aceitos = [set(k.strip("{}").replace(" ", "").split(",")) - {""} for k, v in res.items() if v["aceito"]]
        inter = set.intersection(*aceitos) if aceitos else set()
        return {"conjuntos": res, "preditores_causais_icp": sorted(inter),
                "leitura": f"P(Y | X) {'muda' if not res['{X}']['aceito'] else 'não muda'} entre a coorte observacional e o braço sorteado; "
                           f"P(Y | X, Z) {'é invariante' if res['{X, Z}']['aceito'] else 'NÃO é invariante'}. A interseção dos conjuntos aceitos "
                           f"é {sorted(inter) or '∅'}: com dois ambientes e um efeito de 2 pontos, o ICP é conservador (aceita também conjuntos "
                           "em que a mudança é pequena demais para ser vista); o achado que importa é a rejeição de {X} e a aceitação de {X, Z}."}

    def _braco_sorteado(self):
        cof = getattr(self, "_cofre", None)
        if cof is None:
            return {}
        lot = cof["braco"] == 1
        x, y = cof["x"][lot].astype(int), cof["y"][lot].astype(int)
        p1, p0 = y[x == 1].mean(), y[x == 0].mean()
        se = np.sqrt((1 - p1) / (p1 * (x == 1).sum()) + (1 - p0) / (p0 * (x == 0).sum()))
        pc = self._cofre_previsoes
        b = lambda q: float(np.mean((q - y) ** 2))
        pf = np.array([pc["celulas"][f"x{a}z{c}"] for a, c in zip(x, cof["z"][lot].astype(int))])
        pm = np.array([pc["marginal"][a] for a in x])
        return {"n": int(lot.sum()), "rr_sorteado": float(p1 / p0), "rr_IC95": [float(p1 / p0 * np.exp(-1.96 * se)), float(p1 / p0 * np.exp(1.96 * se))],
                "dr_sorteado": float(p1 - p0),
                "brier_no_braco_sorteado": {"agent-f4-frontier": b(pf), "alegacao-marginal": b(pm), "climatologia": b(np.full(y.size, pc["base"]))},
                "vies_medio_previsto_menos_observado_tomando": {"agent-f4-frontier": float(pf[x == 1].mean() - y[x == 1].mean()),
                                                                 "alegacao-marginal": float(pm[x == 1].mean() - y[x == 1].mean())}}

    def lentes(self, ctx):
        tr = ctx["treino"]
        q = ctx["f4"]["_q"]
        z, x, y, w = (np.asarray(tr[c], int) for c in ("z", "x", "y", "w"))
        per = np.asarray(tr["periodo"], int)
        conf = self.conferencia(tr)
        am = ctx["f4"]["_amostras"]
        pcel = special.expit(am).mean(0)
        suf = ctx["suf"]
        pz_h, px0_h, px1_h = suf["kz"] / suf["nz"], suf["kx"][0] / suf["nx"][0], suf["kx"][1] / suf["nx"][1]
        # Morfonomia: efeito ajustado por período (invariância entre coortes) + ICP com o braço sorteado
        dr_per, var_per = [], []
        for pp in sorted(set(per.tolist())):
            m = per == pp
            k, n = contagens(z[m], x[m], y[m])
            pr = k / n
            pzp = z[m].mean()
            dr_per.append((1 - pzp) * (pr[2] - pr[0]) + pzp * (pr[3] - pr[1]))
            var_per.append((1 - pzp) ** 2 * (pr[2] * (1 - pr[2]) / n[2] + pr[0] * (1 - pr[0]) / n[0]) + pzp ** 2 * (pr[3] * (1 - pr[3]) / n[3] + pr[1] * (1 - pr[1]) / n[1]))
        dr_per, wts = np.array(dr_per), 1 / np.array(var_per)
        Q = float(np.sum(wts * (dr_per - np.sum(wts * dr_per) / wts.sum()) ** 2))
        p_Q = float(stats.chi2.sf(Q, len(dr_per) - 1))
        # Fluxonomia: informação mútua X–Y (bits) vista e feita; laço performativo da manchete
        def mi(pxy):
            px_, py_ = pxy.sum(1, keepdims=True), pxy.sum(0, keepdims=True)
            return float(np.nansum(pxy * np.log2(pxy / (px_ * py_))))
        obs0, obs1 = observacional(pcel, pz_h, px0_h, px1_h)
        pX1 = (1 - pz_h) * px0_h + pz_h * px1_h
        tab_obs = np.array([[(1 - pX1) * (1 - obs0), (1 - pX1) * obs0], [pX1 * (1 - obs1), pX1 * obs1]])
        d0, d1 = ajuste(pcel, P_ALVO)
        tab_do = np.array([[0.5 * (1 - d0), 0.5 * d0], [0.5 * (1 - d1), 0.5 * d1]])
        o0c, o1c = observacional(pcel, P_ALVO, 0.6, 0.8)   # depois da campanha: P(X | Z=0) 0,2 → 0,6
        uptake = ((1 - P_ALVO) * 0.6 + P_ALVO * 0.8) - ((1 - P_ALVO) * px0_h + P_ALVO * px1_h)
        return {
            "conferencia_lab": conf,
            "Morfonomia": {
                "dr_ajustada_por_periodo": dr_per.tolist(), "cochran_Q": Q, "p_homogeneidade": p_Q, "invariante_entre_coortes": bool(p_Q > 0.01),
                "icp_observacional_vs_sorteado": self._icp(tr),
                "d_separacao": {"W_indep_X_dado_Z (grafo assumido)": d_separados(GRAFO, ["W"], ["X"], ["Z"]),
                                "W_indep_X_dado_Z (com U não medido)": d_separados(GRAFO_U, ["W"], ["X"], ["Z"]),
                                "S_indep_X": d_separados(GRAFO, ["S"], ["X"], [])},
                "porta_dos_fundos": {"{Z} admissível": porta_dos_fundos(GRAFO, "X", "Y", ["Z"]), "{} admissível": porta_dos_fundos(GRAFO, "X", "Y", []),
                                     "{Z, W} admissível": porta_dos_fundos(GRAFO, "X", "Y", ["Z", "W"]),
                                     "{Z} admissível se houver U": porta_dos_fundos(GRAFO_U, "X", "Y", ["Z"])},
                "leitura": "a forma do mecanismo que se repete entre coortes e entre o mundo observado e o sorteado é P(Y | X, Z); "
                           "P(Y | X) não se transporta. {Z} é admissível no grafo assumido e deixa de ser se existir U.",
            },
            "Fluxonomia": {"informacao_mutua_X_Y_observada_bits": mi(tab_obs), "informacao_mutua_X_Y_sob_do_bits": mi(tab_do),
                           "laco_performativo": True,
                           "rr_observacional_hoje": float(obs1 / obs0), "rr_observacional_apos_campanha": float(o1c / o0c),
                           "infartos_extras_por_1000_pela_campanha": float(1000 * uptake * np.median(q)),
                           "leitura": "a informação que X carrega sobre Y no dado observado é quase toda de Z, e com sinal oposto ao causal. "
                                      "A manchete realimenta o mundo: se ela leva quem não tem hábitos a tomar a vitamina, o RR observacional "
                                      "sobe na direção do causal e o número de infartos aumenta."},
            "Crononomia": {"dr_ajustada_por_coorte": dr_per.tolist(), "ergodico": "não se aplica de modo relevante",
                           "leitura": "o desfecho é de horizonte fixo (dez anos) e cada pessoa é observada uma vez: não há trajetória "
                                      "multiplicativa nem média temporal a comparar com a de conjunto. A lente pega o post hoc do Lab 13 (0d): "
                                      "'tomei e não infartei' é um relato sem o contrafactual. As coortes são estáveis no tempo (ver Q de Cochran)."},
            "Escalonomia": {"risco_absoluto_do1_por_1000": float(1000 * d1), "risco_absoluto_do0_por_1000": float(1000 * d0),
                            "rr_causal": float(d1 / d0), "dr_causal_mediana": float(np.median(q)),
                            "leitura": "Bernoulli limitada: não há cauda pesada, e a média existe e é estável. A escala que importa é a do "
                                       "efeito: 'RR 1,25' soa grande; em risco absoluto são cerca de 2 infartos a mais por 100 pessoas em dez anos."},
            "Crisonomia": {"placar": ctx["f3"]["placar"], "evpi_posterior": estat.evpi(q[::10], self.utilidade, self.acoes())["evpi"],
                           "degrau_2_out_of_time": self._braco_sorteado(),
                           "leitura": "com a posterior atual P(efeito < 0) ≈ 0: nenhum dado extra mudaria a recomendação (EVPI ≈ 0). "
                                      "O previsor da manchete erra justamente no braço sorteado, onde ver vira fazer."},
        }

    def _bic_rival(self, tr):
        z, x, y = (np.asarray(tr[c], int) for c in ("z", "x", "y"))
        n = y.size

        def ll(grupos):
            out = 0.0
            for g in np.unique(grupos):
                yy = y[grupos == g]
                p = yy.mean()
                out += yy.sum() * np.log(p) + (yy.size - yy.sum()) * np.log(1 - p)
            return out
        ll_m, ll_c = ll(x), ll(2 * x + z)
        return {"bic_so_X (manchete)": float(-2 * ll_m + 2 * np.log(n)), "bic_X_e_Z": float(-2 * ll_c + 4 * np.log(n)),
                "mdl_prefere": "X e Z" if -2 * ll_c + 4 * np.log(n) < -2 * ll_m + 2 * np.log(n) else "só X"}

    def critica(self, ctx):
        tr = ctx["treino"]
        return {"rival": {"modelo": "o da manchete: P(Y | X) sem Z", **self._bic_rival(tr),
                          "leitura": "o rival prevê pior (MDL) e, mais grave, responde à pergunta errada: mesmo que previsse bem, "
                                     "previsão no degrau 1 não sustenta uma alegação no degrau 2"},
                "conferencia_lab": self.conferencia(tr),
                "degrau_2_out_of_time": self._braco_sorteado(),
                "performatividade": "sim: a manchete muda quem toma a vitamina; se pessoas sem hábitos passam a tomar, a associação "
                                    "observacional se desfaz e o dano causal se espalha (ver Fluxonomia)",
                "premissas": ["Z (hábitos) bloqueia todas as portas dos fundos: não testável em geral; o controle negativo W só "
                              "detecta confundidores que também afetam W",
                              "Z medido sem erro; com erro de classificação o ajuste corrige só parte do viés",
                              "positividade: há quem tome e quem não tome nos dois estratos",
                              "P(Z = 1) = 0,5 na população-alvo, importado (padronização direta); idade não entra no SCM do lab",
                              "o conjunto conforme de um desfecho binário a 90% é {0, 1}: a cobertura é trivial e não informativa"]}

    # ---------------- F8 ----------------
    def acoes(self):
        return ["recomendar a vitamina", "não recomendar"]

    def utilidade(self, acao, theta):
        """Em infartos evitados por pessoa em dez anos: recomendar troca −θ (o efeito causal) pelo custo."""
        return float(-theta - CUSTO) if acao == "recomendar a vitamina" else 0.0

    def decisao(self, ctx):
        q = ctx["f4"]["_q"]
        am = ctx["f4"]["_amostras"]
        p = special.expit(am)
        d0, d1 = ajuste(p.T, P_ALVO)
        suf = ctx["suf"]
        o0, o1 = observacional(p.T, suf["kz"] / suf["nz"], suf["kx"][0] / suf["nx"][0], suf["kx"][1] / suf["nx"][1])
        rng = np.random.default_rng(ctx["semente"] + 8)
        i = rng.integers(0, q.size, 5000)
        hops = {"tomar a vitamina, do(X=1)": 1000 * d1[i], "não tomar, do(X=0)": 1000 * d0[i],
                "manchete: quem já toma (observação)": 1000 * o1[i]}
        P_prot = float(np.mean(q < 0))
        P_vale = float(np.mean(-q > CUSTO))
        m1, m0, mo1, mo0 = (round(1000 * float(np.median(v))) for v in (d1, d0, o1, o0))
        return {
            "recomendacao": "não recomendar a vitamina para prevenir infarto",
            "efeito_causal_dr": {"mediana": float(np.median(q)), "q05": float(np.quantile(q, 0.05)), "q95": float(np.quantile(q, 0.95))},
            "rr_causal_mediana": float(np.median(d1 / d0)), "rr_observacional_mediana": float(np.median(o1 / o0)),
            "arvore_CL": {"evento": "infarto em dez anos", "acao_protetora_alegada": "tomar a vitamina",
                          "C": f"custo de dez anos de vitamina, {CUSTO} infarto-equivalente por pessoa", "L": "1 infarto",
                          "P_vitamina_reduz_risco": P_prot, "P_reducao_supera_C_L": P_vale,
                          "regra": "tomar se a redução absoluta de risco causal > C/L",
                          "limiar_robusto": "como P(redução > 0) ≈ 0, a decisão não se inverte para nenhum C/L ≥ 0"},
            "frequencias_naturais": (f"De cada 1 000 pessoas que passam a tomar a vitamina, cerca de {m1} terão infarto em dez anos; "
                                     f"se não tomarem, cerca de {m0}: {m1 - m0} infartos a mais. A manchete compara {mo1} infartos em "
                                     f"cada 1 000 que já tomam com {mo0} em cada 1 000 que não tomam, mas quem já toma tem mais hábitos saudáveis."),
            "classe_de_referencia": "adultos de 40 a 79 anos, população com metade de hábitos saudáveis, dose do estudo, dez anos",
            "circuit_breaker": self.termo_escopo()["circuit_breaker"],
            "hops_titulo": "Infartos em 10 anos por 1 000 pessoas", "hops_rotulo": "infartos por 1 000 pessoas", "_hops": hops,
        }

    def disjuntor(self, dados, ctx):
        z, x, w = (np.asarray(dados[c], int) for c in ("z", "x", "w"))
        pz = float(z.mean())
        pos = [float(x[z == zz].mean()) if (z == zz).any() else float("nan") for zz in (0, 1)]
        nc = self.ppc(dados, ctx["f4"]["_amostras"], np.random.default_rng(5))["z_padronizado"]
        motivos = []
        if abs(pz - P_ALVO) > 0.08:
            motivos.append(f"P(Z=1) = {pz:.3f} longe da população-alvo {P_ALVO}")
        if not all(0.02 <= v <= 0.98 for v in pos):
            motivos.append(f"positividade violada: P(X | Z) = {pos}")
        if abs(nc) > 4:
            motivos.append(f"controle negativo associado a X dentro de Z (z = {nc:.1f})")
        return {"disparou": bool(motivos), "P_Z": pz, "P_X_dado_Z": pos, "controle_negativo_z": float(nc),
                "motivo": "; ".join(motivos) if motivos else "dentro do escopo"}
