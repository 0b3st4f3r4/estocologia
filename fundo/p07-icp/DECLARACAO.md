# Declaração do ponto 7 (ICP): versão robusta ao agrupamento e réplica da genética

Escrita em 2026-10-15, antes de rodar qualquer ICP nesta pasta. Até aqui só vi o código do H2g e os resultados já
publicados em `h2g/saida/` (Ŝ, número de aceitos e melhor p do STAR, da farmacologia e do y08), e conferi estrutura
dos dados sem olhar Y: 80 escolas no STAR, 446 de 11 598 alunos passam por mais de uma escola, e o ECGRDVQ tem
109 blocos voluntário × braço com 16 instantes cada. O SHA-256 deste arquivo fica em `DECLARACAO.sha256`.

## O ponto da auditoria

"O teste supõe linhas independentes (16 medidas por voluntário no ensaio do FDA, H2g); a Educação (STAR) é frágil
(maior p 0,075); a Genética (H2f) foi aprovada com ressalva."

## O que não muda

- O código roda de uma cópia de `h2g/` em `fundo/p07-icp/codigo/`. Os originais não são tocados.
- Os dados, os pacotes, os ambientes de treino (o cofre fica fora), os candidatos, a codificação (`Desenho`), o α de
  cada domínio e a regra de saturação K = 3 ficam como no H2g.
- O ajuste agrupado do teste também fica igual: a mesma expansão (linear + spline restrita), o mesmo resíduo
  r, a mesma escala ŝ e o mesmo u = r / ŝ de `p_invariancia_np`. Só muda o modo de testar se u tem a mesma lei entre
  ambientes.
- Primeiro reproduzo, com o teste original, o Ŝ publicado de cada domínio. Se não reproduzir, paro e relato.

## Teste robusto principal (decide): Wald com erro agrupado, nas mesmas duas partes do teste original

Para cada S, com u calculado como acima:

1. **Locação.** z = posto de u entre todas as linhas (o Kruskal–Wallis é a ANOVA dos postos). Regrido z nas
   indicadoras de ambiente (intercepto + m − 1 indicadoras) por MQO e testo que os m − 1 coeficientes são zero com
   um Wald de covariância agrupada CR1 (fator G/(G − 1) · (n − 1)/(n − k)). Estatística F = W/q, referência
   F(q, G − 1), com q = posto da covariância restrita.
2. **Escala.** d = |u − mediana de u no ambiente| (a transformação de Brown–Forsythe). O mesmo Wald agrupado.
3. p = min(1, 2 · min(p_loc, p_esc)), Bonferroni das duas partes, como no original.

Grupos (o agrupamento):

- **Farmacologia (hERG, `farmacologia_qt`):** o voluntário (`unidade`, 22). O grupo atravessa os braços; o Wald
  agrupado leva em conta a correlação dentro do voluntário, entre instantes e entre braços.
- **Educação (STAR, `educacao_star_turmas`):** a escola (`unidade` = schoolid da série). Os 446 alunos que mudam de
  escola ligam dois grupos; isso fica como limite declarado.
- **Genética (Kemmeren):** a amostra é uma cultura própria e não há rótulo de lote além de `plate` (2 níveis, que
  não servem como grupos). O teste agrupado não se aplica; a correção da genética é a réplica abaixo.

## Teste robusto secundário (informa, não decide): permutação em blocos

Mesmas estatísticas do teste original (H de Kruskal–Wallis em u e F de Brown–Forsythe em u), com o nulo dado por
permutar os rótulos de ambiente de blocos inteiros dentro do grupo:

- farmacologia: bloco = voluntário × braço (os 16 instantes juntos), permutado entre os braços de treino do mesmo
  voluntário (o ensaio é cruzado, cada voluntário passa por todos);
- STAR: bloco = escola × ambiente, permutado entre os ambientes de treino presentes na mesma escola.

B = 999 permutações, `np.random.default_rng(7)`, o mesmo conjunto de permutações para todos os S. p de cada parte =
(1 + #{T* ≥ T}) / (B + 1); depois o mesmo Bonferroni. O STAR tem uma ressalva: blocos de uma escola têm tamanhos
diferentes e dividem alunos entre séries, então a troca é só aproximadamente válida.

## Critérios de sobrevivência

- **Farmacologia sobrevive** se o Ŝ do teste principal for não vazio e não contiver `pr` nem `tamp`. É "sólida" se o
  Ŝ do secundário também for não vazio.
- **Educação sobrevive** se o Ŝ do teste principal for não vazio. "Sólida" se o secundário também der Ŝ ≠ ∅.
- **Genética: réplica com mais genes Y.**
  - Sorteio novo de 40 genes Y entre os nunca deletados, tirando os 10 do H2f:
    `np.random.default_rng(7007).choice(sorted(nunca_deletados − 10_do_H2f), 40, replace=False)`.
  - Para cada Y: mesma pré-seleção (9 genes deletados de maior |correlação| nas selvagens, x1 orientado), `plate`,
    os mesmos ambientes (obs, del_EJ, del_KP; cofre del_AD fora) e o teste original do H2g (resíduos + saturação).
  - α por Y = 0,05 / 40 = 0,00125 (Bonferroni na família nova).
  - O ICP roda em todos os 40, sem os filtros F1–F3 do funil (o Ŝ não depende deles).
  - **Validação por intervenção** de cada gene X de cada Ŝ (fora `plate`), no mutante ΔX das amostras de treino e
    do cofre: (i) a deleção funcionou se X na linha do mutante ΔX é o menor valor de X entre todas as linhas;
    (ii) **validação estrita** (Meinshausen et al. 2016): Y no mutante ΔX fica fora da faixa de Y em todas as outras
    linhas; (iii) **coerência fraca**: o quantil de Y no mutante está nos 5% da cauda prevista pelo sinal do
    coeficiente de X no ajuste agrupado de Ŝ (coeficiente > 0 prevê Y baixo, quantil ≤ 0,05; < 0 prevê ≥ 0,95).
  - **Genética sobrevive** se pelo menos um dos 40 Y novos der um Ŝ com algum gene e pelo menos um gene desses Ŝ
    passar na validação estrita. Se houver Ŝ com gene mas só coerência fraca, a genética fica "frágil" e **não**
    conta. Se nenhum Y novo der Ŝ com gene, a genética não se replica e **não** conta (o y08 sozinho continua sendo
    o que já era: aprovado com ressalva).
- Também aplico a validação aos genes do Ŝ do y08 original, só para conferir a tabela do H2f.

## Veredito do portão do H2 (Ŝ ≠ ∅ em 3 áreas distintas)

- Física (túnel de luz) não faz parte deste ponto e entra como está no H2g.
- O portão **sobrevive** à correção se Física + as áreas que sobreviverem acima somarem pelo menos 3.
- Relato também a contagem só com as áreas "sólidas".

## O que conta como falha

Não reproduzir o Ŝ original; qualquer mudança de teste, grupo, sorteio, α ou critério depois de ver um resultado.
Mudança assim entra como declaração nova e conta no K.
