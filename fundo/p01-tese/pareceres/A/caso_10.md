# Parecer — caso_10: troca de embalagem de envio (não inferioridade)

Script: `bracoA/scripts/caso_10.py` (semente 20261015).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "avaria ~ Bernoulli(p_embalagem, mês); registro feito pelo cliente em até 15 dias"
    camada_onde_pode_falhar: "L1 (heterogeneidade por mês/sazonalidade; registro pelo cliente) e L2 (margem e regra de decisão)"
  nao_linearidade_jensen: "proporções baixas (~2%); custo linear por avaria. Diferença absoluta é a métrica pré-definida (risco relativo 1,04, citado só como complemento)."
  unidade_independente_e_margem:
    unidade: "envio, sorteado individualmente; n = 40000"
    margem: "diferença +0,07 p.p.; IC95% -0,20 a +0,34 p.p.; IC90% -0,16 a +0,30 p.p.; margem pré-registrada 0,5 p.p."
  fisica_ruido_agente_ativo: "eventos raros mas numerosos (382 e 396); sem estados absorventes; a separação poderia tratar pacotes de forma diferente se visse a embalagem (não cegável)"
  escada_causal:
    degrau: "intervenção (sorteio por pedido)"
    identificacao: "randomização; proporção 50,0% nova, estável por mês (49,6%–50,6%)"
    premissa_nao_testavel: "a probabilidade de o cliente registrar a avaria não depende da embalagem"
```

## 2. Ficha da amostra

```yaml
ficha_amostra:
  unidades: "40000 envios, 20000 por braço, 3 meses"
  chave: "envio, sem duplicatas; 0 faltantes; desfecho registrado em todos"
  pre_registro: "margem de 0,5 p.p. e regra de decisão definidas antes"
  por_mes_nova_menos_atual_pp: {mes1: +0.29, mes2: +0.27, mes3: -0.35}
```

## 3. DAG

```
sorteio ──► embalagem ──► avaria ◄── mês (transporte, sazonalidade)
                            │
                            └──► registro pelo cliente
```

## 4. Modelo nulo e teste severo

- Reproduzido: 1,91% vs 1,98%; IC95% -0,20 a +0,34 p.p. (Wald e Newcomb iguais).
- Estratificado por mês: +0,07 p.p. (IC -0,20 a +0,34).
- Bootstrap (5000): IC -0,20 a +0,35; P(diferença ≥ 0,5 p.p.) = 0,0006.
- **Nulo na margem (teste severo):** simulando uma piora real de exatamente +0,5 p.p., a regra "limite superior < 0,5" seria satisfeita em só 2,4% das vezes (erro tipo I ≤ 2,5% unilateral). Se a verdade fosse "sem diferença", a regra aprovaria em 95%. O teste tinha poder para separar as hipóteses.
- Heterogeneidade por mês: interação p = 0,10. Nos meses 1–2 a nova embalagem ficou ~0,3 p.p. acima; os ICs mensais (até +0,76 p.p.) são largos por terem um terço do n. Isso não invalida o resultado combinado, mas justifica monitorar.
- K = 6 especificações.

## 5. Lentes

- **Crisonomia:** a regra C/L foi pré-registrada (margem 0,5 p.p.) e atendida com folga (limite superior 0,34).
- **Crononomia:** 3 meses; sazonalidade de transporte (picos de fim de ano) não coberta, então a validade fora da janela não está garantida.
- **Escalonomia:** eventos raros mas com contagens suficientes; aproximação normal adequada.

## 6. Ficha da crítica de modelos

```yaml
critica_modelos:
  checagem_preditiva: "binomial com taxa comum por braço reproduz as contagens mensais (interação p=0,10)"
  modelo_rival: "nova pior em 0,5 p.p.: incompatível (P bootstrap 0,0006)"
  identificado_pelos_dados: ["diferença de avaria em 3 meses"]
  trazido_de_fora: ["registro pelo cliente não diferencial", "estabilidade em outras estações"]
  performatividade: "a equipe de separação pode tratar a embalagem nova com mais cuidado durante o teste"
  premissas_indecidiveis: "comportamento em meses de pico"
  tentativas_K: 6
```

## 7. Decisão (F8)

De cada 1000 envios, cerca de 19 tiveram avaria na embalagem atual e 20 na nova. O pior cenário compatível com os dados é cerca de 3 a mais por 1000 envios (0,34 p.p.), abaixo do limite combinado de 5 por 1000. A troca está sustentada pela regra pré-registrada.
Cláusula de saída: monitorar a avaria mensal com grupo de referência (ex.: 5–10% dos envios na embalagem atual). Se a diferença acumulada passar de +0,5 p.p. com IC unilateral 95%, ou se num mês de pico passar de +1 p.p., voltar à embalagem atual.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: Com sorteio por envio, desfecho completo e margem pré-registrada, o limite superior do IC95% da diferença (+0,34 p.p.) fica abaixo da margem de 0,5 p.p. A simulação na margem mostra que o teste tinha erro tipo I ≤ 2,5% e poder de ~95%.
```
