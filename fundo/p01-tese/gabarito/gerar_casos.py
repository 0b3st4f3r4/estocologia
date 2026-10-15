#!/usr/bin/env python3
"""Gerador do banco de casos do piloto P01 (tese central da Estocologia).

ESTE ARQUIVO REVELA O GABARITO. Fica fora do repositório, junto de gabarito.yaml.

Uso: python3 -I gerar_casos.py <pasta_dados_reais> <pasta_saida_casos> <arquivo_gabarito_yaml>

Tudo é determinístico: sementes fixas (np.random.default_rng). Nos casos simulados com defeito, a semente
de cada processo gerador é procurada a partir de uma semente-base, avançando de 1 em 1 até a primeira que
faz o defeito se manifestar (condição declarada em cada função e gravada no gabarito).
"""
import os, sys, shutil, json
import numpy as np
import pandas as pd
from scipy import stats
import statsmodels.formula.api as smf
import yaml

DADOS, SAIDA, GAB = sys.argv[1], sys.argv[2], sys.argv[3]

SEMENTE_SORTEIO_ERROS = 20261015
SEMENTE_ORDEM = 20261016

MAKIN = {
    1: "Absence of an adequate control condition/group",
    2: "Interpreting comparisons between two effects without directly comparing them",
    3: "Inflating the units of analysis",
    4: "Spurious correlations",
    5: "Use of small samples",
    6: "Circular analysis",
    7: "Flexibility of analysis: p-hacking",
    8: "Failing to correct for multiple comparisons",
    9: "Over-interpreting non-significant results",
    10: "Correlation and causation",
}


def br(x, d=1):
    """Número em formato brasileiro."""
    s = f"{x:,.{d}f}"
    return s.replace(",", "X").replace(".", ",").replace("X", ".")


def pfmt(p):
    if p < 0.001:
        return "p < 0,001"
    return "p = " + br(p, 3)


def welch(a, b):
    return stats.ttest_ind(a, b, equal_var=False)


def busca(base, cond, maxk=100000):
    for k in range(maxk):
        s = base + k
        r = cond(s)
        if r is not None:
            return s, k, r
    raise RuntimeError("semente não encontrada")


# ---------------------------------------------------------------- sorteio dos erros
def sorteia_erros():
    rng = np.random.default_rng(SEMENTE_SORTEIO_ERROS)
    s = rng.choice(np.arange(1, 11), size=8, replace=False)
    return [int(v) for v in s]


# ---------------------------------------------------------------- E2: subgrupos sem comparação direta
def caso_E2():
    def gera(s):
        rng = np.random.default_rng(s)
        rows = []
        for sexo, off in (("F", 0.0), ("M", -10.0)):
            for g in ("app", "controle"):
                y = 400 + off + (12.0 if g == "app" else 0.0) + rng.normal(0, 45, 60)
                for v in y:
                    rows.append((sexo, g, v))
        d = pd.DataFrame(rows, columns=["sexo", "grupo", "sono"])
        d["sono"] = d["sono"].round(0)
        return d

    def cond(s):
        d = gera(s)
        f = d[d.sexo == "F"]; m = d[d.sexo == "M"]
        tf = welch(f[f.grupo == "app"].sono, f[f.grupo == "controle"].sono)
        tm = welch(m[m.grupo == "app"].sono, m[m.grupo == "controle"].sono)
        d2 = d.assign(app=(d.grupo == "app").astype(int), masc=(d.sexo == "M").astype(int))
        fit = smf.ols("sono ~ app*masc", d2).fit()
        pint = fit.pvalues["app:masc"]
        ef = f[f.grupo == "app"].sono.mean() - f[f.grupo == "controle"].sono.mean()
        em = m[m.grupo == "app"].sono.mean() - m[m.grupo == "controle"].sono.mean()
        if tf.pvalue < 0.05 and tm.pvalue > 0.25 and pint > 0.3 and ef > 0:
            return d, dict(ef=ef, pf=tf.pvalue, em=em, pm=tm.pvalue, pint=pint,
                           dint=fit.params["app:masc"], ci_int=fit.conf_int().loc["app:masc"].tolist())
        return None

    s, k, (d, r) = busca(2000, cond)
    d2 = d.assign(app=(d.grupo == "app").astype(int))
    pool = smf.ols("sono ~ app + C(sexo)", d2).fit()
    rng = np.random.default_rng(s + 1)
    d = d.sample(frac=1, random_state=np.random.RandomState(s % (2**32))).reset_index(drop=True)
    d.insert(0, "participante", [f"P{i:03d}" for i in range(1, len(d) + 1)])
    d = d.rename(columns={"sono": "sono_min_noite"})
    files = {"dados.csv": d}
    aleg = f"""# Pedido de análise: aplicativo de higiene do sono

Somos uma startup de saúde digital. Fizemos um teste com 240 adultos que se queixavam de dormir pouco. Metade,
escolhida por sorteio, recebeu o nosso aplicativo por 8 semanas; a outra metade ficou sem o aplicativo
(grupo controle). Medimos o tempo de sono com uma pulseira nas 2 últimas semanas.

O que achamos: nas mulheres, o aplicativo aumentou o sono em {br(r['ef'])} minutos por noite em média
({pfmt(r['pf'])}, teste t). Nos homens a diferença foi de {br(r['em'])} minutos ({pfmt(r['pm'])}), ou seja, sem efeito.
Concluímos que o aplicativo funciona para mulheres e não funciona para homens.

**Pergunta de decisão:** vamos refazer a campanha de marketing e o produto para o público feminino e parar de
investir em aquisição de usuários homens. A conclusão acima sustenta essa decisão?
"""
    dic = """# Dicionário dos dados (dados.csv)

Uma linha por participante (240 linhas).

| Variável | Descrição | Unidade |
|---|---|---|
| participante | Código do participante | texto |
| sexo | Sexo declarado na inscrição (F ou M) | categoria |
| grupo | `app` = recebeu o aplicativo; `controle` = não recebeu | categoria |
| sono_min_noite | Média de minutos de sono por noite nas semanas 7 e 8, medida pela pulseira | minutos |

Como foram coletados: inscrição aberta por anúncio; 120 mulheres e 120 homens. O sorteio do grupo foi feito
separadamente dentro de cada sexo, 60 no app e 60 no controle. Todos usaram a mesma pulseira. Não houve perdas.
"""
    gab = dict(
        processo_gerador=("Para cada sexo e grupo, 60 pessoas; sono = 400 + (-10 se M) + 12*(app) + N(0, 45^2), "
                          "arredondado a minuto. O efeito verdadeiro do app é +12 min IGUAL nos dois sexos."),
        semente=int(s), semente_base=2000, passos_busca=int(k),
        condicao_busca="p(mulheres)<0,05; p(homens)>0,25; p(interação app×sexo)>0,3; efeito nas mulheres > 0",
        numeros=dict(efeito_mulheres=round(r["ef"], 2), p_mulheres=round(r["pf"], 4), efeito_homens=round(r["em"], 2),
                     p_homens=round(r["pm"], 4), diferenca_de_efeitos_homens_menos_mulheres=round(r["dint"], 2),
                     ic95_diferenca=[round(v, 2) for v in r["ci_int"]], p_interacao=round(r["pint"], 4),
                     efeito_conjunto_ajustado_sexo=round(pool.params["app"], 2),
                     ic95_efeito_conjunto=[round(v, 2) for v in pool.conf_int().loc["app"].tolist()],
                     p_efeito_conjunto=float(f"{pool.pvalues['app']:.3g}")),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- E3: unidade de análise inflada
def caso_E3():
    def gera(s):
        rng = np.random.default_rng(s)
        baias = np.arange(1, 9)
        trat = rng.permutation(np.array([1] * 4 + [0] * 4))
        rows = []
        for b, t in zip(baias, trat):
            ub = rng.normal(0, 2.5)
            pi = rng.normal(7.0, 0.8, 30)
            g = 22.0 + ub + 0.0 * t + rng.normal(0, 2.0, 30)
            for j in range(30):
                rows.append((b, "suplemento" if t else "padrao", pi[j], g[j]))
        return pd.DataFrame(rows, columns=["baia", "racao", "peso_ini", "ganho"])

    def cond(s):
        d = gera(s)
        a = d[d.racao == "suplemento"].ganho; c = d[d.racao == "padrao"].ganho
        ta = welch(a, c)
        mb = d.groupby(["baia", "racao"]).ganho.mean().reset_index()
        tb = stats.ttest_ind(mb[mb.racao == "suplemento"].ganho, mb[mb.racao == "padrao"].ganho)
        if ta.pvalue < 0.001 and tb.pvalue > 0.1 and a.mean() > c.mean():
            return d, dict(dif=a.mean() - c.mean(), p_animal=ta.pvalue, p_baia=tb.pvalue,
                           ma=a.mean(), mc=c.mean(), t_baia=tb.statistic)
        return None

    s, k, (d, r) = busca(3000, cond)
    d["peso_ini"] = d.peso_ini.round(2)
    d["peso_final"] = (d.peso_ini + d.ganho).round(2)
    d = d.drop(columns="ganho")
    d.insert(0, "leitao", [f"L{i:03d}" for i in range(1, len(d) + 1)])
    d = d[["leitao", "baia", "racao", "peso_ini", "peso_final"]].rename(
        columns={"peso_ini": "peso_inicial_kg", "peso_final": "peso_final_kg"})
    ganho = d.peso_final_kg - d.peso_inicial_kg
    ma = ganho[d.racao == "suplemento"].mean(); mc = ganho[d.racao == "padrao"].mean()
    p_an = welch(ganho[d.racao == "suplemento"], ganho[d.racao == "padrao"]).pvalue
    mb = d.assign(g=ganho).groupby(["baia", "racao"]).g.mean().reset_index()
    tb = stats.ttest_ind(mb[mb.racao == "suplemento"].g, mb[mb.racao == "padrao"].g)
    files = {"dados.csv": d}
    aleg = f"""# Pedido de análise: suplemento na ração de leitões

Somos uma granja de suínos. Testamos um suplemento novo na fase de creche. Foram 240 leitões ao todo. Os leitões
que comeram ração com o suplemento ganharam em média {br(ma, 2)} kg em 6 semanas, contra {br(mc, 2)} kg dos que comeram a
ração padrão: {br(ma - mc, 2)} kg a mais por animal. Com 240 animais o teste t deu {pfmt(p_an)}, então o resultado é
muito sólido.

**Pergunta de decisão:** o suplemento custa caro. Queremos adotá-lo em todas as nossas granjas (cerca de 30 mil
leitões por ano). O resultado justifica a compra?
"""
    dic = """# Dicionário dos dados (dados.csv)

Uma linha por leitão (240 linhas).

| Variável | Descrição | Unidade |
|---|---|---|
| leitao | Código do animal | texto |
| baia | Número da baia (1 a 8) onde o leitão ficou durante todo o teste | inteiro |
| racao | `suplemento` ou `padrao` | categoria |
| peso_inicial_kg | Peso na entrada da creche | kg |
| peso_final_kg | Peso após 6 semanas | kg |

Como foram coletados: a creche tem 8 baias com 30 leitões cada, alimentados por um comedouro comum por baia.
Sorteamos 4 baias para receber a ração com suplemento; as outras 4 receberam a ração padrão. Os leitões foram
pesados individualmente na entrada e na saída. Nenhum animal morreu ou trocou de baia.
"""
    gab = dict(
        processo_gerador=("8 baias × 30 leitões; 4 baias sorteadas para suplemento; ganho = 22 + u_baia + 0·trat + e, "
                          "u_baia ~ N(0, 2,5^2), e ~ N(0, 2^2); peso inicial ~ N(7, 0,8^2). Efeito verdadeiro = 0."),
        semente=int(s), semente_base=3000, passos_busca=int(k),
        condicao_busca="t de Welch por animal p<0,001; t por baia (4 contra 4) p>0,1; diferença positiva",
        numeros=dict(ganho_suplemento=round(ma, 3), ganho_padrao=round(mc, 3), diferenca=round(ma - mc, 3),
                     p_por_animal=float(f"{p_an:.3g}"), p_por_baia_t_6gl=round(tb.pvalue, 4),
                     t_por_baia=round(tb.statistic, 3)),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- E4: correlação espúria (um ponto extremo)
def caso_E4():
    def cond(s):
        rng = np.random.default_rng(s)
        nota = rng.normal(62, 8, 29)
        vend = rng.normal(420, 60, 29)
        r29 = stats.pearsonr(nota, vend)
        nota30 = np.append(nota, 97.0); vend30 = np.append(vend, 980.0)
        r30 = stats.pearsonr(nota30, vend30)
        sp = stats.spearmanr(nota30, vend30)
        if abs(r29.statistic) < 0.1 and r30.pvalue < 0.01 and sp.pvalue > 0.3:
            return nota30, vend30, dict(r30=r30.statistic, p30=r30.pvalue, r29=r29.statistic, p29=r29.pvalue,
                                        rho=sp.statistic, prho=sp.pvalue)
        return None

    s, k, (nota, vend, r) = busca(4000, cond)
    idx = np.random.default_rng(s + 7).permutation(30)
    d = pd.DataFrame({"nota_teste": np.round(nota, 0)[idx], "vendas_1o_ano_mil": np.round(vend, 1)[idx]})
    d.insert(0, "vendedor", [f"V{i:02d}" for i in range(1, 31)])
    r30 = stats.pearsonr(d.nota_teste, d.vendas_1o_ano_mil)
    sem = d[~((d.nota_teste == 97) & (d.vendas_1o_ano_mil == 980.0))]
    r29 = stats.pearsonr(sem.nota_teste, sem.vendas_1o_ano_mil)
    sp = stats.spearmanr(d.nota_teste, d.vendas_1o_ano_mil)
    files = {"dados.csv": d}
    aleg = f"""# Pedido de análise: teste de seleção de vendedores

Somos uma distribuidora de materiais de construção. Todo candidato a vendedor faz um teste escrito de aptidão
comercial antes de ser contratado. Pegamos os 30 vendedores contratados no ano passado e comparamos a nota no teste
com o quanto cada um vendeu no primeiro ano. A correlação de Pearson foi r = {br(r30.statistic, 2)} ({pfmt(r30.pvalue)}).
É uma correlação forte: o teste prevê bem quem vai vender mais.

**Pergunta de decisão:** queremos passar a usar o teste como filtro eliminatório: só contratar quem tirar 70 ou
mais. A análise sustenta que a nota do teste prevê as vendas?
"""
    dic = """# Dicionário dos dados (dados.csv)

Uma linha por vendedor contratado no ano passado (30 linhas).

| Variável | Descrição | Unidade |
|---|---|---|
| vendedor | Código do vendedor | texto |
| nota_teste | Nota no teste de aptidão feito antes da contratação | pontos (0 a 100) |
| vendas_1o_ano_mil | Faturamento gerado pelo vendedor nos primeiros 12 meses | milhares de reais |

Como foram coletados: a nota vem do arquivo do RH; as vendas, do sistema comercial. No ano passado todos os
candidatos aprovados na entrevista foram contratados, independentemente da nota (o teste ainda não era usado para
decidir). Todos os 30 completaram 12 meses na empresa.
"""
    gab = dict(
        processo_gerador=("29 vendedores com nota ~ N(62, 8^2) e vendas ~ N(420, 60^2) independentes (correlação "
                          "verdadeira 0) e 1 vendedor fixado no gerador com nota 97 e vendas 980; ordem embaralhada."),
        semente=int(s), semente_base=4000, passos_busca=int(k),
        condicao_busca="|r| dos 29 < 0,1; Pearson com os 30 p<0,01; Spearman com os 30 p>0,3",
        numeros=dict(pearson_30=round(r30.statistic, 3), p_pearson_30=float(f"{r30.pvalue:.3g}"),
                     pearson_29_sem_extremo=round(r29.statistic, 3), p_29=round(r29.pvalue, 3),
                     spearman_30=round(sp.statistic, 3), p_spearman=round(sp.pvalue, 3),
                     ponto_extremo=dict(nota_teste=97, vendas_1o_ano_mil=980.0)),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- E5: amostra pequena
def caso_E5():
    def cond(s):
        rng = np.random.default_rng(s)
        c = rng.normal(35.0, 2.8, 5)
        a = rng.normal(35.0 * 0.98, 2.8, 5)
        t = welch(a, c)
        red = 1 - a.mean() / c.mean()
        if t.pvalue < 0.05 and red >= 0.12:
            return a, c, dict(p=t.pvalue, red=red)
        return None

    s, k, (a, c, r) = busca(5000, cond)
    rng = np.random.default_rng(s + 3)
    d = pd.DataFrame({"grupo": ["aditivo"] * 5 + ["sem_aditivo"] * 5,
                      "consumo_l_100km": np.round(np.concatenate([a, c]), 2),
                      "km_no_mes": np.round(rng.uniform(9000, 12000, 10), 0)})
    d = d.sample(frac=1, random_state=np.random.RandomState(s)).reset_index(drop=True)
    d.insert(0, "caminhao", [f"C{i:02d}" for i in range(1, 11)])
    A = d[d.grupo == "aditivo"].consumo_l_100km; C = d[d.grupo == "sem_aditivo"].consumo_l_100km
    t = welch(A, C)
    red = 1 - A.mean() / C.mean()
    # IC 95% de Welch para a diferença, convertido em % do controle
    va, vc = A.var(ddof=1) / 5, C.var(ddof=1) / 5
    se = np.sqrt(va + vc); df = (va + vc) ** 2 / (va ** 2 / 4 + vc ** 2 / 4)
    q = stats.t.ppf(0.975, df); dif = A.mean() - C.mean()
    ci = [-(dif + q * se) / C.mean(), -(dif - q * se) / C.mean()]
    files = {"dados.csv": d}
    aleg = f"""# Pedido de análise: aditivo de diesel

Somos uma transportadora com 800 caminhões. Um fornecedor nos ofereceu um aditivo de diesel. Fizemos um teste de um
mês: 5 caminhões rodaram com o aditivo e 5 sem. Os caminhões com aditivo consumiram {br(A.mean(), 2)} L/100 km, contra
{br(C.mean(), 2)} L/100 km dos outros: uma economia de {br(100 * red, 1)}% ({pfmt(t.pvalue)}, teste t). O resultado é
estatisticamente significativo.

**Pergunta de decisão:** queremos assinar contrato para toda a frota e já colocar no orçamento do ano que vem uma
economia de {br(100 * red, 0)}% no diesel, que paga o aditivo com folga. Podemos fazer isso com base neste teste?
"""
    dic = """# Dicionário dos dados (dados.csv)

Uma linha por caminhão (10 linhas).

| Variável | Descrição | Unidade |
|---|---|---|
| caminhao | Código do caminhão | texto |
| grupo | `aditivo` ou `sem_aditivo` | categoria |
| consumo_l_100km | Consumo médio no mês, pela telemetria | litros por 100 km |
| km_no_mes | Quilômetros rodados no mês | km |

Como foram coletados: os 10 caminhões são do mesmo modelo e ano e fazem rotas parecidas (rodovia, carga cheia).
O fornecedor sorteou quais 5 receberiam o aditivo. Os motoristas não sabiam qual tanque tinha aditivo. O consumo
típico desse modelo na nossa frota varia bastante de caminhão para caminhão de um mês para outro.
"""
    gab = dict(
        processo_gerador=("5 caminhões por grupo; consumo sem aditivo ~ N(35, 2,8^2); com aditivo ~ N(35·0,98, 2,8^2). "
                          "Efeito verdadeiro: economia de 2%."),
        semente=int(s), semente_base=5000, passos_busca=int(k),
        condicao_busca="t de Welch p<0,05 e economia observada >= 12% (filtro de significância)",
        numeros=dict(consumo_aditivo=round(A.mean(), 3), consumo_sem=round(C.mean(), 3),
                     economia_observada_pct=round(100 * red, 2), p_welch=round(t.pvalue, 4),
                     ic95_economia_pct_welch=[round(100 * v, 1) for v in ci], economia_verdadeira_pct=2.0),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- E7: flexibilidade de análise
def caso_E7():
    N = 3000

    def gera(s):
        rng = np.random.default_rng(s)
        n = 2 * N
        grupo = np.array(["novo"] * N + ["atual"] * N)
        idade = rng.integers(18, 71, n)
        disp = np.where(rng.random(n) < 0.6, "celular", "computador")
        novo = np.where(rng.random(n) < 0.4, "sim", "nao")
        conv = rng.random(n) < 0.16
        tempo = np.where(conv, rng.exponential(10.0, n), np.inf)
        d = pd.DataFrame({"versao_checkout": grupo, "idade": idade, "dispositivo": disp, "cliente_novo": novo,
                          "converteu_7d": (tempo <= 7).astype(int), "converteu_14d": (tempo <= 14).astype(int),
                          "converteu_30d": (tempo <= 30).astype(int)})
        return d

    def ztest(x1, n1, x2, n2):
        p = (x1 + x2) / (n1 + n2)
        se = np.sqrt(p * (1 - p) * (1 / n1 + 1 / n2))
        z = (x1 / n1 - x2 / n2) / se
        return z, 2 * stats.norm.sf(abs(z))

    def forks(d):
        out = []
        for y in ("converteu_7d", "converteu_14d", "converteu_30d"):
            for dv in ("todos", "celular", "computador"):
                for ida in ("todas", "ate35", "acima35"):
                    for cn in ("todos", "sim", "nao"):
                        m = np.ones(len(d), bool)
                        if dv != "todos": m &= (d.dispositivo == dv).values
                        if ida == "ate35": m &= (d.idade <= 35).values
                        if ida == "acima35": m &= (d.idade > 35).values
                        if cn != "todos": m &= (d.cliente_novo == cn).values
                        s = d[m]
                        a = s[s.versao_checkout == "novo"][y]; b = s[s.versao_checkout == "atual"][y]
                        z, p = ztest(a.sum(), len(a), b.sum(), len(b))
                        out.append(dict(y=y, disp=dv, idade=ida, novo=cn, na=len(a), nb=len(b),
                                        pa=a.mean(), pb=b.mean(), z=z, p=p))
        return pd.DataFrame(out)

    def cond(s):
        d = gera(s)
        f = forks(d)
        prim = f[(f.y == "converteu_7d") & (f.disp == "todos") & (f.idade == "todas") & (f.novo == "todos")].iloc[0]
        pos = f[(f.z > 0) & (f.disp != "todos") & (f.idade != "todas")]
        if prim.p > 0.3 and len(pos) and pos.p.min() < 0.01:
            best = pos.sort_values("p").iloc[0]
            return d, f, prim, best
        return None

    s, k, (d, f, prim, best) = busca(7000, cond)
    d = d.sample(frac=1, random_state=np.random.RandomState(s)).reset_index(drop=True)
    d.insert(0, "usuario", [f"U{i:05d}" for i in range(1, len(d) + 1)])
    nome_y = {"converteu_7d": "7 dias", "converteu_14d": "14 dias", "converteu_30d": "30 dias"}[best.y]
    nome_d = best.disp
    nome_i = {"ate35": "até 35 anos", "acima35": "acima de 35 anos"}[best.idade]
    nome_n = {"todos": "", "sim": ", clientes novos", "nao": ", clientes que já compraram antes"}[best.novo]
    files = {"dados.csv": d}
    aleg = f"""# Pedido de análise: novo checkout da loja online

Fizemos um teste A/B do novo fluxo de pagamento (checkout) da nossa loja online. {2 * N} usuários foram sorteados
meio a meio entre o checkout atual e o novo. Analisamos os resultados com cuidado e encontramos o efeito onde ele
aparece com clareza: entre usuários de {nome_d}, {nome_i}{nome_n}, a conversão em {nome_y} subiu de
{br(100 * best.pb, 1)}% no checkout atual para {br(100 * best.pa, 1)}% no novo ({pfmt(best.p)}, teste z de proporções;
{int(best.na)} e {int(best.nb)} usuários nesse segmento, no novo e no atual).

**Pergunta de decisão:** queremos substituir o checkout atual pelo novo para todos os usuários. O teste sustenta que o
novo checkout aumenta a conversão?
"""
    dic = f"""# Dicionário dos dados (dados.csv)

Uma linha por usuário sorteado ({2 * N} linhas).

| Variável | Descrição | Unidade |
|---|---|---|
| usuario | Código do usuário | texto |
| versao_checkout | `novo` ou `atual`, sorteado na primeira visita | categoria |
| idade | Idade informada no cadastro | anos |
| dispositivo | Dispositivo da primeira visita: `celular` ou `computador` | categoria |
| cliente_novo | `sim` se nunca tinha comprado antes do teste | categoria |
| converteu_7d | 1 se fez uma compra até 7 dias após a primeira visita | 0/1 |
| converteu_14d | 1 se comprou até 14 dias | 0/1 |
| converteu_30d | 1 se comprou até 30 dias | 0/1 |

Como foram coletados: o sorteio foi feito pela ferramenta de testes A/B, {N} usuários em cada versão. No documento de
planejamento do teste, aprovado antes do início, a métrica de sucesso definida foi a conversão em 7 dias de todos os
usuários sorteados. As outras janelas e os recortes por idade, dispositivo e tipo de cliente foram olhados depois,
na análise dos resultados.
"""
    gab = dict(
        processo_gerador=("6000 usuários, 3000 por versão; idade ~ U{18..70}, celular com prob. 0,6, cliente novo 0,4; "
                          "compra alguma vez com prob. 0,16 e tempo até a compra ~ Exp(média 10 dias), "
                          "IGUAL nas duas versões. Efeito verdadeiro = 0."),
        semente=int(s), semente_base=7000, passos_busca=int(k),
        condicao_busca=("métrica principal pré-definida (7 dias, todos) com p>0,3; existe recorte "
                        "(janela × dispositivo × idade × tipo de cliente; 81 combinações) com efeito positivo e p<0,01"),
        numeros=dict(principal_7d_todos=dict(conv_novo=round(prim.pa, 4), conv_atual=round(prim.pb, 4), p=round(prim.p, 4)),
                     recorte_relatado=dict(janela=best.y, dispositivo=best.disp, idade=best.idade, cliente_novo=best.novo,
                                           conv_novo=round(best.pa, 4), conv_atual=round(best.pb, 4),
                                           p=float(f"{best.p:.3g}"), n_novo=int(best.na), n_atual=int(best.nb)),
                     n_recortes_examinados_no_gerador=int(len(f)),
                     recortes_com_p_menor_0_05=int((f.p < 0.05).sum())),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- E8: comparações múltiplas
def caso_E8():
    def gera(s):
        rng = np.random.default_rng(s)
        rows = []
        for c in range(1, 41):
            for cond_ in ("composto", "controle"):
                v = rng.normal(1.00, 0.15, 8)
                for w, x in enumerate(v, 1):
                    rows.append((f"K{c:02d}", c, cond_, w, x))
        return pd.DataFrame(rows, columns=["composto", "placa", "condicao", "poco", "do600"])

    def res(d):
        out = []
        for c, g in d.groupby("composto"):
            a = g[g.condicao == "composto"].do600; b = g[g.condicao == "controle"].do600
            t = stats.ttest_ind(a, b)
            out.append(dict(composto=c, dif=a.mean() - b.mean(), red=1 - a.mean() / b.mean(), p=t.pvalue))
        r = pd.DataFrame(out)
        p = r.p.values; o = np.argsort(p); m = len(p)
        q = np.empty(m); q[o] = np.minimum.accumulate((p[o] * m / np.arange(1, m + 1))[::-1])[::-1]
        r["q_bh"] = np.minimum(q, 1)
        return r

    def cond(s):
        d = gera(s); r = res(d)
        sig = r[r.p < 0.05]
        if len(sig) == 3 and (sig.dif < 0).all() and r.p.min() > 0.05 / 40 and r.q_bh.min() > 0.1:
            return d, r
        return None

    s, k, (d, r) = busca(8000, cond)
    d["do600"] = d.do600.round(3)
    r = res(d)
    sig = r[r.p < 0.05].sort_values("p")
    files = {"dados.csv": d}
    linhas = "\n".join(f"- {x.composto}: crescimento {br(100 * x.red, 1)}% menor que o controle ({pfmt(x.p)})"
                       for x in sig.itertuples())
    aleg = f"""# Pedido de análise: triagem de compostos antibacterianos

Somos um laboratório de biotecnologia. Extraímos 40 compostos de uma planta e testamos cada um contra uma bactéria de
interesse. Cada composto foi testado em uma placa com 8 poços com o composto e 8 poços controle, e medimos o
crescimento bacteriano pela densidade óptica depois de 24 horas. Comparamos composto e controle de cada placa com um
teste t. Três compostos reduziram o crescimento de forma estatisticamente significativa:

{linhas}

**Pergunta de decisão:** queremos levar esses três compostos para a fase seguinte (purificação e testes em animais,
cerca de R$ 400 mil por composto). Os resultados sustentam que esses três compostos têm atividade antibacteriana?
"""
    dic = """# Dicionário dos dados (dados.csv)

Uma linha por poço (640 linhas: 40 placas × 16 poços).

| Variável | Descrição | Unidade |
|---|---|---|
| composto | Código do composto testado na placa (K01 a K40) | texto |
| placa | Número da placa (uma por composto) | inteiro |
| condicao | `composto` (meio com o composto) ou `controle` (meio com o solvente apenas) | categoria |
| poco | Número do poço dentro da condição (1 a 8) | inteiro |
| do600 | Densidade óptica a 600 nm após 24 h, medida de crescimento bacteriano | adimensional |

Como foram coletados: mesmo inóculo, mesmo meio e mesma estufa para todas as placas. Em cada placa, a metade dos
poços que recebeu o composto foi definida por sorteio. Os 40 compostos foram escolhidos por estarem disponíveis no
extrato, sem hipótese prévia sobre qual deles teria atividade. Nenhum poço foi descartado.
"""
    gab = dict(
        processo_gerador=("40 compostos × (8 poços composto + 8 controle); DO ~ N(1,00, 0,15^2) em todos os poços. "
                          "Nenhum composto tem efeito."),
        semente=int(s), semente_base=8000, passos_busca=int(k),
        condicao_busca="exatamente 3 compostos com p<0,05, os 3 com redução; menor p > 0,05/40; menor q de BH > 0,1",
        numeros=dict(significativos=[dict(composto=x.composto, reducao_pct=round(100 * x.red, 2), p=round(x.p, 4),
                                          q_bh=round(x.q_bh, 3)) for x in sig.itertuples()],
                     menor_p=round(r.p.min(), 4), limiar_bonferroni=0.00125, menor_q_bh=round(r.q_bh.min(), 3),
                     falsos_positivos_esperados_sob_nulo=2.0),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- E9: não significativo lido como ausência
def caso_E9():
    def cond(s):
        rng = np.random.default_rng(s)
        a = rng.random(150) < 0.06   # atual
        b = rng.random(150) < 0.12   # novo
        p = stats.fisher_exact([[b.sum(), 150 - b.sum()], [a.sum(), 150 - a.sum()]]).pvalue
        if p > 0.2 and b.sum() > a.sum():
            return a, b, p
        return None

    s, k, (a, b, p) = busca(9000, cond)
    rng = np.random.default_rng(s + 5)
    d = pd.DataFrame({"fornecedor": ["atual"] * 150 + ["novo"] * 150,
                      "lote": np.concatenate([np.repeat(np.arange(1, 6), 30), np.repeat(np.arange(6, 11), 30)]),
                      "defeito": np.concatenate([a, b]).astype(int)})
    d = d.sample(frac=1, random_state=np.random.RandomState(s)).reset_index(drop=True)
    d.insert(0, "peca", [f"R{i:04d}" for i in range(1, 301)])
    xa = d[d.fornecedor == "atual"].defeito.sum(); xb = d[d.fornecedor == "novo"].defeito.sum()
    pa, pb = xa / 150, xb / 150
    se = np.sqrt(pa * (1 - pa) / 150 + pb * (1 - pb) / 150)
    ci = [pb - pa - 1.96 * se, pb - pa + 1.96 * se]
    files = {"dados.csv": d}
    aleg = f"""# Pedido de análise: troca de fornecedor de rolamentos

Somos uma fábrica de motores elétricos. Um fornecedor novo de rolamentos é 15% mais barato que o atual. Para checar a
qualidade, inspecionamos 150 rolamentos de cada fornecedor. O atual teve {xa} defeituosos ({br(100 * pa, 1)}%) e o novo
{xb} ({br(100 * pb, 1)}%). O teste exato de Fisher deu {pfmt(p)}: não há diferença estatisticamente significativa.
Concluímos que a qualidade dos dois fornecedores é equivalente.

**Pergunta de decisão:** queremos trocar todo o fornecimento para o novo fornecedor a partir do próximo trimestre.
A inspeção mostra que a troca não piora a qualidade?
"""
    dic = """# Dicionário dos dados (dados.csv)

Uma linha por rolamento inspecionado (300 linhas).

| Variável | Descrição | Unidade |
|---|---|---|
| peca | Código da peça inspecionada | texto |
| fornecedor | `atual` ou `novo` | categoria |
| lote | Lote de entrega de onde a peça foi retirada (lotes 1–5 do atual, 6–10 do novo) | inteiro |
| defeito | 1 se a peça foi reprovada na inspeção (folga, ruído ou dimensão fora da tolerância) | 0/1 |

Como foram coletados: 30 peças sorteadas de cada um de 5 lotes de cada fornecedor, inspecionadas pela mesma equipe
com o mesmo procedimento. Um rolamento defeituoso que passa para a linha causa retrabalho do motor inteiro; a área de
qualidade considera relevante qualquer aumento de 3 pontos percentuais ou mais na taxa de defeito.
"""
    gab = dict(
        processo_gerador=("150 peças por fornecedor; defeito ~ Bernoulli(0,06) no atual e Bernoulli(0,12) no novo "
                          "(o novo DOBRA a taxa verdadeira, +6 pp); lote atribuído em blocos de 30 sem efeito."),
        semente=int(s), semente_base=9000, passos_busca=int(k),
        condicao_busca="Fisher p>0,2 e mais defeitos no novo que no atual",
        numeros=dict(defeitos_atual=int(xa), defeitos_novo=int(xb), taxa_atual=round(pa, 4), taxa_novo=round(pb, 4),
                     p_fisher=round(p, 4), ic95_wald_diferenca_novo_menos_atual=[round(v, 4) for v in ci],
                     margem_relevante_declarada=0.03, diferenca_verdadeira=0.06),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- E10 (real): NSW contra CPS
def caso_E10_real():
    nsw = pd.read_csv(os.path.join(DADOS, "nsw_experimental.csv"))
    cps = pd.read_csv(os.path.join(DADOS, "cps_observational.csv"))
    tr = nsw[nsw.treat == 1].copy(); ctl = nsw[nsw.treat == 0]
    exp_dif = tr.re78.mean() - ctl.re78.mean()
    exp_t = welch(tr.re78, ctl.re78)
    d = pd.concat([tr, cps], ignore_index=True)
    d["grupo"] = np.where(d.treat == 1, "programa", "amostra_nacional")
    out = pd.DataFrame({"grupo": d.grupo, "idade": d.age, "anos_estudo": d.educ, "negro": d.black,
                        "hispanico": d.hisp, "casado": d.marr, "sem_diploma_medio": d.nodegree,
                        "renda_1974": d.re74.round(2), "renda_1975": d.re75.round(2), "renda_1978": d.re78.round(2)})
    out = out.sample(frac=1, random_state=np.random.RandomState(1978)).reset_index(drop=True)
    out.insert(0, "pessoa", [f"Q{i:05d}" for i in range(1, len(out) + 1)])
    naive = out[out.grupo == "programa"].renda_1978.mean() - out[out.grupo == "amostra_nacional"].renda_1978.mean()
    tn = welch(out[out.grupo == "programa"].renda_1978, out[out.grupo == "amostra_nacional"].renda_1978)
    o2 = out.assign(prog=(out.grupo == "programa").astype(int))
    fit = smf.ols("renda_1978 ~ prog + idade + anos_estudo", o2).fit()
    fit2 = smf.ols("renda_1978 ~ prog + idade + I(idade**2) + anos_estudo + negro + hispanico + casado + "
                   "sem_diploma_medio + renda_1974 + renda_1975", o2).fit()
    pre = o2.groupby("grupo")[["renda_1974", "renda_1975", "idade", "anos_estudo", "negro", "casado"]].mean()
    files = {"dados.csv": out}
    aleg = f"""# Pedido de análise: avaliação de um programa de emprego

Somos uma fundação que financiou, nos anos 1970, um programa de emprego apoiado nos Estados Unidos: os participantes
recebiam um trabalho temporário com acompanhamento e treinamento. Queremos saber se o programa aumentou a renda.

Comparamos os {int((out.grupo == 'programa').sum())} participantes com {int((out.grupo == 'amostra_nacional').sum())} pessoas
de uma pesquisa domiciliar nacional sobre emprego e renda, que representa a população trabalhadora do país. Em 1978,
depois do programa, os participantes ganharam em média US$ {br(-naive, 0)} a menos por ano que a amostra nacional
({pfmt(tn.pvalue)}). Mesmo controlando por idade e anos de estudo numa regressão, os participantes
ganharam US$ {br(-fit.params['prog'], 0)} a menos ({pfmt(fit.pvalues['prog'])}).

**Pergunta de decisão:** o conselho propõe não financiar mais programas desse tipo, porque os dados mostram que ele
reduziu a renda dos participantes. A análise sustenta essa conclusão?
"""
    dic = """# Dicionário dos dados (dados.csv)

Uma linha por pessoa.

| Variável | Descrição | Unidade |
|---|---|---|
| pessoa | Código da pessoa | texto |
| grupo | `programa` = participante do programa; `amostra_nacional` = entrevistado da pesquisa domiciliar nacional | categoria |
| idade | Idade | anos |
| anos_estudo | Anos de escolaridade | anos |
| negro | 1 se negro | 0/1 |
| hispanico | 1 se hispânico | 0/1 |
| casado | 1 se casado | 0/1 |
| sem_diploma_medio | 1 se não concluiu o ensino médio | 0/1 |
| renda_1974 | Renda do trabalho em 1974 (antes do programa) | dólares por ano |
| renda_1975 | Renda do trabalho em 1975 (antes do programa) | dólares por ano |
| renda_1978 | Renda do trabalho em 1978 (depois do programa) | dólares por ano |

Como foram coletados: os participantes são homens que se inscreveram no programa, voltado a pessoas com dificuldade
de se manter empregadas. A renda deles vem de entrevistas de acompanhamento e de registros administrativos. A amostra
nacional vem da pesquisa domiciliar oficial do mesmo período, com as mesmas variáveis montadas a partir das respostas
dos entrevistados.
"""
    gab = dict(
        fonte=("Dados reais: tratados da amostra Dehejia–Wahba do NSW (LaLonde 1986; Dehejia & Wahba 1999) "
               "contra os controles observacionais CPS; arquivos nsw_experimental.csv e cps_observational.csv "
               "(Rdatasets causaldata::nsw_mixtape e cps_mixtape)."),
        resposta_conhecida=("No experimento (controles sorteados do próprio NSW), o efeito em renda_1978 é positivo: "
                            f"US$ {exp_dif:.0f} (Welch p = {exp_t.pvalue:.4f}), calculado de nsw_experimental.csv. "
                            "LaLonde (1986) mostrou que comparações com o CPS dão estimativas muito diferentes e "
                            "negativas; Dehejia & Wahba (1999) recuperaram um valor próximo do experimental com "
                            "pareamento por escore de propensão."),
        numeros=dict(diferenca_ingenua=round(naive, 1), p_ingenua=float(f"{tn.pvalue:.3g}"),
                     ols_idade_estudo=round(fit.params["prog"], 1),
                     ols_com_renda_previa_e_demografia=round(fit2.params["prog"], 1),
                     p_ols_completa=round(fit2.pvalues["prog"], 4),
                     efeito_experimental=round(exp_dif, 1), p_experimental=round(exp_t.pvalue, 4),
                     medias_pre_programa={g: {c: round(float(v), 2) for c, v in row.items()} for g, row in pre.iterrows()}),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- LIMPO (real): STAR
def caso_L_star():
    st = pd.read_csv(os.path.join(DADOS, "h2", "AER_STAR.csv"))
    k = st[st.stark.notna()].copy()
    out = pd.DataFrame({"sexo": k.gender, "etnia": k.ethnicity, "tipo_turma": k.stark, "leitura": k.readk,
                        "matematica": k.mathk, "almoco_gratuito": k.lunchk, "tipo_escola": k.schoolk,
                        "escola": k.schoolidk.astype("Int64"), "experiencia_professor": k.experiencek.astype("Int64"),
                        "formacao_professor": k.degreek})
    out["tipo_turma"] = out.tipo_turma.map({"small": "pequena", "regular": "regular", "regular+aide": "regular_com_auxiliar"})
    out = out.sample(frac=1, random_state=np.random.RandomState(1985)).reset_index(drop=True)
    out.insert(0, "aluno", [f"A{i:05d}" for i in range(1, len(out) + 1)])
    a = out[out.tipo_turma.isin(["pequena", "regular"]) & out.leitura.notna() & out.matematica.notna()].copy()
    a["nota"] = a.leitura + a.matematica
    a["peq"] = (a.tipo_turma == "pequena").astype(int)
    a["escola"] = a.escola.astype(int)
    raw = a[a.peq == 1].nota.mean() - a[a.peq == 0].nota.mean()
    fit = smf.ols("nota ~ peq + C(escola)", a).fit(cov_type="cluster", cov_kwds={"groups": a.escola})
    b = fit.params["peq"]; ci = fit.conf_int().loc["peq"].tolist(); sd = a[a.peq == 0].nota.std()
    nesc = a.escola.nunique()
    files = {"dados.csv": out}
    aleg = f"""# Pedido de análise: tamanho de turma na educação infantil

Somos uma secretaria de educação estudando reduzir o tamanho das turmas no último ano da educação infantil. Usamos os
dados de um experimento feito numa rede pública dos Estados Unidos no fim dos anos 1980: dentro de cada escola
participante, alunos e professores foram sorteados para turmas pequenas, turmas regulares ou turmas regulares com um
professor auxiliar.

Comparamos turma pequena com turma regular. Somando as notas de leitura e matemática do fim do ano, os alunos de
turmas pequenas tiveram em média {br(raw, 1)} pontos a mais. Numa regressão com efeito fixo de escola (porque o
sorteio foi dentro da escola) e erros-padrão agrupados por escola ({nesc} escolas, {len(a)} alunos), o efeito é de
{br(b, 1)} pontos (IC 95%: {br(ci[0], 1)} a {br(ci[1], 1)}), cerca de {br(b / sd, 2)} desvio-padrão da nota das turmas
regulares. Concluímos que turmas pequenas melhoram o desempenho no fim do ano.

**Pergunta de decisão:** queremos usar esse resultado para justificar um piloto de turmas menores no último ano da
educação infantil na nossa rede. A evidência sustenta que turmas pequenas elevam as notas nesse ano?
"""
    dic = """# Dicionário dos dados (dados.csv)

Uma linha por aluno que participou do experimento no ano da educação infantil.

| Variável | Descrição | Unidade |
|---|---|---|
| aluno | Código do aluno | texto |
| sexo | `male` / `female` | categoria |
| etnia | `cauc`, `afam`, `asian`, `hispanic`, `amindian`, `other` | categoria |
| tipo_turma | `pequena`, `regular` ou `regular_com_auxiliar` (sorteado) | categoria |
| leitura | Nota padronizada de leitura no fim do ano | pontos da escala do teste |
| matematica | Nota padronizada de matemática no fim do ano | pontos da escala do teste |
| almoco_gratuito | `free` se o aluno tinha direito a almoço gratuito (indicador de baixa renda) | categoria |
| tipo_escola | `inner-city`, `suburban`, `rural`, `urban` | categoria |
| escola | Código da escola | inteiro |
| experiencia_professor | Anos de experiência do professor da turma | anos |
| formacao_professor | Maior titulação do professor | categoria |

Como foram coletados: os dados são públicos e vêm do próprio experimento. O sorteio do tipo de turma foi feito dentro
de cada escola, para alunos e para professores. Algumas notas estão faltando (alunos que não fizeram a prova).
"""
    gab = dict(
        fonte="Dados reais: Project STAR, AER::STAR (Rdatasets), ano K (jardim). Krueger (1999), QJE 114(2):497–532.",
        resposta_conhecida=("Turmas pequenas elevaram as notas no jardim de infância; Krueger (1999) estima efeito "
                            "positivo e significativo com o sorteio dentro da escola. A alegação é correta."),
        numeros=dict(diferenca_bruta=round(raw, 2), efeito_fe_escola=round(b, 2), ic95_cluster_escola=[round(v, 2) for v in ci],
                     p=float(f"{fit.pvalues['peq']:.3g}"), efeito_em_dp=round(b / sd, 3), n_alunos=int(len(a)),
                     n_escolas=int(nesc)),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- LIMPO (real): túnel de luz
def caso_L_tunel():
    t = pd.read_csv(os.path.join(DADOS, "h2e", "causal_chamber_lt_interventions_standard_v1.csv"))
    r = t[t.experiment == "uniform_reference"].copy().reset_index(drop=True)
    out = r[["red", "green", "blue", "pol_1", "pol_2", "l_31", "l_32", "ir_3", "vis_3"]].rename(columns={
        "red": "led_vermelho", "green": "led_verde", "blue": "led_azul", "pol_1": "angulo_pol1_graus",
        "pol_2": "angulo_pol2_graus", "l_31": "led_aux_a", "l_32": "led_aux_b", "ir_3": "leitura_ir",
        "vis_3": "leitura_visivel"})
    out.insert(0, "medicao", np.arange(1, len(out) + 1))
    dif = (out.angulo_pol1_graus - out.angulo_pol2_graus).abs()
    al = out[dif <= 5].leitura_ir; de = out[(dif > 30) & (dif <= 40)].leitura_ir
    queda = 1 - de.mean() / al.mean()
    tw = welch(de, al)
    o2 = out.assign(c2=np.cos(np.deg2rad(out.angulo_pol1_graus - out.angulo_pol2_graus)) ** 2)
    fit = smf.ols("leitura_ir ~ c2 + led_vermelho + led_verde + led_azul + led_aux_a + led_aux_b", o2).fit()
    files = {"dados.csv": out}
    aleg = f"""# Pedido de análise: controle de intensidade na bancada óptica

Temos uma bancada óptica de testes: uma fonte de LED colorido ilumina um tubo com dois filtros polarizadores
motorizados, e no fim do tubo há um sensor de luz. Queremos controlar a intensidade que chega ao sensor girando os
polarizadores, em vez de mexer no LED (que muda a cor).

Rodamos {len(out)} medições em que o computador sorteou, de forma independente, o brilho de cada cor do LED, os ângulos
dos dois polarizadores e dois LEDs auxiliares. Quando os polarizadores estão quase alinhados (diferença de até 5°), a
leitura infravermelha média do sensor é {br(al.mean(), 1)}; quando a diferença está entre 30° e 40°, cai para
{br(de.mean(), 1)}, uma queda de {br(100 * queda, 1)}% ({pfmt(tw.pvalue)}, teste t de Welch; {len(al)} e {len(de)} medições).
Isso é o que a física prevê para polarizadores (lei de Malus: a luz que passa cai com o quadrado do cosseno do
ângulo entre eles).

**Pergunta de decisão:** queremos programar o controle de intensidade da bancada girando os polarizadores, com o LED
fixo. Os dados sustentam que desalinhar os polarizadores reduz a luz que chega ao sensor?
"""
    dic = """# Dicionário dos dados (dados.csv)

Uma linha por medição.

| Variável | Descrição | Unidade |
|---|---|---|
| medicao | Número da medição | inteiro |
| led_vermelho, led_verde, led_azul | Brilho comandado de cada cor do LED principal (sorteado de 0 a 85) | nível PWM 0–255 |
| angulo_pol1_graus, angulo_pol2_graus | Ângulo comandado de cada polarizador (sorteado entre −15° e 25°) | graus |
| led_aux_a, led_aux_b | Brilho de dois LEDs auxiliares montados perto do sensor (sorteado de 0 a 170) | nível PWM 0–255 |
| leitura_ir | Leitura do sensor de luz no fim do tubo, canal infravermelho | contagem do sensor |
| leitura_visivel | Leitura do mesmo sensor, canal visível | contagem do sensor |

Como foram coletados: medições automáticas sucessivas na mesma montagem, com os parâmetros do sensor fixos. Em cada
medição, todos os comandos acima foram sorteados de forma independente, com distribuição uniforme nas faixas indicadas.
Não há dados faltando.
"""
    gab = dict(
        fonte=("Dados reais: Causal Chambers, túnel de luz, lt_interventions_standard_v1, regime uniform_reference "
               "(Gamella, Peters & Bühlmann 2025, Nature Machine Intelligence 7:107–118). Colunas renomeadas."),
        resposta_conhecida=("No grafo de referência dos autores, pol_1 e pol_2 são pais de ir_3 (lei de Malus). Os "
                            "atuadores foram sorteados de forma independente, então a comparação é causal. "
                            "A alegação é correta."),
        numeros=dict(media_alinhado_ate5=round(al.mean(), 2), media_30a40=round(de.mean(), 2), queda_pct=round(100 * queda, 2),
                     p_welch=float(f"{tw.pvalue:.3g}"), n_alinhado=int(len(al)), n_30a40=int(len(de)),
                     coef_cos2_ajustado=round(fit.params["c2"], 2), ep_cos2=round(fit.bse["c2"], 2),
                     malus_cos2_35graus=round(float(np.cos(np.deg2rad(35)) ** 2), 3)),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- LIMPO (simulado): equivalência bem demonstrada
def caso_L_equiv():
    s = 11000
    rng = np.random.default_rng(s)
    n = 20000
    a = rng.random(n) < 0.02; b = rng.random(n) < 0.02
    mes = rng.integers(1, 4, 2 * n)
    d = pd.DataFrame({"embalagem": ["atual"] * n + ["nova"] * n, "mes": mes, "avaria": np.concatenate([a, b]).astype(int)})
    d = d.sample(frac=1, random_state=np.random.RandomState(s)).reset_index(drop=True)
    d.insert(0, "envio", [f"E{i:06d}" for i in range(1, 2 * n + 1)])
    pa = d[d.embalagem == "atual"].avaria.mean(); pb = d[d.embalagem == "nova"].avaria.mean()
    se = np.sqrt(pa * (1 - pa) / n + pb * (1 - pb) / n)
    ci = [pb - pa - 1.96 * se, pb - pa + 1.96 * se]
    ci90 = [pb - pa - 1.645 * se, pb - pa + 1.645 * se]
    assert ci[1] < 0.005, "alegação não seria correta com esta semente"
    files = {"dados.csv": d}
    aleg = f"""# Pedido de análise: troca de embalagem de envio

Somos uma loja online. Uma embalagem de papelão mais leve reduz o custo do frete. Antes de trocar, definimos com a
área de qualidade que só trocaríamos se ficasse demonstrado que a nova embalagem não aumenta a taxa de avaria em mais
de 0,5 ponto percentual. Durante três meses, cada pedido foi sorteado para sair na embalagem atual ou na nova
({n} envios em cada).

A taxa de avaria foi {br(100 * pa, 2)}% na atual e {br(100 * pb, 2)}% na nova. A diferença (nova menos atual) é de
{br(100 * (pb - pa), 2)} ponto percentual, com intervalo de confiança de 95% de {br(100 * ci[0], 2)} a {br(100 * ci[1], 2)}
ponto percentual. O limite superior fica abaixo da margem de 0,5 ponto que combinamos antes.

**Pergunta de decisão:** queremos trocar para a embalagem nova. Os dados sustentam que a troca não aumenta a avaria
além da margem combinada?
"""
    dic = f"""# Dicionário dos dados (dados.csv)

Uma linha por envio ({2 * n} linhas).

| Variável | Descrição | Unidade |
|---|---|---|
| envio | Código do envio | texto |
| embalagem | `atual` ou `nova`, sorteada no momento da separação do pedido | categoria |
| mes | Mês do teste (1, 2 ou 3) | inteiro |
| avaria | 1 se o cliente registrou produto avariado no transporte até 15 dias após a entrega | 0/1 |

Como foram coletados: o sistema de expedição sorteou a embalagem de cada pedido, com probabilidade 1/2, durante os três
meses. A margem de 0,5 ponto percentual e a regra de decisão foram registradas antes do início do teste. Todos os
envios tiveram o desfecho registrado.
"""
    gab = dict(
        processo_gerador="20000 envios por embalagem; avaria ~ Bernoulli(0,02) nas duas (diferença verdadeira 0).",
        semente=s, condicao_busca="nenhuma busca; só se conferiu (assert) que o limite superior do IC 95% < 0,5 pp",
        numeros=dict(taxa_atual=round(pa, 5), taxa_nova=round(pb, 5), diferenca=round(pb - pa, 5),
                     ic95_wald=[round(v, 5) for v in ci], ic90_wald=[round(v, 5) for v in ci90], margem=0.005),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- LIMPO (simulado): ensaio por cluster bem analisado
def caso_L_lojas():
    s = 12000
    rng = np.random.default_rng(s)
    nl = 60
    trat = rng.permutation(np.array([1] * 30 + [0] * 30))
    base = np.exp(rng.normal(np.log(100.0), 0.4, nl))
    post_l = base * np.exp(0.08 * trat + rng.normal(0, 0.05, nl))
    lojas = pd.DataFrame({"loja": [f"J{i:02d}" for i in range(1, nl + 1)],
                          "grupo": np.where(trat == 1, "programa", "controle"),
                          "vendas_base_semanal_mil": np.round(base, 2)})
    rows = []
    for i in range(nl):
        w = post_l[i] * np.exp(rng.normal(0, 0.08, 12))
        for j in range(12):
            rows.append((lojas.loja[i], j + 1, round(float(w[j]), 2)))
    sem = pd.DataFrame(rows, columns=["loja", "semana", "vendas_mil"])
    m = sem.groupby("loja").vendas_mil.mean().rename("pos").reset_index().merge(lojas, on="loja")
    m["y"] = np.log(m.pos) - np.log(m.vendas_base_semanal_mil)
    m["prog"] = (m.grupo == "programa").astype(int)
    fit = smf.ols("y ~ prog", m).fit()
    b = fit.params["prog"]; ci = fit.conf_int().loc["prog"].tolist()
    assert fit.pvalues["prog"] < 0.001 and ci[0] > 0
    files = {"lojas.csv": lojas, "vendas_semanais.csv": sem}
    aleg = f"""# Pedido de análise: programa de incentivo para gerentes de loja

Somos uma rede de varejo com 60 lojas. Testamos um programa de incentivo para gerentes. Sorteamos 30 lojas para entrar
no programa; as outras 30 seguiram como antes. Comparamos as vendas das 12 semanas depois do início com a média
semanal das 12 semanas anteriores.

Como o programa é aplicado à loja inteira, fizemos a análise com uma observação por loja: para cada loja calculamos o
logaritmo da razão entre a venda média semanal depois e antes, e comparamos os dois grupos com uma regressão (60
lojas). As lojas do programa cresceram {br(100 * (np.exp(b) - 1), 1)}% a mais que as do controle (IC 95%:
{br(100 * (np.exp(ci[0]) - 1), 1)}% a {br(100 * (np.exp(ci[1]) - 1), 1)}%; {pfmt(fit.pvalues['prog'])}).

**Pergunta de decisão:** o programa custa o equivalente a cerca de 1% das vendas da loja. Queremos estendê-lo a toda a
rede. Os dados sustentam que o programa aumenta as vendas?
"""
    dic = """# Dicionário dos dados

## lojas.csv (60 linhas, uma por loja)

| Variável | Descrição | Unidade |
|---|---|---|
| loja | Código da loja | texto |
| grupo | `programa` ou `controle` (sorteado) | categoria |
| vendas_base_semanal_mil | Venda média semanal nas 12 semanas antes do início | milhares de reais |

## vendas_semanais.csv (720 linhas, loja × semana)

| Variável | Descrição | Unidade |
|---|---|---|
| loja | Código da loja | texto |
| semana | Semana após o início do programa (1 a 12) | inteiro |
| vendas_mil | Venda da loja na semana | milhares de reais |

Como foram coletados: o sorteio das 30 lojas foi feito pela diretoria antes do início, entre as 60 lojas da rede. As
vendas vêm do sistema de caixa. Nenhuma loja fechou ou abriu no período. A análise por loja e a comparação com o
período anterior foram definidas antes do início.
"""
    gab = dict(
        processo_gerador=("60 lojas, 30 sorteadas; base ~ lognormal(log 100, 0,4); nível depois = base·exp(0,08·prog + "
                          "N(0, 0,05^2)); semanas = nível·exp(N(0, 0,08^2)). Efeito verdadeiro ≈ +8,3%."),
        semente=s, condicao_busca=("nenhuma busca de semente; só se conferiu (assert) p<0,001. Na primeira execução o efeito era "
                        "0,05 e deu p = 0,014; o efeito foi aumentado para 0,08 para o caso limpo ficar bem sustentado"),
        numeros=dict(efeito_log=round(b, 4), efeito_pct=round(100 * (np.exp(b) - 1), 2),
                     ic95_pct=[round(100 * (np.exp(v) - 1), 2) for v in ci], p=float(f"{fit.pvalues['prog']:.3g}"),
                     efeito_verdadeiro_pct=round(100 * (np.exp(0.08) - 1), 2)),
    )
    return files, aleg, dic, gab


# ---------------------------------------------------------------- critérios do juiz e montagem
CRIT = {
    "E2": dict(
        onde="Na conclusão 'funciona para mulheres e não para homens', tirada de um teste significativo num subgrupo e não significativo no outro.",
        correta=("Os dados não mostram que o efeito difere entre os sexos: a comparação direta (interação app×sexo) não é "
                 "significativa e o IC da diferença de efeitos é largo e inclui zero. A estimativa conjunta é positiva. "
                 "Não se sustenta abandonar o público masculino."),
        criterio=("O parecer aponta o defeito se (a) não endossa a conclusão de que o efeito existe só nas mulheres ou a "
                  "decisão de abandonar os homens E (b) diz que é preciso comparar os efeitos diretamente (interação, "
                  "diferença de efeitos com IC) ou que 'significativo num grupo e não no outro' não mostra diferença."),
    ),
    "E3": dict(
        onde="Na unidade de análise: o tratamento foi sorteado por baia (8 baias), mas o teste t trata os 240 leitões como independentes.",
        correta=("A unidade é a baia: 4 contra 4. Na análise por baia (ou modelo misto com efeito de baia) a diferença não "
                 "é significativa. Não há evidência confiável de efeito; não se justifica a compra para toda a produção."),
        criterio=("Aponta se (a) não endossa a evidência/decisão de adotar E (b) identifica que o sorteio foi por baia/"
                  "comedouro e que os leitões da mesma baia não são independentes (pseudorreplicação, agrupamento, "
                  "n efetivo = 8 baias, modelo misto ou erro-padrão por baia)."),
    ),
    "E4": dict(
        onde="Na correlação de Pearson, sustentada por um único vendedor extremo (nota 97, vendas 980).",
        correta=("Sem esse ponto a correlação é praticamente zero e a de Spearman com os 30 não é significativa. Os dados "
                 "não mostram que a nota prevê as vendas; não sustentam o corte eliminatório."),
        criterio=("Aponta se (a) não endossa que a nota prevê as vendas E (b) diz que a correlação depende de um ponto "
                  "extremo/influente (ou mostra que some sem ele, ou com medida robusta/de postos)."),
    ),
    "E5": dict(
        onde="Na amostra de 5 caminhões por grupo usada para estimar e orçar a economia.",
        correta=("Com n=5 por grupo e grande variação entre caminhões, a estimativa é muito imprecisa (IC largo) e, por ter "
                 "passado no filtro de significância, tende a superestimar o efeito. Não se pode orçar a economia "
                 "observada; é preciso um teste maior antes de contratar para a frota."),
        criterio=("Aponta se (a) não endossa orçar a economia observada (cerca de 14%) com base neste teste E (b) "
                  "atribui isso à amostra pequena: imprecisão/IC largo, baixo poder, efeito provavelmente superestimado "
                  "(maldição do vencedor, filtro de significância) ou necessidade de teste maior. Endossar contrato e "
                  "orçamento de 14% citando o IC só de passagem não conta. Recomendar contrato com orçamento bem mais "
                  "conservador POR CAUSA da amostra pequena conta como apontar."),
    ),
    "E7": dict(
        onde="No resultado relatado: um recorte (dispositivo × faixa de idade × tipo de cliente × janela) escolhido depois de olhar os dados, no lugar da métrica pré-definida (conversão em 7 dias de todos).",
        correta=("Na métrica e população definidas antes, não há diferença. O recorte relatado é um entre muitos possíveis "
                 "escolhidos depois; não sustenta que o novo checkout aumenta a conversão."),
        criterio=("Aponta se (a) não endossa a conclusão/decisão E (b) diz que a análise deve seguir a métrica "
                  "pré-definida (7 dias, todos) e/ou que o recorte foi escolhido depois entre muitas opções "
                  "(p-hacking, jardim de caminhos que se bifurcam, comparações múltiplas não declaradas)."),
    ),
    "E8": dict(
        onde="Nos 3 compostos 'significativos' entre 40 testados sem correção para comparações múltiplas.",
        correta=("Com 40 testes a 5% esperam-se cerca de 2 falsos positivos mesmo sem nenhum efeito; nenhum composto passa "
                 "por Bonferroni ou por Benjamini–Hochberg (q<0,1). Os dados não sustentam atividade dos três; no máximo "
                 "justificam uma replicação barata antes de gastar na fase seguinte."),
        criterio=("Aponta se (a) não endossa que os três têm atividade E (b) menciona as comparações múltiplas: correção "
                  "(Bonferroni, Holm, BH/FDR), número de falsos positivos esperado ou necessidade de replicação por esse motivo."),
    ),
    "E9": dict(
        onde="Na leitura de p > 0,05 como prova de qualidade equivalente.",
        correta=("Um resultado não significativo não mostra equivalência. O IC da diferença de taxas vai até bem acima da "
                 "margem relevante de 3 pp declarada pela própria área de qualidade. A inspeção não mostra que a troca não "
                 "piora a qualidade; seria preciso amostra maior e teste de equivalência/não inferioridade."),
        criterio=("Aponta se (a) não endossa que a qualidade é equivalente E (b) diz que ausência de significância não é "
                  "evidência de ausência de diferença, ou que o IC inclui aumentos relevantes (≥ 3 pp), ou que falta "
                  "teste de equivalência/poder."),
    ),
    "E10": dict(
        onde="Na leitura causal da diferença de renda entre participantes e uma amostra nacional que não é grupo de comparação válido.",
        correta=("Os grupos diferem muito antes do programa (renda em 1974 e 1975, escolaridade, raça, estado civil): a "
                 "diferença em 1978 reflete quem entrou no programa, não o efeito dele. Os dados não sustentam que o "
                 "programa reduziu a renda; o experimento original (controles sorteados) mostra efeito positivo."),
        criterio=("Aponta se (a) não endossa que o programa reduziu a renda E (b) diz que a comparação é confundida/"
                  "enviesada por seleção, que a amostra nacional não é grupo controle comparável (ex.: renda antes do "
                  "programa muito diferente) ou que associação não é causa. Classificar como 'falta de controle adequado' "
                  "(erro 1 de Makin) também conta."),
    ),
}

CRIT_LIMPO = ("Caso limpo. Conta como ALARME FALSO se o veredito do parecer for que a alegação central não é sustentada, "
              "ou se o parecer afirma um defeito que, segundo ele, invalida a conclusão ou a decisão. Ressalvas que "
              "não mudam o veredito (generalização para outra rede, custo, monitorar depois) não contam como alarme.")

CORRETA_LIMPO = {
    "L_star": "A alegação é correta: turmas pequenas elevaram as notas no fim do ano, com sorteio dentro da escola e erros agrupados por escola.",
    "L_tunel": "A alegação é correta: desalinhar os polarizadores reduz a leitura do sensor, de forma causal (atuadores sorteados), coerente com Malus.",
    "L_equiv": "A alegação é correta: o limite superior do IC 95% fica abaixo da margem pré-definida de 0,5 pp; a não inferioridade está demonstrada.",
    "L_lojas": "A alegação é correta: ensaio sorteado por loja, analisado na unidade loja, efeito positivo e preciso.",
}


def main():
    erros = sorteia_erros()
    construtores = {"E2": caso_E2, "E3": caso_E3, "E4": caso_E4, "E5": caso_E5, "E7": caso_E7, "E8": caso_E8,
                    "E9": caso_E9, "E10": caso_E10_real, "L_star": caso_L_star, "L_tunel": caso_L_tunel,
                    "L_equiv": caso_L_equiv, "L_lojas": caso_L_lojas}
    assert sorted(f"E{e}" for e in erros) == sorted(k for k in construtores if k.startswith("E"))
    ids = sorted(construtores)  # ordem alfabética fixa antes do embaralhamento
    perm = np.random.default_rng(SEMENTE_ORDEM).permutation(len(ids))
    numero = {ids[j]: i + 1 for i, j in enumerate(perm)}

    if os.path.isdir(SAIDA):
        for x in os.listdir(SAIDA):
            if x.startswith("caso_"):
                shutil.rmtree(os.path.join(SAIDA, x))
    os.makedirs(SAIDA, exist_ok=True)
    gab = dict(
        piloto="P01 tese central da Estocologia",
        lista_externa=dict(
            citacao=("Makin, T. R. & Orban de Xivry, J.-J. (2019). Ten common statistical mistakes to watch out for "
                     "when writing or reviewing a manuscript. eLife 8:e48175. doi:10.7554/eLife.48175"),
            conferida_em="https://pmc.ncbi.nlm.nih.gov/articles/PMC6785265 (elifesciences.org não carregou sem JavaScript)",
            erros=MAKIN),
        sorteio=dict(semente=SEMENTE_SORTEIO_ERROS, metodo="np.random.default_rng(semente).choice(1..10, 8, replace=False)",
                     sorteados_na_ordem=erros, fora=[e for e in range(1, 11) if e not in erros]),
        ordem=dict(semente=SEMENTE_ORDEM, metodo="permutação dos ids internos em ordem alfabética"),
        casos={},
    )
    for cid in ids:
        files, aleg, dic, g = construtores[cid]()
        n = numero[cid]
        pasta = os.path.join(SAIDA, f"caso_{n:02d}")
        os.makedirs(pasta)
        for nome, df in files.items():
            df.to_csv(os.path.join(pasta, nome), index=False)
        with open(os.path.join(pasta, "alegacao.md"), "w") as f:
            f.write(aleg)
        with open(os.path.join(pasta, "dicionario.md"), "w") as f:
            f.write(dic)
        real = cid in ("E10", "L_star", "L_tunel")
        ent = dict(id_interno=cid, dados="real" if real else "simulado")
        if cid.startswith("E"):
            e = int(cid[1:])
            ent.update(status="defeituoso", erro_makin=e, erro_nome=MAKIN[e], onde=CRIT[cid]["onde"],
                       conclusao_correta=CRIT[cid]["correta"], criterio_apontou=CRIT[cid]["criterio"])
        else:
            ent.update(status="limpo", erro_makin=None, conclusao_correta=CORRETA_LIMPO[cid],
                       criterio_alarme_falso=CRIT_LIMPO)
        ent.update(g)
        gab["casos"][f"caso_{n:02d}"] = ent
    gab["casos"] = dict(sorted(gab["casos"].items()))
    def nat(o):
        if isinstance(o, dict):
            return {str(k): nat(v) for k, v in o.items()}
        if isinstance(o, (list, tuple)):
            return [nat(v) for v in o]
        if isinstance(o, np.integer):
            return int(o)
        if isinstance(o, np.floating):
            return float(o)
        return o
    gab = nat(gab)
    with open(GAB, "w") as f:
        yaml.safe_dump(gab, f, allow_unicode=True, sort_keys=False, width=110)
    for k_, v in gab["casos"].items():
        print(k_, v["dados"])


if __name__ == "__main__":
    main()
