# Brief da passada de dois-pontos

Segunda passada, focada em **um** padrão: o dois-pontos de prosa que só introduz um período explicativo.
O resto do capítulo já foi reescrito e **não volta atrás**: não reintroduza travessão apostivo, não
devolva `mesmo/mesma`, não desfaça nenhuma troca anterior.

Leia `scripts/style/style-sheet.md` (termos fixados e alvos) e `scripts/style/chapter-brief.md` (a lista fechada
do que não se toca). A lista fechada vale igual: números, código, blocos ` ```text `, cabeçalhos,
marcadores, tabelas, listas, referências cruzadas e os estilemas do autor (parágrafo **Declaração de
divergência**, seção 9.7, frases curtas que fecham argumento de propósito).

## Como medir

```bash
python3 scripts/style/dois_pontos.py book/capNN.md   # from the root of the repository
```

A primeira linha dá três números: **prosa**, tabela e cerca de código. Tabela e cerca não entram na
reescrita; o alvo é o de **prosa**. Sem argumento de arquivo, o comando mostra todos os arquivos com a
coluna `prosa`, que é a que interessa aqui. Com o nome do arquivo, ele lista as linhas de prosa que têm
dois-pontos, com o número da linha.

Conferência final de integridade, obrigatória depois de editar:

```bash
python3 scripts/style/check_pass.py book/capNN.md     # compara contra a edição de antes: números, cercas, blocos
python3 scripts/book/build.py --check
python3 scripts/book/check_labs.py
```

## O que fica e o que sai

**Fica** o dois-pontos que:
- abre lista de itens, quadro ou tabela;
- abre definição formal, equação ou símbolo;
- abre citação literal ou fala entre aspas;
- abre bloco de código, bloco ` ```text ` ou saída de instrumento;
- separa horas, proporções, escalas e pares de valor (`10:00`, `3:1`, `q50: 0,0320`);
- vive dentro de cabeçalho, de célula de tabela ou de equação.

**Ruling do revisor sobre lista.** A prosa **dentro** de um item de lista, de exercício ou de resumo é
prosa: pode ser reescrita, desde que o item continue com o mesmo marcador, a mesma ordem, a mesma
quantidade e a mesma afirmação. O que não se toca é o esqueleto (o número do item, o `-`, a sequência) e
o marcador `**Exercícios.**` / `**Resumo.**`. Célula de tabela continua congelada, porque ali a coluna
manda no comprimento.

**Sai** o dois-pontos que apenas introduz um período explicativo, uma consequência ou um desfecho. Nos
casos cortados, o período seguinte ganha ponto final e o sujeito volta a aparecer, ou o trecho entra por
`porque`, `já que`, `ou seja` só quando a relação pede — sem enfeite de coesão.

Exemplo do corte que se espera: `Nenhuma média mudou: 8 nos dois casos.` vira
`Nenhuma média mudou. São 8 nos dois casos.`

## Alvo

Corte de **60% dos dois-pontos de prosa** do arquivo, medidos pela coluna `prosa`. Onde não der, deixe o
motivo por linha: o que sobrou abre lista, definição, citação, quadro ou cerca. Piso real é o que a
classificação justifica, não o gosto.

## O que entregar

1. A coluna `prosa` antes e depois, e a porcentagem de corte.
2. A **lista das linhas de prosa que sobraram**, uma por linha, cada uma com a classificação:
   `lista`, `definição`, `citação`, `quadro`, `cerca`, `valor` (horas, proporções, pares) ou `estrutura`
   (cabeçalho, exercício, resumo, tabela).
3. De três a seis pares antes/depois dos casos difíceis, no formato `antes:` / `depois:` / `o que mudou`.
4. O que ficou intacto de propósito e por quê.
5. Pendências para o autor, com número de linha e citação.

Não escreva em `scripts/style/`: as pendências vêm na resposta. Não mexa em outro arquivo que não seja o seu.
Não rode `cargo`, não faça `git commit`, não parafraseie conteúdo, não invente número.
