# Parecer — Caso 01: controle de intensidade por polarizadores

## O que foi pedido
Saber se desalinhar os dois polarizadores reduz a luz que chega ao sensor, para controlar a intensidade girando os polarizadores com o LED fixo.

## Desenho e por que ele permite conclusão causal
As 10.000 medições têm todos os comandos (cores do LED, ângulos dos polarizadores, LEDs auxiliares) sorteados de forma independente e uniforme. Os ângulos são, portanto, independentes de qualquer outro fator: uma diferença de leitura entre faixas de ângulo é efeito do ângulo, não confusão. Conferi o balanceamento: as médias dos brilhos do LED e dos LEDs auxiliares são praticamente iguais nas faixas de 0–5° e 30–40° de diferença (ex.: vermelho 42,1 contra 43,0; aux A 86,1 contra 84,3). Todas as correlações entre comandos ficam abaixo de 0,03 em módulo.

## Resultados
- Reproduzi a conta do cliente: leitura IR média de 455,9 (|Δθ| ≤ 5°, n = 2443) contra 344,2 (30°–40°, n = 511); Welch t = 13,8, p ≈ 1e-39. O canal visível mostra o mesmo padrão (123,6 contra 92,0; p ≈ 1e-38).
- A queda é monotônica por faixa de 5° na diferença: 457 → 470 → 434 → 438 → 404 → 377 → 349 → 319 (IR), sem salto artificial na escolha das faixas.
- Regressão de cada canal em cos²(Δθ), controlando pelas três cores do LED, pelos LEDs auxiliares e pelos ângulos individuais (erros HC3): coeficiente de cos² = 430 (IC 95% 414 a 445) no IR e 116 (111 a 120) no visível. Os LEDs auxiliares não têm efeito detectável (p > 0,25), e por isso não contaminam a leitura.
- Ajuste não linear do modelo de Malus, leitura = a₀ + (Σ bᵢ·LEDᵢ)·cos²(θ₂ − θ₁ − φ): o modelo descreve bem os dados, com um pequeno desvio de calibração φ ≈ 2,9° (EP 0,12°) nos dois canais. O máximo de transmissão ocorre com o polarizador 2 cerca de 3° à frente do 1, e não em 0° exato.

## Ressalvas para a implementação
- Os dados cobrem diferenças de até 40°, porque cada ângulo foi sorteado entre −15° e 25°. A extrapolação para atenuações maiores, até perto de 90°, segue a física, mas não foi testada nesta bancada.
- O controle deve usar o zero calibrado (Δθ ≈ +2,9°), e não o zero nominal.
- A atenuação é multiplicativa sobre a contribuição do LED. Existe um termo constante (intercepto) que não cai com o ângulo, e por isso há um piso de leitura.

## Conclusão
Num experimento com fatores sorteados, o desalinhamento dos polarizadores reduz a leitura nos dois canais. A redução tem forma consistente com cos² e o efeito é grande e preciso.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: Com ângulos sorteados independentemente dos demais comandos, a leitura cai de forma monotônica e consistente com a lei de Malus à medida que a diferença de ângulo cresce, nos canais IR e visível (cos²: IC 95% 414–445 no IR), sem confusão pelos LEDs. Há apenas um desvio de calibração de ~3° no zero, que deve ser incorporado ao controle.
```
