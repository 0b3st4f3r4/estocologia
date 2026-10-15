# Parecer — caso_04: novo checkout (teste A/B)

Script: `bracoA/scripts/caso_04.py` (semente 20261015).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "conversão binária por usuário; versão sorteada 50/50; heterogeneidade por segmento desconhecida"
    camada_onde_pode_falhar: "L3/L2: a escolha do segmento foi feita depois de ver os dados (jardim dos caminhos que se bifurcam)"
  nao_linearidade_jensen: "proporções; desfecho binário, sem cauda pesada"
  unidade_independente_e_margem:
    unidade: "usuário (sorteio individual); n = 6000"
    margem_metrica_pre_registrada: "diferença 7d = -0,13 p.p. (IC95% -1,53 a +1,26 p.p.)"
    margem_segmento: "+7,1 p.p. com n = 478; IC 1,9 a 12,7 p.p., não corrigido por seleção"
  fisica_ruido_agente_ativo: "sem estados absorventes; janelas 7/14/30d aninhadas e coerentes (0 violações de monotonicidade)"
  escada_causal:
    degrau: "intervenção (A/B)"
    identificacao: "randomização válida (idade, dispositivo e cliente novo balanceados)"
    premissa_nao_testavel: "nenhuma crítica; o problema é inferencial (seleção de recorte), não de identificação"
```

## 2. Ficha da amostra

```yaml
ficha_amostra:
  unidades: "6000 usuários, 3000/3000"
  chave: "usuario (sem duplicatas); 0 faltantes"
  pre_registro: "métrica primária: conversão 7d de TODOS os sorteados"
  recortes: "idade, dispositivo, tipo de cliente e janelas 14/30d escolhidos depois (não pré-registrados)"
  balanco: "idade 43,8 vs 44,3; computador 39,0% vs 39,5%; cliente novo 39,4% vs 41,0%"
```

## 3. DAG

```
sorteio ──► versão ──► conversão ◄── idade, dispositivo, histórico
                                 (modificadores de efeito, não confundidores)
analista ──► escolha_do_recorte ◄── dados de resultado   (seleção dependente do desfecho)
```

## 4. Modelo nulo e teste severo

- **Métrica pré-registrada:** novo 8,20% vs atual 8,33%, p = 0,85. Nas janelas de 14d e 30d o novo é numericamente **pior** (-0,8 e -1,4 p.p.).
- Reproduzido: segmento alegado 13,0% vs 5,9%, p = 0,0072.
- Em uma grade de 162 recortes (3 dispositivos × 6 cortes de idade × 3 tipos de cliente × 3 janelas), 18 têm p < 0,05, e os mais fortes favorecem o checkout **atual** (ex.: > 35 anos, cliente antigo, 14d: -4,1 p.p., p = 0,002).
- **Nulo simulado:** permutando os rótulos de versão (efeito zero por construção) e buscando o menor p na mesma grade, o mínimo fica ≤ 0,007 em **30%** das simulações e < 0,05 em 83%. O p = 0,007 do segmento é compatível com puro ruído mais a busca.
- Bonferroni para K = 162: limiar 0,0003; nenhum recorte passa.

## 5. Lentes

- **Crisonomia:** a decisão cobre todos os usuários. Para esse público, a melhor estimativa é ~0 (IC -1,5 a +1,3 p.p.), e as janelas mais longas sugerem possível perda.
- **Morfonomia (invariância):** o efeito não é invariante entre janelas nem entre recortes; muda de sinal.
- **M12 (contagem de tentativas):** K efetivo desconhecido do lado do cliente; a grade reconstruída tem pelo menos 162.

## 6. Ficha da crítica de modelos

```yaml
critica_modelos:
  checagem_preditiva: "sob o nulo global, a distribuição do mínimo p da grade tem mediana 0,016; o 0,007 observado está no terço inferior, não é extremo"
  modelo_rival: "efeito nulo homogêneo: explica a métrica primária e o achado do segmento"
  identificado_pelos_dados: ["efeito médio global ≈ 0"]
  trazido_de_fora: ["nenhuma hipótese a priori sobre o segmento"]
  performatividade: "não se aplica"
  premissas_indecidiveis: "efeito real em subgrupos pequenos; exigiria um teste confirmatório dedicado"
  tentativas_K: ">= 162 (reconstruídas)"
```

## 7. Decisão (F8)

De cada 100 usuários, cerca de 8 convertem em 7 dias com qualquer das duas versões. O teste não mostra ganho com o novo checkout para o público total, que é a métrica combinada antes do teste. O segmento destacado foi escolhido depois de ver os resultados, e um padrão igual aparece por acaso em ~3 de cada 10 buscas sem efeito real. Não substituir com base neste teste. Se o segmento interessar, fazer um novo A/B confirmatório só nele, com métrica e tamanho pré-registrados.
Cláusula de saída: se trocar mesmo assim, monitorar a conversão em 30d e reverter se cair ≥ 1 p.p.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: Na métrica pré-registrada (conversão 7d de todos os usuários) não há efeito: -0,13 p.p., p = 0,85. O segmento significativo foi escolhido depois, entre pelo menos 162 recortes possíveis, e um p igual ou menor sai da busca sob o nulo em ~30% das vezes.
```
