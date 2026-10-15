# Parecer — Caso 09: turmas pequenas na educação infantil

## Desenho
Os alunos e professores foram sorteados para o tipo de turma dentro de cada escola, de modo que a comparação pequena × regular dentro da escola é experimental. Os dados têm 6.325 alunos (1.900 em turma pequena e 2.194 em turma regular, além das turmas com auxiliar, que excluí desta comparação, como o cliente).

## Verificações
- **Faltantes**: 8,5% sem nota na turma pequena e 8,6% na regular. Dentro da escola, a diferença é de −0,3 p.p. (p = 0,80). A perda é pequena e não difere entre braços.
- **Balanceamento** (dentro da escola) de raça negra, almoço gratuito e sexo: diferenças < 0,6 p.p., todas com p > 0,6. A experiência do professor é semelhante (8,9 contra 9,1 anos).
- **Reprodução**: diferença bruta de 13,9 pontos. Com efeito fixo de escola e erro agrupado por escola (79 escolas, 3.743 alunos): 16,0 pontos (IC 95% de 7,4 a 24,6; p = 0,0003), igual a 0,22 DP da nota das turmas regulares (DP = 73,1).
- **Robustez**:
  - Com covariáveis (sexo, etnia, almoço, experiência do professor): 16,1 (IC de 7,9 a 24,4).
  - Por componente: leitura +6,6 (IC de 3,1 a 10,1) e matemática +8,8 (IC de 3,3 a 14,4).
  - Em 78 escolas com os dois tipos de turma, a diferença média dentro da escola é de +15,3, positiva em 63% delas.
- **Limite de pior caso para os faltantes**: imputando o percentil 5 aos faltantes da turma pequena e o percentil 95 aos da regular, o efeito vira −7,3 (IC de −16,7 a 2,1). É um cenário extremo e implausível, dado que a perda é igual entre braços e as covariáveis estão balanceadas. Sob qualquer mecanismo que não seja fortemente diferencial, o efeito se mantém.

## Ressalvas
- O agrupamento real é por turma/professor dentro da escola. Agrupar por escola é conservador nesse sentido, e o resultado se mantém.
- A validade externa é o ponto fraco: trata-se de outro país, de outra época e de turmas "pequenas" com tamanho específico (~13–17 alunos). O resultado justifica um piloto, que é o que se pede, mas não garante o mesmo tamanho de efeito na rede do cliente.

## Conclusão
A evidência experimental sustenta que turmas pequenas elevaram as notas no fim desse ano, num efeito de ~0,2 DP, robusto e com perdas equilibradas. É suficiente para justificar um piloto, de preferência ele próprio avaliado com sorteio.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: Com sorteio dentro da escola, efeito fixo de escola e erros agrupados, turmas pequenas tiveram +16 pontos (IC 95% de 7,4 a 24,6; ~0,22 DP), com faltantes equilibrados (8,5% contra 8,6%), covariáveis balanceadas e resultados consistentes em leitura e matemática. A transposição para outra rede é a principal ressalva, adequada a um piloto.
```
