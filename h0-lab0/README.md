# H0 · Semente epistemológica no Lab 0 (posto de saúde)

Primeiro subgrafo da Estocologia, montado conforme os documentos **Arquitetura**, **PLAN.md** (Horizonte H0) e **AGENTS.md** do Drive do projeto. O domínio-piloto é a Parte 11 do Lab 0. O processo gerador é o do notebook de soluções do curso (`materiais/m0/lab0-a-funcao-da-media-solucoes.ipynb`), com a mesma semente (1909).

## Como rodar

```bash
pip install numpy==2.5.3 pyyaml hypothesis pytest
python3 portoes.py          # Φ1–Φ4 + conferência; grava resultados.json; ~8 s
```

## Arquivos

| Arquivo | Papel na arquitetura |
| --- | --- |
| `ficha.yaml` | Ficha do barramento: cabeçalho de 5 perguntas, 7 nós tipados (camada × incerteza), arestas (causal, ponte, patologia), 3 lentes, termo de escopo e circuit breaker |
| `estocologia/lab0.py` | agent-f1-code: o processo gerador (Lindley), os 5 mundos, a margem por dia e o modelo nulo |
| `estocologia/esquema.py` | Contrato do hipergrafo: valida os campos obrigatórios dos nós e as referências das arestas |
| `tests/test_invariantes.py` | Portão 1: 6 propriedades com Hypothesis (espera ≥ 0, monotonia no serviço, W crescente e convexa, eCDF, semente) |
| `portoes.py` | Orquestrador M15 do H0: roda tudo duas vezes, compara os hashes, aplica Φ1–Φ4 e confere os números publicados |
| `resultados.json` | Saída completa, com o hash SHA-256 dos resultados |

## Resultado

O resultado passa nos quatro portões. Os 6 números publicados na Fundamentação foram reproduzidos, e com contagem idêntica de pacientes (161 867 e 160 220).

| | Mundo da alegação (λ = 8) | Mundo real (λ = 5 ou 11) |
| --- | --- | --- |
| Espera média por paciente | 9,3 ± 0,16 min | 20,7 ± 0,51 min |
| Mediana | 3,8 min | 9,3 min |
| Percentil 90 | 26,7 min | 59,3 min |
| Pacientes com espera > 30 min | 8 em cada 100 | 25 em cada 100 |

Com reforço só nos dias de surto, a espera média cai para 2,5 min e 0,4% dos pacientes esperam mais de 30 min.

## O que o H0 acrescenta ao Lab 0

1. **O nulo arbitra a margem (Φ2).** Repartindo ao acaso 4 000 dias do mundo da alegação em duas metades, a diferença entre elas tem desvio de 0,216 min. A margem calculada por dia prevê 0,227, e a calculada por paciente prevê 0,047. A pseudorreplicação não é só uma questão de convenção, porque o próprio modelo nulo a refuta empiricamente.
2. **Teste severo (Φ3).** A diferença entre os mundos, 11,4 min, fica a 53 desvios do nulo. Em 2 000 repartições, o nulo nunca passou de 0,81 min.
3. **A mediana do mundo real (9,3 min) coincide com a média do mundo da alegação.** Um gestor que olhe "o paciente típico" no mundo real vê o mesmo número que a alegação promete. A diferença está toda na cauda, o que reforça a regra de proibir a média solitária.

## Pendências para o H1

- Laço performativo (M11): desistência ou troca de horário quando a fila cresce.
- Partição out-of-time (F3): até agora só há mundo simulado, sem dados reais.
- Elicitação SHELF (F7) da demanda diária real, que hoje é assumida bimodal.
- Barramento: a ficha está em YAML, mas ainda não circula entre agentes distintos.
