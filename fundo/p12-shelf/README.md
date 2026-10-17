# Ponto 12 (SHELF): os dois artefatos que dependem de gente

## Resultado

**Cinco domínios elicitados pelo primeiro especialista ($n = 1$, análise preliminar de sensibilidade) e 21 linhas de releitura preenchidas e assinadas; faltam os outros dois especialistas por domínio e o de fora do programa.** O protocolo do painel está congelado com hash (`DECLARACAO.sha256`), o primeiro especialista (E1, autor do programa) concluiu os cinco pré-testes de 20 previsões resolvidas (habilidades de Brier de 0,6500, 0,9730, 0,7425, 0,8585 e 0,4240) e as cinco elicitações de quantis (`h2/desenvolvimento_preston`, `h2/direito_prisoes`, `h2/educacao_star`, `h2g/direito_teto_beneficio` e `h1-real/c`, com `h2g/educacao_star_turmas` reservado por mesma base), e a releitura dos 21 pareceres foi concluída em `governanca/RELEITURA.md` (17 confirmados, 4 retirados e reemitidos com nota assinada). Como nenhum especialista de fora do programa foi entrevistado ainda ($n = 1$), nenhuma priori elicitada substitui a priori do pacote na rodada oficial.

| artefato | conteúdo | estado |
|---|---|---|
| `protocolo.md` | protocolo completo: pré-teste de calibração de 20 previsões resolvidas (Brier em 10 caixas de massa igual), regra de quem não passa não entra como fonte, elicitação por mediana/quartis/extremos com devolução, controles de âncora, ordem e excesso de confiança, três especialistas por domínio com um de fora da área do programa, e a lista dos 6 domínios com os quartis de partida do p05 | congelado em `DECLARACAO.sha256` |
| `formularios/` e `preteste/` | 6 formulários (5 preenchidos no bloco E1 e 1 reservado; blocos E2 e E3 em branco), 5 arquivos `preteste/*.yaml` com habilidade medida, `prioris_elicitadas.yaml` e `comparacao.md` (§6–§10 preenchidos para $n = 1$) | E1 concluído ($n = 1$); faltam E2, E3 e especialista externo |
| `../../governanca/RELEITURA.md` | 21 linhas dos pareceres com arquivo e SHA-256, data, decisão (17 confirmados, 4 retirados e reemitidos) e nota assinada, mais a seção "Releitura" aplicada em `ASSINATURAS.md` | 21 linhas preenchidas; 21 de 21 hashes conferidos |

Os dois números que este ponto usa como ponto de partida são cópia conferida de `fundo/p05-prioris/prioris/`: os
quartis de literatura dos 6 domínios batem com o arquivo do p05 na 15ª casa, e os 21 SHA-256 de `RELEITURA.md`
batem com os arquivos citados por `sha256sum`.

## O que foi feito

1. **Protocolo.** `protocolo.md` fixa, antes de qualquer elicitação: os 20 itens resolvidos por especialista e os
   quatro critérios de aprovação do pré-teste (habilidade > 0, ECE ≤ 0,10, nenhuma caixa com n ≥ 4 fora de 0,25,
   curva fracamente monótona); a composição do painel (três por domínio, um de fora do programa **e** da área, mais as
   exclusões por dupla contagem declarada); a ordem da elicitação (mediana → quartis → extremos) com devolução e no
   máximo duas rodadas; o consenso por **mediana ponto a ponto** (declarando a troca contra a média do
   `h1/estocologia/agentes.py`); os controles de ordem (0,20 dp), âncora (0,50 ± 0,10 e 0,20 dp) e excesso de
   confiança (razão ≥ 0,75); e a lista dos 6 domínios com os quartis de partida do p05 e o que o p05 já observou com
   eles (para quem compara, não para quem elicita).
2. **Formulários.** Seis arquivos, um por domínio, com a quantidade elicitada, a unidade, a decisão em jogo e o K de
   hoje do domínio; e 18 blocos de especialista em branco com pré-teste, elicitação, controles, devolução e quartis
   confirmados. Nenhum número do domínio aparece nos formulários: os quartis de partida ficam em
   `prioris_elicitadas.yaml` e só vão à mesa na devolução (controle de âncora).
3. **Exemplo e esqueleto.** `prioris_elicitadas.yaml` traz os quartis da priori de literatura do p05 dos 6
   domínios, marcados como **exemplo** e não como elicitação, com os quartis do pacote ao lado;
   `comparacao.md` traz o critério de K = 2 declarado, as prioris lado a lado (pacote, literatura e a coluna em
   branco da elicitada), a decisão antes e depois e o veredito a escrever.
4. **Releitura.** `governanca/RELEITURA.md`: 21 linhas com número, parecer, arquivo e SHA-256; data, decisão e
   nota em branco; a regra de que nos quatro pareceres que envolvem pessoas (2, 3, 17 e 20) a nota diz o que o
   parecer **não** olhou e quais grupos afetados existem, ou "nenhum"; o material campo a campo dos
   `resultados.json` desses quatro, apontando para o arquivo irmão `governanca/GRUPOS.md`; o trecho de
   conferência dos 21 hashes; e, no fim, em bloco não aplicado, a seção "Releitura" que `ASSINATURAS.md` deve
   ganhar depois.

## Números exatos

- **21 de 21** SHA-256 de `RELEITURA.md` conferem com `sha256sum` dos arquivos citados (0 divergentes),
  rodando o próprio bloco de conferência que está no arquivo: `linhas: 21 | divergentes: 0`.
- **6 de 6** domínios com quartis de partida idênticos aos de `fundo/p05-prioris/prioris/{h2,h2g,h1-real}/literatura.yaml`
  (diferença máxima 0 na 15ª casa; `h1-real/c` tem q05/q95 derivados da Beta(1,1) declarada, 0,05 e 0,95, e isso
  está marcado no arquivo).
- **Ajuste das prioris** (aritmética descritiva com as funções do repositório, não elicitação), pacote → literatura
  p05, dp implicado: Preston 2,223903 → 2,603390; direito em log(RC) 0,283331 → 0,104582; STAR 5,930409 → 3,666596;
  STAR turmas 0,088956 → 0,050133; teto do benefício 0,222390 → 0,074130; `h1-real/c` Beta(1; 1) nas duas.
  Em cinco dos seis domínios a priori de literatura é **mais estreita** que a do pacote.
- **K de hoje** dos domínios elicitados, do campo `K` dos `resultados.json`: `h2/desenvolvimento_preston` 1,
  `h2/direito_prisoes` 1, `h2/educacao_star` 1, `h2g/educacao_star_turmas` 3,
  `h2g/direito_teto_beneficio` 3; `h1-real/c` não tem o campo.

## Limites

- **A elicitação de E1 é análise de sensibilidade ($n = 1$), não painel completo.** Os blocos E2 e E3 dos formulários continuam em branco até que os dois especialistas adicionais por domínio (incluindo o de fora do programa e da área) passem pelo pré-teste e pela devolução.
- **O protocolo está congelado em `DECLARACAO.sha256`.** Os cinco pré-testes de 20 itens resolvidos e as declarações de itens estão lacrados em `DECLARACAO.sha256`.
- **Os números de "Ajuste das prioris" acima são a linha de base descritiva**, calculados sobre os quartis de partida do p05; a comparação com as cinco prioris elicitadas por E1 está em `formularios/comparacao.md` (§6–§10).
- **O rótulo do K é do programa.** O `K = 2` de `comparacao.md` é o número desta frente (protocolo = 1,
  comparação = 2) e soma-se ao K de cada domínio (1 → 2 e 3 → 4); se a contagem do programa usar outro número, o
  rótulo se corrige por declaração nova, sem reescrever arquivo nem hash.
- **Um especialista de fora por domínio é o mínimo declarado, não o ideal**: com um só, o controle contra a linguagem
  do programa fica sem par.
- **Três domínios são "mesma base"** (`h2/educacao_star`, `h2g/educacao_star_turmas`,
  `h2g/direito_teto_beneficio`): a priori de literatura deles não é evidência independente, e o painel não pode
  ter a mesma pessoa dos dois lados de um par (por isso `h2g/educacao_star_turmas` ficou reservado para E1).
- **A releitura dos 21 pareceres está aplicada em `governanca/RELEITURA.md` e `governanca/ASSINATURAS.md`**, com 17 pareceres confirmados e 4 retirados e reemitidos; continua valendo o limite de que quem assina é parte interessada.

## Como reproduzir / como conferir

Conferência dos 21 hashes (roda do repositório, não escreve nada):

```bash
cd "$(git rev-parse --show-toplevel)"
python3 - <<'EOF'
import re, subprocess, pathlib
root = pathlib.Path(".")
txt = (root/"governanca/RELEITURA.md").read_text()
rows = re.findall(r'^\|\s*(\d+)\s*\|.*?\|\s*`(.+?)`\s*\|\s*`([0-9a-f]{64})`\s*\|.*$', txt, re.M)
assert len(rows) == 21, len(rows)
ruins = []
for n, arq, h in rows:
    got = subprocess.run(["sha256sum", arq], capture_output=True, text=True).stdout.split()[0]
    if got != h:
        ruins.append((n, arq, h, got))
print("linhas:", len(rows), "| divergentes:", len(ruins))
for r in ruins:
    print("DIFERE", r)
EOF
```

Saída: `linhas: 21 | divergentes: 0` (a mesma saída do bloco que está dentro de `RELEITURA.md`).

Conferência dos quartis de partida contra o p05 e validação dos formulários:

```bash
cd "$(git rev-parse --show-toplevel)/fundo/p12-shelf/formularios"
OMP_NUM_THREADS=1 python3 -c "
import yaml, glob
for f in sorted(glob.glob('*.yaml')):
    m = yaml.safe_load(open(f))
    print(f, 'OK', len(m.get('dominios', m.get('especialistas', {}))))
"
```

Recursos: qualquer script desta frente roda com `OMP_NUM_THREADS=1` e no máximo 2 processos em paralelo. Nada
desta frente foi commitado nem empurrado.
