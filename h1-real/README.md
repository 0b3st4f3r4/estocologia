# H1 em domínio real

Este é o mesmo arcabouço do H1 simulado (`../h1/`): barramento com cadeia de hash, diário Brier/Kelly, F1–F8, M15 e a banca
MB1–MB8. Os três mundos simulados deram lugar a bases públicas reais, descritas com hash em `../dados-reais/FONTES.md`.

```
python3 rodar.py                    # ~3 min; grava saida/
python3 portoes.py --reproducao     # os 5 portões, com uma segunda execução limpa
```

## Resultado dos portões

| Portão | Resultado |
|---|---|
| 1 · invariantes e reprodução | **passa**: 40 propriedades; reprodução idêntica nos três projetos |
| 2 · contrato e dados | **passa** |
| 3 · convergência, SBC e conforme | **passa**: R-hat ≤ 1,007; ESS ≥ 547; cobertura 96% / 100% / 92% |
| 4 · corte temporal e habilidade | **falha por C**: habilidade −0,015 no cofre |
| 5 · governança e decisão | **falha por C**: o M15 reprovou, então o F8 não emitiu decisão |
| humano | os três pareceres de risco seguem sem a assinatura de sil |

A e B foram aprovados pelo M15. C foi reprovado. Os testes não foram ajustados para passar: a reprovação de C é
o resultado do projeto.

## A · Aditivo vs. multiplicativo no Dow Jones (1885–2023)

**Alegação:** "o Dow rende μ por dia; alavancado 3× (rebalanceado diariamente), rende 3μ, e quem segura por décadas sempre
ganha."

**Dados e unidade:** os períodos são 1885–1929, 1930–69, 1970–99 e o cofre 2000–23. A unidade é o ano civil: são 115 anos de
treino, e coortes de 20 anos valem só cerca de 5,8 blocos independentes.

- **Crescimento temporal.** A soma aritmética 3μ daria cerca de 0,20 ao ano. O crescimento temporal realizado é
  0,051 com 1×, 0,072 com 2× e 0,060 com 3×. O arrasto da volatilidade em 3× é de 0,14 ao ano.
- **Kelly.** L\* da posterior é 2,35 [1,26; 3,44].
- **Coortes de 20 anos acima do ponto de partida.** São 96% com 1×, 78% com 2× e 64% com 3×. Em 3×, a riqueza
  média é 19× o capital inicial, mas a mediana é só 2,1×.
- **Dias absorventes.**
  - 19/10/1987 (−22,6%) tira 68% do capital em 3×; a ruína começa em L = 4,4.
  - De 1929 a 1932, o capital cai a 0,11 com 1× e a 0,00025 com 3×.
- **Teste severo.** O mundo da alegação prevê 93% das coortes 3× acima do ponto de partida; o real dá 65%
  (p = 0,01).
- **O que a alegação conserva.** Sem custo de financiamento, 2–3× ainda crescem mais que 1× no tempo. O cofre
  2000–23 favorece a alegação: o previsor sem arrasto teve habilidade 0,44, contra 0,40 do modelo. A refutação vale
  para "3μ" e para "sempre ganha", não para "alavancar nunca compensa".

## B · LaLonde: treino profissional, NSW contra CPS

**Alegação:** comparar participantes do NSW com trabalhadores do CPS, crua ou ajustada, recupera o efeito do treino.

**Desfecho e cofre:** o desfecho é estar empregado em 1978. O cofre são os 260 controles sorteados do NSW. Ele é fora da
amostra por construção, não fora do tempo.

| | ΔR de emprego | renda de 1978 (US$) |
|---|---|---|
| comparação crua | −0,107 [−0,170; −0,045] | −8 498 |
| ajuste por escore de propensão | +0,096 [−0,006; 0,208] | +1 570 [218; 3 265] |
| **experimento** | **+0,111 [0,026; 0,196]** | **+1 794** |

- **O ajuste acerta o ponto, mas não é severo.** Um tratamento inerte daria +0,090 dentro da faixa [−0,103;
  0,119] (p = 0,08). O ajuste se apoia em cerca de 103 controles efetivos, e 72% do CPS fica abaixo do menor
  escore dos tratados.
- **Placebo.** Com o emprego de 1975 como desfecho, o ajuste dá −0,123 (p = 0,04), contra +0,085 no experimento.
  Isso indica seleção por choques de 1975.
- **Cofre.** A habilidade do modelo é +0,18 (previu 0,655 de emprego e o observado foi 0,646). A regressão
  logística tem +0,09, e o contrafactual cru do CPS, −0,002.
- **Justiça.** As taxas de falso negativo diferem por grupo: 0,72 para hispânicos, 0,50 para negros e 0,57 para
  outros. Igualá-las capta mais 22 desempregados e custa 87 falsos positivos.
- **Privacidade.** O k-anonimato vai de 1 a 11 depois da generalização.

## C · A cauda dos incêndios dinamarqueses (1980–1990)

**Alegação:** no fim de 1988, uma resseguradora vê 36 meses sem sinistro na camada "100 xs 50" (de 50 a 150 M DKK). Ela
conclui que o risco é desprezível e escreve 3× o capital.

**Dados e unidade:** a unidade é o mês. A retenção de 50 M DKK fica no percentil 99,7 das perdas, mas foi escolhida com a
série inteira à vista. Por isso o cofre não é cego para o analista, e há uma tabela de sensibilidade para retenções de
20 a 50.

- **Cauda.** O estimador de Hill dá α = 1,81 [1,41; 2,56] (bootstrap em blocos). O ajuste GPD dá ξ = 0,41
  [0,02; 0,72], ou seja, variância perto de infinita.
- **Meses com a camada atingida.** Foram 4, 1 e 0 em cada período de 36 meses, e 2 nos 24 meses do cofre
  (152 M DKK em ago/1989 e 145 M DKK em out/1990).
- **A janela limpa como evidência.** Vale 2,2 bits. A regra de Laplace dá 1/38 por mês, não zero. Um teste
  severo precisaria de 69 meses limpos.
- **Ruína.** Com f = 3 (k/n) e com meio Kelly, a resseguradora é arruinada em ago/1989. Com Laplace termina com
  0,19 do capital; com POT, 0,78. O Kelly da posterior é f\* = 0,46.
- **Por que o M15 reprovou.** No cofre, nenhum previsor vence a climatologia: modelo −0,015, POT −0,003,
  Laplace −0,024 e a alegação −0,052. A taxa do cofre (1/12) ficou acima da do treino, e a frequência de
  sinistros cresce 4,6% ao ano (p = 2·10⁻⁶). A posterior também é dominada pela priori elicitada simulada
  (dp posterior / dp priori = 0,87). A alegação perde, mas o modelo não ganha o direito de decidir.

## Mudanças no arcabouço

- **Correção.** `Diario.placar(projeto=...)`: o placar somava o mesmo previsor nos três projetos. Os números do H1
  simulado foram corrigidos no `../h1/README.md`: B passa a +0,014 e C a +0,97, e os portões seguem aprovados.
- **F5.** O F5 agora lê do projeto `dados_simulados`, `origem_dos_dados` e `tem_titulares`. Em A e C, que não têm
  pessoas, a LGPD é marcada como não aplicável, e o k-anonimato fica informativo (sigilo comercial).
- **Portão 4.** O portão 4 agora aceita a chave de invariância do B real.

## Limites declarados

- **Ainda simulados.** As elicitações SHELF (F7) continuam simuladas e rotuladas como tal. Os cenários de deriva e
  patológicos da banca são perturbações declaradas de linhas reais.
- **Sem custos e sem dividendos.** O Dow é só preço e não há custo de financiamento. As duas ausências favorecem a
  alegação em A.
- **Hipergrafo.** O hipergrafo tem 85 nós, longe da meta de cerca de 10³.
