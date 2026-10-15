# Declaração H2e: domínios novos com intervenção em X contínuo

Escrita em 2026-10-15, **antes** de qualquer execução do código do H2e, em simulação ou em dados reais. O SHA-256
deste arquivo e a hora UTC do registro ficam em `DECLARACAO_H2e.sha256`.

Todo o trabalho fica em `h2e/`, que é uma cópia de `h2d/`. As pastas `h2/`, `h2b/`, `h2c/`, `h2d/` e `h3/` não são
tocadas. Em `dados-reais/h2e/` só foi corrigida a linha do SHA256SUMS que o FONTES.md tinha invalidado.

## O pedido e o que já se sabe

- **O pedido.** sil disse: "Buscar dados novos: … intervenções sobre um X contínuo". O portão G3 continua o do
  PLAN, sem mudança: Ŝ ≠ ∅ em pelo menos 3 domínios de áreas distintas.
- **Onde se está.** No K = 3 (H2d), só a educação (STAR) tem Ŝ ≠ ∅.
- **O que se viu dos dados novos.** Só o esquema: colunas, tipos, contagens por ambiente, os alvos de cada
  intervenção e os grafos de referência dos autores. Nenhum resumo de Y por ambiente foi calculado e nenhum modelo
  foi ajustado. Há uma exceção, que declaro: no CigarettesSW, a busca imprimiu a média e os quartis de `packs` por
  ano. Esse domínio fica de fora por outro motivo (ver abaixo).

## O que entra

Os 5 domínios do H2d são rodados **sem nenhuma mudança**: mesmo pacote, mesmo código, K = 3. Entram dois
domínios novos, os dois do projeto Causal Chambers (Gamella, Peters & Bühlmann 2025, Nature Machine Intelligence;
dados CC BY 4.0). São câmaras físicas com atuadores contínuos sob intervenção conhecida e com o grafo causal
documentado pelos autores.

O código do ICP é o do H2d sem nenhuma mudança: o teste K = 2 e a regra de saturação K = 3. Para os domínios novos
este é o K = 1, porque nenhuma alternativa foi rodada.

### (1) `fisica_tunel_luz`, área "Física (óptica e eletrônica)"

- **Dados.** `lt_interventions_standard_v1`: 59 experimentos, cada linha um sorteio independente dos atuadores.
- **Y = `current`**, a corrente elétrica que a fonte de luz consome.
  - Pais segundo os autores: red, green e blue (o brilho dos LEDs), mais osr_c e v_c, que são parâmetros do
    próprio sensor de corrente.
  - Y é padronizado pela média e pelo dp do ambiente de referência, que está no treino.
- **Candidatos do ICP (10).**
  - Os pais: red, green, blue.
  - Não pais: pol_1, pol_2, ir_1, vis_1, ir_2, vis_2, ir_3. Os polarizadores são atuadores independentes. Os
    sensores de luz são filhos de red, green e blue, e por isso se correlacionam com Y sem causá-lo.
- **A chave** é `red` reescalado para [0, 1] (red/255).
- **Ambientes.**
  - Entram a referência e os experimentos de intervenção sobre X: os LEDs e polarizadores (faixas mid e strong),
    as fontes l_ij e os parâmetros dos sensores de luz e de ângulo (t_*, diode_*, osr_angle_*, v_angle_*).
  - Ficam fora, por decisão anterior a qualquer ajuste, os experimentos `uniform_osr_c_*` e `uniform_v_c_*`. Eles
    mexem no mecanismo do sensor de Y, isto é, agem em Y, o que viola a premissa do ICP.
  - Isso deixa 54 ambientes.
- **Cofre: `uniform_red_strong`** (red entre 171 e 255, nunca visto). O `uniform_red_mid` fica no treino.
- **Unidade:** a linha. Os atuadores são sorteados de forma independente em cada medição.
- **Verdade de referência para comparar com Ŝ:** {red, green, blue}.
- **Por que não ir_3.** O ir_3 seria um Y mais rico, mas ele é o produto da intensidade pela lei de Malus,
  cos²(θ₁ − θ₂). O teste K = 2 é aditivo, então a escolha de ir_3 testaria a forma funcional, não a invariância.
  Esta escolha foi feita antes de ver os dados.

### (2) `engenharia_tunel_vento`, área "Engenharia (aerodinâmica)"

- **Dados.** `wt_walks_v1 / regime_jumps_single`: 32 regimes de 10 000 medições. As cargas dos ventiladores seguem
  passeios aleatórios, e em cada regime há uma intervenção de alvo único, ou nenhuma.
- **Y = `pressure_downwind`** (Pa).
  - Pais segundo os autores: load_in, load_out, hatch, osr_downwind.
  - Y é padronizado pela média e pelo dp do treino agrupado, sem o cofre.
- **Candidatos do ICP (10).**
  - Os pais: load_in, load_out, hatch, osr_downwind.
  - Não pais: rpm_in, rpm_out, current_in, pressure_upwind, pressure_intake, pressure_ambient.
  - `pressure_ambient` entra porque os barômetros medem pressão absoluta, e uma deriva atmosférica comum a todos
    agiria em Y sem estar no grafo. Se Ŝ trouxer `pressure_ambient`, isso será lido como sinal dessa causa oculta,
    não como erro.
- **A chave** é `load_in` (0–1).
- **Ambientes:** os 32 regimes (`flag`); nenhum fica fora. As intervenções em hatch e osr_downwind agem em Y,
  mas as duas variáveis são candidatas medidas.
- **Cofre: o regime 18** (hatch 0 → 45, a segunda vez). O regime 5, com a mesma intervenção, fica no treino.
- **Série temporal.** As medições são autocorrelacionadas, e o teste do ICP supõe linhas independentes. Por isso,
  antes de qualquer ajuste:
  - fica uma medição a cada 20 (500 por regime, 16 000 no total);
  - a **unidade** é o bloco de 50 medições consecutivas dentro do regime (10 por regime), para o EP do F2 e a
    permutação do F3.
  - A autocorrelação que resta depois do desbaste é uma ressalva declarada: ela torna o teste mais propenso a
    rejeitar, nunca a aceitar.
- **Verdade de referência para comparar com Ŝ:** {load_in, load_out, hatch, osr_downwind}.

### Priori SHELF e decisão dos dois domínios novos

Não há um especialista humano. As prioris vêm de um argumento físico, citado como tal.
- **Luz.** θ é o efeito de red ir de 0 a 255, em dp de Y na referência.
  - Se os três LEDs pesam igual, θ ≈ 6. Se só o vermelho pesasse, θ ≈ 10.
  - Quartis: q05 1, q25 4, q50 6, q75 8, q95 10,5.
- **Vento.** θ é o efeito de load_in ir de 0 a 1, em dp de Y.
  - Se as duas cargas pesam igual, θ ≈ 2,4. O sinal provável é positivo, porque o ventilador de entrada pressuriza
    a câmara.
  - Quartis: q05 −1, q25 1, q50 2, q75 3, q95 4,5.
- **Decisão.** A decisão é inventada, como no Sachs: usar a chave como alavanca vale se θ passar de um custo
  posto perto da mediana da priori (custo 6 na luz, 2 no vento; custo de pesquisa 0,05). O F1 desses dois
  domínios não informa nada sobre o mundo e é relatado assim.

## O que fica de fora, e por quê (decidido antes de rodar)

- **`agridat::lasrosas.corn`.** Há só dois anos, e o clima de cada ano age direto na produção sem estar medido.
  Os ambientes possíveis (ano × posição no relevo) misturam essa ação direta com a posição. Além disso, as doses de
  N são diferentes entre os anos, de modo que a dose quase determina o ano (saturação). A pergunta do ICP fica mal
  posta.
- **`AER::CigarettesSW`.** Tem 2 ambientes (1985 e 1995) e 96 linhas: não sobra treino com ≥ 2 ambientes mais um
  cofre.
- **`Ecdat::Cigar`.** O imposto não é observado; só o preço. As tendências seculares de consumo (os relatórios de
  saúde a partir de 1964) agem direto em Y e não estão medidas.

## Regra do G3 e leitura das áreas

- O G3 é calculado como está escrito em `portoes.py`, sem mudança, com os rótulos de área declarados acima.
- **Ressalva que sil precisa julgar.** Os dois domínios novos vêm do mesmo projeto e da mesma família de
  instrumentos. Os mecanismos físicos são diferentes (luz e corrente elétrica num caso, escoamento de ar no outro),
  mas um leitor estrito pode contá-los como uma área só. Por isso o resultado será relatado de duas formas:
  - (a) pelo critério como está escrito;
  - (b) pela leitura conservadora, com as câmaras contando como uma área.
- Se (a) passar e (b) não, a saída do H2 fica **pendente da decisão de sil**, e não aprovada por mim.

## Depois desta declaração

- **Testes.** Antes dos dados reais, os testes do H2d continuam passando, mais um teste de carga dos dois pacotes
  novos.
- **Uma execução.** Uma execução do funil nos 7 domínios. Nada é ajustado depois de ver os resultados. Se aparecer
  um erro de código que impeça a execução, a correção é relatada no README com o motivo.
