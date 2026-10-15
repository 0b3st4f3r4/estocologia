# Parecer — Caso 05: teste de seleção de vendedores

## Resultados
- Pearson r = 0,65 (p = 0,0001) se reproduz. Mas as correlações de postos, robustas a valores extremos, são quase nulas: Spearman ρ = 0,14 (p = 0,45) e Kendall τ = 0,11 (p = 0,39).
- Um único vendedor, V29 (nota 97, vendas de 980 mil, mais que o dobro do segundo maior, 493 mil), concentra toda a correlação. Sua distância de Cook é 9,1 (o limiar habitual de alerta é ~1) e o resíduo estudentizado é 8,4. As notas dos outros 29 vão de 45 a 75.
- Sem o V29: r = 0,11 (p = 0,58), Spearman 0,05. Entre 45 e 75 pontos, faixa em que está praticamente toda a decisão, a nota não prevê as vendas.
- O bootstrap do r (10.000 reamostras) dá IC 95% de −0,19 a 0,87, muito instável. Os resíduos da regressão não são normais (Jarque-Bera p = 0,001), o que invalida o p de Pearson.
- Corte proposto de 70 pontos: quem tirou ≥ 70 (n = 6) tem mediana de vendas de 413,5 mil, contra 413,2 mil de quem tirou < 70 (n = 24); Mann-Whitney p = 1,0. Afora o V29, os aprovados no corte venderam igual ou menos (ex.: V24, nota 75, vendeu 339 mil).

## Ponto positivo do desenho
No ano passado a nota não foi usada para contratar, então não há restrição de amplitude por seleção. O problema é outro: a amostra é pequena e a associação depende de um ponto.

## Conclusão
A "correlação forte" é artefato de um único caso atípico. Na massa dos dados não há relação, e o corte em 70 não separa melhores vendedores. Adotar o filtro eliminatório agora descartaria candidatos sem ganho demonstrado. Antes disso, convém acumular mais dados de validação com a nota ainda não usada como filtro.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: O r = 0,65 é produzido por um único vendedor atípico (nota 97, vendas de 980 mil; Cook = 9,1). Sem ele, r = 0,11 (p = 0,58), e a correlação de Spearman com todos é 0,14 (p = 0,45). O corte em 70 não separa vendas (medianas de 413 contra 414 mil).
```
