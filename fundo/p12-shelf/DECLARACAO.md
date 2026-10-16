# Declaração do ponto 12 (SHELF): congelamento do protocolo e ajuste do pré-teste

**Congelada em 2026-10-16T14:55:54Z**, antes de qualquer especialista responder qualquer coisa.

| arquivo | SHA-256 | o que é |
| --- | --- | --- |
| `protocolo.md` | `175004ac45caa0fc8e231ea0bd85466b913ef7da104f3d0c4d394cb7e6e95058` | o protocolo, congelado: daqui em diante não é reescrito |
| `DECLARACAO.md` (esta) | `5b4d32a17ca84e4cd6e6c5233f143aa40509d890945c5410ec06f5ffe694690a` | esta declaração, com o ajuste e o registro do congelamento |
| `DECLARACAO.sha256` | — | os dois hashes acima, em forma de `sha256sum` |

## Declaração de ajuste 1 — o portão do pré-teste

O protocolo fixa, na seção 2, quatro critérios de aprovação e o limiar de ECE de 0,10 em 10 caixas de massa
igual. Antes de congelar, o desenho foi medido com o próprio instrumento:

~~~bash
cargo run --release --bin labs -- --preteste
~~~

| desenho | ECE p50 | p90 | p95 | fração que passa no limiar de 0,10 |
| --- | --- | --- | --- | --- |
| 20 itens, 10 caixas (o do protocolo) | 0,2272 | 0,3002 | 0,3218 | 0,0008 |
| 20 itens, 5 caixas | 0,1584 | 0,2316 | 0,2559 | 0,1072 |
| 20 itens, 20 caixas | 0,3325 | 0,4017 | 0,4222 | 0,0000 |
| 200 itens, 10 caixas | 0,0694 | 0,0932 | 0,1002 | 0,9486 |

O nulo é o especialista **perfeitamente calibrado**: a priori é uniforme e o desfecho é sorteado dela, no mesmo
desenho. A leitura:

1. **O limiar de 0,10 não é portão, é parede.** No desenho do protocolo ele aprova 0,08% dos calibrados — 1 em
   1 250. A causa é o tamanho da amostra: o ECE cai com a raiz de n, e com 200 itens o mesmo limiar aprova 95%.
2. **O critério 3 fica vacuoso** no desenho do protocolo: 20 itens em 10 caixas de massa igual dão 2 itens por
   caixa, e a regra só morde a partir de n ≥ 4.
3. **O critério 4 é inerte como escrito.** Ele compara a média prevista da caixa de cima com a de baixo, e as
   caixas são cortadas por p ordenado: a de cima supera a de baixo sempre, com o especialista calibrado ou
   invertido. O teste `o_criterio_4_do_protocolo_nao_sabe_reprovar` prova isso.

**Portão ajustado, declarado antes de rodar:**

- o pré-teste passa a ser lido em **5 caixas de massa igual, 4 itens cada**, com **ECE ≤ 0,25**, que é o p90 do
  nulo calibrado nesse desenho (0,2316) arredondado para cima;
- o **critério 3 volta a morder** nesse desenho: n = 4 por caixa, exatamente no limite;
- o **critério 4 fica como escrito** — inerte, e declarado assim — e o relatório do pré-teste traz ao lado o
  diagnóstico que morde: a **taxa observada** da caixa de cima acima da de baixo;
- os critérios 1 (habilidade > 0) e o Brier contra a climatologia do próprio pool seguem como o protocolo fixou.

Isto é declaração nova, como a regra de casa exige: conta no K da frente, que passa de 2 (protocolo = 1,
comparação = 2) para **K = 3**.

## Lista de itens, por domínio

O protocolo exige semente e lista congeladas antes de rodar. A lista é **por domínio**, e cada domínio tem a sua
declaração de itens, com hash próprio, antes do pré-teste daquele domínio:

| domínio | arquivo da lista | hash | estado |
| --- | --- | --- | --- |
| `h2/desenvolvimento_preston` | `preteste/desenvolvimento_preston.yaml` | *(entra na declaração de itens do domínio)* | em preparação |

## O que esta declaração não faz

- **Não substitui o painel.** O protocolo pede três especialistas por domínio, com um de fora do programa. Um
  especialista aprovado não fecha painel nenhum (seção 2.4), e o consenso de n = 1 **não** substitui a priori do
  pacote na rodada oficial: fica como sensibilidade.
- **Não preenche formulário de ninguém.** O que não estiver escrito com nome e data não aconteceu.
