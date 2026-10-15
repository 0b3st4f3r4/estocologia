# Parecer — caso_06: programa de emprego apoiado (anos 1970)

Script: `bracoA/scripts/caso_06.py` (semente 20261015; usa scikit-learn para o escore de propensão).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "renda 1978 = f(histórico de emprego, escolaridade, idade, raça, estado civil) + efeito do programa; participação autosselecionada entre pessoas com dificuldade de emprego"
    camada_onde_pode_falhar: "L1/L2: os grupos comparados vêm de populações diferentes (seleção para o programa); a regressão com idade e estudo não remove esse viés"
  nao_linearidade_jensen: "renda com massa em zero (programa: 71% com renda zero em 1974; nacional: 12%) e cauda à direita; o efeito médio deve ser complementado por proporção com renda > 0"
  unidade_independente_e_margem:
    unidade: "pessoa; 185 tratados vs 15992 comparações"
    n_efetivo_comparavel: "só ~236 controles com escore de propensão > 0,1; o pareamento usou 113 controles distintos"
  fisica_ruido_agente_ativo: "renda zero é quase um estado absorvente (desemprego); regressão à média esperada: quem se inscreve está num vale de renda"
  escada_causal:
    degrau: "intervenção alegada (o programa reduziu a renda) a partir de dados observacionais"
    identificacao: "ajuste por porta dos fundos com todas as covariáveis pré-tratamento (inclusive renda 1974/75), pareamento por propensão, diferença-em-diferenças"
    premissa_nao_testavel: "ignorabilidade condicional às covariáveis observadas"
```

## 2. Ficha da amostra

```yaml
ficha_amostra:
  tratados: "185 homens inscritos num programa voltado a pessoas com dificuldade de emprego (filtro de entrada = desvantagem)"
  comparacao: "15992 entrevistados de uma pesquisa nacional (população trabalhadora geral)"
  desbalanco_pre_tratamento:
    idade: "25,8 vs 33,2"
    anos_estudo: "10,3 vs 12,0"
    negro: "84% vs 7%"
    casado: "19% vs 71%"
    sem_diploma_medio: "71% vs 30%"
    renda_1974: "2096 vs 14017"
    renda_1975: "1532 vs 13651"
  suporte_comum: "propensão dos controles: mediana 0,0001; a maioria dos controles não é comparável"
  chave: "pessoa, sem duplicatas; 0 faltantes"
```

## 3. DAG

```
desvantagem_laboral ──► inscrição_no_programa
desvantagem_laboral ──► renda_1974/75 ──► renda_1978
desvantagem_laboral ──────────────────────► renda_1978
programa ──► renda_1978
```

A comparação crua mistura o efeito do programa com a porta dos fundos "desvantagem → participação" e "desvantagem → renda". Idade e estudo bloqueiam só uma parte; a renda pré-programa é o proxy mais forte da desvantagem e foi omitida.

## 4. Modelo nulo e teste severo

- Reproduzido: diferença crua -8498; com idade e estudo -6485.
- **Teste placebo (nulo pré-tratamento):** a mesma regressão aplicada à renda de **1975**, antes do programa, dá -8906. O "efeito" negativo já existia antes do programa; ele mede a seleção, não o programa.
- Ajuste completo (todas as covariáveis, renda 74/75 e indicadores de renda zero): **+1184** (IC95% -46 a +2414).
- Diferença-em-diferenças 1975→1978: +3621 (IC 2426 a 4817). Parte disso pode ser regressão à média.
- Pareamento 1-NN por propensão (ATT): **+1914** (bootstrap IC95% -50 a +3593); balanço após o pareamento bom (idade 25,8 vs 25,8; negro 0,8 vs 0,8; renda 1975 1532 vs 1374).
- IPW (ATT): +1600. Amostra aparada (ps ≥ 0,01): +1248 (IC -21 a +2517).
- Nenhuma especificação que respeita a porta dos fundos dá efeito negativo. K = 7.

## 5. Lentes

- **Escalonomia:** renda com massa em zero e cauda; a média sozinha é insuficiente. Proporção com renda zero em 1978: 24% no programa (contra 60–71% antes) e 14% na amostra nacional.
- **Crononomia:** regressão à média (inscrição num vale de renda) infla a DiD; por isso o pareamento sobre a renda pré é preferível.
- **Crisonomia:** a decisão proposta (cortar o financiamento porque o programa reduz a renda) se apoia num efeito que os dados não mostram. O sinal provável é positivo.

## 6. Ficha da crítica de modelos

```yaml
critica_modelos:
  checagem_preditiva: "o modelo do cliente prevê diferença de ~-8900 em 1975, antes do programa: é falsificado pelo placebo"
  modelo_rival: "ajuste por propensão/renda pré: efeito ≈ +1200 a +1900, IC próximo de incluir 0"
  identificado_pelos_dados: ["desbalanço extremo", "falta de suporte comum para a maioria dos controles"]
  trazido_de_fora: ["ignorabilidade dado o histórico de renda"]
  performatividade: "não se aplica"
  premissas_indecidiveis: "variáveis não observadas (motivação, saúde); a comparação ideal é um grupo controle sorteado entre inscritos"
  tentativas_K: 7
```

## 7. Decisão (F8)

A comparação com a população geral compara pessoas muito diferentes: antes do programa, os participantes já ganhavam cerca de 12 mil dólares a menos por ano. Comparados com pessoas parecidas, os participantes ganharam em 1978 cerca de 1,2 a 1,9 mil a **mais**, com incerteza que vai de ~0 a ~3,6 mil. A conclusão "o programa reduziu a renda" não se sustenta e não deve fundamentar o corte. Se houver avaliação nova, sortear a vaga entre inscritos.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: A diferença negativa reflete a seleção dos participantes, não o programa. O mesmo "efeito" já aparece na renda de 1975, antes do programa. Com ajuste por renda pré-programa e covariáveis (regressão, pareamento, IPW), a estimativa fica entre 0 e positiva, nunca negativa.
```
