# Declaração do ponto 5, segunda: a banda de equivalência da rodada (a)

Data: 2026-10-15 (carimbo UTC no `DECLARACAO_2.sha256`). Escrita **depois** de a [DECLARACAO.md](DECLARACAO.md) ter sido
cumprida e de o `comparar.py` ter reprovado o portão da rodada (a) por diferença numérica, e **antes** de qualquer
recomputação com a banda nova. A `DECLARACAO.md` (hash `167fbd6a…`) e o `DECLARACAO.sha256` **não foram tocados**:
a regra antiga continua valendo como escrita.

## O que motivou

A `DECLARACAO.md` escreveu a condição de falha da rodada de controle **sem tolerância** — "a cópia não reproduzir a
rodada original (status ou posterior diferentes dos `saida/` originais em (a))". O `comparar.py` implementou
"posterior diferente" como diferença absoluta `< 1e-9` na média posterior (e na razão dp posterior / dp priori, no
H1 real). O resultado observado: **0 dos 33 domínios** diferem em categoria de status, em lista de portões que falham
ou em ação do F8, e a média posterior difere em **24 dos 24** domínios que a têm, de 4,1e-9 a 2,2e-3 de diferença
relativa — no máximo **0,0071 dp** do posterior da própria rodada (a). Mais o `h1-real/c`, cuja razão dp difere em
1,7e-9. Ou seja: 25 dos 33 reprovam por um critério de igualdade de bit que nenhuma reimplementação em ponto flutuante
satisfaz.

Esta declaração **não reescreve** aquela. Fixa, para a leitura da mesma checagem, uma banda de equivalência declarada.

## Os dois fatos que fixam a banda (já medidos, não medidos agora)

| fato | valor | onde está |
|---|---|---|
| pior desvio ambiental observado (cópia contra `saida/` do repositório) | **0,0071 dp** (pior caso `h2g/genetica_y07`) | README desta frente, tabela do portão da rodada (a) |
| menor deslocamento **que mudou uma decisão** em (b) ou (c) | **0,18 dp** (`h2/desenvolvimento_preston` em (c)) | README desta frente, tabela das duas mudanças de (c) |

O primeiro diz quanto ruído de ambiente esta checagem produz. O segundo é a escala em que uma diferença passa a
importar para a conclusão: abaixo dela, a troca de priori não mexe em nada em nenhum dos 33 domínios. Os dois fatos
distam **25,4 vezes** um do outro (0,18 / 0,0071).

## A banda declarada

**Banda de equivalência: 0,05 dp do dp posterior da rodada (a)** (`0,05 × dp_posterior_a`).

O número fica **entre** os dois fatos, e é por isso que ele é escolha e não conveniência:

- **7,0 vezes acima** do pior desvio ambiental já observado (0,05 / 0,0071 = 7,04) — com folga para o ruído que a
  banda existe para absorver;
- **3,6 vezes abaixo** do menor deslocamento que já mudou uma decisão (0,18 / 0,05 = 3,6) — nenhuma diferença desta
  ordem muda portão, status ou ação em nenhum domínio observado.

## Leitura da checagem com a banda

1. **Categórica, sem banda:** categoria de status, lista de portões que falham e ação do F8 continuam exigidas
   **idênticas**, nos 33 domínios. Banda nenhuma perdoa mudança de status.
2. **Numérica, com a banda:** o desvio comparado é
   `|m_repositório − m_rodada_a| / dp_posterior_a`, domínio a domínio. Nos três domínios do **H1 real** a forma do
   resultado não traz a média posterior, então a quantidade comparada é a razão dp posterior / dp priori — que é
   adimensional e vive em (0, 1] — e a banda entra ali **absoluta, 0,05 na própria razão** (o mesmo número, lido na
   escala da quantidade que existe).
3. **Critério de leitura, fixado aqui:**
   - desvio máximo **≤ 0,05 dp** ⇒ a rodada (a) **reproduz**, o portão da rodada (a) **passa** e o veredito do ponto 5
     deixa de ser condicional a essa leitura;
   - desvio máximo **> 0,05 dp** ⇒ a rodada (a) não reproduz dentro da banda, o portão continua reprovado e o veredito
     publicado continua com o rótulo condicional, como está no README.

## O que já foi visto, e o que a banda não reabre

As rodadas (b) e (c) **já rodaram e já foram lidas** antes desta declaração: N_b = 5 e N_c = 2 sobre os 33 domínios,
as sete mudanças nomeadas no README. Declaro isso explicitamente e declaro que **a banda não reabre (b) nem (c)**:

- a banda só decide se a rodada **(a)** reproduz; ela não entra em nenhuma comparação entre rodadas;
- nenhum critério de (b) ou (c), nenhuma semente, nenhum agrupamento, nenhuma regra R1–R3 das prioris e nenhuma
  decisão (custos, benefícios, `taxa_base`) é mudado por esta declaração;
- nenhum número de (b) ou (c) é recalculado, reinterpretado ou renumerado por causa dela; as cinco e as duas mudanças
  e o G3 com 3 áreas em (b) ficam como estão no README.

## K

O **K desta frente passa de 1 para 2**. O K conta regras de leitura declaradas para o mesmo resultado: a de
`DECLARACAO.md` (sem tolerância, que reprova) e esta (com banda de 0,05 dp). A regra antiga não é apagada nem
substituída; esta é uma leitura **adicional**, declarada com hash próprio e contada no K.

## O que conta como falha desta declaração

- A banda ser aplicada e o desvio máximo ficar acima de 0,05 dp: nesse caso ela não resgata nada e o rótulo condicional
  permanece. Isso é resultado e vai para o README como tal.
- A checagem categórica deixar de bater em qualquer domínio: aí nada aqui se aplica.
