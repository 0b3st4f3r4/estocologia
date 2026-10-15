# Declaração do fundo/p09 (escala e dados), escrita antes de rodar o funil

Data: 2026-10-15. Pasta: `fundo/p09-escala-dados/`, cópia do `h2g/` sem a pasta `saida/` (o código do funil não
muda). O hash desta declaração fica em `DECLARACAO.sha256`. Qualquer mudança depois do hash entra como nova
declaração e conta no K.

## O que já foi visto antes desta declaração (para ninguém achar que é cego)

- **indo_rct.** Contei os pacientes por centro e, por descuido, a tabela cruzada centro × braço × desfecho, que
  inclui o centro do cofre (Michigan). Vi: placebo 25 de 87 e indometacina 11 de 77 em Michigan; 26 de 207 e 15 de
  206 em Indiana. O cofre deste domínio, portanto, **não é cego** para o autor. A especificação abaixo segue a
  priori da literatura anterior ao ensaio e não foi mexida por isso, mas o leitor deve descontar.
- **OWID.** Só contei linhas e países por continente depois do recorte (3 167 países-mês; América do Sul com 10
  países e 240 linhas). Não olhei mortes nem a relação com a vacinação.
- **Rdatasets.** Só o catálogo de metadados (3 648 conjuntos; 837 passam o filtro de metadados) e o log do
  download (817 baixados, 20 falharam por nome com espaço). Nenhum CSV foi aberto por mim.
- O F1 do funil não toca nos dados; as prioris ajustadas aos quartis foram calculadas (Indo: média −0,69, dp 0,49
  em log-razão de chances; OWID: média −1,53, dp 1,11).

## Parte 1. Dois domínios novos de saúde, com o funil do H2g sem mudança

Arquivos congelados (SHA-256):

```
feccf561de081bf8a7d2b3857237800025cb188ae274d40fab6138112a62e7a1  dominios/saude_indometacina_ercp/pacote.yaml
6849b031ec8a6366cd31fc6bf16ed60560680ec2774fdb237c763f236b044b33  dominios/saude_indometacina_ercp/preparo.py
b1829124559b021e1b2827969ac2286675a110f086cec9b0730c48c725fc65ed  dominios/saude_covid_vacina_paises/pacote.yaml
23ed3e38264177ad091fcb585634339e9167b186cb64f0a8dd0d62f72d91c044  dominios/saude_covid_vacina_paises/preparo.py
f9df4fc224dbd40e69508ca0003f2ec96545a84d444f2bef4c94b31534423fad  medicaldata_indo_rct.csv (dados-reais/fundo/p09-escala-dados)
8473d0f0fdf962e1ffbd5b85b18726fc96a49bab109e271186c339725a12b10c  owid-covid-data.csv (commit owid dc00e729…)
```

### saude_indometacina_ercp (saúde, ensaio sorteado de 2009–2011)

- **Alegação.** "Uma dose de indometacina retal corta a pancreatite pós-CPRE do mesmo jeito em qualquer hospital:
  o resultado de um centro vale para o outro."
- **Ambientes.** Centro: Indiana (413), "outros" (Kentucky 22 + Case Western 3, juntos por regra). **Cofre:**
  Michigan (164), nunca usado no ajuste (mas visto na tabela cruzada, ver acima).
- **Intervenção conhecida.** O braço é sorteado; a chave é `indometacina`.
- **Critério.** O domínio **confirma a alegação** se (a) o funil chega ao F4, (b) Ŝ do ICP contém `indometacina`,
  (c) a probabilidade posterior de β < 0 passa de 0,95 e (d) a habilidade do modelo ajustado no cofre é positiva.
  **Falha** se parar no F1–F3, se Ŝ não contiver a chave ou se a habilidade no cofre for ≤ 0. Ŝ = ∅ com β < 0
  conta como "efeito sem invariância certificada", não como confirmação.
- **Previsão do autor.** Com só dois ambientes de treino, um com 25 pacientes, espero pouco poder: Ŝ = ∅ e
  habilidade pequena no cofre, porque o evento é raro (≈ 13%). Isso é o ponto 10 (eventos raros): vou medir a
  resolução e a confiabilidade (decomposição de Murphy do escore de Brier) das previsões do diário no cofre.

### saude_covid_vacina_paises (saúde, dados de 2021–2022)

- **Alegação.** "Vacinar mais derruba as mortes por COVID do mês seguinte do mesmo jeito em qualquer continente."
- **Ambientes.** Continente (África, Ásia, Europa, América do Norte, Oceania). **Cofre:** América do Sul.
- **Intervenção.** Não sorteada (observacional, ecológica); a cobertura varia por acesso às vacinas.
- **Critério.** Mesmo de cima, com a chave `vacinados`.
- **Previsão do autor.** O continente provavelmente age em Y (notificação), e então o ICP rejeita todos os
  conjuntos ou dá Ŝ = ∅; o sinal da inclinação ecológica é incerto.

Os dois rodam com `escala/um.py`, sementes 2027 (covid) e 2127 (indo), cada um com armazém, barramento e diário
próprios. A decomposição de Murphy usa 10 caixas de probabilidade de mesma largura.

## Parte 2. Escala do funil sobre o catálogo do Rdatasets

Arquivos congelados (SHA-256):

```
ec314520aeef4a3bc71e8a48eb2e4394e4a0ab26e384540dbc493d30e74f6890  escala/baixar.py
eec1f51df01053be64dfe6326320734883bd02c3be3d4669d1a005f3ffce9929  escala/gerar.py
75e0fa7ae2974e7cdcf64adf71c878df59777a05ac00910b001d7aa34375ae47  escala/lote.py
0a3c85ec6b49e03ae66e353716f1936020ef109dc9d0d5dddcc96deb393918e8  escala/um.py
2931aec87285e5faf127e06863816cb8a050904ebaf96fd110d741290229631a  grafo.py
1e9ef466be15c70e8779d9324f74f7691c8beba68af40a8c93f2f6f2a35c9d9d  rdatasets/catalogo.csv (commit 1dcc2bf5…)
```

### Regra de seleção R1 (implementada em `escala/gerar.py`)

1. **Metadados.** 300 ≤ linhas ≤ 200 000, pelo menos 3 colunas numéricas e pelo menos 1 fator, texto ou binária.
   CSV baixado do commit fixo; falha de download descarta o conjunto.
2. **Nomes.** Saem as colunas cujo nome tem como palavra `rownames`, `id`, `code`, `index`, `idx`, `row`, `obs`,
   `name` ou `x` (no plural também). Precisa sobrar pelo menos 4 colunas.
3. **Ambiente.** A primeira coluna, na ordem do arquivo, com 3 a 8 valores distintos, cada um com pelo menos 30
   linhas, e que não seja decimal.
4. **Alvo.** A primeira coluna numérica (fora o ambiente) com no máximo 20% de faltantes, pelo menos 10 valores
   distintos e que não seja monótona na ordem das linhas.
5. **Candidatos.** As colunas numéricas seguintes na ordem do arquivo (com no máximo 20% de faltantes, dp > 0,
   pelo menos 2 valores, |r| com o alvo < 0,999, que é só checagem de cópia), até 6. A chave é a primeira.
   Precisa de pelo menos 2.
6. **Casos completos.** Níveis com menos de 30 linhas saem; precisa sobrar 3 ambientes e 300 linhas.
7. **Cofre.** O último nível em ordem alfabética.
8. **Duplicatas.** Sai o conjunto cujo alvo (valores ordenados) repete o de um conjunto anterior no catálogo.

Pacote mecânico: família gaussiana; y e chave padronizados pelo treino; priori N(0, 1) na inclinação padronizada;
DAG em estrela (cada candidato → y); ajuste = os outros candidatos; decisão sintética (adotar se θ > 0, custo 0,
custo da pesquisa 0,01). **Esses pacotes não têm semântica de domínio**: nenhum humano leu a documentação.

### Medidas

- Para cada domínio: tempo de parede e de CPU, pico de RSS, etapa em que parou, erro (tipo) e prazo estourado
  (900 s). 4 processos em paralelo (a máquina tem 4 núcleos e 15 GB).
- **cProfile e tracemalloc** num lote de perfil: os 30 primeiros domínios gerados em ordem alfabética, em série.
- **Hipergrafo.** `grafo.py` sobre os 18 domínios do H2g (resultados copiados de `h2g/saida`), os 2 de saúde e
  os automáticos; nós, arestas de árvore, pontes e hiperarestas, e o tempo e a memória do próprio `grafo.py`.
- **Nós honestos.** Conto em três classes: (i) nós de conceito (domínio, camadas, variáveis, ambientes, filtros,
  decisão) dos domínios com pacote curado; (ii) os mesmos nos domínios automáticos (têm dado e artefato
  simulável, mas não têm conhecimento de domínio); (iii) nós de hipótese do ICP (um por subconjunto testado,
  2^k por domínio), que crescem de forma combinatória e eu não conto como conhecimento novo.
- **Onde quebra.** Cada passo da R1 que descarta, cada exceção no funil e cada recurso que cresce mais que
  linearmente com o número de domínios.

### Critério da Parte 2

Não há hipótese a confirmar; é medição. Declaro só o que conta como resposta: o número de nós da classe (i) + (ii)
alcançado com dado real, o número total, o custo por domínio e a extrapolação linear (marcada como tal) do tempo
para 10⁴, 10⁵ e 10⁶ nós, com o gargalo de cada ordem. Se o funil quebrar em mais da metade dos domínios gerados,
isso é o resultado principal, e não será "consertado" nesta rodada.
