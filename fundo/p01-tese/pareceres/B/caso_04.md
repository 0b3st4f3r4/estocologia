# Parecer — Caso 04: novo checkout

## Métrica pré-registrada
O planejamento aprovado antes do teste definiu como métrica de sucesso a conversão em 7 dias de todos os sorteados. Os recortes e as outras janelas foram explorados depois.

## Resultado na métrica pré-registrada (3000 contra 3000)
- Conversão em 7 dias: 8,20% no novo contra 8,33% no atual. Diferença de −0,13 p.p., IC 95% de −1,53 a +1,26 p.p., p = 0,85.
- As janelas exploratórias apontam até para pior: 14 dias, 11,7% contra 12,5% (p = 0,32); 30 dias, 14,5% contra 15,9% (p = 0,14).

## O segmento destacado
- O recorte "computador, até 35 anos, cliente que já comprou" se reproduz: 13,0% contra 5,9%, p = 0,007.
- É um dos 8 cruzamentos de dispositivo × faixa etária × tipo de cliente, e o corte de idade em 35 também é uma escolha livre. Somando janelas, recortes e cortes, o número efetivo de comparações é muito maior que 8. Só Bonferroni sobre as 8 células já leva o p a 0,058.
- Outra célula, "celular, mais de 35, cliente que já comprou" (n = 1451), mostra o novo checkout pior: 6,0% contra 9,1%, p = 0,027. O padrão de células "significativas" em sentidos opostos é o esperado por acaso quando se garimpa recortes.
- Além disso, a decisão é trocar para todos os usuários. Mesmo que o efeito no segmento fosse real, ele não sustentaria a troca geral: o efeito médio é nulo e o IC exclui ganhos acima de ~1,3 p.p.

## Conclusão
O teste, analisado como planejado, não mostra aumento de conversão. O segmento é resultado de busca post hoc e não sobrevive a correção por multiplicidade. Se houver interesse no segmento, ele deve ser testado num novo experimento confirmatório, com hipótese pré-registrada.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: Na métrica pré-registrada (conversão em 7 dias de todos os usuários), o novo checkout não difere do atual: 8,20% contra 8,33%, IC 95% de −1,5 a +1,3 p.p., p = 0,85. O efeito no segmento foi achado post hoc entre vários recortes e janelas, não sobrevive a correção por multiplicidade e não justifica trocar para todos.
```
