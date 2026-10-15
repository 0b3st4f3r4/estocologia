# Parecer — Caso 11: aditivo de diesel

## Resultados
- Consumo: 30,92 L/100 km com aditivo contra 36,01 sem, diferença de −5,09 L/100 km (−14,1%). Pelo teste t de Student, p = 0,0067 (Welch p = 0,009). O IC 95% da diferença vai de −8,3 a −1,9 L/100 km, ou seja, de 5% a 23% de economia.
- Teste de aleatorização exato (252 alocações possíveis): p = 0,008. A separação é completa: os 5 caminhões com aditivo consomem menos que os 5 sem (Mann-Whitney U = 0).
- Os quilômetros rodados são parecidos (10.732 contra 10.187 km).

## Por que isso não autoriza orçar 14% de economia para 800 caminhões
1. **Imprecisão**: o IC vai de 5% a 23%. Os 14% são uma estimativa pontual com 10 caminhões, e orçar no valor central ignora que 5% é igualmente compatível com os dados. O aditivo pode não se pagar nesse cenário.
2. **Plausibilidade e "maldição do vencedor"**: uma economia de 14% vinda de um aditivo é muito grande para a física do motor diesel. Efeitos grandes e "significativos" vindos de amostras minúsculas tendem a ser superestimados. O próprio cliente diz que o consumo varia bastante de caminhão para caminhão e de mês para mês. Com n = 5 por grupo, um ou dois caminhões num mês atípico (o C06, com 26,65 L/100 km, é o menor de todos) movem muito a média.
3. **Duração e desenho**: foi um único mês, sem medida de consumo dos mesmos caminhões antes do teste (que permitiria comparação pareada ou ajuste pela linha de base), e o sorteio foi feito pelo fornecedor, parte interessada. Não sabemos se o efeito se mantém em outras rotas, estações e motoristas.
4. **Generalização**: 10 caminhões do mesmo modelo e ano em rotas parecidas não representam uma frota de 800.

## Recomendação
Fazer um teste maior antes do contrato: dezenas de caminhões, vários meses, sorteio feito pela própria transportadora, comparação com o consumo prévio de cada caminhão (ou desenho cruzado), e decisão pelo limite inferior do IC da economia contra o custo do aditivo.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: Há sinal de redução, mas com 5 contra 5 caminhões em um mês o IC da economia vai de 5% a 23%. Os 14% são uma estimativa pontual imprecisa e provavelmente inflada, que não sustenta orçar essa economia nem contratar para 800 caminhões sem teste maior, mais longo e com sorteio independente do fornecedor.
```
