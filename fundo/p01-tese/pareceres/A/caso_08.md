# Parecer — caso_08: troca de fornecedor de rolamentos

Script: `bracoA/scripts/caso_08.py` (semente 20261015).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "defeito ~ Bernoulli(p_fornecedor, lote), com possível variação entre lotes"
    camada_onde_pode_falhar: "L2/L3: tratar 'não significativo' como 'equivalente' (ausência de evidência ≠ evidência de ausência)"
  nao_linearidade_jensen: "custo do defeito é assimétrico (retrabalho do motor inteiro); a perda esperada é convexa em p"
  unidade_independente_e_margem:
    unidade: "peça dentro de lote; 10 lotes (5 por fornecedor); a variação entre lotes é pequena aqui"
    margem: "diferença novo-atual = +1,3 p.p.; IC95% (Newcomb) -4,4 a +7,1 p.p.; IC90% -3,4 a +6,1 p.p.; IC agrupado por lote -4,0 a +6,7 p.p."
  fisica_ruido_agente_ativo: "eventos raros (8 e 10 defeitos); ruído binomial domina"
  escada_causal:
    degrau: "comparação entre fornecedores (associação, mas é a quantidade de interesse)"
    identificacao: "amostragem aleatória dentro dos lotes; mesma equipe e procedimento"
    premissa_nao_testavel: "os 5 lotes representam a produção futura do fornecedor novo"
```

## 2. Ficha da amostra

```yaml
ficha_amostra:
  unidades: "300 peças: 30 sorteadas de cada um de 10 lotes"
  chave: "peca, sem duplicatas; 0 faltantes"
  defeitos_por_lote: {atual: [2,0,2,0,4], novo: [3,1,3,1,2]}
  margem_relevante_definida_pelo_cliente: "+3 p.p."
  escopo: "5 lotes do fornecedor novo é uma amostra pequena da sua variação de processo"
```

## 3. DAG

```
fornecedor ──► processo_fabril ──► lote ──► defeito ──► (inspeção, igual para ambos)
```

## 4. Modelo nulo e teste severo

- Reproduzido: 8/150 vs 10/150, Fisher p = 0,81.
- A pergunta é de **não inferioridade** com margem de +3 p.p. O teste certo é verificar se o limite superior do IC é < 3 p.p. Ele é **+7,1 p.p.** (IC95%) ou +6,1 p.p. (IC90%, unilateral 5%). **Não demonstrado.**
- **Poder (simulação):** se o novo fosse de fato 3 p.p. pior (8,3% vs 5,3%), o teste de Fisher com 150+150 detectaria a diferença em só **12%** das vezes. O "p = 0,81" tinha 88% de chance de aparecer mesmo com a piora relevante.
- Posterior (Beta(1,1)): P(diferença ≥ 3 p.p.) ≈ **27%**; P(novo pior) ≈ 68%.
- Para demonstrar não inferioridade com margem de 3 p.p. (taxas ~6%, poder 80%) seriam necessárias ~770 peças por fornecedor, de mais lotes.
- K = 6 especificações.

## 5. Lentes

- **Crisonomia:** risco assimétrico: 15% de economia contra retrabalho de motor inteiro. Com ~1 chance em 4 de a taxa piorar ≥ 3 p.p., a decisão depende de C/L. Mesmo sem esses números, a premissa "não piora" não está demonstrada.
- **Escalonomia:** eventos raros; IC exato/score em vez de Wald.
- **Crononomia:** a qualidade do fornecedor pode mudar de lote a lote; 5 lotes não caracterizam o processo.

## 6. Ficha da crítica de modelos

```yaml
critica_modelos:
  checagem_preditiva: "modelo binomial sem efeito de lote é compatível (variação entre lotes pequena)"
  modelo_rival: "novo 3 p.p. pior: compatível com os dados (dentro do IC)"
  identificado_pelos_dados: ["taxas ~5–7%"]
  nao_identificado: ["se a diferença é < 3 p.p."]
  performatividade: "fornecedor pode enviar lotes melhores durante a qualificação"
  premissas_indecidiveis: "qualidade dos lotes futuros"
  tentativas_K: 6
```

## 7. Decisão (F8)

De cada 100 rolamentos, cerca de 5 do atual e 7 do novo vieram defeituosos. Com 150 peças de cada, os dados são compatíveis tanto com "igual" quanto com "até 7 a mais por 100", que é bem acima do limite de 3 por 100 aceito pela qualidade. A inspeção não mostra que a troca é segura. Opções: ampliar a inspeção (~770 peças por fornecedor, de mais lotes) antes de trocar, ou trocar parcialmente com inspeção de entrada.
Cláusula de saída: se a taxa do novo, acumulada por lote, superar a do atual em 3 p.p. com IC unilateral, voltar ao fornecedor atual.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: p = 0,81 não demonstra equivalência. O IC95% da diferença (-4,4 a +7,1 p.p.) inclui com folga a piora relevante de 3 p.p., e o teste tinha só ~12% de poder para detectá-la. A não inferioridade não foi demonstrada.
```
