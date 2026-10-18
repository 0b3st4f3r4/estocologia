# Opinion — Case 01: intensity control by polarizers

## What was asked
To know whether misaligning the two polarizers reduces the light reaching the sensor, in order to control the intensity by rotating the polarizers with the LED fixed.

## Design and why it allows a causal conclusion
The 10,000 measurements have all commands (LED colors, polarizer angles, auxiliary LEDs) drawn independently and uniformly. The angles are therefore independent of any other factor: a difference in reading between angle bands is an effect of the angle, not confounding. I checked the balancing: the means of the LED and of the auxiliary-LED brightnesses are practically equal in the 0–5° and 30–40° difference bands (e.g. red 42.1 versus 43.0; aux A 86.1 versus 84.3). All correlations between commands stay below 0.03 in absolute value.

## Results
- I reproduced the client's calculation: mean IR reading of 455.9 (|Δθ| ≤ 5°, n = 2443) versus 344.2 (30°–40°, n = 511); Welch t = 13.8, p ≈ 1e-39. The visible channel shows the same pattern (123.6 versus 92.0; p ≈ 1e-38).
- The fall is monotonic per 5° band in the difference: 457 → 470 → 434 → 438 → 404 → 377 → 349 → 319 (IR), with no artificial jump in the choice of bands.
- Regression of each channel on cos²(Δθ), controlling for the three LED colors, for the auxiliary LEDs and for the individual angles (HC3 errors): coefficient of cos² = 430 (95% CI 414 to 445) in IR and 116 (111 to 120) in visible. The auxiliary LEDs have no detectable effect (p > 0.25), and therefore they do not contaminate the reading.
- Nonlinear fit of the Malus model, reading = a₀ + (Σ bᵢ·LEDᵢ)·cos²(θ₂ − θ₁ − φ): the model describes the data well, with a small calibration deviation φ ≈ 2.9° (SE 0.12°) in both channels. The transmission maximum occurs with polarizer 2 about 3° ahead of 1, and not at exactly 0°.

## Caveats for the implementation
- The data cover differences of up to 40°, because each angle was drawn between −15° and 25°. The extrapolation to larger attenuations, up to near 90°, follows the physics, but was not tested on this bench.
- The control must use the calibrated zero (Δθ ≈ +2.9°), and not the nominal zero.
- The attenuation is multiplicative on the contribution of the LED. There is a constant term (intercept) that does not fall with the angle, and therefore there is a reading floor.

## Conclusion
In an experiment with drawn factors, the misalignment of the polarizers reduces the reading in both channels. The reduction has a shape consistent with cos² and the effect is large and precise.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: With angles drawn independently of the other commands, the reading falls monotonically and consistently with the Malus law as the angle difference grows, in the IR and visible channels (cos²: 95% CI 414–445 in IR), without confounding by the LEDs. There is only a calibration deviation of ~3° in the zero, which must be incorporated into the control.
```
