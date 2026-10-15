# Parecer — caso_05: teste de seleção de vendedores

Script: `bracoA/scripts/caso_05.py` (semente 20261015).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "vendas do 1º ano = f(aptidão, carteira/território, mercado) + ruído; nota = medida ruidosa da aptidão"
    camada_onde_pode_falhar: "L2: Pearson é dominado por um único ponto extremo; L3: a regra de corte em 70 extrapola uma relação que não existe no grosso dos dados"
  nao_linearidade_jensen: "a regra de corte é uma função degrau da nota; a média de vendas acima de 70 (496) é puxada por um ponto. A mediana acima de 70 (413,5) é igual à mediana abaixo (413,2)."
  unidade_independente_e_margem:
    unidade: "vendedor, n = 30"
    margem: "bootstrap de r: IC95% -0,19 a 0,87 (bimodal, conforme o ponto V29 entra ou não)"
  fisica_ruido_agente_ativo: "vendas têm cauda à direita (V29 = 980 mil, z = 12 em relação aos demais); sem estados absorventes; todos completaram 12 meses"
  escada_causal:
    degrau: "associação (predição), adequada a uma decisão de seleção"
    identificacao: "não precisa de causalidade, mas precisa de associação robusta e generalizável"
    premissa_nao_testavel: "a contratação no ano passado não dependeu da nota (afirmado); a entrevista filtrou de forma igual"
```

## 2. Ficha da amostra

```yaml
ficha_amostra:
  populacao_alvo: "candidatos aprovados na entrevista"
  amostra: "30 contratados do ano passado (todos os aprovados, sem filtro pela nota)"
  chave: "vendedor, sem duplicatas; 0 faltantes"
  ponto_influente: "V29: nota 97 (z = 4,9) e vendas 980 (z = 12,3); a ficha deve verificar erro de registro ou circunstância especial (ex.: carteira herdada)"
  com_nota_>=_70: "6 vendedores, 5 deles com vendas entre 339 e 436"
```

## 3. DAG

```
aptidão ──► nota_teste
aptidão ──► vendas ◄── território/carteira ◄── (alocação do vendedor?)
entrevista (filtro) ──► amostra
```

Se V29 recebeu uma carteira grande por ser "promissor", haveria um caminho nota → alocação → vendas que não generaliza para uma regra de corte.

## 4. Modelo nulo e teste severo

- Reproduzido: Pearson r = 0,648, p = 1,1e-4.
- **Sem V29:** r = 0,106 (p = 0,58); Spearman 0,05.
- **Com todos, mas medidas robustas:** Spearman 0,14 (p = 0,45); Kendall 0,11 (p = 0,39).
- Leave-one-out: r varia de 0,11 (sem V29) a 0,71. **Um único vendedor fabrica a correlação.**
- Nulo por permutação: com V29, p = 0,0099 (a permutação mantém a alavanca do ponto, e o resultado continua dependente de um ponto); sem V29, p = 0,58.
- Regra proposta (≥ 70): sem V29, os 5 com nota ≥ 70 venderam em média 399 contra 414 dos demais (Mann-Whitney p = 0,52). A regra eliminaria vendedores tão bons quanto os aprovados.
- K = 6 especificações.

## 5. Lentes

- **Escalonomia:** com cauda à direita, a média e a correlação de Pearson são dominadas por um ponto; usar estatísticas por postos, a mediana e a eCDF. A eCDF de vendas por grupo de nota é praticamente sobreposta sem V29.
- **Crisonomia:** o filtro eliminatório tem custo: perder bons vendedores, e 24 de cada 30 contratados do ano passado teriam sido reprovados. O benefício depende de uma única observação.
- **Morfonomia:** a relação não é invariante à retirada de um ponto.

## 6. Ficha da crítica de modelos

```yaml
critica_modelos:
  checagem_preditiva: "uma regressão linear ajustada sem V29 prevê para nota 97 vendas de ~450, contra 980 observadas: V29 é inconsistente com o resto"
  modelo_rival: "nenhuma associação + um outlier: explica os dados igualmente (r_sem_V29 = 0,11)"
  identificado_pelos_dados: ["ausência de associação entre notas 45 e 75"]
  trazido_de_fora: ["natureza do caso V29"]
  performatividade: "um filtro em 70 muda a população contratada (amostra truncada no futuro); validar depois fica impossível sem manter candidatos abaixo do corte"
  premissas_indecidiveis: "se V29 reflete aptidão excepcional ou sorte/carteira"
  tentativas_K: 6
```

## 7. Decisão (F8)

De 29 dos 30 vendedores, a nota não ordena as vendas: quem tirou 70 ou mais vendeu, na mediana, o mesmo que os outros. A "correlação forte" vem de um único vendedor. Não adotar o corte eliminatório. Investigar V29 e coletar mais coortes, mantendo o teste como informação não eliminatória.
Cláusula de saída: só reconsiderar se, em ≥ 2 coortes, a correlação por postos for > 0,3 sem depender de um ponto.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: O r = 0,65 vem quase todo de um único vendedor extremo (V29, nota 97, vendas 980). Sem ele r = 0,11, e as medidas robustas (Spearman 0,14) não mostram associação. Acima de 70, a mediana de vendas é a mesma dos demais.
```
