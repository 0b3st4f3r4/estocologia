//! Capítulo 4 — sorte, ignorância e o peru: moeda cujo vício muda devagar.

use stochology_core::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cobertura {
    pub passos: usize,
    pub repeticoes: usize,
    pub desconto: f64,
    /// Fração de passos em que o intervalo de 95% do modelo fixo cobre a verdade.
    pub cobertura_estatica: f64,
    /// Fração de passos em que o intervalo do modelo com fator de esquecimento cobre a verdade.
    pub cobertura_descontada: f64,
    pub largura_estatica_100: f64,
    pub largura_estatica_final: f64,
    pub largura_descontada_final: f64,
}

/// Viés que sobe de 0,2 a 0,6 ao longo dos passos, com sorteio reprodutível.
pub fn vies_em(passo: usize, passos: usize) -> f64 {
    0.2 + 0.4 * (passo as f64 / passos as f64)
}

fn largura(p: f64, n_efetivo: f64) -> f64 {
    if n_efetivo <= 1.0 {
        return 1.0;
    }
    2.0 * 1.96 * (p * (1.0 - p) / n_efetivo).sqrt()
}

/// Roda o mundo que muda e mede a cobertura dos dois modelos.
///
/// Configuração do livro: 6.400 passos, 200 repetições, desconto 0,99.
/// Instrumento: ~cargo run --release --bin labs -- --cap 4~
pub fn moeda_movel(semente: u64, passos: usize, repeticoes: usize, desconto: f64) -> Cobertura {
    let mut rng = Rng::novo(semente);
    let mut cobre_estatico = 0usize;
    let mut cobre_descontado = 0usize;
    let mut contados = 0usize;
    let mut largura_estatica_100 = 0.0;
    let mut largura_estatica_final = 0.0;
    let mut largura_descontada_final = 0.0;
    let n_efetivo = (1.0 + desconto) / (1.0 - desconto);

    for _ in 0..repeticoes {
        let mut caras = 0.0_f64;
        let mut soma = 0.0_f64;
        let mut peso = 0.0_f64;
        for passo in 1..=passos {
            let verdade = vies_em(passo, passos);
            let x = if (rng.proximo() as f64) / (u64::MAX as f64) < verdade {
                1.0
            } else {
                0.0
            };
            caras += x;

            let p_estatico = caras / passo as f64;
            let l_estatico = largura(p_estatico, passo as f64);

            soma = desconto * soma + x;
            peso = desconto * peso + 1.0;
            let p_descontado = soma / peso;
            let l_descontado = largura(p_descontado, n_efetivo);

            if passo >= 100 {
                contados += 1;
                if (p_estatico - verdade).abs() <= l_estatico / 2.0 {
                    cobre_estatico += 1;
                }
                if (p_descontado - verdade).abs() <= l_descontado / 2.0 {
                    cobre_descontado += 1;
                }
            }
            if passo == 100 {
                largura_estatica_100 += l_estatico;
            }
            if passo == passos {
                largura_estatica_final += l_estatico;
                largura_descontada_final += l_descontado;
            }
        }
    }

    let r = repeticoes as f64;
    let c = contados as f64;
    Cobertura {
        passos,
        repeticoes,
        desconto,
        cobertura_estatica: cobre_estatico as f64 / c,
        cobertura_descontada: cobre_descontado as f64 / c,
        largura_estatica_100: largura_estatica_100 / r,
        largura_estatica_final: largura_estatica_final / r,
        largura_descontada_final: largura_descontada_final / r,
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn modelo_fixo_erra_quase_sempre_e_o_modelo_com_esquecimento_cobre() {
        let c = moeda_movel(2026, 6_400, 200, 0.99);
        assert!(
            c.cobertura_estatica < 0.2,
            "cobertura estatica: {}",
            c.cobertura_estatica
        );
        assert!(
            c.cobertura_descontada > 0.90,
            "cobertura descontada: {}",
            c.cobertura_descontada
        );
    }

    #[test]
    fn intervalo_fixo_encolhe_e_o_descontado_fica_largo() {
        let c = moeda_movel(2026, 6_400, 200, 0.99);
        assert!(c.largura_estatica_final < c.largura_estatica_100);
        assert!(c.largura_descontada_final > c.largura_estatica_final);
    }

    #[test]
    fn o_vies_muda_de_dois_decimos_a_seis_decimos() {
        assert!((vies_em(0, 3_200) - 0.2).abs() < 1e-12);
        assert!((vies_em(3_200, 3_200) - 0.6).abs() < 1e-12);
    }
}
