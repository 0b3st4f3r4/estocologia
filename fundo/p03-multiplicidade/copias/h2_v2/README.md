# H2: transferência interdomínios

**Saída do H2: NÃO aprovada.** Os portões de reprodução, funil, contenção de varredura e trilha passam. O portão que o
PLAN escolheu para o H2, a Predição Causal Invariante (ICP) entre ambientes, falha: o critério pedia Ŝ ≠ ∅ em pelo
menos 3 domínios de áreas distintas, e só 1 (direito) entregou.

São 12 domínios reais, plugados pelo pacote padronizado do F7 (`dominios/<nome>/pacote.yaml` + `preparo.py`). Os dados
estão em `../dados-reais/h2/` (Rdatasets, com SHA-256).

```
python3 rodar.py                    # funil nos 12 domínios (~4 min, a logística sozinha leva ~3)
python3 portoes.py --reproducao     # portões G1–G4, com segunda execução limpa
python3 grafo.py                    # hipergrafo: 1 538 nós, 1 700 arestas de árvore, 62 pontes, 630 hiperarestas
```

## O que foi construído

- **Pacote de domínio plug-and-play.** É o contrato do F7: pergunta substantiva, alegação, glossário de 10 termos,
  fontes, unidade independente, ambientes e cofre, DAG, conjunto de ajuste, elicitação SHELF, decisão e escopo. O
  validador recusa o pacote incompleto antes de qualquer leitura de dados. Dez dos doze pacotes foram escritos por
  agentes que só leram o contrato e dois exemplos: é o teste de plugabilidade.
- **Funil de 4 filtros** (PLAN §2):
  - **F1:** EVPI sob a priori contra o custo da pesquisa.
  - **F2:** porta dos fundos no DAG e informatividade na unidade.
  - **F3:** nulo de permutação no nível em que a chave foi atribuída.
  - **F4:** ICP, MCMC, SBC, pré-registro, cofre de outro ambiente, conforme, M15 e F8.
- **Amostrador adaptativo.** Metropolis de Haario com proposta congelada depois do aquecimento, misturado a um passo
  independente t₅ centrado no MAP com a hessiana numérica. O R-hat ficou abaixo de 1,008 e o ESS acima de 900 nos
  11 domínios, com dimensão de até 15. O passo fixo do H1 falhava em cerca de 40% das sementes com d ≥ 5.
- **Armazém DuckDB.**
  - Lê Parquet particionado por ambiente.
  - Recusa `SELECT *`.
  - Confere a projeção no `EXPLAIN` de cada leitura.
  - Cobra os bytes de um orçamento por agente.
  - Mantém o cofre trancado até o F3 apresentar o hash do pré-registro.

## Resultados por domínio

| domínio | chave → y | cofre | n / unidades (efeito de desenho) | efeito (IC 90%) | ICP | habilidade no cofre | status |
|---|---|---|---|---|---|---|---|
| educação (STAR) | turma pequena → nota | escolas urbanas | 5 250 / 72 escolas (3,3) | +13,5 pontos [7,5; 19,5] | rejeitado | 0,17 | aprovado, adotar (P = 0,84) |
| logística (voos NY) | hora programada → atraso | LaGuardia | 225 753 / 364 dias (11,5) | +2,3 p.p. por hora [2,2; 2,4] | rejeitado | 0,09 | aprovado, adotar (P = 1,00); K = 2 |
| agronomia (milho, Argentina) | N → produtividade | baixada | 2 558 / 36 faixas (1,4) | 0,059 q/ha por kg [0,046; 0,074] | rejeitado | 0,55 | aprovado, adotar (P = 0,86) |
| clima (ciclones) | √(1013 − p) → vento | Golfo do México | 17 597 / 563 tempestades (22,8) | 11,8 nós [11,5; 12,1] | rejeitado | 0,79 | aprovado, manter (P = 0,14) |
| crédito (HMDA Boston) | requerente negro → negativa | desemprego alto | 1 757 / 1 757 (1,0) | +6,7 p.p. [4,0; 9,3] | vazio aceito | 0,16 | aprovado, adotar (P = 0,95) |
| desenvolvimento (Preston) | log PIB → expectativa de vida | 1997–2007 | 1 278 / 142 países (9,8) | +5,1 anos [4,1; 6,0] | rejeitado | 0,82 | aprovado, adotar (P = 0,57) |
| energia (Victoria) | graus-dia de resfriamento → demanda | 2014 | 730 / 105 semanas (1,6) | 126 MW [118; 134] | rejeitado | 0,88 | aprovado, adotar (P = 1,00) |
| epidemiologia (Bangladesh) | ter filhos → contracepção | distritos g4 | 1 456 / 45 distritos (1,5) | +21,7 p.p. [14,4; 29,2] | Ŝ = ∅ (aceitos disjuntos) | 0,04 | aprovado, adotar (P = 0,64) |
| saúde (NMES 1988) | seguro privado → consultas | Oeste | 3 608 / 3 608 (1,1) | +0,31 log-consultas [0,24; 0,37] | vazio aceito | 0,14 | aprovado, adotar (P = 1,00) |
| direito (Toronto) | ser negro → liberação | 2002 | 4 949 / 4 949 (1,0) | −10,3 p.p. [−12,1; −8,5] | **Ŝ = {cidadão, empregado}** | −0,01 | reprovado: perde para a climatologia |
| trabalho (currículos) | nome afro → retorno | Boston | 2 704 / 426 anúncios (0,7) | −2,3 p.p. [−3,5; −1,1] | vazio aceito | 0,02 | reprovado: cobertura conforme 0,79 |
| trânsito (imposto da cerveja) | imposto → mortes | Nordeste | 273 / 39 estados (2,4) | — | — | — | podado no F3 (p = 0,41) |

## O que o H2 ensina

1. **O ICP quase nunca acha um conjunto invariante nestes dados.** Em 6 domínios ele rejeitou todos os conjuntos: os
   ambientes disponíveis (tipo de escola, aeroporto, relevo, região de gênese, ano) agem direto em Y, ou há pai
   oculto, ou a relação não é linear no n grande. Em outros 4 aceitou tanto que Ŝ ficou vazio, porque os ambientes
   mexem pouco em X. Só no direito Ŝ saiu não vazio, e saiu contra a alegação: a cor da pele não está em Ŝ; cidadania
   e emprego estão. Com 95% de cobertura, isso diz que os pais invariantes da liberação incluem cidadania e emprego.
   Não diz que a cor não importa, porque o ICP só garante Ŝ ⊆ pais.
2. **O efeito transporta melhor que o conjunto.** Mesmo com o ICP rejeitado, o modelo ajustado previu bem o ambiente
   nunca visto em 9 domínios (habilidade de 0,04 a 0,88). A alegação "vale em qualquer escola, aeroporto ou região"
   cai na forma forte (a distribuição condicional muda), mas a direção e o tamanho da chave sobrevivem. Para decidir,
   é essa versão fraca que importa, e o PLAN pedia a forte.
3. **A pseudorreplicação aparece em toda parte.** O efeito de desenho chega a 11,5 nos voos (225 mil voos valem 364
   dias), 22,8 nos ciclones e 9,8 no Preston. A verossimilhança temperada por 1/efeito de desenho alarga a posterior
   de acordo.
4. **A literatura erra para os dois lados.**
   - Nos voos, a priori (razão de chances de 1,08 por hora) ficou abaixo do dado (1,14).
   - Nos ciclones, a priori de 15 nós/√hPa ficou acima de 11,8.
   - Nos currículos, a priori de "um terço a menos" virou −2,3 p.p.
5. **O funil podou pelos motivos certos.**
   - O F2 barrou a primeira especificação dos voos: o ajuste incluía a temperatura, descendente da hora no próprio
     DAG. O pacote foi corrigido, e K = 2 ficou registrado.
   - O F3 podou o imposto da cerveja: com efeito fixo de estado, o imposto não se distingue do nulo.

## Mudanças depois de ver resultados (o K do programa)

- **Arcabouço v2.** A auditoria dos agentes de domínio achou quatro falhas, corrigidas para todos os domínios de uma
  vez:
  - **Nulo do F3.** Ele agora permuta no nível em que a chave varia: entre unidades, dentro da unidade quando o
    sorteio é declarado, ou médias e desvios quando a chave é observacional.
  - **Verossimilhança temperada por 1/efeito de desenho.** A posterior iid era estreita demais.
  - **Conforme por ambiente deixado de fora.** A calibração num quinto sorteado do treino não carregava a mudança
    entre ambientes.
  - **Rótulos do ICP.** "Rejeitado" agora se distingue de "vazio aceito".

  O v2 mudou o status de 4 domínios. Com o v1, energia e ciclones caíam na cobertura conforme, o crédito no R-hat
  (1,011) e o Preston no F3. Os números acima são do v2.
- **Critério do G2.** O critério declarado era "erro < 1e-6 no ajuste SHELF", impossível para quartis assimétricos
  (razão de chances em log). Na primeira execução, o G2 falhou só por isso. O critério passou a ser 10% do dp
  ajustado; o maior erro é 5%. A troca está registrada em `portoes.py` e não toca o G3, que decide a saída.

## Limites declarados

- **Elicitação.** Nenhum especialista humano foi consultado. Todas as prioris SHELF vêm de literatura citada de
  memória e marcada "por conferir".
- **Custos.** Benefícios e custos das decisões são suposições nos pacotes.
- **Agrupamento.** Em 3 domínios (crédito, direito, saúde) a base não tem identificador de agrupamento, então a
  unidade é a linha e o erro padrão pode ser otimista. O F2 marca esses casos.
- **Poder do ICP.** O ICP com só 2 ambientes de treino (currículos) tem pouco poder.
- **Limiares do M15.** Os limiares do M15 (R-hat < 1,01, cobertura ≥ 0,85) são cortes duros.
- **Escala.** O hipergrafo tem 1 538 nós, contra a meta de 10⁴–10⁵ do PLAN. Os dados somam 1,6 MB varridos, longe
  de "bases Parquet" em escala.
- **Pareceres.** Os 12 pareceres de risco esperam a assinatura de sil.

## O próximo passo para passar no portão

O critério do PLAN exige ambientes que intervenham em X sem agir em Y. Isso pede dados com intervenções conhecidas:
vários sorteios, mudanças de regra, ou experimentos multicêntricos. Ambientes de conveniência (ano, região) não
servem. Há duas saídas:

- **Trocar os ambientes.** É preciso buscar bases assim, e várias estão fora do alcance da rede deste ambiente.
- **Rever o critério.** Aceitar a forma fraca, invariância do efeito da chave com intercepto livre por ambiente, é
  uma decisão sua sobre o que o H2 deve demonstrar.
