# H2g: terceira área sólida — farmacologia cardíaca (ensaio do FDA, ECGRDVQ)

**O G3 passa sem depender da genética.** O ICP dá Ŝ ≠ ∅ em 4 áreas: Educação, Física, Genética e Farmacologia.
Sem o y08 da levedura, sobram 3 áreas, que é o que o PLAN exige.

O Ŝ da farmacologia é **{c_dof, c_qui, c_ran}**: as concentrações das três drogas que bloqueiam o hERG. É o resultado
que a verdade de referência, declarada antes, chama de coerente, e não traz `pr` nem `tamp`.

Os outros portões passam: G1 (27 testes e reprodução limpa idêntica nos 18 domínios), G2, G4 e a trilha
(`saida/portoes.json`).

```
python3 rodar.py                    # funil nos 18 domínios
python3 portoes.py --reproducao     # portões G1–G5, com segunda execução limpa
```

## Ordem dos passos (pré-registro)

1. **A declaração veio antes.** `DECLARACAO_H2g.md` foi escrita antes de qualquer execução, com o hash
   `a5d879b3…`, às 2026-10-15T03:08:50Z.
2. **O pacote veio depois do hash e implementa a declaração.** São `dominios/farmacologia_qt/` e
   `tests/test_h2g.py`.
3. **Testes.** São 27 e passaram de primeira.
4. **Uma execução** do funil e dos portões, sem nada ajustado depois.

## farmacologia_qt

- **Funil.**
  - F1: EVPI 4,7, mas a decisão é inventada.
  - F2: EP por voluntário de 1,4 ms por ng/mL, contra um dp da priori de 11,9.
  - F3: p = 0,005, o piso.
  - O M15 aprova.
- **ICP.** Aceita 35 conjuntos de 512, e nenhum é saturado.
  - Todos os 35 contêm as três concentrações.
  - A interseção é {c_dof, c_qui, c_ran}.
  - `pr`, `tamp`, `rr`, o sexo, a idade e o peso aparecem em alguns conjuntos aceitos e em outros não, por isso
    ficam fora de Ŝ.
- **Posterior da chave.** A inclinação do QTcF na dofetilida é 25,2 ms por ng/mL, com IC90 de 23,0 a 27,5. Está
  dentro dos quartis da priori, que foi tirada de memória com mediana 28.
- **Fora do ambiente, no verapamil, nunca visto.**
  - A habilidade é +0,57 com o modelo invariante e +0,65 com o ajustado.
  - A cobertura conforme é 0,89.
  - O modelo com **todos** os candidatos explode: a perda é da ordem de 10¹⁵. O verapamil alonga o PR para fora
    da faixa do treino, e a spline extrapola. É o exemplo de livro do que o ICP protege: o PR prevê bem dentro do
    treino e falha sob uma intervenção nova.

## Leitura

- **Pelo critério escrito,** o G3 passa com 4 áreas.
- **Sem a genética,** que sil aprovou com ressalva, sobram 3 áreas: Educação (STAR), Física (túnel de luz, que
  recupera a verdade do grafo dos autores) e Farmacologia (o ensaio do FDA, que recupera as drogas que bloqueiam o
  hERG). Duas das três têm verdade de referência externa e coincidem com ela.
- **Ressalva que continua.** O teste supõe linhas independentes, e aqui há 16 instantes por voluntário. Isso
  tenderia a rejeitar mais, e mesmo assim os conjuntos com as drogas foram aceitos.
