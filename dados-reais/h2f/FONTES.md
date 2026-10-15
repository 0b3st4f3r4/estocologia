# H2f: Kemmeren et al. (2014), deleções de genes em levedura (conjunto canônico de validação do ICP)

Baixado em 2026-10-15. Só o esquema foi inspecionado (dimensões, nomes, alvos, placas, NaN, faixa global).
Nenhum Y foi escolhido nem resumido por ambiente, e nenhum modelo foi ajustado.

## Origem

- Arquivo original: `Kemmeren.RData` (86 328 652 bytes), copiado de
  https://raw.githubusercontent.com/longjp/causal-bias-code/HEAD/data/Kemmeren.RData
  (commit 6e72635ae7ed2bfdd5eb4d123472471907067821, 2021-10-04). O README desse repositório diz que o arquivo veio de
  http://deleteome.holstegelab.nl/downloads_causal_inference.php, a página de download que o Holstege lab montou para
  Meinshausen, Hauser, Mooij, Peters, Versteeg & Bühlmann (2016), "Methods for causal inference from gene perturbation
  experiments and validation", PNAS 113(27):7361–7368. Os servidores originais (deleteome.holstegelab.nl, GEO
  GSE42527/GSE42526, EBI) estão bloqueados pela rede deste ambiente, e por isso não foi possível comparar com eles.
- Dados primários: Kemmeren P. et al. (2014), "Large-scale genetic perturbations reveal regulatory networks and an
  abundance of gene-specific repressors", Cell 157(3):740–752. GEO: GSE42527 (mutantes) e GSE42526 (selvagem).
- Licença: o repositório espelho não declara licença. Os dados do GEO são públicos e não têm restrição de uso declarada;
  a citação de Kemmeren et al. (2014) é a norma. Use os dados apenas para pesquisa e com citação.

## Arquivos

| Arquivo | Conteúdo |
|---|---|
| `Kemmeren.RData` | Original, sem alterações. Objeto `data` (lista): `obs` 262×6170, `int` 1479×6170, `plateobs` (262, lógico), `plateint` (1479, lógico), `intpos` (1479, índice 1-based da coluna do gene deletado), `p`=6170, `nObs`=262, `nInt`=1479, `genenames` (6170 ORFs), `mutantnames` (1479 ORFs). |
| `kemmeren2014_obs_int.parquet` | Os mesmos números, sem subconjunto de genes nem de amostras: 1741 linhas (262 obs + 1479 int) × (5 metadados + 6170 genes). Metadados: `sample_id`, `kind` (observational/interventional), `deleted_gene` (ORF; nulo nas obs), `deleted_gene_col_index_1based` (= `intpos`), `plate` (= `plateobs`/`plateint`). As colunas dos genes têm os nomes sistemáticos dos ORFs (por exemplo YAL002W). Os valores são float64 e não foram transformados. Conversão feita com o pacote Python `rdata` 1.1.0. |

## Desenho

- Uma linha corresponde a um perfil de expressão de microarranjo (log2 da razão contra um pool de referência de
  RNA selvagem; os nomes das obs terminam em `-vs-wt`). Faixa global: obs de −3,36 a 2,83; int de −6,87 a 6,11. Não há NaN.
- **Observacional (262):** culturas selvagens (BY4742) cultivadas como controle ao longo do projeto. Os nomes
  identificam lote ou experimentador: 96 `wt-htp*` (high-throughput); o resto tem prefixos de pessoas (tl, mmk, ea, sv, ic…).
  Alguns nomes sugerem condições diferentes (`wt-330rpm-1`, `wt-16400rpm`, `wt_m15pl-mutC`).
- **Intervencional (1479):** 1479 mutantes de deleção de gene único, cada um com um alvo conhecido. O gene deletado é
  sempre uma das 6170 colunas (conferido: `genenames[intpos] == mutantnames` nos 1479). **Cada mutante tem uma única
  linha**: cada alvo é um ambiente de n=1. Para chegar a centenas de linhas por ambiente é preciso agrupar os mutantes.
- `plate` (lógico, sem documentação no arquivo): obs False 160 / True 102; int False 633 / True 846. Nas obs, as 96
  `wt-htp` caem todas em True (mais 6 outras), o que sugere um indicador de lote ou formato de cultivo. Trate-o como
  covariável técnica medida.

## Como definir ≥3 ambientes (contagens; regras que não usam Y)

- E0 = observacional (262).
- Opção A (por `plate`): E1 = mutantes com plate False (633), E2 = mutantes com plate True (846). Aqui o ambiente se
  confunde com o lote técnico. Como o lote é medido, inclua `plate` como covariável ou confira dentro de cada placa.
- Opção B (cromossomo do gene deletado, a 2ª letra do ORF): A–D 352, E–J 415, K–P 712. A regra é determinística e não
  olha Y. Contagens por letra: A25 B93 C30 D204 E65 F28 G140 H61 I51 J70 K81 L139 M142 N95 O122 P133.
- Para qualquer Y escolhido: retire o mutante em que o próprio Y foi deletado (no máximo 1 linha), porque aquela linha é
  intervenção em Y.

## Escolha de Y e dos preditores (a pré-registrar)

- Y deve ser contínuo; qualquer coluna de gene serve. Para evitar intervenção direta em Y, prefira um dos 4691 genes
  que não foram deletados em nenhum mutante.
- Os candidatos X (3 a 10) deveriam ficar entre os 1479 genes deletados, para que as intervenções sobre eles sejam
  conhecidas. Fixe a regra antes de ver os resultados. Exemplos: pares regulador→alvo da literatura (YEASTRACT etc.), ou
  o procedimento do PNAS, que pré-seleciona os candidatos com dados observacionais.
- Verdade de referência usada na literatura: Meinshausen et al. (2016) validam pares X→Y por **strong intervention
  effects (SIE)** em dados retidos. Um par conta como SIE quando, no mutante ΔX, a expressão de Y fica fora da faixa vista
  em todos os outros dados (observacionais e intervencionais), desde que a deleção de X tenha "funcionado" (X com a menor
  expressão no próprio mutante). O ICP e outros métodos são comparados pela fração de SIEs entre os pares mais bem
  ranqueados. Os números exatos ficam na Fig. 3 do artigo; não foram conferidos aqui. `longjp/causal-bias-code`
  (`analysis/0-data-setup.R`) reimplementa esse critério.

## Limitações (importantes para "intervenções limpas")

1. Os ambientes agrupados reúnem alvos diferentes. Uma deleção de um gene fora dos candidatos que regula Y diretamente
   é uma intervenção não medida que age sobre Y, e isso viola o critério (5). Numa rede de ~6000 genes isso é provável.
   O ICP supõe que nada intervém em Y; aqui isso vale só em aproximação.
2. Há variáveis latentes: crescimento lento e ciclo celular. Muitas deleções mudam a taxa de crescimento, que altera
   centenas de genes ao mesmo tempo, Y possivelmente entre eles (veja a "slow growth signature" no deleteome). Essa causa
   comum não está medida.
3. As obs não são i.i.d. no sentido estrito: são lotes de vários experimentadores ao longo de anos, alguns com
   condições diferentes (rpm). Os mutantes também foram medidos em lotes, e só o indicador `plate` está disponível.
4. Os valores já são log-razões normalizadas contra a referência, não intensidades absolutas.
5. Faltam efeitos de dose: as deleções são nocautes completos, e o nível de X no mutante fica perto do ruído de fundo.
