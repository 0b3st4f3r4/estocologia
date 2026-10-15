# Estocologia

Estocologia é a ciência (ou arte) que estuda o julgamento e a decisão sob incerteza.

Este repositório guarda a pesquisa conduzida segundo a arquitetura do projeto (hipergrafo tipado, camadas
Mundo/Modelo/Agente, agentes F1–F8 e orquestrador M15, fichas YAML, invariantes Φ1–Φ4), do H0 ao H3, em dados reais.
A pesquisa foi encerrada em 2026-10-15.

| Pasta | Conteúdo |
|---|---|
| `h0-lab0/` | H0 simulado (Lab 0, posto de saúde) |
| `h0-real/` | H0 real: central de atendimento bancária, 2003 |
| `h1/` | H1 simulado: projetos A multiplicativo, B causal, C cauda |
| `h1-real/` | H1 real: Dow Jones (aprovado), NSW de LaLonde (aprovado), incêndios dinamarqueses (rejeitado) |
| `h2/` … `h2g/` | H2: 12 domínios reais, reexecuções com intervenções (K=1–3), genética de levedura (H2f) e ensaio cardíaco da FDA (H2g) |
| `h3/` | H3: malha contínua em 6 fluxos, com as revisões H3b–H3d |
| `dados-reais/` | Dados, origens (`FONTES.md`) e somas `SHA256SUMS` |
| `governanca/` | Pareceres de risco F5 assinados |
| `referencias/` | Conferência das referências bibliográficas |
| `artigo/` | Artigo "Estocologia": Velhas Fronteiras, Novos Horizontes, com as figuras |

Cada horizonte tem seu `README.md` com resultados, comandos de reprodução e limites declarados.
Arquivos grandes ficaram fora do Git; veja [FORA_DO_GIT.md](FORA_DO_GIT.md).
