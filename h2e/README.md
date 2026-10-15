# H2e: dois domínios novos com intervenção em X contínuo (Causal Chambers)

**Saída do H2e: NÃO aprovada.** O G3 falha: Ŝ ≠ ∅ em **2 domínios, 2 áreas** (Educação e Física), e o critério pede
3. O túnel de luz recupera exatamente os pais documentados pelos autores. O túnel de vento rejeita todos os 1 024
conjuntos, inclusive o dos pais verdadeiros. G1 (24 testes e a reprodução limpa idêntica nos 7 domínios), G2, G4 e a trilha passam (`saida/portoes.json`).

```
python3 rodar.py                    # funil nos 7 domínios
python3 portoes.py --reproducao     # portões G1–G5, com segunda execução limpa
```

## Ordem dos passos (pré-registro)

1. **A declaração veio antes.** `DECLARACAO_H2e.md` foi escrita antes de qualquer execução do código do H2e. O
   SHA-256 e a hora UTC estão em `DECLARACAO_H2e.sha256` (`22c4d007…`, 2026-10-15T00:26:52Z). Os pacotes dos dois
   domínios novos foram escritos depois do hash, e implementam a declaração.
2. **Testes.** `tests/test_h2e.py` confere a carga dos pacotes novos e o formato das tabelas.
   - Falhou uma vez por um erro de código: no pandas 3, `groupby().apply` descarta a coluna do grupo.
   - O desbaste foi reescrito com `cumcount`, sem mudar o que faz. Depois disso, os 24 testes passaram.
3. **Uma execução** do funil nos 7 domínios. Nada foi ajustado depois.
4. **Uma olhada a mais, declarada.** Ao conferir o cabeçalho do CSV do túnel de vento, apareceram as duas
   primeiras linhas brutas do regime 0, com os valores de pressão. Não foi feito nenhum resumo por ambiente.

## Resultado por domínio

O ICP é o mesmo do H2d, sem mudança: teste K = 2 mais a regra de saturação K = 3.

| domínio | área | Ŝ | aceitos | nota |
|---|---|---|---|---|
| fisica_tunel_luz (novo, K = 1) | Física | **{red, green, blue}** | 128 de 1 024 | igual aos pais do grafo dos autores; p({red, green, blue}) = 0,29 |
| engenharia_tunel_vento (novo, K = 1) | Engenharia | ∅ (todos rejeitados) | 0 | p dos pais verdadeiros ≈ 0; melhor p = 7e-5 |
| educacao_star_turmas | Educação | {turma_pequena, anos_pequena_antes} | 57 | igual ao H2d |
| biologia_sachs | Biologia | ∅ (todos rejeitados) | 0 | igual ao H2d |
| saude_rand | Saúde | ∅ (todos rejeitados) | 0 | igual ao H2d |
| direito_teto_beneficio | Direito | ∅ | 578 | igual ao H2d |
| transporte_cinto | Transporte | ∅ | 496 | igual ao H2d |

### Túnel de luz
- **ICP.** O Ŝ é exatamente o conjunto de pais da corrente no grafo de referência. Os sensores de luz se
  correlacionam com a corrente, porque são filhos dos LEDs, e mesmo assim ficam de fora; os polarizadores também. É
  a primeira vez no projeto que o ICP é conferido contra uma verdade conhecida, e ele acerta.
- **Fora do ambiente.** O cofre tem o vermelho entre 171 e 255, nunca visto. Ali a habilidade é +0,959 tanto com o
  modelo invariante quanto com o ajustado, e a cobertura conforme é 0,92.
- **Funil.**
  - F1: EVPI 1,19 contra um custo de 0,05. A decisão é inventada e não informa nada.
  - F3: p = 0,005, o piso da permutação.
  - Posterior do efeito do vermelho: θ = 4,5 dp, abaixo da mediana da priori (6). Os pesos dos três LEDs não são
    iguais.

### Túnel de vento
- **ICP.** Rejeita tudo. Os pais verdadeiros {load_in, load_out, hatch, osr_downwind} têm p ≈ 0. Os melhores
  conjuntos trocam as cargas por `pressure_intake` e `pressure_ambient`. Isso combina com a deriva atmosférica
  declarada antes: os barômetros medem pressão absoluta, e o clima muda Y entre regimes sem estar no grafo.
- **A deriva não explica tudo.** Nem com `pressure_ambient` como proxy algum conjunto fica invariante: o melhor p é
  7e-5, em {hatch, osr_downwind, pressure_ambient, pressure_intake}. Há duas outras causas plausíveis, não
  testadas:
  - a autocorrelação que sobra depois do desbaste;
  - a forma aditiva do ajuste, contra uma pressão que é quadrática na rotação e tem interação entre os dois
    ventiladores.
- **Funil.** O M15 reprova porque o modelo ajustado perde para a climatologia no cofre (−2,18). O modelo com todos
  os candidatos faz +0,999.

## Leitura do G3

- **Pelo critério escrito:** são 2 áreas (Educação e Física), e o G3 falha.
- **Pela leitura conservadora declarada,** com as duas câmaras contando como uma área só, dá o mesmo: 2 áreas,
  porque só a câmara de luz deu Ŝ. A ressalva sobre as câmaras não precisou ser decidida.

## Os dados

Os dados estão em `../dados-reais/h2e/FONTES.md`, com SHA256SUMS. Ficaram de fora, por motivos declarados antes de
rodar, Las Rosas (agricultura), CigarettesSW e Cigar (economia).
