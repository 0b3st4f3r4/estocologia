# Declaração 2 do ponto 13 (escuta): a escolha da condução — protocolo cego

**Congelada em 2026-10-16T21:35:00Z**, antes de qualquer tentativa de contato com qualquer grupo. A
[`DECLARACAO.md`](DECLARACAO.md) fica como está: esta segunda declaração vive ao lado dela, que é o que o
protocolo da frente manda para a escolha da seção 3.

| arquivo | antes | agora | por quê |
| --- | --- | --- | --- |
| `registro.tsv` | `646f880ad35f53d6289bf4f18e855966fec9e844279e17d8a2ab432ba8417f47` | `353a7638b1d7cfd77f61ecd97409881c7399b7477a50b5c6186693ccc1d09dde` | entrou a linha `# conducao` com a escolha desta declaração |
| `protocolo.md` | `fee645912c789b4ab8a0d4ba5585e0f36d4f41eb23c4fd82d56a0191d2fd4b02` | o mesmo | o protocolo não é reescrito |
| `DECLARACAO.md` | `3631bb81ef17b2e3e04b521bd6e2a08e510a4b974881f08c35e40f17d047a51b` | o mesmo | fica como registro do congelamento |
| `DECLARACAO_2.md` (esta) | — | — | a escolha; o hash próprio sai em `DECLARACAO.sha256`, calculado depois da gravação |

## Declaração 6 — a condução é o protocolo cego (forma B)

O protocolo deixou duas formas abertas e mandou declarar a escolha antes da primeira conversa. **A escolha é a
forma B: protocolo cego.** O motivo declarado: o operador é quem construiu os ajustes e as prioris, e um facilitador
neutro ou institucional **não está contratado no escopo atual**. Como não há terceiro para conduzir, a neutralidade
passa a depender do desenho, e não da pessoa.

O que "cego" quer dizer, em cinco regras que valem para cada conversa:

1. **Só o roteiro da seção 4 do protocolo entra na conversa.** As três perguntas, na ordem, sem acrescentar nada.
2. **Nenhum número do programa entra.** Nem quantil, nem mediana, nem distribuição, nem Brier, nem EVPE/EVPI, nem
   as saídas dos funis F1 a F8, nem o nome do domínio. O entrevistado fala primeiro, e o programa escuta depois.
3. **O que for pedido como número é respondido com "isso não faz parte desta conversa"**, e a pergunta fica
   registrada como pergunta do entrevistado.
4. **A resposta é registrada literal, em frequências naturais** ("de cada 100 pessoas, quantas"), com data, local e
   consentimento registrado. Nada é corrigido, resumido ou arredondado por quem conduz.
5. **A tradução, se houver, é feita à parte e marcada como leitura do programa**, nunca dentro do registro do grupo.

O operador conduz, e a condução está **declarada**: ele é parte interessada e não pode virar voz do grupo
(declaração 4). O risco que sobra é o de ancoragem na pergunta, e é ele que o roteiro fixo e a ausência de números
reduzem; o risco que sobra depois disso fica registrado aqui, em vez de escondido.

## Declaração 7 — a retificação do registro, declarada com os dois hashes

O `registro.tsv` ganhou uma linha de comentário, `# conducao protocolo cego`, para que o instrumento imprima a
condução junto com o lacre. Os dois hashes ficam lado a lado na tabela acima: o primeiro é o do congelamento de
2026-10-16T21:19:42Z, gravado na `DECLARACAO.md`; o segundo é o deste ato. Não é mudança de critério — é a escolha
que o protocolo deixou aberta e mandou declarar antes do primeiro contato —, e por isso **não entra no K do
programa**.

**Uma nota de leitura.** O bloco do instrumento transcrito na `DECLARACAO.md` é o da medição de
2026-10-16T21:19:42Z: a linha de abertura dele não traz o campo `conducao`, que só passou a existir com esta
retificação. Os dois ficam como estão, e o que vale daqui em diante é a linha com `conducao: protocolo cego` — a
mesma que o Capítulo 21 do livro transcreve. Nada mais muda: o objeto (as sete decisões), a cobertura mínima de
2 de 7, o prazo de 2026-12-15, a regra de não substituição e a exclusão da voz do operador seguem como estavam.

## Verificação

Rodada em 2026-10-16, depois da escolha. Código de saída entre parênteses.

| Comando | Resultado |
| --- | --- |
| `labs --escuta` | a primeira linha agora traz `conducao: protocolo cego` ao lado do prazo e da cobertura mínima; seguem 7 decisões, 0 de 7 ouvidos e a caixa lacrada (0) |
| `cargo test -p stochology-book` | o teste `a_conducao_declarada_e_protocolo_cego` prende a linha nova no registro (0) |
| `sha256sum -c DECLARACAO.sha256` | os quatro arquivos conferem com os hashes atualizados (0) |
