# H1 · Ciclo completo nas 8 faixas

Este é o Horizonte H1 do **PLAN.md**: três projetos concorrentes passam pelo ciclo de 6 estágios da **Arquitetura**. Os agentes F1–F8 e o orquestrador M15 trocam informação só por fichas estruturadas num barramento encadeado por hash, como pede o **AGENTS.md**. O H0 (Lab 0) está em `../h0-lab0`.

## Resultado

**Os três projetos foram aprovados** no portão M15 e resistiram aos 8 ataques da banca (MB1–MB8). Os **5 portões de CI passam**. Uma segunda execução, numa pasta limpa, reproduz resultados idênticos.

| | A · Aditivo vs. multiplicativo | B · Causal | C · Cauda pesada |
| --- | --- | --- | --- |
| Base | Lab 8, moeda de Peters | Lab 13, vitamina e infarto | Lab 9, Kelly que não viu a cauda |
| Alegação | "+5% esperado por rodada; aplicar tudo é bom" | "quem toma vitamina tem 21% menos infartos; ela protege" | "nunca perdeu em 5 anos; o risco é desprezível, alavanque 3×" |
| Conferência com o lab | 13,6% acima do início em 100 rodadas (dados: 16%) | RR observacional 0,78 / ajustado 1,25 / do(X) 1,25 (lab: 0,79 / 1,26 / 1,24) | f* = 1,592, g* = 0,00937, P(sem perda em 250 sem.) = 0,367 (idênticos) |
| Teste severo contra o nulo | No modelo aditivo da alegação, 88% terminariam acima do início; observado 16% (p < 1/2000) | O nulo (vitamina inerte, mesmo confundimento) dá RR 0,66, mais "protetor" que o observado: **a evidência da manchete não é severa** | 37% dos fundos com p = 0,004 não mostram perda: "nunca perdeu" vale 1,3 bit; um teste severo exigiria 14 anos limpos |
| Posterior (F4) | p = 0,503 [0,500; 0,505] | ΔR causal = +0,020 [0,015; 0,025] | p = 0,0041 [0,0040; 0,0041] |
| R-hat / ESS / SBC p | 1,001 / 2 837 / 0,52 | 1,006 / 815 / 0,95 | 1,003 / 2 422 / 0,95 |
| Habilidade fora do tempo (Brier vs. climatologia) | modelo **+0,22**; alegação −4,58 | modelo **+0,014**; manchete −0,001 | modelo **+0,97**; alegação −0,14 |
| Cobertura conforme (meta 90%) | 94,0% | 100% (trivial: desfecho binário) | 93,8% |
| Decisão (F8) | Kelly f = 0,26, não a aposta total | não recomendar a vitamina | alavancagem ≤ 1,59, com p pela regra de Laplace |
| Frequências naturais | De 100 que aplicam tudo por 1 000 rodadas, 0 terminam acima e 100 perdem mais da metade; com Kelly, 97 terminam acima | De 1 000 que passam a tomar, ~102 infartam; sem tomar, ~82 | De 100 fundos que alavancam pelo próprio histórico, 35 se arruínam; com Laplace, 0 |
| k-anonimato bruto → generalizado | 2 → 26 | 1 → 546 | 1 → 30 |
| EVPI posterior | 0 | 0 | 0 |

Os três EVPI nulos querem dizer que coletar mais dados não mudaria nenhuma das três decisões: seria "curiosidade sem uso" (M11/M14).

## O que a banca atacou (os mesmos 8 ataques, nos três projetos)

| Ataque | O que faz | A | B | C |
| --- | --- | --- | --- | --- |
| MB1 | Hypothesis com 500 casos e semente alheia | 6 propriedades | 11 | 7 |
| MB2 | Lote com nulos, fora de faixa e chave duplicada, reingerido duas vezes; `SELECT *` | 15/15 em quarentena, idempotente, recusado | idem | idem |
| MB3 | Deriva injetada; o KS precisa disparar | p ≈ 10⁻⁵⁴ | KS 0,37 | KS 0,79 |
| MB4 | Mundo que viola o modelo; o PPC precisa acusar | autocorrelação 0,50 (p = 0) | controle negativo acusa confundidor oculto (z = −6,9) | dispersão 5,35 sob contágio (p = 0) |
| MB5 | Adulteração de uma linha do barramento | detectada | detectada | detectada |
| MB6 | Inversão da métrica de justiça | mediana vs. média | trilema: FNR igual custa 1 273 falsos positivos | cotistas vs. gestores |
| MB7 | Priori atípica (vendedor, fabricante, pós-crise) | decisão robusta | robusta | robusta |
| MB8 | Violação da cláusula de saída | disparou só na deriva | idem | idem |

## Achados que valem além dos labs

1. **B, o nulo é mais forte que a alegação.** Com a vitamina inerte, o confundimento sozinho já produz RR 0,66, e o estudo observacional confirmaria a manchete de qualquer jeito. O teste ajustado por Z, ao contrário, é severo e aponta dano (RR 1,25, fora do intervalo nulo [0,94; 1,06]).
2. **C, a elicitação não corrige a âncora; os dados corrigem.** Especialistas simulados que nunca viram uma perda chegam ao consenso de p ≈ 0,0009, 4,4 vezes abaixo do mundo. Os extremos do SHELF levam P(p ≥ 0,004) de 5,0% só para 5,6%.
3. **B, o trilema medido.** O escore é calibrado nos dois sexos, as taxas-base diferem (0,069 contra 0,111) e os FNR divergem (0,24 contra 0,36). Igualar o FNR põe 134 homens a mais no programa ao custo de 1 273 falsos positivos.
4. **Diário de previsões.** Os previsores que leem a alegação perdem para a climatologia nos três projetos, e o orquestrador zera o peso deles.

## Limites declarados

- **Tudo é simulado.** Não há dados reais nem especialistas humanos. As elicitações SHELF trazem a nota "ELICITAÇÃO SIMULADA", e os pareceres F5 aguardam a **assinatura de sil** (o Portão 5 a lista como pendência humana).
- **A meta de ~10³ nós do H1 não foi atingida.** O hipergrafo exportado (`saida/hipergrafo.json`) tem 86 nós tipados, 72 arestas e 16 hiperarestas. Contar investidores ou pessoas como nós inflaria o número sem conteúdo.
- **O amostrador é fraco.** O Metropolis de passo fixo reprova o R-hat em cerca de 40% das sementes quando d ≥ 5, por isso o projeto B colapsou o modelo para 4 dimensões. É o primeiro conserto para o H2.
- **A resolução do C vem da regra de alavancagem.** A habilidade do modelo depende da alavancagem que cada fundo escolhe pelo próprio histórico, não de prever quando a perda chega.
- **A habilidade do B é pequena (+0,014).** O efeito da vitamina é de 2 pontos, e prever infarto individual é difícil. O braço sorteado do período 4 confirma o degrau 2: RR 1,32 e ΔR +0,025, perto da posterior do modelo (+0,020).

## Como rodar

```bash
pip install numpy scipy duckdb pyarrow matplotlib pyyaml hypothesis pytest
python3 rodar.py                # os 3 projetos, ~3 min; grava saida/
python3 portoes.py --reproducao # os 5 portões + segunda execução limpa, ~4 min
python3 grafo.py                # exporta o hipergrafo
```

## Arquivos

- `estocologia/`: o framework.
  - `barramento.py`: fichas com encadeamento de hash.
  - `diario.py`: Brier e capital de Kelly por previsor.
  - `agentes.py`: F1–F8.
  - `orquestrador.py`: M15, com os 6 estágios, a poda por EVPI, a banca e a ficha da crítica.
  - `estat.py`: MCMC, R-hat, ESS, SBC, predição conforme, Hill, SHELF, privacidade diferencial em bits e EVPI/EVSI.
  - `projeto.py`: a interface.
- `projetos/`: os três projetos. `tests/`: as propriedades Hypothesis de cada um.
- `saida/`:
  - `barramento.jsonl`: a trilha de auditoria.
  - `diario.json`: as previsões pré-registradas e resolvidas.
  - `<projeto>/resultados.json`, `datacontract.yaml` e `hops-<projeto>.gif`.
  - `portoes.json` e `hipergrafo.json`.
  - `loja/`: o Parquet particionado.

## Próximo horizonte (H2, segundo o PLAN.md)

Transferir para domínios externos reais pelo F7. Isso pede especialistas humanos de verdade no SHELF, dados reais em Parquet, um amostrador adaptativo (ou HMC) e a assinatura do responsável humano.

## Correção (2026-10-14, durante o H1 em domínio real)

O placar do diário somava as previsões do mesmo previsor (`agent-f4-frontier`) nos três projetos. As habilidades
publicadas antes para B (+0,020) e C (+0,14) eram misturas. Por projeto, recalculadas de `saida/diario.json`, elas são
**B +0,014** e **C +0,97**. A de A (+0,22) não mudou. Os portões continuam aprovados, porque todas seguem
positivas. A correção do código (`Diario.placar(projeto=...)`) está em `../h1-real/estocologia/`.
