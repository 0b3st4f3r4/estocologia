# Declaração de itens do pré-teste — `h2/desenvolvimento_preston`

**Congelada em 2026-10-16**, antes de o especialista responder qualquer coisa. É a "lista com hash" que o
protocolo (seção 2.1) exige, por domínio.

| o que | valor |
| --- | --- |
| pool de origem | `fundo/p12-shelf/preteste/desenvolvimento_preston.yaml`, 22 itens, SHA-256 `c287a013d0865be891c6368da5f4f68f364412396ed11b6d9f77a3463598cb1e` |
| semente do sorteio | `2026` |
| regra do sorteio | embaralhamento de Fisher-Yates sobre os 22 identificadores, na ordem do arquivo, com o `SplitMix64` do próprio repositório (`crates/stochology-core/src/rng.rs`); entram os 20 primeiros |
| descartados | `P03`, `P16` |
| gabarito | fica no arquivo do pool, item a item, com a URL da fonte e o número; o especialista vê **só os enunciados** desta declaração, e o facilitador guarda o gabarito |

Regra do sorteio, para quem quiser reproduzir:

~~~python
MASK = (1 << 64) - 1
def proximo(est):
    est = (est + 0x9E3779B97F4A7C15) & MASK
    z = est
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
    return est, (z ^ (z >> 31)) & MASK
# embaralha com semente 2026 e toma os 20 primeiros
~~~

## Os 20 itens, na ordem em que o especialista os vê

| # | id | enunciado | dificuldade declarada |
| --- | --- | --- | --- |
| 1 | P05 | No ano-base 2019, a expectativa de vida ao nascer da Nigéria era menor que 55 anos? | facil |
| 2 | P01 | No ano-base 2019, a expectativa de vida ao nascer do Japão era maior que 80 anos? | facil |
| 3 | P06 | No ano-base 2019, a taxa de mortalidade de menores de 5 anos da Índia era maior que 40 óbitos por mil nascidos vivos? | facil |
| 4 | P18 | No ano-base 2019, a parcela da população da Nigéria com pelo menos serviços básicos de água potável era maior que 80%? | dificil |
| 5 | P10 | No ano-base 2019, a expectativa de vida ao nascer da Rússia era maior que 75 anos? | media |
| 6 | P07 | No ano-base 2019, a expectativa de vida ao nascer do Brasil era maior que 75 anos? | media |
| 7 | P22 | No ano-base 2019, o gasto total em saúde como porcentagem do PIB da África do Sul era maior que 10%? | dificil |
| 8 | P09 | No ano-base 2020, a queda da expectativa de vida ao nascer dos Estados Unidos em relação a 2019 era maior que 1 ano? | media |
| 9 | P17 | No ano-base 2019, a taxa de mortalidade de menores de 5 anos de Serra Leoa era maior que 100 óbitos por mil nascidos vivos? | dificil |
| 10 | P11 | No ano-base 2019, a expectativa de vida ao nascer de Bangladesh era maior que 70 anos? | media |
| 11 | P21 | No ano-base 2021, a expectativa de vida ao nascer do mundo era menor que 72 anos? | dificil |
| 12 | P02 | No ano-base 2019, o PIB per capita em paridade de poder de compra da China era maior que 20.000 dólares internacionais? | facil |
| 13 | P14 | No ano-base 2019, o PIB per capita a preços correntes do Brasil era maior que 10.000 dólares? | media |
| 14 | P13 | No ano-base 2019, o PIB per capita em paridade de poder de compra da Índia era menor que 9.000 dólares internacionais? | media |
| 15 | P08 | No ano-base 2019, a expectativa de vida ao nascer da Índia era maior que 72 anos? | media |
| 16 | P20 | No ano-base 2019, a diferença entre a expectativa de vida das mulheres e a dos homens na Rússia era maior que 11 anos? | dificil |
| 17 | P19 | No ano-base 2021, a expectativa de vida ao nascer da Índia era menor que 68 anos? | dificil |
| 18 | P04 | No ano-base 2019, a parcela da população com 65 anos ou mais no Japão era maior que 30%? | facil |
| 19 | P12 | No ano-base 2019, o gasto total em saúde como porcentagem do PIB do Brasil era maior que 10%? | media |
| 20 | P15 | No ano-base 2019, o PIB per capita em paridade de poder de compra do Catar era maior que 100.000 dólares internacionais? | dificil |

## O que o especialista recebe e o que ele devolve

- Recebe: os 20 enunciados acima, o pedido de `p em (0, 1)` para cada um (0 e 1 exatos são recusados, porque sem
  previsão não há Brier) e a instrução de responder de cabeça, sem consulta.
- Devolve: 20 probabilidades. Nada mais.
- O que o facilitador faz com isso: Brier item a item, Brier da climatologia do próprio pool, habilidade, ECE nas
  5 caixas de massa igual (o portão ajustado na `DECLARACAO.md`), a curva observada e a decomposição de Murphy,
  com o detalhe item a item ao lado dos agregados.

## Limites declarados desta lista

- **O pool não é do cofre do domínio, mas é do mesmo assunto.** Os itens são fatos de desenvolvimento, saúde
  populacional e renda; nenhum deles sai do cofre do `h2/desenvolvimento_preston`, e nenhum mede a quantidade
  elicitada (elasticidade da expectativa de vida em relação ao log do PIB per capita).
- **Um item usa 2021 e outro compara 2020 com 2019.** São dados publicados e estáveis, e a resolução não muda
  com revisão subsequente da série.
- **O especialista desta mesa é o autor do programa.** O protocolo (seção 3) exige, no painel, um especialista
  **de fora do programa** e **de fora da área**; o autor não é nem uma coisa nem outra, e isso fica declarado:
  o pré-teste dele mede calibração de quem escreveu o livro, não de quem não o escreveu. O painel segue
  incompleto, e nenhum domínio fecha consenso com um especialista só (seção 2.4).
