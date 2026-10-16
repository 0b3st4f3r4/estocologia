# Quais resultados do livro são significantes para a teoria

Triagem feita em 2026-10-16 sobre os 39 resultados levantados em
`resultados-que-dependem-de-python.md`. A pergunta aqui não é "de onde vem o número", é outra: **o que, se
caísse, derrubaria a teoria?** A resposta separa três camadas: o que sustenta um axioma, o que confirma um
princípio conhecido com número próprio, e o que é ilustração didática.

## Camada A — o que a teoria não pode perder

| Resultado | O que ele sustenta |
| --- | --- |
| **LaLonde: o pareamento acerta o número (+9,6 pp contra +11,1 pp) e é reprovado no placebo de 1975 (−12,3 pp)** | A severidade não é estética: acurácia sem poder de errar não é evidência. É o resultado que dá conteúdo ao $\Phi_3$ e à banca $A_4$. |
| **Incêndios dinamarqueses: a alegação da seguradora cai e o modelo do próprio agente também (habilidade de Brier −0,015, mundo crescendo 4,6% ao ano)** | O portão julga o **agente**, não só a alegação. Sem isso, "reprovar o próprio modelo" seria retórica; com isso, é o mecanismo do $\Phi_4$. |
| **Unidade independente em quatro escalas: dia contra paciente (0,51 contra 0,07 min), 2,65 milhões de chamadas que valem 81 dias, 1 134 coortes que valem 5,8 blocos, 11 598 alunos que valem 80 escolas** | É o axioma operacional do programa: margem calculada na unidade errada é margem falsa. Quatro domínios diferentes, mesma conclusão — isso é generalidade, não exemplo. |
| **Não-ergodicidade com estado absorvente: mediana 2,10× contra média 19,1×, ruína a partir de $L = 4{,}42$, a moeda de Peters** | A média de conjunto deixa de ser resumo legítimo quando o processo é multiplicativo e tem absorção. Derruba o uso ingênuo de valor esperado em decisão. |
| **Cauda selvagem com janela limpa: $\hat\alpha = 1{,}81$, 36 meses limpos valem 2,2 bits, a chance mensal segue 1/38** | Silêncio não é evidência sob cauda de potência. É o fundamento do $H_1$ rejeitado e do tratamento de eventos raros. |
| **O empate no piloto cego de 12 casos** | Delimita a tese: no catálogo clássico de erros, a arquitetura empata com um analista cuidadoso. Um resultado negativo que define escopo é teorização — sem ele, a tese ficaria sem falsificador. |
| **ICP: os três sucessos (túnel de luz, hERG, levedura) e a queda do STAR ao corrigir o agrupamento** | A causa é o que sobrevive à troca de ambiente, e o agrupamento decide o veredito. É a definição operacional de causalidade que o programa adota — e o segundo teste da unidade independente. |
| **Goodhart medido: 0,145, quatro vezes menos que o 0,6 declarado** | A performatividade é grandeza, não desculpa: dá para medir o quanto a previsão deforma o mundo que prevê. |
| **Calibração contra resolução: ECE 0,0048 com habilidade modesta de +0,019** | Calibração se conserta, resolução não. Separa o que é consertável do que é limite de dado — a base da decomposição de Murphy usada no livro. |

## Camada B — princípios conhecidos, medidos por este programa

| Resultado | Papel |
| --- | --- |
| Erlang C promete 82% e entrega 57%, com 26 dias falhos em 100; +7,2% de agente-horas nos picos derruba a falha para menos de 4 em 100 | Convexidade da espera em operação real, com o conserto declarado |
| Dow Jones: 5,1% / 7,2% / 6,0% ao ano em 1×, 2× e 3×, arrasto de 0,1403, −22,6% em 1987 matando 68% de quem estava a 3× | O pedágio de Jensen no tempo, medido em 37 930 pregões |
| O posto de saúde: 9,1 contra 19,3 min com a mesma média de demanda | Jensen no caso mínimo, com a ambigüidade da média exposta |
| *Literary Digest*: 2 400 031 cédulas que valem 7,2 pessoas, erro de 18,2 pontos, 661 margens de erro | A identidade de Meng (qualidade × quantidade × dificuldade) com número |
| Brier e Murphy fechando a identidade até $3{,}5\times10^{-15}$; decibans de 10 contra 6,23 | As ferramentas de escore e de força de evidência funcionando |
| Colisor e Berkson medidos (+0,0012 e −0,679); Simpson invertendo de novo na ponderação | As pontes entre confundidor e colisor são operacionais, não decorativas |
| Três cofres até a calibração (Toronto queima, Platt corrige, Rhode Island confirma) | A governança por cofre, com a regra de que trocar de cofre é o conserto, não ajustar no mesmo dado |

## Camada C — ilustração ou escala

Mamografia e Sally Clark, peru de Russell, matriz de Stirling, Walker, regra de Laplace de 1/38 como cálculo,
morfonomia 24,37 e −4,60, EVPI de 3,70, Hill 1,7901 como método, certificado de Fisher–Tippett, os 454 178 nós
do hipergrafo, $K = 38$ como contabilidade, os 21 pareceres. Sustentam o ensino e a auditabilidade; não
sustentam nada que caia se o número mudar.

## O que a triagem muda no plano

- Os invariantes do $\Phi_1$ ao $\Phi_4$ ficam com fundamento declarado: $\Phi_2$ tem quatro escalas, $\Phi_3$
  tem o LaLonde, $\Phi_4$ tem os incêndios.
- Duas entradas teóricas nasceram da própria pesquisa e não da literatura: **o portão que reprova o agente**
  (Camada A, incêndios) e **a performatividade como grandeza** (Camada A, Goodhart).
- Um resultado negativo (o empate dos 12 casos) e um limite ético (nenhum grupo afetado ouvido) ficam no corpo
  da teoria, porque delimitam o que ela promete.
