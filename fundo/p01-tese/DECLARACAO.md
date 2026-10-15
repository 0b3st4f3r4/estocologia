# Declaração do piloto P01: a tese central da Estocologia

Declaração escrita em 2026-10-15, depois de montar o banco de casos e o gabarito e antes de qualquer parecer dos
analistas. O SHA-256 deste arquivo está em `DECLARACAO.sha256`. Qualquer mudança depois disso entra como nova
declaração e conta no K.

## 1. Hipótese

A tese central diz que a organização em camadas, fichas e portões da Estocologia (camadas Mundo/Modelo/Agente,
fichas YAML, invariantes e portões de aprovação, como descritos em AGENTS.md, PLAN.md e na Arquitetura) pega erros
de julgamento que uma análise estatística comum e competente deixa passar, sem dar mais alarmes falsos.

O piloto testa uma versão estreita e operacional da tese: diante de alegações de clientes com dados, o braço que
segue a arquitetura aponta mais defeitos plantados que o braço de análise comum, sem acusar mais casos limpos.

## 2. Material

- Banco de 12 casos em `/mnt/project-files/estocologia/fundo/p01-tese/casos/caso_01` a `caso_12` (hashes em
  `CASOS.sha256`). Cada caso tem `alegacao.md` (a alegação do cliente com a pergunta de decisão), um ou mais CSV e
  `dicionario.md`.
- Oito casos têm um defeito plantado cada, sorteado com semente fixa da lista externa de Makin & Orban de Xivry
  (2019); quatro são limpos (a alegação é correta e bem sustentada pelos dados). A ordem foi embaralhada com
  semente fixa; o número do caso não diz nada.
- Três casos usam dados reais com resposta conhecida já publicada; os demais são simulados com processo gerador e
  semente declarados.
- O gabarito (`gabarito.yaml`) e o gerador (`gerar_casos.py`) ficam fora do repositório, na pasta `p01-gabarito`
  do coordenador. No repositório ficam só os hashes: `GABARITO.sha256` e `GERADORES.sha256`.

Lista externa usada: Makin, T. R. & Orban de Xivry, J.-J. (2019), "Ten common statistical mistakes to watch out
for when writing or reviewing a manuscript", *eLife* 8:e48175, doi:10.7554/eLife.48175. Conferida em
https://pmc.ncbi.nlm.nih.gov/articles/PMC6785265 (a página elifesciences.org não carregou sem JavaScript). Os dez
erros, na ordem e com os títulos do artigo:

1. Absence of an adequate control condition/group
2. Interpreting comparisons between two effects without directly comparing them
3. Inflating the units of analysis
4. Spurious correlations
5. Use of small samples
6. Circular analysis
7. Flexibility of analysis: p-hacking
8. Failing to correct for multiple comparisons
9. Over-interpreting non-significant results
10. Correlation and causation

## 3. Os três papéis

- **Plantador** (já feito): montou os casos e o gabarito. Não participa da análise nem do julgamento.
- **Braço A (arquitetura):** analisa os 12 casos seguindo a arquitetura da Estocologia: monta as camadas, preenche as
  fichas e passa pelos portões antes de dar o parecer. Pode ler AGENTS.md, PLAN.md e a Arquitetura.
- **Braço B (análise comum):** analisa os mesmos 12 casos como um estatístico competente faria, sem a arquitetura:
  não recebe AGENTS.md, PLAN.md, a Arquitetura nem nenhum texto da Estocologia. A instrução é "analise os dados e
  responda à pergunta de decisão do cliente com o rigor de um bom estatístico".
- **Juiz:** recebe os 24 pareceres sem a identificação do braço e o gabarito, e codifica cada parecer.

Regras comuns aos dois braços:

- Os dois recebem só as pastas `caso_XX`. Nenhum braço recebe esta declaração, o README desta pasta, o gabarito,
  o gerador ou a informação de quantos casos têm defeito. A instrução não diz que há defeitos plantados; diz só que
  são pedidos de clientes.
- Mesmo modelo, mesmas ferramentas (Python com pandas, scipy e statsmodels), cada caso analisado de forma
  independente (sem consultar os outros casos), e orçamento parecido de tempo e de tokens.
- Cada parecer termina com um bloco fixo:

  ```
  VEREDITO: SUSTENTADA | NAO_SUSTENTADA
  MOTIVO PRINCIPAL: <uma a três frases>
  ```

  `SUSTENTADA` quer dizer que os dados sustentam a alegação e a decisão perguntada; `NAO_SUSTENTADA`, que não
  sustentam.

Cegamento do juiz: o juiz decide a nota lendo só o bloco final (veredito e motivo principal) e, se precisar, o texto
do parecer sem as fichas. O coordenador apaga desse material as menções ao braço e os termos próprios da arquitetura
(nomes de camadas, fichas, portões, invariantes) e rotula os dois pareceres de cada caso como X e Y por sorteio, com
semente fixa registrada antes de abrir os pareceres. O juiz pode ver o parecer completo só depois de dar a nota do
caso, e não pode mudá-la.

## 4. Codificação do juiz

Para cada parecer, o juiz aplica o critério escrito no gabarito para aquele caso:

- **Caso com defeito:** "apontou o defeito" = 1 se o veredito é `NAO_SUSTENTADA` (ou o parecer recusa a conclusão ou
  a decisão perguntada) **e** o motivo nomeia o mecanismo do defeito plantado, conforme o critério objetivo do
  gabarito. Um veredito `NAO_SUSTENTADA` por outro motivo, sem o mecanismo, vale 0.
- **Caso limpo:** "alarme falso" = 1 se o veredito é `NAO_SUSTENTADA`, ou se o parecer afirma um defeito que, segundo
  ele, invalida a conclusão ou a decisão. Ressalvas que não mudam o veredito não contam.

O juiz registra, para cada parecer, a nota (0/1) e uma frase citando o trecho do parecer em que se apoiou.

## 5. Métricas

- **Sensibilidade** de cada braço = defeitos apontados / 8.
- **Taxa de alarme falso** de cada braço = casos limpos acusados / 4.
- **Comparação caso a caso:** teste exato de McNemar (binomial bilateral sobre os pares discordantes) nos 8 casos com
  defeito; o mesmo teste nos 4 limpos, como descrição.
- Descritivo: tabela caso a caso (A, B), sem pesos.

## 6. Regra de decisão (fixada antes)

- **A tese é sustentada no piloto** se a sensibilidade de A for maior que a de B **e** a taxa de alarme falso de A
  não for maior que a de B.
- **A tese não é sustentada no piloto** em qualquer outro caso: sensibilidade de A igual ou menor que a de B
  (inclusive empate no teto, 8/8 contra 8/8), ou A com mais alarmes falsos que B.
- O p do McNemar é relatado, mas não muda a regra acima.

Poder: com 12 casos o piloto não tem poder para conclusão firme e isto fica escrito aqui. No McNemar exato com os 8
casos com defeito, o menor p bilateral possível com d pares discordantes, todos a favor de A, é 2·0,5^d: 0,0625 com
d=5, 0,031 com d=6, 0,016 com d=7 e 0,0078 com d=8 (calculado com `scipy.stats.binomtest`). Ou seja, só haveria
p < 0,05 se A acertasse e B errasse em pelo menos 6 dos 8 casos com defeito e não houvesse nenhum par discordante no
sentido contrário. Com 4 casos limpos, uma diferença de um alarme falso é 25 pontos percentuais e nada se pode
concluir sobre a taxa de alarme. Um resultado favorável justifica um estudo maior; não confirma a tese. Um resultado
desfavorável conta contra a tese e deve ser relatado como tal.

## 7. O que conta como falha

- Do piloto como teste: se um braço não entregar o bloco de veredito em algum caso, esse caso conta como não
  apontado (defeito) ou como não acusado (limpo) para aquele braço, e isso é relatado.
- Do banco de casos: nenhum caso é retirado depois de ver os pareceres. Se um parecer mostrar que um caso limpo tem
  um problema real não plantado, ou que um caso com defeito tem um segundo defeito, isso é registrado como falha do
  plantio; o resultado principal continua sendo o dos 12 casos como declarados, e o resultado sem o caso afetado é
  relatado ao lado.
- Da tese: o critério da seção 6 não satisfeito.

## 8. Limitações declaradas

- Os três papéis (plantador, analistas e juiz) são modelos de linguagem da mesma família. Erros comuns ao modelo
  aparecem nos dois braços e o juiz pode partilhar o viés de leitura dos analistas.
- O plantador conhece o catálogo da Estocologia e a lista de Makin. Pode ter escrito casos que a arquitetura pega com
  mais facilidade, mesmo sem querer. Os erros vieram de uma lista externa, sorteados, para limitar esse efeito, mas
  o texto das alegações e dos dicionários é do plantador.
- Os erros 1 (falta de controle adequado) e 6 (análise circular) ficaram fora do sorteio.
- Os três casos com dados reais vêm de conjuntos famosos. Mesmo com colunas renomeadas e descrição neutra, um modelo
  pode reconhecê-los e lembrar a resposta publicada, o que favorece os dois braços igualmente.
- Os casos simulados com defeito foram gerados procurando, a partir de uma semente-base, a primeira semente em que o
  defeito aparece (por exemplo, um teste por animal significativo com o teste por baia não significativo). Isso é
  parte do plantio e está declarado no gabarito.
- Doze casos são poucos e as métricas são discretas (passos de 12,5 e 25 pontos percentuais).
- O braço B é "análise estatística comum e competente" feita por um modelo instruído para isso; não é um estatístico
  humano.
