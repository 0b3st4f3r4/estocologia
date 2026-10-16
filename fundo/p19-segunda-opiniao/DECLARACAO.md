# Declaração do ponto 19: a segunda opinião dos resultados marcados P

**Congelada em 2026-10-16T23:31:41Z.**

| arquivo | SHA-256 | o que é |
| --- | --- | --- |
| `fundo/p19-segunda-opiniao/registro.tsv` | `e7034ec098d2b70f3344e3b426043a28ade28f1511027016c6c12a6e7683bb49` | as catorze linhas marcadas P, com a classe e o que a segunda opinião faz |
| `DECLARACAO.md` (esta) | — | as cinco declarações; o hash próprio sai em `DECLARACAO.sha256` |
| `README.md` | `325928f7d3d25dc3949a0a4ebf16fef089097aff937b7a142a4cbbeb1cd94e17` | o resultado em uma página |
| `DECLARACAO.sha256` | — | os três hashes, em forma de `sha256sum` |

## Declaração 1 — as três classes, e o que faz uma linha ser conferida

**Uma linha só é `conferida` quando uma segunda implementação refaz a conta por um caminho diferente do primeiro,
com o número gravado como testemunha.** É `derivada` quando o veredito foi re-derivado de um registro em árvore
(os pontos 14 e 16), e `registro` quando o dado bruto não está mais na árvore e não há segunda opinião possível.
Nada é promovido por vontade: a classe sai do que o instrumento conseguiu fazer.

## Declaração 2 — as cinco conferidas, com o resultado

| linha | o que a segunda opinião refez | resultado |
| --- | --- | --- |
| multiplicidade (K = 38) | os limiares `α/M` gravados no registro do p03, recalculados como 0,05/M na forma exata do arquivo | **confere** nos três valores: 0,05/32 = 0,0015625, 0,05/38 = 0,0013157894736842105, 0,05/72 = 0,0006944444444444445 |
| laço performativo (0,145) | o viés do previsor ingênuo, procurado em **dois arquivos independentes** | **confere dígito por dígito**: 0,29124674633711956 no p03 e no p06 |
| H0 real (2,65 M e Var/média 273) | a base recontada em Rust por um caminho próprio, com a dispersão entre dias por uma passada só | **confere com uma diferença atribuída**: cofre 2 654 950 contra 2 649 354, Var/média 261,7 (todos) e 247,4 (cofre) contra 261,1 e 246,7. A diferença de 0,2% é a **janela de horas** que o laboratório usa (7 a 20 h) e a recontagem não |
| pareceres | as linhas da releitura e os arquivos reemitidos, contados | **30 linhas** na tabela e **4 pareceres** na pasta. A nota: o registro fala de 21 pareceres, e a tabela tem as 21 decisões mais as linhas de fechamento — os dois números medem coisas diferentes |
| prioris (33 de 33) | a reprodução do p05, lida do registro em máquina | **33 domínios, dos quais 8 marcam `true`** — ver a declaração 3 |

## Declaração 3 — a divergência declarada, e ela não se resolve aqui

O registro publicado diz, na linha da auditoria de fundo, **“33 dos 33 idênticos”**, pelo critério da banda de
equivalência de 0,05 dp fixada na `DECLARACAO_2.md` do p05, com desvio máximo de 0,0071 dp. O campo em máquina
`reproducao_rodada_a` tem **33 domínios e marca `true` em 8 deles**.

**O campo não declara o próprio sentido** — se `true` quer dizer “reproduziu” ou “divergiu” —, e é por isso que a
segunda opinião registra os dois valores e **não resolve a divergência**: resolver seria adivinhar o sentido de um
campo, e adivinhação não é segunda opinião. Fica declarado o que se sabe: a frase publicada se apoia na banda de
0,05 dp, e o campo em máquina **não sustenta sozinho** essa frase. A decisão é humana, e enquanto ela não vier, a
linha carrega as duas leituras.

Isto **não derruba o veredito do p05** — o veredito é sobre a banda, e a banda é do documento —, mas mostra uma
coisa que o programa deve registrar: um campo em máquina sem sentido declarado não é prova de nada.

## Declaração 4 — as sete que só restam como registro, e as duas derivadas

Sete linhas ficam em `registro`, todas pelo mesmo motivo: **o dado bruto saiu da árvore com o andaime Python**
(`DECLARACAO_REMOVIDO_NAO_SIGNIFICANTE.md`). São as coortes de 20 anos, o Brier dos incêndios, o túnel de luz, o
ensaio de farmacologia, a educação do STAR, os seis fluxos do H3 e o hipergrafo. Duas ficam em `derivada`: a
réplica da genética (o desenho é medido no ponto 14) e o piloto cego (as notas do juiz são re-derivadas no ponto
16). Para nenhuma delas existe segunda opinião possível sem reaver o dado — e é isso que está escrito.

## Declaração 5 — o limite e o K

**O limite, declarado como o do Φ5.** Catorze linhas marcadas **P**, cinco com segunda implementação, duas com
veredito re-derivado e sete que permanecem registro. O item fecha **com o limite dentro**: não há segunda opinião
para o que não tem dado, e a promessa de uma segunda opinião futura seria a pendência eterna que este programa
recusa.

**O K.** As classes e o tratamento da divergência são escolha de especificação, feita uma vez: **K = 1**.

## Verificação

Rodada em 2026-10-16, depois do congelamento. Código de saída entre parênteses.

| Comando | Resultado |
| --- | --- |
| `cargo test -p stochology-book` | 43 testes aprovados, entre eles os seis do `segunda`: os limiares α/M, os 33 domínios com a divergência declarada, as duas fontes do laço, a recontagem do banco contra o publicado, a contagem dos pareceres e as classes do registro (0) |
| `labs --segunda` | as catorze linhas com a classe, as cinco segundas opiniões medidas e as sete que só restam como registro (0) |
| `python3 book/gerar_resultados.py --check` | `RESULTADOS_RUST.md` atualizado, com o bloco da segunda opinião (0) |
