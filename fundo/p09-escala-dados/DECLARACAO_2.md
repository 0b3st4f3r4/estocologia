# Declaração 2 do fundo/p09 (K = 2 só para a pergunta de evento raro), escrita antes de rodar

Data: 2026-10-15. Motivo: a DECLARACAO.md previa medir a resolução das previsões do diário no cofre do domínio
`saude_indometacina_ercp`, mas o funil podou o domínio no F3 (p de permutação 0,060 contra 0,05) e o diário ficou
vazio. A poda vale e não é revista. Esta declaração abre uma análise **exploratória**, fora do funil, que não emite
decisão nem conta como confirmação da alegação.

- **Pergunta.** Quanta resolução tem uma previsão de pancreatite pós-CPRE (evento raro) num centro novo?
- **Método** (`escala/raro_indo.py`, SHA-256 ef543c0eb22991a19db133aa73ceb888f5afdca5e2d60d2ae23518c51eaa2776):
  logística em indometacina, risco, idade e sexo ajustada no treino (Indiana + outros); previsão para Michigan;
  Brier contra a climatologia do treino; habilidade = 1 − Brier/Brier_clima, com IC90 por bootstrap (2 000
  reamostragens, semente 2027); decomposição de Murphy com 10 caixas de mesma largura.
- **Leitura declarada.** Habilidade < 0,05 ou IC90 que cruza zero = "resolução baixa, como no H3" (+0,019 em
  Rhode Island). Habilidade ≥ 0,05 com IC90 acima de zero = "resolução útil".
- **Ressalva já conhecida.** O cofre não é cego (ver DECLARACAO.md) e Michigan tem taxa de evento diferente; a
  climatologia do treino pode errar o nível, o que entra na confiabilidade.
