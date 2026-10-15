# H2f: terceira área — deleções de genes em levedura (Kemmeren et al. 2014)

**O G3 passa pela regra declarada, mas a evidência é fraca.** Ŝ ≠ ∅ aparece em 3 áreas: Educação, Física e
Genética. Na genética, porém, há um único domínio, `genetica_y08`, com 2 conjuntos aceitos em 1 024, e o melhor tem
p = 0,007 contra α = 0,005. Os três genes do Ŝ **não** passam na validação de efeito forte de intervenção de
Meinshausen et al., que era informativa. A minha leitura está no fim: cabe a sil decidir se isto aprova a saída do H2.
Em `saida/portoes.json`, G1 (26 testes e reprodução limpa idêntica nos 17 domínios), G2, G3, G4 e a trilha passam, e o script registra "SAÍDA DO H2: aprovada".

```
python3 rodar.py                    # funil nos 17 domínios
python3 portoes.py --reproducao     # portões G1–G5, com segunda execução limpa
```

## Ordem dos passos (pré-registro)

1. **A declaração veio antes.** `DECLARACAO_H2f.md` foi escrita antes de qualquer execução. O hash e a hora UTC estão
   em `DECLARACAO_H2f.sha256` (`2d94bed8…`, 2026-10-15T02:05:14Z).
2. **O código veio depois do hash e implementa a declaração.**
   - `dominios/_kemmeren/comum.py` faz o sorteio dos Y, a pré-seleção e os ambientes.
   - Há 10 pacotes, `genetica_y01` … `y10`.
   - O funil passou a ler `alfa_icp` do pacote; o padrão continua 0,05.
3. **Testes.** São 26 e passaram de primeira. Um deles confere que a pré-seleção não muda quando os valores dos
   mutantes mudam.
4. **Uma execução** do funil nos 17 domínios, sem nada ajustado depois.

## Resultado nos 10 genes Y (α = 0,005 cada)

| domínio | Y | situação | Ŝ | aceitos |
|---|---|---|---|---|
| y01, y05, y06, y09, y10 | — | podados no F3: a chave (o gene mais correlacionado nas selvagens) não vence o nulo de permutação | — | — |
| y02 | | aprovado no F4 | ∅ | 194 |
| y03 | | reprovado no F4 (conforme) | ∅ (todos rejeitados) | 0 |
| y04 | | aprovado no F4 | ∅ (todos rejeitados) | 0 |
| y07 | | aprovado no F4 | ∅ | 775 |
| **y08** | YIL096C | aprovado no F4 | **{plate, x1 = YKL009W, x5 = YNL023C, x9 = YPL193W}** | **2** |

No y08, os dois conjuntos aceitos são {plate, x1, x5, x9} (p = 0,0072) e {plate, x1, x5, x8, x9}. Nenhum dos dois é
saturado. Como o Ŝ tem genes além de `plate`, conta pela regra declarada.

## Validação informativa: efeito forte de intervenção (Meinshausen et al. 2016)

| gene do Ŝ | a deleção funcionou | quantil de Y no mutante ΔX | Y fora da faixa de todas as outras amostras | sinal coerente com o coeficiente (+) |
|---|---|---|---|---|
| x1 YKL009W | sim | 0,056 (Y cai) | não | sim |
| x5 YNL023C | sim | 0,855 (Y sobe) | não | **não** |
| x9 YPL193W | sim | 0,983 (Y sobe) | não | **não** |

Nenhum gene passa no critério estrito. Só o x1 move Y na direção prevista. Para o x5 e o x9, deletar o gene
**aumenta** Y, ao contrário do que o coeficiente invariante positivo prevê.

## Leitura

- **Pela regra escrita,** o G3 passa: Educação (STAR), Física (túnel de luz) e Genética (y08).
- **A evidência da genética é a mais fraca das três.**
  - (a) Há 1 domínio em 10. Com α de Bonferroni isso é legítimo na família, mas o Ŝ nasce de um ICP que rejeitou
    1 022 dos 1 024 conjuntos, com o melhor p perto do limiar. É a situação em que o próprio ICP avisa que o modelo
    está quase rejeitado.
  - (b) A validação intervencional não confirma dois dos três genes.
  - (c) As violações declaradas (efeitos globais de crescimento não medidos, lotes) tornam essa fragilidade
    esperada.
- **Minha recomendação** é registrar o G3 como formalmente aprovado com ressalva, e não dar a saída do H2 por
  aprovada sem a palavra de sil.

## Os outros 7 domínios

Rodaram sem mudança e deram o mesmo resultado do H2e.

## Decisão de sil (2026-10-15T02:34Z)

sil escolheu **"Aprovar com ressalva"**. **A saída do H2 fica aprovada**, com a ressalva registrada acima: a
terceira área (Genética) vem de um único gene Y, num ICP quase rejeitado, e a validação intervencional não confirma
dois dos três genes do Ŝ.
