# Parecer — caso_02: suplemento na ração de leitões

Script: `bracoA/scripts/caso_02.py` (semente 20261015).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "ganho de peso = efeito da baia (comedouro comum, ambiente, competição, sanidade) + efeito da ração + variação individual"
    camada_onde_pode_falhar: "L2: o teste t trata 240 leitões como independentes, mas o tratamento foi sorteado por baia (8 unidades)"
  nao_linearidade_jensen: "desfecho aditivo (kg); sem cauda pesada (ganho 12,9–29,8 kg). Jensen pouco relevante; a decisão (30 mil leitões/ano) é linear no ganho médio."
  unidade_independente_e_margem:
    unidade: "baia (unidade de randomização e de alimentação)"
    n_efetivo: "8 baias (4 vs 4); ICC = 0,58, então o efeito de desenho é ~1+29*0,58 ≈ 18"
    margem: "diferença 2,26 kg; EP por baia 1,68–1,82 kg; IC95% ≈ -1,3 a 5,8 kg"
  fisica_ruido_agente_ativo: "sem estados absorventes (nenhuma morte). A variação entre baias (var 6,4 kg²) é maior que a variação dentro da baia (4,6 kg²)."
  escada_causal:
    degrau: "intervenção (sorteio de baias)"
    identificacao: "randomização por conglomerado; a inferência precisa ser feita na baia"
    premissa_nao_testavel: "não há interferência entre baias"
```

## 2. Ficha da amostra (M4)

```yaml
ficha_amostra:
  unidades: "240 leitões em 8 baias de 30; 4 baias sorteadas para o suplemento"
  chave_primaria: "leitao (sem duplicatas)"
  faltantes: 0
  selecao: "nenhuma perda nem troca de baia"
  balanco_baseline: "peso inicial médio por baia entre 6,7 e 7,3 kg"
  pseudorreplicacao: "SIM: o tratamento varia só entre baias; os leitões são subamostras"
```

## 3. DAG

```
sorteio ──► ração(baia) ──► ganho_leitão ◄── efeito_baia (comedouro, ambiente)
                                  ▲
                              peso_inicial
```

O efeito da baia não é confundidor (o sorteio o torna independente da ração), mas é fonte de variância **correlacionada**. Com só 4 baias por braço, ele pode produzir sozinho diferenças de 2 kg.

## 4. Modelo nulo e teste severo

- Teste da alegação (t por leitão): p = 3,6e-8 (reproduzido).
- Médias por baia, suplemento: 20,4; 18,7; 25,2; 22,9. Padrão: 21,1; 20,6; 16,2; 20,2.
- **Teste exato de randomização sobre as 70 alocações possíveis de 4 baias:** p = 0,29.
- t nas médias de baia: p = 0,26. EP agrupado por baia: p = 0,18. Modelo misto com peso inicial: 2,27 kg (IC95% -1,3 a 5,8), p = 0,21.
- **Nulo simulado** (sem efeito do suplemento, com a variância entre baias estimada): o teste t por leitão dá p < 0,05 em **68%** das simulações. O "p < 0,001" é o que se espera mesmo sem efeito nenhum.
- K = 5 especificações (t individual, t por baia, cluster, misto, permutação exata); todas as que respeitam a unidade concordam.

## 5. Lentes

- **Escalonomia/M4 (unidade):** n efetivo = 8, não 240.
- **Crisonomia:** a decisão é cara e cobre 30 mil leitões/ano. Com IC de -1,3 a +5,8 kg, o resultado pode ir de prejuízo a ganho grande. O EVSI de um novo ensaio com mais baias é alto.
- Crononomia e Fluxonomia: não se aplicam.

## 6. Ficha da crítica de modelos

```yaml
critica_modelos:
  checagem_preditiva: "um modelo de leitões independentes prevê médias de baia com DP ~0,5 kg; as observadas variam de 16,2 a 25,2 kg. O modelo é rejeitado."
  modelo_rival: "modelo misto com intercepto aleatório de baia: efeito 2,3 kg, IC inclui 0"
  identificado_pelos_dados: ["diferença média", "variâncias entre e dentro de baia"]
  trazido_de_fora: ["ausência de interferência entre baias"]
  performatividade: "nenhuma"
  premissas_indecidiveis: "com 8 baias não dá para separar o efeito do suplemento da heterogeneidade entre baias"
```

## 7. Decisão (F8)

Os dados não justificam a compra para todas as granjas. Na escala certa (baias), a diferença observada aparece em cerca de 3 de cada 10 sorteios possíveis mesmo sem efeito nenhum. Recomenda-se um ensaio com mais baias (ex.: ≥ 12 por braço, em mais de uma granja), analisado por baia e com custo-benefício pré-definido (kg mínimos que pagam o suplemento). Cláusula de saída: só adotar se o limite inferior do IC por baia superar o ganho que paga o custo.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: O suplemento foi sorteado por baia (8 unidades), não por leitão. O p < 0,001 vem de pseudorreplicação. Analisado por baia, o efeito tem p ≈ 0,2–0,3 e um IC95% que inclui zero (cerca de -1,3 a 5,8 kg).
```
