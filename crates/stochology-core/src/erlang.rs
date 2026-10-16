//! A fórmula de Erlang e a escala mínima que entrega uma meta de serviço.
//!
//! Mora no núcleo porque dois horizontes precisam dela: o H0 (a fila e a central de chamadas, que a
//! re-exporta) e o eixo R (as previsões 3 e 5, que dimensionam a escala do dia médio e a do dia).

/// Erlang B: probabilidade de bloqueio de `agentes` servidores com carga `carga = lambda/mu`.
/// Recorrência estável, presa em [0, 1].
pub fn erlang_b(agentes: u32, carga: f64) -> f64 {
    if carga <= 0.0 || agentes == 0 {
        return 0.0;
    }
    let mut b = 1.0_f64;
    for k in 1..=agentes {
        let kf = k as f64;
        b = carga * b / (kf + carga * b);
    }
    b.clamp(0.0, 1.0)
}

/// Erlang C: probabilidade de esperar, com `agentes` servidores e carga `carga`.
pub fn erlang_c(agentes: u32, carga: f64) -> f64 {
    if agentes == 0 {
        return 1.0;
    }
    if carga >= agentes as f64 {
        return 1.0;
    }
    let b = erlang_b(agentes, carga);
    let rho = carga / agentes as f64;
    (b / (1.0 - rho * (1.0 - b))).clamp(0.0, 1.0)
}

/// Menor número de servidores que faz o serviço imediato cobrir `meta` da carga: Erlang C <= 1 - meta.
pub fn escala_minima(lambda: f64, mu: f64, meta: f64) -> u32 {
    if mu <= 0.0 || lambda <= 0.0 {
        return 0;
    }
    let carga = lambda / mu;
    let mut agentes = carga.ceil().max(1.0) as u32;
    while erlang_c(agentes, carga) > 1.0 - meta && agentes < 100_000 {
        agentes += 1;
    }
    agentes
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn a_escala_minima_entrega_a_meta_e_uma_a_menos_nao() {
        for (lambda, mu, meta) in [(8.0_f64, 1.0_f64, 0.8_f64), (234.0, 1.0, 0.8), (5.0, 2.0, 0.9)] {
            let agentes = escala_minima(lambda, mu, meta);
            let carga = lambda / mu;
            assert!(erlang_c(agentes, carga) <= 1.0 - meta + 1e-12);
            if (agentes as f64) > carga.ceil() {
                assert!(erlang_c(agentes - 1, carga) > 1.0 - meta - 1e-12);
            }
        }
    }

    #[test]
    fn erlang_b_e_erlang_c_ficam_na_faixa() {
        for carga in [0.5_f64, 1.0, 4.0, 8.0, 40.0] {
            for agentes in 1..40u32 {
                let b = erlang_b(agentes, carga);
                let c = erlang_c(agentes, carga);
                assert!((0.0..=1.0).contains(&b), "B fora da faixa: {b}");
                assert!((0.0..=1.0).contains(&c), "C fora da faixa: {c}");
            }
        }
        assert!(erlang_c(0, 1.0) == 1.0);
        assert!(erlang_c(3, 4.0) == 1.0, "carga acima da capacidade espera sempre");
    }
}
