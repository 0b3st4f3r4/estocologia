# Parecer — caso_03: programa de incentivo para gerentes de loja

Script: `bracoA/scripts/caso_03.py` (semente 20261015).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "venda semanal da loja = nível da loja × sazonalidade comum × efeito do programa × ruído semanal"
    camada_onde_pode_falhar: "L1 (choque comum ou tendência diferente entre grupos) e L2 (escala log, unidade)"
  nao_linearidade_jensen:
    dinamica: "multiplicativa (as lojas variam de 37 a 272 mil/semana); a escala log é a adequada"
    jensen: "exp(E[log razão]) - 1 = 7,0%; razão das médias aritméticas das razões T/C = 1,070. A diferença é desprezível."
  unidade_independente_e_margem:
    unidade: "loja (sorteio por loja); n = 60, sem pseudorreplicação das 720 semanas"
    margem: "IC95% OLS 3,9% a 10,3%; HC3 3,9% a 10,2%; bootstrap por loja 4,0% a 10,0%"
  fisica_ruido_agente_ativo: "sem estados absorventes (nenhuma loja fechou). Gerentes incentivados podem antecipar vendas ou manipular a métrica (Goodhart). Verificado: o efeito se mantém nas 12 semanas, sem pico inicial seguido de queda."
  escada_causal:
    degrau: "intervenção (sorteio de 30 de 60 lojas)"
    identificacao: "randomização + diferença pré/pós; análise pré-especificada"
    premissa_nao_testavel: "sem contaminação entre lojas; o sistema de caixa mede vendas reais (não deslocadas de outras lojas)"
```

## 2. Ficha da amostra

```yaml
ficha_amostra:
  unidades: "60 lojas (toda a rede), 30/30"
  chave: "loja; vendas loja×semana completas (720, sem duplicatas nem faltantes)"
  selecao: "nenhuma; nenhuma loja abriu ou fechou"
  balanco_baseline: "venda base 108,8 vs 102,5 mil (t p = 0,59)"
  pre_registro: "análise por loja e comparação com o período anterior definidas antes do início"
  valores_extremos: "J25 (~300 mil/semana) e J06 (~33 mil); influência máxima (Cook) 0,13; leave-one-out 6,3%–7,3%"
```

## 3. DAG

```
sorteio ──► programa ──► venda_pós ◄── venda_pré (nível da loja)
                              ▲
                     choque_comum(semana)  (atinge os dois grupos e é cancelado no contraste)
```

O log da razão pós/pré remove o nível da loja. O choque comum é cancelado pelo grupo controle.

## 4. Modelo nulo e teste severo

- Reproduzido: diferença em log 0,0679, ou seja +7,0% (IC 3,9%–10,3%), p = 2,7e-5.
- **Nulo por permutação dos rótulos (20000):** p = 5e-5.
- Mann-Whitney p = 1,9e-5; ANCOVA log(pós) ~ T + log(pré): 6,9% (IC 3,9%–9,9%).
- **Severidade:** sob o nulo, uma diferença ≥ 6,8% aparece em menos de 1 de 10000 sorteios. Se o efeito real fosse ≤ 1% (o custo), o limite inferior observado (3,9%) seria muito improvável.
- Trajetória semanal: razão pós/pré do programa entre 1,05 e 1,10 em todas as 12 semanas; controle entre 0,98 e 1,04. Não há sinal de um efeito transitório.
- K = 5 especificações; todas concordam.

## 5. Lentes

- **Crononomia:** dinâmica multiplicativa tratada em log. O horizonte de 12 semanas não garante persistência; o efeito não decai dentro da janela.
- **Morfonomia:** o efeito é estável no leave-one-out e entre semanas.
- **Crisonomia:** o efeito estimado (7%, limite inferior 3,9%) supera o custo de 1% das vendas. Atenção: o custo é sobre vendas e o benefício também, mas o lucro depende da margem; com margem bruta baixa, +3,9% em vendas pode não pagar 1% de vendas. Essa conta é do cliente, e a pergunta é sobre vendas.
- **Fluxonomia/Goodhart:** o incentivo pode deslocar vendas entre períodos ou canais; não há queda pós-pico na janela.

## 6. Ficha da crítica de modelos

```yaml
critica_modelos:
  checagem_preditiva: "distribuição dos log-razões nos dois grupos com DP semelhante (0,057 vs 0,058); compatível com efeito de deslocamento"
  modelo_rival: "ANCOVA em log e teste não paramétrico; mesmo resultado"
  identificado_pelos_dados: ["efeito médio relativo em 12 semanas"]
  trazido_de_fora: ["persistência além de 12 semanas", "ausência de contaminação"]
  performatividade: "risco de Goodhart (gerentes otimizam a métrica); monitorar margem, devoluções e vendas entre lojas"
  premissas_indecidiveis: "efeito de longo prazo; efeito no lucro"
  tentativas_K: 5
```

## 7. Decisão (F8)

Em 60 lojas sorteadas, as lojas do programa cresceram cerca de 7 de cada 100 reais a mais que as do controle (IC 4 a 10). O custo do programa é cerca de 1 de cada 100 reais vendidos. A extensão à rede é sustentada para o objetivo "aumentar vendas".
Cláusula de saída: manter um grupo controle de lojas (ou um rodízio) por 2 trimestres. Se o efeito medido cair abaixo de 1% (o custo), ou se margens e devoluções piorarem, suspender.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: O desenho é aleatorizado por loja e pré-especificado, e a unidade de análise está correta. O efeito de cerca de +7% (IC 3,9%–10,3%) resiste à permutação, ao leave-one-out e a especificações alternativas, e se mantém nas 12 semanas.
```
