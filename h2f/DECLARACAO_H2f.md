# Declaração H2f: terceira área — deleções de genes em levedura (Kemmeren et al. 2014)

Escrita em 2026-10-15, **antes** de qualquer execução do código do H2f. Destes dados só se viu o esquema: número de
linhas, colunas, tipos, a lista de genes deletados, a coluna `plate` cruzada com o tipo de amostra, e as contagens por
cromossomo do gene deletado. Nenhuma expressão foi resumida, nenhum Y foi escolhido e nenhum modelo foi ajustado. O
SHA-256 deste arquivo e a hora UTC ficam em `DECLARACAO_H2f.sha256`.

O trabalho fica em `h2f/`, uma cópia de `h2e/`. As pastas anteriores não são tocadas.

## O pedido

sil disse: "Procure uma terceira área com intervenções limpas". O G3 continua o do PLAN, sem mudança: Ŝ ≠ ∅ em pelo
menos 3 domínios de áreas distintas. Hoje há 2 áreas, Educação e Física.

## Os dados

- **Origem.** Kemmeren et al. (2014, Cell) — citado de memória, conferir. São 1 741 perfis de expressão de levedura
  (log2 da razão contra um pool selvagem) em 6 170 genes:
  - 262 amostras selvagens, que são o regime observacional;
  - 1 479 mutantes, cada um com a deleção de um gene conhecido.
- **Por que estes dados.** São os dados que Meinshausen, Hauser, Mooij, Peters, Versteeg & Bühlmann (2016, PNAS)
  usaram para validar o ICP — citado de memória, conferir. O arquivo é o mesmo da página de download feita para esse
  artigo (cópia no GitHub; ver `../dados-reais/h2f/FONTES.md`).
- **Área: "Genética (expressão gênica em levedura)".** É distinta de Educação e Física, e também da Biologia de
  sinalização do Sachs, que não contou para o G3.

## Duas violações conhecidas das premissas

São declaradas antes de rodar.

- **(i) Efeitos globais não medidos.**
  - Uma deleção fora dos candidatos pode mudar Y diretamente, por exemplo pela taxa de crescimento ou pelo ciclo
    celular, e isso não está medido.
  - Agrupar os mutantes em ambientes mistura essas ações diretas.
  - O efeito esperado é rejeitar mais conjuntos, nunca aceitar um falso.
- **(ii) A coluna `plate` não tem documentação.** Ela parece marcar o lote ou o formato de cultivo, e seu efeito é
  técnico e direto sobre Y. Por isso entra como candidata medida.

## Desenho (igual para todos os Y)

- **Y.** São 10 genes sorteados uniformemente entre os 4 691 genes que nunca foram deletados.
  - Sorteio: `numpy.random.default_rng(2026).choice(sorted(nunca_deletados), 10, replace=False)`.
  - O sorteio só usa os nomes dos genes.
  - Cada Y é um domínio, `genetica_y01` … `genetica_y10`.
- **Ambientes.** O tipo de amostra e o cromossomo do gene deletado (a 2ª letra do ORF), regra que não olha Y:
  - `obs`: as 262 amostras selvagens;
  - `del_EJ`: deleções nos cromossomos E–J, 415;
  - `del_KP`: deleções nos cromossomos K–P, 712;
  - **cofre `del_AD`**: deleções nos cromossomos A–D, 352, nunca vistas.
  - Sai a linha em que o próprio Y foi deletado, mas nenhum Y sorteado foi deletado, então nada sai.
- **Candidatos (10), pré-seleção como no PNAS.**
  - **Nove genes:** entre os 1 479 genes que têm uma deleção (intervenção conhecida), os 9 de maior |correlação de
    Pearson| com Y nas 262 amostras observacionais, que fazem parte do treino.
  - A pré-seleção nunca toca os mutantes nem o cofre.
  - Os nove ficam como `x1` … `x9`, em ordem de |correlação|. O `x1` é multiplicado pelo sinal da sua correlação,
    para que a priori da chave tenha sinal conhecido.
  - **O décimo é `plate`** (0/1).
- **Chave:** `x1`. **Ajuste:** x2 … x9 e `plate`.
- **DAG declarado:** `plate` → cada x e `plate` → y; cada x → y. É genérico, porque a verdade não é conhecida.
- **Unidade:** a amostra.
- **α do ICP.** α = **0,005** por Y, porque 0,05 é dividido pelos 10 Y (Bonferroni). Isso controla a chance de algum
  Ŝ trazer um não pai na família de 10. É a única mudança de código: o funil passa a ler `alfa_icp` do pacote, e o
  padrão continua 0,05, de modo que os 7 domínios anteriores não mudam.
- **O teste.** O teste de invariância e a regra de saturação são os do H2d, sem mudança.
- **Priori SHELF da chave.** É o coeficiente de log-razão de Y em x1, com x1 orientado positivamente.
  - Quartis: q05 −0,1, q25 0,1, q50 0,3, q75 0,5, q95 0,9.
  - Vem de um argumento genérico: as correlações de expressão de levedura são moderadas, e o coeficiente parcial é
    menor que a correlação. Nenhum especialista humano foi consultado.
- **Decisão.** É inventada, como nas câmaras, com custo 0,3, perto da mediana; o F1 não informa nada sobre o mundo.

## Regra do G3 para esta área

- A área Genética conta se **algum** dos 10 domínios tiver um Ŝ ≠ ∅ que contenha pelo menos um gene. Com o α de
  Bonferroni, isso é legítimo na família.
- Um Ŝ = {plate}, sozinho, não conta, porque é um efeito técnico de lote. O G3 de `portoes.py` é calculado como está, e
  esta leitura é aplicada e relatada ao lado.
- **Validação informativa,** que não decide nada: para cada gene X que aparecer num Ŝ, confiro o critério de "efeito
  forte de intervenção" de Meinshausen et al. Na linha do mutante ΔX, Y precisa ficar fora da faixa de todas as outras
  linhas, desde que a deleção de X tenha funcionado (X no seu valor mínimo na própria linha).

## Depois desta declaração

- Testes: os do H2e, mais a carga dos 10 pacotes.
- Uma execução nos 17 domínios: os 7 de antes e os 10 novos, todos K = 1 para os novos. Nada é ajustado depois.
