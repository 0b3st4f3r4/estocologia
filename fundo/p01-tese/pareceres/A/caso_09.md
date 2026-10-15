# Parecer — caso_09: tamanho de turma na educação infantil

Script: `bracoA/scripts/caso_09.py` (semente 20261015).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "nota = efeito escola + efeito professor/turma + efeito do tamanho da turma + características do aluno + ruído"
    camada_onde_pode_falhar: "L1 (atrito seletivo: notas faltantes), L2 (unidade de inferência: turmas/escolas, não alunos), validade externa (EUA anos 1980 → rede local)"
  nao_linearidade_jensen: "escala de pontos aditiva; nota com teto (627 aparece repetido em leitura e matemática); distribuição sem cauda pesada"
  unidade_independente_e_margem:
    unidade: "turma dentro de escola (randomização); inferência agrupada por escola (79), mais conservadora"
    margem: "efeito FE 16,0 pontos, IC95% 7,4 a 24,6 (cluster por escola); bootstrap de escolas 6,9 a 23,7"
  fisica_ruido_agente_ativo: "sem estados absorventes; professores sabem a condição (possível efeito de expectativa)"
  escada_causal:
    degrau: "intervenção (sorteio dentro da escola)"
    identificacao: "efeito fixo de escola + randomização"
    premissa_nao_testavel: "faltantes ignoráveis, condicionais ao braço"
```

## 2. Ficha da amostra

```yaml
ficha_amostra:
  base: "6325 alunos no ano da educação infantil; 1900 pequena, 2194 regular, 2231 regular+auxiliar"
  chave: "aluno, sem duplicatas"
  faltantes_nota: "8,5% (pequena) vs 8,6% (regular); diferença dentro da escola -0,3 p.p., p = 0,80 (atrito não diferencial)"
  analisados: "3743 alunos com as duas notas, 79 escolas (78 com os dois braços)"
  balanco_dentro_da_escola: "almoço gratuito -0,04 p.p. (p=0,98); sexo F -0,5 p.p. (p=0,74); afro-americano -0,3 p.p. (p=0,63); experiência do professor 9,0 vs 9,1 anos"
  comparacao: "o braço 'regular com auxiliar' foi excluído do contraste (como pede a pergunta)"
```

## 3. DAG

```
escola ──► sorteio_dentro_escola ──► tamanho_turma ──► nota
escola ──────────────────────────────────────────────► nota
aluno (renda, etnia, sexo) ──────────────────────────► nota
professor (sorteado) ────────────────────────────────► nota
nota faltante ◄── (S) — não depende do braço (verificado)
```

## 4. Modelo nulo e teste severo

- Reproduzido: diferença crua 13,9; FE escola 16,0 (IC 7,4–24,6); 0,22 DP.
- Por componente: leitura +6,6 (IC 3,1–10,1); matemática +9,4 (IC 3,7–15,0). As duas na mesma direção.
- Com covariáveis (sexo, etnia, almoço gratuito): 16,1 (IC 7,8–24,4).
- **Nulo na unidade escola (troca aleatória de sinal das 78 diferenças dentro da escola, 20000 réplicas):** p = 0,00045. A diferença média dentro da escola é 15,3 e a mediana 10,3; 63% das escolas têm diferença positiva.
- O resultado não depende do número de alunos (pseudorreplicação): a análise só com escolas mantém o efeito.
- K = 6 especificações; todas na faixa 14–16 pontos com IC excluindo zero.

## 5. Lentes

- **Morfonomia (invariância):** efeito presente em leitura e matemática e em 63% das escolas. A heterogeneidade entre escolas é grande (por isso o IC é largo).
- **Escalonomia:** teto da escala em 627 pode subestimar o efeito; não muda a conclusão.
- **Crisonomia:** a decisão é um **piloto** (reversível, barato em relação a uma adoção ampla). Com efeito de ~0,22 DP (limite inferior ~0,1 DP), o piloto está justificado. O risco principal é de transferência (outro país e outra época).

## 6. Ficha da crítica de modelos

```yaml
critica_modelos:
  checagem_preditiva: "distribuição das diferenças por escola compatível com um efeito médio positivo heterogêneo"
  modelo_rival: "efeito nulo com heterogeneidade de escola: rejeitado pelo teste de troca de sinal (p < 0,001)"
  identificado_pelos_dados: ["efeito médio no ano da educação infantil, no contexto do experimento"]
  trazido_de_fora: ["transportabilidade para a rede local", "ausência de efeito de expectativa dos professores"]
  performatividade: "professores e famílias sabiam a condição; possível efeito Hawthorne"
  premissas_indecidiveis: "atrito ignorável (apoiado pelo balanço); transporte"
  tentativas_K: 6
```

## 7. Decisão (F8)

No experimento, em cada 100 escolas, cerca de 63 tiveram notas maiores nas turmas pequenas que nas regulares, com ganho médio de ~16 pontos (0,22 DP; IC 7–25). A evidência sustenta que turmas pequenas elevaram as notas nesse ano e justifica um **piloto**, que deve medir o efeito localmente (de preferência com sorteio entre turmas/escolas).
Cláusula de saída: se o piloto, com amostra pré-dimensionada, não mostrar efeito ≥ 0,1 DP, não expandir.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: O sorteio foi feito dentro da escola, o atrito não difere entre braços e as covariáveis estão balanceadas. O efeito de cerca de 16 pontos (IC 7,4–24,6) se mantém na unidade escola (troca de sinal p < 0,001), em leitura e em matemática. Para justificar um piloto, isso basta.
```
