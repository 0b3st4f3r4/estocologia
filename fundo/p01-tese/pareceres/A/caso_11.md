# Parecer — caso_11: aditivo de diesel

Script: `bracoA/scripts/caso_11.py` (semente 20261015).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "consumo mensal do caminhão = nível do caminhão + rota/carga/clima do mês + motorista + efeito do aditivo + ruído"
    camada_onde_pode_falhar: "L1 (variação caminhão-a-caminhão e mês-a-mês descrita como grande); L2 (n=5 vs 5, um mês, sem medida de linha de base); L3 (orçar a estimativa pontual de 14%; teste conduzido pelo fornecedor)"
  nao_linearidade_jensen: "economia anual = consumo × km × preço: linear, mas orçar o ponto (14%) ignora a distribuição; o IC da economia vai de 5% a 23%"
  unidade_independente_e_margem:
    unidade: "caminhão-mês; n = 10"
    margem: "diferença -5,09 L/100 km; IC95% -8,3 a -1,9 (−23% a −5%); teste de randomização exato p = 0,0079 (o menor p possível com 5 vs 5)"
  fisica_ruido_agente_ativo: "sem estados absorventes; ruído entre caminhões grande segundo o cliente, mas não medido aqui (sem consumo pré-teste); um único mês"
  escada_causal:
    degrau: "intervenção (sorteio pelo fornecedor, motoristas cegos)"
    identificacao: "randomização com 252 alocações possíveis; sem pré-período não há ajuste por nível do caminhão"
    premissa_nao_testavel: "o sorteio do fornecedor foi honesto e não escolheu caminhões mais econômicos (conflito de interesse; não auditável)"
```

## 2. Ficha da amostra

```yaml
ficha_amostra:
  unidades: "10 caminhões do mesmo modelo/ano, rotas parecidas, de uma frota de 800"
  chave: "caminhao, sem duplicatas"
  duracao: "1 mês"
  linha_de_base: "AUSENTE (consumo dos mesmos caminhões antes do teste não informado)"
  quem_sorteou: "o próprio fornecedor do aditivo"
  escopo_da_decisao: "800 caminhões, possivelmente outros modelos e rotas, por 12 meses"
  km_no_mes: "10732 vs 10187 (sem correlação km–consumo, r=0,18)"
```

## 3. DAG

```
fornecedor ──► sorteio ──► aditivo ──► consumo ◄── nível_caminhão, rota_mês, motorista
                 ? (se o sorteio não foi cego ao nível do caminhão: fornecedor → seleção → consumo)
```

## 4. Modelo nulo e teste severo

- Reproduzido: 30,92 vs 36,01 (−14,1%), t p = 0,0067 (Welch 0,0093).
- **Nulo exato:** todos os 5 com aditivo consumiram menos que todos os 5 sem. Isso ocorre em 2 de 252 alocações sob o nulo, p = 0,0079. Há sinal, mas a evidência está no limite máximo que um desenho 5 vs 5 consegue dar, e depende inteiramente da integridade de um sorteio feito pela parte interessada.
- **Filtro de significância (maldição do vencedor), simulado com o DP observado (2,2 L/100 km):** com efeito real de 3%, os testes que dão significância estimam em média −8,6%; com 5%, −9,5%. Estudos pequenos que "dão certo" exageram o efeito. Os 14% são uma estimativa inflada, condicional ao resultado ter sido significativo.
- O cliente informa que o consumo "varia bastante de caminhão para caminhão de um mês para outro". O DP neste mês (1,6 e 2,7) não representa necessariamente essa variação, e não há pré-período para separar o nível de cada caminhão.
- K = 6 especificações.

## 5. Lentes

- **Crisonomia:** a decisão é dupla: contratar para 800 caminhões **e orçar 14%**. Mesmo que o aditivo funcione, o IC vai de 5% a 23%, e o efeito provável é menor que 14% (maldição do vencedor). Orçar o ponto expõe o orçamento a um déficit. O EVSI de um teste maior (ex.: 40–80 caminhões, 3 meses, com pré-período e sorteio pela transportadora) é alto em relação ao seu custo.
- **Crononomia:** um único mês; não captura sazonalidade nem persistência do efeito.
- **Morfonomia:** sem replicação em outros modelos ou rotas; a invariância não foi testada.

## 6. Ficha da crítica de modelos

```yaml
critica_modelos:
  checagem_preditiva: "modelo gaussiano com DP comum é plausível; com n=5 não se testa"
  modelo_rival: "efeito real pequeno (3–5%) + acaso favorável/seleção: compatível com o que se observa após o filtro de significância"
  identificado_pelos_dados: ["sinal provável de redução neste grupo e neste mês"]
  trazido_de_fora: ["honestidade do sorteio do fornecedor", "transferência para a frota inteira e para 12 meses"]
  performatividade: "o fornecedor controla o desenho; incentivo a apresentar o melhor resultado"
  premissas_indecidiveis: "nível prévio de cada caminhão"
  tentativas_K: 6
```

## 7. Decisão (F8)

Neste teste, os 5 caminhões com aditivo gastaram menos, mas com só 10 caminhões em um mês a economia real pode estar em qualquer ponto entre ~5 e ~23 por 100 litros, e o mais provável é que seja menor que 14. Não assinar para toda a frota com 14% no orçamento. Recomenda-se um teste próprio: sorteio pela transportadora, ≥ 40 caminhões, consumo de 2–3 meses antes e durante, e análise de diferença-em-diferenças. Se precisar decidir já, contrato piloto com orçamento conservador (limite inferior, ~5%) e cláusula de rescisão.
Cláusula de saída: rescindir se, após 3 meses, a economia medida contra um grupo controle interno for menor que o custo do aditivo.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: Com 5 contra 5 caminhões, um único mês, sem linha de base e com sorteio feito pelo próprio fornecedor, a economia fica entre ~5% e ~23%, e os 14% estão inflados pelo filtro de significância. Os dados não sustentam orçar 14% para 800 caminhões.
```
