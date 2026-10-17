# PLAN.md — Plano Diretor, Arquitetura Técnica, Pedagogia e Conteúdo Consolidado da Estocologia (Rust Edition 2024)

Este documento estabelece o plano diretor exaustivo e expositivo da **Estocologia**, unificando os
fundamentos epistemológicos, a estrutura didática progressiva, a arquitetura cognitiva híbrida, a
governança criptográfica e o ecossistema modular de engenharia em Rust.

Duas camadas convivem aqui, e a distinção é honesta:

- **Camada de engenharia viva (Rust, Edition 2024):** os crates de `crates/`, com testes e portões
  determinísticos que rodam neste repositório hoje (seções 7 e 12).
- **Camada de registro empírico (Python):** as frentes `h0-lab0/`, `h0-real/`, `h1/`, `h1-real/`,
  `h2/…h2g/`, `h3/`, `fundo/`, `governanca/`, `dados-reais/`, `artigo/` e o livro
  `estocologia-livro-didatico.md`. É o rastro da pesquisa encerrada em 2026-10-15, descrita nas
  seções 6 e 11.

---

## 1. Visão Geral e Fundamentos Epistemológicos

A **Estocologia** é definida formalmente como a ciência (e a arte) do julgamento e da decisão sob
incerteza. Ela costura três tradições históricas que operavam isoladas: a filosofia da indução
(Hume, Popper, Lakatos), a inferência estatística e a mecânica dos processos estocásticos no tempo.

O axioma central da disciplina postula que **quase toda falácia, ilusão analítica ou erro de projeto
não decorre de falhas de aritmética interna, mas sim de desalinhamentos nas pontes entre as três
camadas fundamentais da realidade**:

- **Mundo ($\mathcal{L}_1$):** a realidade física ou social geradora de dados, repleta de ruído,
  dependência de trajetória e caudas pesadas.
- **Modelo ($\mathcal{L}_2$):** a representação formal, simplificada e simulável construída para
  aproximar o mundo.
- **Agente ($\mathcal{L}_3$):** quem decide (pessoa, instituição ou algoritmo), limitado por tempo,
  atenção, utilidades e laços performativos onde a própria ação deita abaixo o modelo.

No código, essa separação é um tipo:

```rust
pub enum Camada { Mundo, Modelo, Agente }        // crates/stochology-core/src/camada.rs
pub enum TipoIncerteza { Aleatoria, Epistemica, Knightiana }
```

Misturar camadas deixa de ser descuido de texto e passa a ser erro de compilação ou ficha reprovada.

---

## 2. Rampa Pedagógica e Estrutura do Livro Didático

Para que qualquer estudante (do Ensino Médio à pós-graduação) atravesse a curva de conhecimento sem
saltos abruptos, a obra adota uma rampa progressiva (*scaffolding*) de **3 degraus por capítulo**:

1. **Intuição Fenomenológica e Decisão:** frequências naturais, árvores de contagem e gráficos
   visuais para expor o paradoxo em linguagem natural.
2. **Tratamento Teórico-Formal:** definições explícitas, condições de identificação e teoremas
   canônicos (Jensen, Meng, Murphy, Pearl, Kelly, Chouldechova).
3. **Laboratório em Rust:** implementação autocontida com semente fixa, tipagem estrita de
   invariantes e testes de propriedades.

O desdobramento completo em 21 capítulos e 5 partes está na seção 9.

---

## 3. A Máquina Epistemológica e o Hipergrafo Tipado

O conhecimento não é mantido em texto ambíguo, mas estruturado como um hipergrafo tipado executável:

$$\mathcal{G} = (V, E_{\text{tree}}, E_{\text{graph}}, H, \Phi)$$

- **Vértices ($V$):** nós que guardam ID, camada, tipo de incerteza (*aleatória*, *epistêmica* ou
  *knightiana*), definição formal em uma linha e ponteiro para o artefato simulável.
- **Árvores e arestas ($E_{\text{tree}}, E_{\text{graph}}$):** pré-requisitos dedutivos, pontes
  matemáticas, arestas causais $\operatorname{do}(x)$ e arestas de patologia que denunciam
  desalinhamentos (falácia da média, paradoxo da inspeção, peru de Russell).
- **Hiperarestas / as cinco lentes ($H$):** ferramentas transversais que inspecionam o problema sob
  óticas específicas:
  1. *Fluxonomia:* fluxo e perda de informação nas correntes de dados ($I(X;Z) \le I(X;Y)$).
  2. *Crononomia:* ergodicidade, média temporal contra média de conjunto, estados absorventes.
  3. *Escalonomia:* caudas pesadas, expoente de Hill, existência de momentos teóricos.
  4. *Morfonomia:* invariabilidade estrutural sob mudança de regime e intervenção (ICP).
  5. *Crisonomia:* valor esperado da informação (EVPI), escores próprios de pontuação e calibração
     fora do tempo.

O validador é executável: `Grafo::validar()` recusa nós duplicados ou incompletos, arestas para nós
ausentes, ciclos dedutivos em $E_{\text{tree}}$ e hiperarestas com menos de dois membros;
`Grafo::nos_sem_lente()` cobra que todo nó esteja sob ao menos uma das cinco lentes.

---

## 4. Arquitetura Cognitiva Híbrida: Sistema 1 e Sistema 2

A cognição da máquina opera em modos acoplados sob governança central:

- **Modo Sistema 1 ($O(1)$):** triagem rápida e classificação categórica fechada de fichas YAML via
  árvores de sufixos determinísticas, emitindo probabilidades restritas ao intervalo epistemológico
  $p \in [0{,}01;\ 0{,}99]$. O nome *LLM-as-Jev* cobre duas receitas, e o estado delas não é o mesmo: o
  **Jev determinístico** existe em `crates/stochology-core/src/s1.rs` (877 linhas, 12 testes, `labs --s1`),
  com o lugar do LLM ocupado por um juiz mudo, porque um LLM livre não fecha o G1; o **LLM-as-Jev** de pesos
  abertos congelados segue declarado e não implementado, como item 13 de `PROPOSTAS_DECLARADAS.md`, e a
  conferência da transcrição que o propôs está em `INTEGRACAO_JEV.md`.
- **Modo Sistema 2 (deliberativo):** busca em árvore (*LATS* + *Belief-State Engine* sobre o
  hipergrafo $\mathcal{G}$) para síntese profunda de código ($F_1$), inferência MCMC e estresse
  adversarial. Vive em `crates/stochology-s2`: a busca sobre o hipergrafo, o motor de estado de crença, a
  inferência de Metropolis e o estresse deliberativo da banca estão de pé com teste e comando
  (`labs --s2`); a síntese de código ($F_1$) segue declarada e não implementada.
- **Orquestrador ($\mathcal{M}$) e agentes de faixa ($F_1$ a $F_8$):** o orquestrador conduz o ciclo
  de seis estágios e submete os artefatos às faixas especialistas: $F_1$ (código e simulação),
  $F_2$ (grão do dado e ausentes), $F_3$ (relógio, deriva e "não sei"), $F_4$ (estatística e SBC),
  $F_5$ (privacidade e trilha SHA-256), $F_6$ (justiça e quem paga pelo erro), $F_7$ (priores
  SHELF) e $F_8$ (comunicação e cláusula de saída).

A **banca adversarial** é o braço de ataque do orquestrador: cada faixa tem um agente $A_i$ que
tenta quebrar o artefato antes da aprovação (seção 8).

---

## 5. Governança, Invariantes e Portões Determinísticos

- **Quatro invariantes estruturais ($\Phi_1$–$\Phi_4$):** $\Phi_1$ (simulabilidade com semente
  fixa), $\Phi_2$ (margem calculada na unidade independente real), $\Phi_3$ (camadas explícitas e
  teste severo contra o modelo nulo) e $\Phi_4$ (escopo ecológico, auditoria de quem paga pelo erro
  e cláusula de saída).
- **Prevenção ao erro tipo III:** campo obrigatório `pergunta_do_decisor`, validado por código,
  exigindo interrogação final e correspondência literal com a ação do pacote.
- **Trilha criptográfica:** livro-razão encadeado por hashes SHA-256 e Merkle DAG, cofre fora do
  tempo e contador de tentativas $K$.
- **Sete portões automáticos:** G1 determinismo, G2 semente fixa, G3 escopo declarado, G4 invariantes
  e tipo III, G5 integridade da trilha, G6 banca adversarial $A_1$–$A_8$, G7 cláusula de saída.

---

## 6. Horizontes Empíricos ($H_0$ a $H_3$) e Auditoria de Fundo

- **$H_0$ (semente epistemológica):** piloto simulado e real (posto de saúde e central de chamadas),
  testando filas não lineares e a convexidade da espera.
- **$H_1$ (ciclo completo):** processos aditivos contra multiplicativos (Dow Jones e arrasto de
  volatilidade $\mu - \tfrac{1}{2}\sigma^2$), causalidade (LaLonde) e caudas pesadas (incêndios
  dinamarqueses).
- **$H_2$ (transferência entre domínios):** predição causal invariante (ICP) em múltiplas rodadas e
  domínios heterogêneos (túnel de luz, ensaio cardíaco da FDA, genética de levedura).
- **$H_3$ (malha contínua sob relógio):** operação sob restrição estrita de tempo ($t_{\text{disp}}$),
  canários de vazamento, controle de Goodhart e recalibração contínua de Platt.
- **Auditoria de fundo:** o teste severo de 10 premissas do próprio programa, incluindo o laço
  performativo, a calibração dependente de $n$ e dados reais de saúde da PhysioNet.

---

## 7. Workspace Técnico em Rust (Edition 2024)

O ecossistema é modularizado em crates dedicados e determinísticos. **Dezesseis existem e passam em teste hoje**
(2026-10-17, 350 testes), e nenhum nome do roteiro ficou para trás: os dez que faltavam nasceram com teste próprio, o
décimo sexto é o motor ondulatório (`stochology-wave`, binário `onda`), e a triagem do Sistema 1 continua como módulo de `core`.

| Crate | Estado | Conteúdo |
|---|---|---|
| `crates/stochology-core` | vivo | camadas, ficha, invariantes $\Phi_1$–$\Phi_5$, tipo III, banca $A_1$–$A_8$, orquestrador $\mathcal{M}$, RNG SplitMix64, triagem do Sistema 1 e funil de filtros |
| `crates/stochology-book` | vivo | laboratórios do livro capítulo por capítulo, com o binário `labs --cap N` e `--todos` |
| `crates/stochology-hypergraph` | vivo | hipergrafo $\mathcal{G}$, validador de estrutura, ordem topológica e as cinco lentes |
| `crates/stochology-audit` | vivo | SHA-256 sem dependências, livro-razão encadeado e raiz de Merkle |
| `crates/stochology-gates` | vivo | binário `portoes`: 7 portões determinísticos de governança por horizonte (G1 a G7) |
| `crates/stochology-schema` | vivo | o contrato em arquivo: texto canônico da ficha, versão 1.2 e a **cadeia de migrações em `migracoes.tsv`**, embutida na compilação |
| `crates/stochology-s2` | vivo | deliberativo (10 módulos, 81 testes unitários e 7 de integração): LATS (`lats`), estado de crença (`crenca`), hipergrafo (`hipergrafo`), Metropolis (`inferencia`), estresse da banca (`estresse`), eixo R (`planejamento`), bancada FSM sem orquestrador G8.1–G8.4 (`banca`), ponte Jev-LLM (`ponte`), runner local (`runner`) e calibração de Brier (`calibracao`), mais o binário `ingestar-jev` |
| `crates/stochology-h0` | vivo | Erlang B e C, escala mínima, nulo de Poisson e o resumo obrigatório (media, mediana, quantis) |
| `crates/stochology-h1` | vivo | arrasto da alavancagem, viés de seleção e o estimador de Hill da cauda |
| `crates/stochology-h2` | vivo | as rodadas de invariância causal, o contador $K$ e Bonferroni, com nulo por embaralhamento e resolução fina da genética |
| `crates/stochology-h3` | vivo | relógio estrito com canário, ECE, habilidade de Brier e o laço performativo |
| `crates/stochology-icp` | vivo | predição causal invariante: ambientes, mínimos quadrados, Kolmogorov-Smirnov e a área que sobrevive |
| `crates/stochology-drift` | vivo | CUSUM bilateral com o nulo estacionário medido e a divergência de Kullback-Leibler |
| `crates/stochology-fairness` | vivo | paridade demográfica, igualdade de oportunidades e calibração por subgrupo |
| `crates/stochology-wasm` | vivo | playground WebAssembly: ponte C mínima |
| `crates/stochology-wave` | vivo | motor ondulatório (binário `onda`): superposição fasorial local, Laplaciano magnético com propagador de Cayley, plasticidade Kuramoto-Oja e barramento de espículas com STDP, com seis portões próprios |

**Desambiguação dos portões `G1`–`G7` (`portoes`) e `G1`–`G6` (`onda`).** Os dois binários usam a sigla `G` em
escopos distintos:
- Em `portoes` (`stochology-gates`), **G1 a G7** cobram a governança das fichas por horizonte: G1 determinismo
  (raiz de Merkle idêntica em duas passadas), G2 semente fixa, G3 escopo declarado, G4 invariantes $\Phi_1$–$\Phi_5$
  e prevenção ao erro tipo III, G5 integridade da trilha SHA-256, G6 banca adversarial $A_1$–$A_8$ (piso 6) e G7
  cláusula de saída.
- Em `onda` (`stochology-wave`), **G1 a G6** cobram a física do motor ondulatório: G1 hermiticidade do Laplaciano
  magnético $L^{(q)}$, G2 unitariedade do propagador de Cayley, G3 cadeia de selos sobre o estado fasorial, G4
  determinismo da evolução de fase, G5 ganho de ressonância no alvo sob plasticidade Kuramoto-Oja e G6 seta causal
  do barramento de espículas sob STDP.

Verificação dos portões determinísticos em máquina limpa:

```bash
cargo test --workspace
cargo run --release --bin portoes -- --horizonte todos --reproducao
cargo run --release --bin onda -- --check
```

Saída real medida neste repositório (12 fichas, 4 horizontes):

```text
horizonte=todos fichas=12 defeito=nenhum
  lentes: hill_alfa=1.7901 cauda_selvagem=true mediana_log_riqueza=-10.5361 arruinadas=0.892 ...
raiz_merkle=9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81
reproducao: segunda execucao identica = true (raiz 9716e28a87bec9cf)
G1 ok       determinismo da raiz 9716e28a87bec9cf
G2 ok       semente fixa em 12 fichas
G3 ok       escopo declarado: so artefato simulado
G4 ok       Phi1-Phi5 e tipo III (falhas=0)
G5 ok       corrente SHA-256 integra e adulteracao detectada
G6 ok       banca A1-A8 com piso 6 e pior 8
G7 ok       clausula de saida declarada e A8 acima do piso
hash_do_relatorio=08e7c8f0e8e706f9bf95286dd5f6b265c965125e5f496c83b8d9417f53d0e5c2
veredito: 7 portoes ok
```

Um portão que nunca falha não é portão. O injetor de defeito prova que a banca morde:

```bash
cargo run --release --bin portoes -- --horizonte h3 --defeito h3.voos23
# G6 reprovado  banca A1-A8 com piso 6 e pior 2
# veredito: 1 portoes reprovados  (exit 1)
```

---

## 8. Banca Adversarial: Formalização de $A_1$ a $A_8$

- **O orquestrador ($\mathcal{M}$):** coordena o hipergrafo e gerencia os portões de aceitação.
- **Os agentes da banca adversarial ($A_1 \dots A_8$):** cada $A_i$ aplica o teste de estresse
  específico sobre a sua faixa:
  - $A_1$: testes de propriedades em código.
  - $A_2$: injeção de ruído e valores sentinela em dados.
  - $A_3$: simulação de deriva temporal ao vivo.
  - $A_4$: testes de patologia inferencial.
  - $A_5$: adulteração e auditoria da trilha de hash.
  - $A_6$: inversão de métricas de subgrupo (justiça).
  - $A_7$: choques de priori e cenários de domínio.
  - $A_8$: simulação de crises violando a cláusula de saída.

A condição de aprovação do artefato $X$ pelo conjunto de agentes é:

$$\text{Aprovado}(X) = \bigwedge_{i=1}^{8} \Big( A_i(X) \ge \text{Severidade}_{\min} \Big)$$

com $\text{Severidade}_{\min} = 6$ na escala $0$–$10$. Cada $A_i$ computa a severidade a partir do
cuidado correspondente da ficha (teste de propriedade, tratamento de ausentes, deriva declarada,
patologias cobertas, trilha íntegra, subgrupos auditados, priori elicitada, cláusula de saída) e de
uma semente explícita — mesma ficha e mesma semente devolvem a mesma severidade, sempre.

```rust
use stochology_core::{AgenteA, Ficha, OrquestradorM, Severidade, SEVERIDADE_MINIMA};

/// Orquestrador M: conduz o ciclo de seis estágios e a banca adversarial.
pub struct OrquestradorM {
    pub id_sessao: String,
    pub hash_trilha: String,
}

impl OrquestradorM {
    /// Conduz o estresse adversarial usando o agente A_i especificado.
    pub fn conduzir_ataque(&self, agente: AgenteA, ficha: &Ficha) -> Result<Severidade, String> {
        if ficha.id.trim().is_empty() {
            return Err("ficha sem identificador".to_string());
        }
        Ok(stochology_core::estressar(
            agente,
            ficha,
            ficha.semente.unwrap_or(2026),
        ))
    }

    /// Roda a banca inteira: AND das oito severidades contra o piso.
    pub fn banca(&self, ficha: &Ficha) -> stochology_core::Aprovacao {
        stochology_core::aprovado(ficha, ficha.semente.unwrap_or(2026))
    }
}
```

O enum de agentes, na forma canônica:

```rust
pub enum AgenteA {
    A1Codigo,
    A2Dados,
    A3CicloVida,
    A4Inferencia,
    A5Governanca,
    A6Justica,
    A7Dominio,
    A8Decisao,
}
```

---

## 9. Pedagogia: o Livro Reestruturado em 21 Capítulos

### 9.1 Filosofia pedagógica da edição em 21 capítulos

Na versão anterior de 14 capítulos, assuntos densos (Bayes, entropia, três tipos de incerteza,
desigualdade de Jensen) estavam comprimidos num único capítulo, exigindo pré-requisitos
universitários de cálculo e probabilidade. Para que **qualquer estudante do Ensino Médio** atravesse
toda a curva do conhecimento sem perder uma única ideia ou resultado da pesquisa real, a obra foi
desdobrada em **21 capítulos agrupados em 5 partes**, seguindo a arquitetura em 3 degraus:

1. **Degrau 1 — o mundo real e o paradoxo (nível Ensino Médio):** todo capítulo começa com uma
   história concreta, contagens em frequências naturais ("em cada 100 pessoas...") e matemática
   visual (tabelas, árvores de possibilidades, gráficos de funções do 1º e 2º grau). O cálculo
   diferencial é substituído por intuição geométrica (curvatura e áreas) antes da fórmula geral.
2. **Degrau 2 — a regra do jogo (formalização acessível):** definições formais, teoremas e condições
   de identificação, traduzindo cada símbolo em palavras simples antes da equação.
3. **Degrau 3 — mão na massa (laboratório e exercícios):** simulações passo a passo em Rust/Python
   com semente fixa, onde o aluno vê o teorema acontecer rodando dados, seguidas de exercícios de
   *Conceito*, *Conta* e *Laboratório*.

### 9.2 Mapa geral

| Parte | Capítulos | Objetivo pedagógico |
|---|---|---|
| **I. A arte de mirar no escuro** | 1 a 4 | Separar Mundo, Modelo e Agente; dominar árvores de contagem (Bayes) e diferenciar sorte, ignorância e ponto cego |
| **II. As armadilhas do mundo e dos dados** | 5 a 8 | Por que a média mente (curvas e Jensen), como amostras gigantes enganam (Meng) e como medir quem prevê bem (Brier) |
| **III. Causa, tempo e sobrevivência** | 9 a 11 | Subir a escada de causa e efeito (Pearl/ICP), juros compostos com sorteio (Kelly, não-ergodicidade) e Goodhart |
| **IV. Construindo a máquina auditável** | 12 a 15 | Transformar ciência em engenharia: hipergrafo e 5 lentes, agentes $F_1$–$F_8$, banca $A_1$–$A_8$, fichas, cofre e hash |
| **V. A máquina no mundo real** | 16 a 21 | Os testes reais da pesquisa, de $H_0$ a $H_3$, até a auditoria final e o projeto do aluno |

### 9.3 Parte I — A arte de mirar no escuro

**Capítulo 1. O alvo invisível: o que é a Estocologia?** Do *stókhos* de Jacob Bernoulli (1713) à
decisão antes do resultado; a costura entre filosofia da dúvida (Hume, Popper, Lakatos — quando
consertar uma ideia é progresso como descobrir Netuno e quando é desculpa como inventar Vulcano),
estatística e processos no tempo. *Laboratório:* simulador de sorteios com semente fixa e a razão de
dois computadores precisarem sortear os mesmos números.

**Capítulo 2. Mundo, Modelo e Agente: onde moram os erros?** A diferença entre o território, o mapa
de papel e o motorista. A tese da ponte epistêmica: os erros moram nas pontes ($L_1 \to L_2$,
$L_2 \to L_3$, $L_3 \to L_1$). Os três modos de Peirce (dedução, indução, abdução) e os três
sentidos de "viés" (do dado, da fórmula, da mente). *Caso:* a falácia do promotor no tribunal e o
teste cego dos 12 casos. *Laboratório:* `enum Camada` impedindo o programador de misturar camadas.

**Capítulo 3. Probabilidade sem mistério: apostas, árvores e Bayes.** As 5 interpretações da
probabilidade (clássica, frequentista, propensão, lógica, subjetiva) ligadas a cada camada;
coerência e livro holandês; frequências naturais e árvores de 1.000 pessoas; a versão em chances
(razão de verossimilhança e *decibans* de Turing) e as três perguntas de Royall. *Laboratório:*
calculadora de árvores de 10.000 casos, dois exames independentes (10 dB) contra dependentes
(6,2 dB).

**Capítulo 4. Sorte, ignorância e o peru de Natal: os três tipos de incerteza.** O peru de Russell
alimentado 100 dias às 9h, com 99% de certeza na véspera do Natal que nunca esteve na lista dele.
Incerteza aleatória ($L_1$), epistêmica ($L_2$) e knightiana ($L_2/L_3$, onde mais dados só
aumentam a falsa confiança). Lei da variância total, máxima entropia (o dado honesto de Jaynes) e
matriz de Stirling. *Laboratório:* moeda cujo vício muda de 0,2 para 0,6 — o modelo de moeda fixa cobre a verdade em 12% dos passos (erra 88% das vezes) e o modelo com fator de esquecimento 0,99 cobre 93,98%, com intervalo 5,7 vezes mais largo. A cobertura medida por `cargo run --release --bin labs -- --cap 4` é 0,1199 e 0,9398; a edição anterior registrava 11% e 99%, e o Capítulo 4 declara a divergência com a grade de parâmetros que explica o efeito.

### 9.4 Parte II — As armadilhas do mundo e dos dados

**Capítulo 5. Quando a média mente: curvas e Jensen.** Média 6,2 com corte 6,0 não salva a turma; a
fila que dobra nos dias cheios e nunca fica negativa nos vazios. $\mathbb{E}[f(X)] \ge f(\mathbb{E}[X])$
em parábolas, e o pedágio de variância proporcional à curvatura. *Laboratório:* função da lacuna de
Jensen em curvas simples e no tempo de espera de uma fila.

**Capítulo 6. O filtro invisível: por que 2,4 milhões de votos valem 7 pessoas?** *Literary Digest*
1936: 2,4 milhões de respostas errando o presidente e 50 mil acertando. A identidade de Meng
$\text{Erro} = \text{Qualidade} \times \text{Quantidade} \times \text{Dificuldade}$; MCAR, MAR e
MNAR (Rubin); pseudorreplicação e a unidade independente $n_{\text{ef}}$ (1.000 alunos da mesma
escola, 2 milhões de ligações dos mesmos 81 dias). *Laboratório:* pesquisa viciada gigante errando
tanto quanto sortear 7 pessoas na rua.

**Capítulo 7. Caçando fantasmas no ruído: testes severos e falsas descobertas.** 100 pessoas
jogando moedas geram uma "mão quente" por acaso; Miller & Sanjurjo (2018): depois de uma cara em 3
lances a chance é $5/12$, não metade. Atalhos mentais contra ambiente (Kahneman e Gigerenzer); o
jardim dos caminhos que se bifurcam, severidade de Mayo, teorema de Ioannidis e os cinco erros —
Tipo I (alarme falso), II (cegueira), III (resposta certa para a pergunta errada), S (sinal trocado)
e M (exagero). *Laboratório:* 500 modelos em ruído puro e a contagem dos "descobridores" acidentais.

**Capítulo 8. O termômetro do adivinho: calibração, resolução e Brier.** Dois meteorologistas: o que
diz 20% todo dia e o que avisa na tempestade. Escores próprios (Brier) onde mentir sobre a própria
crença sempre piora a nota; decomposição de Murphy ($\text{REL} - \text{RES} + \text{UNC}$):
calibração se conserta com Platt, falta de resolução não se conserta. A regra do tamanho da amostra
($\text{ECE} \le 0{,}03$ é duro demais em teste pequeno e frouxo demais em 100 mil casos).
*Laboratório:* avaliador de 10 faixas do escore de Brier.

### 9.5 Parte III — Causa, tempo e sobrevivência

**Capítulo 9. A escada de causa e efeito: grafos, confundidores e ICP.** Cidades com mais hospitais
têm mais mortes? O paradoxo de Simpson nas cirurgias de pedra nos rins. Os 3 degraus de Pearl
(ver, agir, imaginar), grafos causais, confundidor contra colisor (viés de Berkson), a decomposição
$\text{diferença ingênua} = \text{efeito real} + \text{viés de seleção}$ e a predição causal
invariante de Peters. *Laboratório:* causa real e alarme falso em 3 ambientes, com o ICP descobrindo
quem é quem.

**Capítulo 10. O tempo e a ruína: a moeda de Peters, Kelly e caudas selvagens.** $+50\%$ contra
$-40\%$: média $+5\%$ por rodada, ruína certa em 100 rodadas ($1{,}5 \times 0{,}6 = 0{,}90$).
Média da multidão contra média no tempo (não-ergodicidade), arrasto da volatilidade
$\mu - \tfrac{1}{2}\sigma^2$ sem cálculo avançado, critério de Kelly com suavização de Laplace e o
vendedor de seguros que passa 5 anos sem acidentes. *Laboratório:* 10.000 seguradoras de catástrofe —
35,8% quebram com a média histórica simples, 0% com Laplace + Kelly.

**Capítulo 11. O preço do erro, Goodhart e quem paga a conta.** Quando pagar por seguro ou dique;
regra do custo/perda ($p > C/L$) e EVPI; os diques da Holanda de van Dantzig (Wald, Savage,
precaução); performatividade (Goodhart e crítica de Lucas); o trilema de Chouldechova e a
necessidade de declarar **quem paga pelo erro**; comunicação honesta com frequências naturais, HOPs
e cláusula de saída. *Laboratório:* calculadora de custo/perda no problema do navio de Bernoulli e
o trilema numa tabela de decisões.

### 9.6 Parte IV — Construindo a máquina auditável

**Capítulo 12. O mapa do conhecimento: o hipergrafo tipado e as cinco lentes.** Textos longos
escondem erros; o cartão (nó) obrigado a declarar identificador, camada, tipo de incerteza, fórmula
e código executável; arestas de patologia em vermelho; as cinco lentes (Fluxonomia, Crononomia,
Escalonomia, Morfonomia, Crisonomia). *Laboratório:* o primeiro mini-hipergrafo, com o validador
recusando cartões incompletos.

**Capítulo 13. A equipe de especialistas ($F_1$ a $F_8$), a banca adversarial e a IA sob controle.**
Como usar IA na ciência sem ser enganado: cada faixa cobre um ponto cego ($F_1$ simulação, $F_2$
grão e ausentes, $F_3$ relógio e deriva, $F_4$ estatística e SBC, $F_5$ privacidade e SHA-256, $F_6$
justiça, $F_7$ priores SHELF, $F_8$ comunicação e saída); o ciclo de seis estágios e a banca
$A_1$–$A_8$; Sistema 1 e Sistema 2 (*LLM-as-Jev*, árvore de sufixos determinística). *Laboratório:*
corretor de mudança de cenário de Saerens ($F_3$) e calibração por simulação (SBC).

**Capítulo 14. O contrato de ferro: fichas, proibições, invariantes e portões.** O `checklist` do
piloto como `ficha.yaml`; as 5 perguntas do cabeçalho e a trava do erro tipo III
(`pergunta_do_decisor` com "?"); as 4 proibições (nada de texto livre entre agentes, nada de conta de
cabeça por IA, ninguém aprova o próprio trabalho, proibido 0% e 100%); os invariantes $\Phi_1$–$\Phi_4$,
o funil de 4 filtros e os 5 portões. *Laboratório:* validador de `ficha.yaml` e teste de propriedades
que tenta quebrar uma função com milhares de valores sorteados.

**Capítulo 15. O cofre, a senha criptográfica e a auditoria contra si mesmo.** Pintar o alvo depois
do tiro; as três travas (pré-registro com hash SHA-256, contador de tentativas $K$, cofre fora do
tempo); o diário de Brier e Kelly e os 21 pareceres assinados; o espírito da auditoria de fundo
(10 testes duros contra a própria máquina). *Laboratório:* corrente encadeada por SHA-256 onde mudar
uma vírgula no passado invalida o relatório — exatamente o que `LivroRazao::verificar()` faz.

### 9.7 Parte V — A máquina no mundo real ($H_0$ a $H_3$ e balanço)

**Capítulo 16. $H_0$: o posto de saúde e os 2,65 milhões de chamadas do banco.** Posto com 12
atendimentos/hora e chegada média 8: dia constante dá 9,3 minutos de espera; dia alternando entre 5
e 11 dá 20,7 minutos e triplica quem espera mais de meia hora (de 8 para 25 em 100). O teste das
duas metades mostra que a unidade é o **dia** ($\pm 0{,}51$ min), não o paciente ($\pm 0{,}07$ min).
No caso real, 2,65 milhões de ligações valem **81 dias** de cofre: a fórmula de Erlang C promete 82%
de atendimento rápido, entrega 57% e falha em 26 de cada 100 dias; $+7\%$ de atendentes só nos
picos derruba a falha para menos de 4 em 100 dias. *Laboratório:* recursão de Lindley com semente
`1909`.

**Capítulo 17. $H_1$, parte 1: a ilusão de multiplicar por 3 na bolsa e o programa de emprego.**
Projeto A — Dow Jones alavancado $3\times$ (1885–2023): o crescimento real sobe de 5,1%/ano ($1\times$)
para 7,2%/ano ($2\times$) e **cai para 6,0%/ano ($3\times$)**; o crash de 1987 ($-22{,}6\%$) destrói
68% de quem estava $3\times$, e a riqueza mediana fica em $2{,}1\times$ enquanto a média enganosa diz
$19\times$. Projeto B — LaLonde (NSW contra CPS): diferença bruta de $-\$8.498$ contra o sorteio de
$+\$1.794$ (viés de seleção de $-\$10.292$); o pareamento acerta o número ($+9{,}6$ pp contra $+11{,}1$ pp)
mas é reprovado em severidade porque, no placebo de 1975, inventou efeito fantasma de $-12{,}3$ pp.
*Laboratório:* choque de 1987 em carteiras $1\times$, $2\times$ e $3\times$ e a decomposição do viés.

**Capítulo 18. $H_1$, parte 2: os incêndios dinamarqueses e a arte de reprovar o próprio modelo.**
36 meses sem incêndio gigante (50 a 150 milhões de coroas) levaram a seguradora a triplicar a aposta;
a cauda é selvagem ($\hat{\alpha} = 1{,}81$, variância quase infinita) e 36 meses limpos valem só
$2{,}2$ bits (Laplace: chance mensal ainda $1/38$). Em agosto de 1989, um incêndio de 152 milhões
quebra a imprudente. A banca testou o modelo do próprio agente no cofre fora do tempo: ele perdia
para a média histórica simples (habilidade de Brier $-0{,}015$) porque os incêndios cresciam 4,6% ao
ano — **a alegação da seguradora caiu, e o modelo do agente foi reprovado e proibido de decidir**.
*Laboratório:* estimador de Hill e a regra de Laplace de $1/38$.

**Capítulo 19. $H_2$: sete rodadas para encontrar causas verdadeiras na ciência.** A meta é achar
$\hat{S} \neq \emptyset$ em ao menos 3 áreas. A história honesta de 5 falhas seguidas (usar ano ou
região como ambiente invalida o teste; erro escondido no código na rodada $H_{2b}$; regra de
saturação na $H_{2d}$), cada uma gravada com SHA-256 no contador $K$. Os três sucessos de pé após a
auditoria de fundo: física (o túnel de luz, com os três LEDs `{red, green, blue}` emergindo de 1.024
combinações), farmacologia (FDA/hERG, 16 medidas por voluntário, os 3 canais iônicos certos e o
verapamil previsto no cofre com habilidade $+0{,}57$) e genética de levedura (40 genes, $p = 0{,}001$),
com a queda da educação (STAR) para $\hat{S} = \emptyset$ ao agrupar 11.598 alunos pelas 80 escolas
reais.

**Capítulo 20. $H_3$: o relógio ao vivo, o canário de vazamento e a polícia no cofre.** Seis fluxos
sob relógio estrito ($t_{\text{disp}}$) que proíbe espiar um segundo do futuro; o canário plantado de
propósito para provar que o alarme toca; a Lei de Goodhart medida por instrumento (4 vezes menor que
o chute inicial de 0,6); três cofres até acertar a calibração (Toronto queima, correção de Platt,
Rhode Island), chegando a $\text{ECE} = 0{,}0048$ com habilidade modesta de $+0{,}019$ por falta de
resolução, e a auditoria invertida de quem paga pelo erro nos três fluxos.

**Capítulo 21. As sete lições, os dez pontos da auditoria e o que falta fazer.** As 7 lições: a média
esconde o que importa; a unidade independente decide a margem; acertar o número não basta se o teste
não podia errar; reprovar o próprio modelo é vitória científica; a previsão viaja melhor que a lista
de causas, mas só a causa protege contra o inédito; calibração se conserta, resolução não; declarar
antes de rodar é o que separa ciência de trapaça. O balanço dos 10 pontos da auditoria de fundo. O
que continua em aberto, com honestidade: (1) no protocolo SHELF (`fundo/p12-shelf/`), o primeiro
especialista (E1) elicitou cinco domínios com habilidade medida no pré-teste (0,4240 a 0,9730) e releu os
21 pareceres, mas **faltam os outros dois especialistas por domínio e o de fora do programa** (e 5 de 33
domínios mudam de decisão dependendo da priori); (2) a resolução fina da genética fechou em código em
2026-10-16 (`labs --genetica`: 4 de 40 com uma linha, 1 de 40 com duas linhas), restando fora do programa
apenas eventual bancada úmida com segunda linhagem real; (3) **a voz de quem paga pelo erro**
(`governanca/GRUPOS.md`): nas 7 decisões que afetam pessoas reais, nenhum membro dos grupos afetados foi
ouvido (prazo 2026-12-15, `labs --escuta`) — assinar dá nome ao responsável, escutar é outra coisa; (4) **a
tese da ponte em escala** (`labs --ponte`, 0 de 12 casos com escolhedor e juiz de fora, prazo 2027-01-14); e
(5) os pesos congelados e os desfechos rotulados da ponte Jev-LLM (`labs --jev`). *Projeto final:*
roteiro guiado para o aluno escolher um problema da escola ou da cidade, preencher a `ficha.yaml`,
simular o mundo e defender a conclusão.

### 9.8 Apêndices pedagógicos

- **Apêndice A — dicionário visual da Estocologia:** todos os termos (de *abdução* e *Brier* a
  *severidade* e *unidade independente*) em uma frase simples, com exemplo do dia a dia.
- **Apêndice B — guia de bolso das fórmulas:** como ler cada símbolo
  ($\sum, \mathbb{E}, \operatorname{Var}, \mid, \operatorname{do}(x), \ln$) sem medo, com tradução
  para o português e para o código.
- **Apêndice C — guia de reprodução (`stochology-rs` e Python):** comandos exatos para rodar os 21
  laboratórios e conferir os hashes SHA-256 na própria máquina.
- **Apêndice D — bibliografia comentada para jovens cientistas:** de Jacob Bernoulli (1713) aos
  artigos de 2025/2026.

### 9.9 Nomes: pasta, crates e binários

| Antes | Depois |
| --- | --- |
| pasta `livro/` | pasta `book/` |
| `crates/estocologia-core` | `crates/stochology-core` (lib `stochology_core`) |
| `crates/estocologia-hypergraph` | `crates/stochology-hypergraph` |
| `crates/estocologia-audit` | `crates/stochology-audit` |
| `crates/estocologia-livro` | `crates/stochology-book` |
| `crates/portoes-cli` | `crates/stochology-gates` |
| crates do roteiro (`estocologia-schema`, `-s1`, `-s2`, `-h0` … `-h3`, `-icp`, `-drift`, `-fairness`, `-wasm`) | mesmo prefixo `stochology-`, e os dez nomes passaram a viver em 2026-10-16 |

**Os binários não mudaram de nome**: `labs` e `portoes`. Motivo: o livro, este plano e os
`portoes.py` das frentes em Python chamam por esses nomes, e o comando que o leitor digita fica o mesmo
em qualquer idioma de pasta. O arquivo do livro continua `estocologia-livro-didatico.md`: o título da
obra é Estocologia, e nome de arquivo de texto não é nome de crate.

### 9.10 Estado da edição e como montar

A edição é escrita em `book/`, um arquivo por capítulo, e só substitui
`estocologia-livro-didatico.md` quando os 21 capítulos estiverem prontos.

| Parte | Capítulos | Estado |
| --- | --- | --- |
| I. A arte de mirar no escuro | 1 a 4 | escritos, com laboratórios medidos |
| — | 21 capítulos | edição montada em `estocologia-livro-didatico.md` (5 927 linhas, 75 289 palavras, conferida caractere por caractere contra `book/` e contra a contagem declarada por `gerar.py --check`) |
| II. As armadilhas do mundo e dos dados | 5 a 8 | escritos, com laboratórios medidos |
| III. Causa, tempo e sobrevivência | 9 a 11 | escritos, com laboratórios medidos |
| IV. Construindo a máquina auditável | 12 a 15 | escritos, com laboratórios medidos |
| V. A máquina no mundo real | 16 a 21 | escritos, com laboratórios medidos (17 e 18 leem as bases de `dados-reais/`) |
| Apêndices A a D | — | escritos (A com 73 verbetes) |

```bash
python3 book/gerar.py --check    # exige, por capítulo: título, três degraus, exercícios e resumo
python3 book/gerar.py            # monta a edição quando os 21 existirem
cargo run --release --bin labs -- --todos    # roda os laboratórios com semente fixa
```

---

## 9.11 Horizontes de resolução da pergunta (eixo novo)

Segundo eixo da teoria, ortogonal ao empírico (H0 a H3): **quão fina é a pergunta do decisor**, e quanto custa
perguntar grosso. Escada R0 (agregada) → R1 (dia) → R2 (hora) → R3 (subpergunta), com instrumento próprio:

```bash
cargo run --release --bin labs -- --resolucao
```

Medido com o mesmo recurso total e braços pareados: a perda de resolução é **46,24 min** onde a demanda oscila
de 5 a 11, **5,97 min** onde oscila de 7 a 9 e **zero** onde não oscila. O tipo `Resolucao` e
`medir_perda_de_resolucao` vivem em `crates/stochology-core/src/resolucao.rs`; a especificação completa, as
cinco previsões falsificáveis e o estado da regra estão em `book/horizontes-de-resolucao-da-pergunta.md`. O
campo entrou na ficha como `Ficha.resolucao` e a regra virou $\Phi_5$, contado pelo binário de portões; as
previsões 3 e 5 foram medidas em 2026-10-16 (item 8 de `PROPOSTAS_DECLARADAS.md` §2): o limiar só se sustenta
na amplitude de 2,5 com o plano calibrado, e a transferência teve 0 violações e 3 contraexemplos. O inventário
completo do que
segue como proposta declarada — teoria, método, pesquisa em aberto e limites — está em
`PROPOSTAS_DECLARADAS.md`.

---

## 9.12 Regra dos arquivos novos

Arquivo novo de trabalho — cópia intermediária de laboratório, script de uma vez, rascunho — é **apagado depois
da integração**. Fica no repositório só artefato com função contínua (`book/gerar.py`, `book/conferir_labs.py`,
`book/conferir_livro.py`, os crates) ou registro (capítulos, declarações, manifestos `python-removido.txt` e
`nao-significante-removido.txt`, relatórios). A última aplicação da regra apagou `book/labs/` (17 cópias
intermediárias, já integradas em `crates/stochology-book/src/capNN.rs`) e os scripts de uma vez usados na
migração de nomes, na remoção do Python e na atualização do Apêndice D.

---

## 9.13 Documentos integrados (arquitetura, escala, fundamentação e artigo)

Os documentos externos do projeto foram baixados para consulta em 2026-10-16 e **integrados aqui**, como manda a
regra dos arquivos novos. O que cada um virou:

| Documento baixado | O que foi integrado | Onde |
| --- | --- | --- |
| *Arquitetura* | camadas de dado (hipergrafo, colunar, índice vetorial), runtime isolado por semente, agentes por faixa, barramento de fichas | seção 3, 4 e 7 deste plano; `crates/stochology-hypergraph` e `crates/stochology-core` |
| *Plano escalável de pesquisa* (5 eixos) | eixos de maturidade, funil com poda por EVSI, escala horizontal, governança anti-sobreajuste e infraestrutura | tabela abaixo |
| *Fundamentação: três camadas e inferência* | taxonomia de incerteza e as três camadas | Capítulos 2 e 4 do livro; seções 1 e 3 deste plano |
| Artigo *Velhas Fronteiras, Novos Horizontes* | já era o texto canônico de `artigo/README.md` (mais novo e mais longo); a cópia baixada era versão anterior | `artigo/` |
| *Integração de Modelos Jev à Estocologia* (conversa do Gemini) | a conferência de cada alegação contra o código: o que já existe com arquivo e comando, as divergências — a numeração do roteiro antigo ($M_{15}$ e $MB_1$–$MB_8$) contra a vigente ($\mathcal{M}$ e $A_1$–$A_8$), quatro filtros contra cinco, $\Phi_1$–$\Phi_4$ contra $\Phi_1$–$\Phi_5$, o *LLM-as-Jev* — e a ponte Jev-LLM como proposta declarada | `INTEGRACAO_JEV.md`; item 13 de `PROPOSTAS_DECLARADAS.md` |

| Eixo de escala | O que pede | Estado no repositório |
| --- | --- | --- |
| 1. Maturidade em horizontes ($H_0 \to H_3$) | portões de saída por horizonte | cumprido no registro empírico; a esteira virou o binário `portoes` |
| 2. Funil com poda por EVSI | não gastar computação sem valor de informação | **feito em código**: `crates/stochology-core/src/funil.rs` (878 linhas, 17 testes) tem a poda por valor da informação e o contador $K$, verificado por `cargo test --workspace` e `labs --s1`. A linha dizia "declarado, não implementado" e a **declaração de divergência** fica aqui, com o valor antigo e o novo, porque o critério saiu do Capítulo 11 para o código depois da redação |
| 3. Escala horizontal `plug-and-play` (F7) | domínio novo entra por ficha padronizada | parcial: a ficha existe e é validada; o motor de domínio não existe |
| 4. Governança anti-sobreajuste | declarar tentativas e corrigir multiplicidade | cumprido em registro ($K = 38$; Bonferroni aplicado nas 4 áreas do $H_{2g}$) |
| 5. Infraestrutura e custos | ambiente versionado, execução repetível | cumprido em Rust: `cargo test --workspace`, semente fixa e `RESULTADOS_RUST.md`; a parte Python foi aposentada |

---

## 10. Fichas, Proibições e Contrato de Ferro

- **As 5 perguntas do cabeçalho:** processo gerador; não-linearidade; unidade independente; física do
  ruído (Itô contra Stratonovich); escada causal.
- **Trava de tipo III:** `pergunta_do_decisor` precisa terminar em "?" e citar literalmente a ação do
  pacote (validado em `crates/stochology-core/src/invariantes.rs`).
- **4 proibições:** texto livre entre agentes; conta de cabeça por IA; autoaprovação; probabilidades
  fora de $[0{,}01; 0{,}99]$.
- **4 invariantes e 7 portões:** verificados pelo binário `portoes`, com código de saída 0/1.

---

## 11. Ponte com o Registro Empírico (consolidado em Rust)

| Frente do plano | Onde vive hoje |
|---|---|
| $H_0$ simulado e real | `REGISTRO_EMPIRICO.md`; medido por `labs --cap 16` e `labs --banco` |
| $H_1$ simulado e real | `REGISTRO_EMPIRICO.md`; medido por `labs --cap 17` e `labs --cap 18` sobre `dados-reais/` |
| $H_2$ e reexecuções $K=1..3$ | `REGISTRO_EMPIRICO.md`; o método roda simulado em `labs --cap 19` |
| $H_3$ (malha sob relógio) | `REGISTRO_EMPIRICO.md`; o mecanismo roda simulado em `labs --cap 20` |
| Dados e somas | `dados-reais/` com `FONTES.md` e `SHA256SUMS` |
| Pareceres de risco $F_5$ | `governanca/` |
| Auditoria de fundo (10 pontos) | `fundo/p01-tese/`, `p02-independencia/`, `p03-multiplicidade/`, `p05-prioris/`, `p06-laco/`, `p07-icp/`, `p08-calibracao/`, `p12-shelf/`; os pontos 9 e 10 (escala e dados), com as extensões de evento raro e de saúde, saíram da árvore em 2026-10-16 por não sustentarem resultado central, e voltam pelo commit `1bfec905` |
| Livro didático (21 capítulos em 5 partes, com os apêndices A a D) | `estocologia-livro-didatico.md`, escrito em `book/` |
| Artigo | `artigo/` |

Comandos de reprodução do registro empírico (Python 3.13, semente fixa). **O código Python foi aposentado em
2026-10-16** e vive no commit `1bfec905`: rode `git checkout 1bfec905 -- <pasta>` antes de usar os comandos
abaixo; veja `DECLARACAO_REMOVIDO_PYTHON.md`.

```bash
cd h1 && python3 rodar.py && python3 portoes.py --reproducao && python3 grafo.py
cd h3 && python3 rodar.py && python3 grafo.py && python3 figura.py && python3 portoes.py --reproducao
```

Medição feita antes da aposentadoria: `cd h1 && python3 -m pytest -q` devolveu `24 passed in 17.51s`. O livro em
`estocologia-livro-didatico.md` já é a edição de 21 capítulos: a seção 9 deste plano deixou de ser
especificação e passou a descrever o que está no disco. A edição de 14 capítulos vive no histórico, no commit
`1bfec905`.

---

## 12. Como Verificar Tudo

```bash
# Portões determinísticos em máquina limpa
cargo test --workspace
cargo run --release --bin portoes -- --horizonte todos --reproducao

# Um portão precisa saber reprovar
cargo run --release --bin portoes -- --horizonte h3 --defeito h3.voos23   # exit 1

# Registro empírico (Python), frente por frente
git checkout 1bfec905 -- h1/          # o codigo Python vive no historico
cd h1 && python3 -m pytest -q
cd h1 && python3 portoes.py --reproducao
```

Estado medido nesta sessão (2026-10-17): `cargo test --workspace` passa com **350 testes nos dezesseis crates**,
nenhum reprovado; os **vinte e quatro instrumentos** do `labs` rodam com código 0; `--horizonte todos --reproducao`
devolve "7 portoes ok" com segunda execução idêntica e raiz de Merkle
`9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81`; e a prova negativa
`--horizonte h3 --defeito h3.voos23` sai com 1, reprovando no G6.
