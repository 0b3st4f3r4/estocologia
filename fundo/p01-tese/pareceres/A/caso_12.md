# Parecer — caso_12: triagem de compostos antibacterianos

Script: `bracoA/scripts/caso_12.py` (semente 20261015).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "DO600 por poço = nível da placa + efeito do composto (provavelmente 0 para a maioria) + ruído de poço (DP ~0,13–0,15)"
    camada_onde_pode_falhar: "L2/L3: 40 testes independentes a α = 0,05 sem correção (comparações múltiplas); ~2 falsos positivos esperados sob o nulo global"
  nao_linearidade_jensen: "DO aproximadamente linear no crescimento na faixa observada (0,55–1,43); efeito relativo = razão de médias"
  unidade_independente_e_margem:
    unidade: "poço dentro de placa (8 vs 8 por composto, sorteados dentro da placa)"
    margem: "efeitos individuais com EP ~6–7% do controle; os 3 'significativos' têm p entre 0,030 e 0,037"
  fisica_ruido_agente_ativo: "sem estados absorventes; variação entre placas nos controles com DP 0,047"
  escada_causal:
    degrau: "intervenção (composto sorteado nos poços)"
    identificacao: "válida por composto; o problema é a inferência simultânea"
    premissa_nao_testavel: "independência entre poços (sem efeito de borda/posição)"
```

## 2. Ficha da amostra

```yaml
ficha_amostra:
  unidades: "640 poços, 40 placas × 16"
  chave: "(placa, condicao, poco), sem duplicatas; 0 faltantes; nenhum poço descartado"
  hipotese_previa: "nenhuma (triagem exploratória de 40 compostos)"
  escopo: "uma placa por composto, sem réplica biológica independente"
```

## 3. DAG

```
sorteio_poço ──► composto ──► crescimento ──► DO600
placa (inóculo, posição) ─────► crescimento
analista ──► seleção dos "vencedores" por p<0,05 ◄── ruído
```

## 4. Modelo nulo e teste severo

- Reproduzido: K40 −18,2% (p 0,030), K12 −11,3% (p 0,035), K19 −11,6% (p 0,037).
- Esperado sob o nulo global: 40 × 0,05 = 2 "significativos"; observados 3.
- **Nulo simulado** (permutação dos 16 poços dentro de cada placa, 2000 réplicas): número médio de reduções significativas = 1,0; P(≥ 3) = 0,085. P(menor p ≤ 0,030) = **0,71**.
- Correções: Bonferroni e Holm não deixam nenhum composto; Benjamini-Hochberg dá q = 0,49 para os três (cerca de metade das "descobertas" seria falsa).
- A distribuição dos 40 p-valores não difere da uniforme (KS p = 0,11). Os efeitos estimados se distribuem em torno de 0 (média −0,9%, DP 7,0%), simétricos: o maior aumento é +15% (K11, p = 0,07), da mesma magnitude que as reduções.
- K = 40 testes primários + 3 correções.

## 5. Lentes

- **Crisonomia:** cada composto custa R$ 400 mil na fase seguinte. Com probabilidade alta de serem falsos positivos, levar os três custa R$ 1,2 milhão de valor esperado em grande parte desperdiçado. Uma replicação (algumas placas por composto, custo baixo) tem EVSI muito alto.
- **Escalonomia:** os efeitos "vencedores" estão inflados (maldição do vencedor); numa replicação, espere efeitos menores.
- **Morfonomia:** sem réplica, a invariância entre placas não foi testada.

## 6. Ficha da crítica de modelos

```yaml
critica_modelos:
  checagem_preditiva: "o nulo global reproduz o número de 'significativos', a magnitude do maior efeito e a simetria aumento/redução"
  modelo_rival: "nenhum composto ativo: compatível com tudo o que foi observado"
  identificado_pelos_dados: ["ruído por poço e por placa"]
  nao_identificado: ["atividade de qualquer composto individual"]
  performatividade: "não se aplica"
  premissas_indecidiveis: "efeito pequeno real em algum composto; exige replicação"
  tentativas_K: 40
```

## 7. Decisão (F8)

Testando 40 compostos sem atividade, espera-se que cerca de 2 "deem significativo" por acaso; aqui deram 3, e nenhum sobrevive à correção para comparações múltiplas. Não levar os três diretamente à fase de R$ 400 mil. Antes, replicar K40, K12 e K19 (e talvez K31) em placas novas e independentes (ex.: 3–4 placas cada, com o critério de sucesso pré-registrado).
Cláusula de saída: avançar apenas compostos cuja redução se repita na replicação com p < 0,05 (unilateral pré-registrado) e efeito ≥ 10%.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: Foram 40 testes sem hipótese prévia: ~2 falsos positivos eram esperados e houve 3, com p entre 0,030 e 0,037. Nenhum sobrevive a Bonferroni, Holm ou BH (q ≈ 0,49), e o nulo simulado reproduz o padrão. Falta replicação antes de gastar R$ 400 mil por composto.
```
