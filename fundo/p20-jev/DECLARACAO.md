# Declaração do ponto 20: a ponte Jev-LLM

**Congelada em 2026-10-16T23:57:11Z.**

| arquivo | SHA-256 | o que é |
| --- | --- | --- |
| `fundo/p20-jev/registro.tsv` | `27596bdfd62b5bfb18bab099a9c4bfcfc6d1ad06d2550bc064ec53ebe4339199` | o estado da ponte, item por item, em máquina |
| `DECLARACAO.md` (esta) | — | as cinco declarações; o hash próprio sai em `DECLARACAO.sha256` |
| `README.md` | `e4c3fe9fa4266c8d0328fc8d861ccbed474f9d58333894e24f5f2ec23a163481` | o resultado em uma página |
| `DECLARACAO.sha256` | — | os três hashes, em forma de `sha256sum` |

## Declaração 1 — o que o item pede e o que a árvore tem

O item 13 pede seis coisas: o runner local de pesos congelados, o `sha256` dos pesos, a decodificação restrita ao
esquema da ficha, a prova de determinismo na forma do G1, a calibração contra o Brier do Jev determinístico, o custo
por chamada e a pergunta paralela de adequação do espaço de hipóteses. **Os pesos não estão nesta árvore, e não há
runner local aqui.** Então a ponte fica declarada, e este módulo entrega o arcabouço inteiro: o que fecha sem os
pesos, fecha; o que só fecha com eles, fica com o portão armado e o número em branco.

## Declaração 2 — os portões que já fecham, com a prova

| portão | como fecha | a prova negativa |
| --- | --- | --- |
| **pesos** | o arquivo tem de existir e o `sha256` tem de bater com o declarado; sem isso a ponte não roda e nenhum número sai | um caminho ausente devolve `PesosAusentes` com abstensão `Outros`, e um hash diferente devolve `PesosComHashDiferente` com os dois valores |
| **decodificação restrita** | a saída tem de ser o documento canônico da ficha, e a margem não pode encostar em 0% nem em 100% | texto livre devolve `ForaDoEsquema` e a margem em 100% devolve `ExtremoDogmatico`; os dois abstêm em `Categoria::Outros`, que é o "não sei" auditável que já existia em `s1.rs` |
| **determinismo (G1)** | duas execuções, o mesmo `sha256` da sequência de saídas | o modelo de ensaio fecha (hashes idênticos); o **modelo instável não fecha** (dois hashes diferentes) — que é exatamente o que o `INTEGRACAO_JEV.md` declara sobre o LLM livre, agora medido |
| **custo por chamada** | a contabilidade soma chamadas, tokens e tempo, e divide | roda no ensaio: 42 tokens e o tempo por chamada; com os pesos, o mesmo código mede o custo real |
| **adequação do espaço** | a fração de abstenções sobre os pedidos, contra o limite declarado de **30%** | o modelo de ensaio abstém em 0% ("o esquema serve") e o instável em 100% ("o esquema NÃO serve") |

## Declaração 3 — o limite, declarado como limite

**Sem os pesos verificados, nenhum número da ponte é publicado**: nem o Brier calibrado contra o Jev determinístico,
nem o custo por chamada real, nem a fração de abstenções do modelo de verdade. O que o instrumento publica é o
mecanismo, com um modelo de ensaio declarado como tal, e o portão armado para quando os pesos existirem. É a mesma
regra dos pontos 13, 16 e 19: não se promete medição futura, declara-se o estado de hoje.

## Declaração 4 — a pergunta paralela, com o critério declarado antes

"O esquema da ficha é um espaço de hipóteses adequado para este modelo?" A resposta fica falsificável pelo critério
declarado agora, antes de ver qualquer modelo real: **se a fração de pedidos em que a ponte tem de abster passa de
30%, o esquema não serve ao modelo**, e a ponte reporta isso em vez de forçar uma resposta fora do esquema. O limite
de 30% é escolha de especificação declarada uma vez.

## Declaração 5 — o K e o que não muda

**K = 1**: o limite de abstenção e as regras de recusa são escolha de especificação, declarada uma vez. Não muda
nada do que já está publicado: a ponte é um arcabouço novo, e nenhum número do livro, do registro empírico ou dos
horizontes passa por ela.

## Verificação

Rodada em 2026-10-16, depois do congelamento. Código de saída entre parênteses.

| Comando | Resultado |
| --- | --- |
| `cargo test -p stochology-s2` | 36 testes aprovados, entre eles os sete do `ponte`: sem pesos não roda, hash diferente recusa, texto livre abstém, margem em 100% recusa, o G1 fecha no modelo de ensaio e **não** fecha no instável, e o custo e a adequação saem do ensaio (0) |
| `labs --jev` | o portão dos pesos recusando com os pesos ausentes, a decodificação restrita com as duas recusas, a prova de determinismo nos dois modelos, o custo por chamada e a adequação nos dois modelos (0) |
| `python3 book/gerar_resultados.py --check` | `RESULTADOS_RUST.md` atualizado, com o bloco da ponte (0) |
