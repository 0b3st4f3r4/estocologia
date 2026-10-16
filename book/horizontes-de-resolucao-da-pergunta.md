# Horizontes de resolução da pergunta

Proposta de eixo novo para a teoria, escrita em 2026-10-16. O eixo empírico atual (H0 a H3) diz **onde** a
pergunta é feita: semente, ciclo completo, transferência entre domínios, malha ao vivo. Este documento
propõe um segundo eixo, ortogonal ao primeiro: **quão fina** é a pergunta do decisor, e quanto custa
perguntar grosso.

## Por que este eixo é da teoria, e não do método

Os invariantes do programa pendem todos da pergunta. $\Phi_2$ exige a margem na unidade independente; $\Phi_4$
exige escopo e quem paga; o erro de tipo III é responder a outra pergunta. Até aqui, porém, a pergunta entrava
na conta como binária, declarada ou não. A tese que se propõe é mais forte: **a resolução da pergunta é uma
variável do modelo, com efeito medível no número e na ação.**

## A escada de resolução

| Nível | A pergunta | O que ela decide | Onde já aparece no livro |
| --- | --- | --- | --- |
| **R0 agregada** | "Em média, o serviço dá conta?" | dimensionar pelo dia médio | Erlang C sobre a taxa média (Capítulo 16) |
| **R1 dia** | "Cada dia, o serviço dá conta?" | dimensionar pelo dia que existe | o posto que oscila entre 5 e 11 (Capítulos 5 e 16) |
| **R2 hora** | "Cada hora, o serviço dá conta?" | dimensionar pelo pico | o reforço nos picos do banco (Capítulo 16) |
| **R3 subpergunta** | "Cada hora, e para quem?" | dimensionar por subgrupo, com custo declarado | o trilema de Chouldechova (Capítulo 11) |

Comparar resoluções só faz sentido quando uma **refina** a outra. R1 responde à pergunta de R0 num grão menor,
e assim por diante.

## O instrumento

```bash
cargo run --release --bin labs -- --resolucao
```

Os braços vêm pareados. As chegadas e as bases de serviço não mudam de um braço para o outro; muda só a taxa
de atendimento, e o **mesmo recurso total** vale nos dois braços (o agregado usa a capacidade média todos os
dias; o fino usa a capacidade daquele dia). Saída medida:

```text
demanda 5 ou 11 (o posto do Capitulo 16): capacidade media 8.0 | espera agregada 86.48 min, espera fina 40.24 min, perda 46.24 min
demanda 7 ou 9 (oscilacao leve): capacidade media 8.0 | espera agregada 47.70 min, espera fina 41.73 min, perda 5.97 min
demanda 8 ou 8 (sem oscilacao): capacidade media 8.0 | espera agregada 41.56 min, espera fina 41.56 min, perda 0.00 min
```

A perda de resolução é **46,24 min** onde a demanda oscila de 5 a 11, **5,97 min** onde oscila de 7 a 9 e
**zero** onde não oscila. Os dois braços operam perto da saturação, e é por isso que as esperas absolutas são
grandes; aqui se mede a **diferença**, que não depende da calibração da taxa.

## O que o registro já mostra, sem dado novo

| Caso | R0 | R1 ou mais fina | Fonte |
| --- | --- | --- | --- |
| Central de chamadas | Erlang C promete 82% | 57% medido; 26 dias falhos em 100; com +7,2% de agente-horas nos picos, menos de 4 em 100 | `h0-real/README.md` |
| Posto de saúde | dia constante: 9,1 min | dia que oscila: 19,3 min, com a mesma média | `labs --cap 16` |
| STAR (educação) | agrupar por aluno mantém a área | agrupar pelas 80 escolas derruba para $\hat S = \emptyset$ | `governanca/GRUPOS.md`, Capítulo 19 |
| hERG (farmacologia) | 16 instantes por voluntário | agrupar por voluntário mantém a área, com habilidade +0,57 | `h2g/README.md` |
| Multiplicidade | 4 áreas antes da correção | 3 áreas depois do agrupamento | Capítulos 15 e 19 |
| Incêndios | janela de 36 meses | ano civil de 1929 a 1932: o número muda de janela | Capítulo 18 |
| Cofre do H3 | laboratório: ECE 0,0022 | cofre real: ECE 0,0048 | Capítulo 20 |

Em todos os casos, mudar a resolução **não mudou a aritmética**; mudou a pergunta, e com ela o número e a
ação. A lição da unidade independente reaparece aqui, agora no eixo do grão da pergunta.

## Previsões falsificáveis

1. **Monotonicidade.** Com recurso total igual, refinar não aumenta a espera média ($R_{k+1} \le R_k$).
2. **Convexidade da perda.** A perda de resolução cresce mais rápido que a oscilação da demanda (0,00 → 5,97 →
   46,24 nos três casos medidos).
3. **Limiar de oscilação.** Existe um ponto a partir do qual uma decisão tomada em R0 falha em mais de um
   quarto dos dias, e abaixo dele refinar deixa de compensar.
4. **Custo de refinar.** O ganho de R1 sobre R0 é limitado pelo dado disponível, e refinar além do que o dado
   sustenta vira a inflação de $K$ que o contador de tentativas existe para conter.
5. **Transferência.** Um resultado que sobrevive a R1 sobrevive a R0; o contrário não é verdadeiro.

## O que a arquitetura precisava ganhar

- **Campo novo na ficha.** A resolução da pergunta (R0 a R3) fica ao lado de `pergunta_do_decisor` e da unidade
  independente. No código o nome é `resolucao`; o plano o chamava de `resolucao_da_pergunta`.
- **Regra de acoplamento (candidata a $\Phi_5$).** A ação declarada tem de estar no grão da unidade
  independente; ação por dia com margem por paciente é desalinhamento, ainda que o número acerte.
- **Teste empírico barato.** As seis frentes da tabela acima já permitem medir a perda de resolução
  retroativamente, sem dado novo. Só a previsão 3 (limiar de oscilação) exige dado novo, e ela pede uma frente
  com a mesma decisão tomada em dois grãos.
- **Estado do código.** O tipo `Resolucao` e o instrumento `medir_perda_de_resolucao` estão em
  `crates/stochology-core/src/resolucao.rs`, com 3 testes. O campo na ficha e a regra de acoplamento saíram da
  proposta e passaram a invariante cobrado. O campo entrou como `Ficha.resolucao`
  (`crates/stochology-core/src/camada.rs:52`, com a troca de resolução em `camada.rs:98`) e a regra virou
  $\Phi_5$ em `crates/stochology-core/src/invariantes.rs:110`, contado pelo binário de portões
  (`crates/stochology-gates/src/main.rs:316`). Limite declarado: a trava só morde quando a unidade independente
  é o dia ou mais grossa, e o teste `unidade_mais_fina_que_o_dia_fica_fora_do_phi5_declarado` guarda essa
  fronteira.
