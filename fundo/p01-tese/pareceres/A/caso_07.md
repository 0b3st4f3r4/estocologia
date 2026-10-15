# Parecer — caso_07: aplicativo de higiene do sono (efeito por sexo)

Script: `bracoA/scripts/caso_07.py` (semente 20261015).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "sono = nível individual + efeito do app (possivelmente heterogêneo por sexo) + ruído de medida da pulseira"
    camada_onde_pode_falhar: "L2/L3: concluir 'sem efeito nos homens' a partir de p > 0,05 e 'diferença entre sexos' sem testar a interação"
  nao_linearidade_jensen: "desfecho aditivo em minutos, sem cauda pesada (285–516); média adequada"
  unidade_independente_e_margem:
    unidade: "participante, 60 por célula"
    margens:
      mulheres: "+17,5 min (IC95% 3,9 a 31,1)"
      homens: "+8,2 min (IC95% -8,2 a 24,6)"
      diferenca_entre_sexos: "-9,3 min (IC95% -30,4 a +11,9)"
  fisica_ruido_agente_ativo: "sem estados absorventes; ruído gaussiano; DP 37–47 min"
  escada_causal:
    degrau: "intervenção (sorteio estratificado por sexo)"
    identificacao: "randomização dentro de cada sexo; a interação é identificada, mas com pouca precisão"
    premissa_nao_testavel: "a pulseira mede sono igualmente nos dois sexos"
```

## 2. Ficha da amostra

```yaml
ficha_amostra:
  unidades: "240 adultos (inscrição por anúncio, queixa de pouco sono)"
  estratos: "120 F / 120 M; 60 app e 60 controle em cada sexo"
  chave: "participante, sem duplicatas; 0 faltantes; 0 perdas"
  escopo: "adultos autosselecionados com queixa; não é a população geral de usuários"
```

## 3. DAG

```
sorteio(estrato sexo) ──► app ──► sono ◄── sexo
                              (sexo como possível modificador de efeito)
```

## 4. Modelo nulo e teste severo

- Reproduzido: F +17,5 (p = 0,012); M +8,2 (p = 0,32).
- **Teste da alegação real (interação app × sexo):** -9,25 min, p = 0,39 (OLS); permutação p = 0,40. Não há evidência de que o efeito difere entre os sexos.
- Efeito comum (modelo aditivo): +12,9 min (IC95% 2,3 a 23,4), p = 0,017.
- **Nulo simulado:** se o efeito fosse **igual** nos dois sexos (+12,9 min, DP observado), a probabilidade de sair "um sexo significativo e o outro não" é **48%**. O padrão observado é o resultado típico de um efeito homogêneo com 60 por braço.
- "p = 0,32 nos homens" significa falta de precisão, não ausência de efeito: o IC dos homens inclui o efeito estimado nas mulheres.
- K = 4 especificações.

## 5. Lentes

- **Crisonomia:** a decisão (abandonar homens) tem custo alto: perder metade do mercado com base numa diferença que não se distingue de zero. O EVSI de um teste da interação com mais n é alto.
- **Morfonomia:** a melhor descrição compacta (MDL) é um efeito comum: o modelo com interação não melhora o ajuste (p = 0,39).
- Escalonomia e Crononomia: não se aplicam.

## 6. Ficha da crítica de modelos

```yaml
critica_modelos:
  checagem_preditiva: "réplicas de um efeito homogêneo reproduzem o par (p=0,012; p=0,32) com frequência alta"
  modelo_rival: "efeito comum de ~13 min nos dois sexos: explica os dados tão bem quanto o modelo heterogêneo"
  identificado_pelos_dados: ["efeito médio total > 0"]
  nao_identificado: ["diferença de efeito entre sexos (IC -30 a +12)"]
  performatividade: "não se aplica"
  premissas_indecidiveis: "efeito real nos homens entre -8 e +25 min"
  tentativas_K: 4
```

## 7. Decisão (F8)

O app parece aumentar o sono em cerca de 13 minutos por noite no total (IC 2 a 23). Os dados não mostram que o efeito seja diferente entre mulheres e homens: a diferença entre os dois efeitos tem IC de -30 a +12 minutos. Não reorientar o produto nem cortar a aquisição de homens com base nisso. Se a segmentação importar, fazer um ensaio dimensionado para a interação (centenas por célula).

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: Ter p < 0,05 num sexo e p > 0,05 no outro não mostra que os efeitos são diferentes. A interação app × sexo não é significativa (p ≈ 0,39) e o IC dos homens inclui o efeito das mulheres. Um efeito igual nos dois sexos produziria esse padrão em ~48% das vezes.
```
