//! Capítulo 1 — o alvo invisível: por que a semente fixa é a base da conferência.

use stochology_core::Rng;

/// Uma máquina que sorteia: mesma semente, mesma sequência.
pub fn maquina(semente: u64, n: usize) -> Vec<u64> {
    let mut rng = Rng::novo(semente);
    (0..n).map(|_| rng.uniforme(1_000_000)).collect()
}

/// Duas máquinas independentes com a mesma semente concordam lance a lance.
pub fn duas_maquinas_concordam(semente: u64, n: usize) -> bool {
    maquina(semente, n) == maquina(semente, n)
}

/// Quantos dos primeiros n sorteios coincidem quando a semente muda.
pub fn coincidencias_com_outra_semente(semente: u64, n: usize) -> usize {
    let a = maquina(semente, n);
    let b = maquina(semente + 1, n);
    a.iter().zip(b.iter()).filter(|(x, y)| x == y).count()
}

/// Frequência de caras nas primeiras n jogadas de uma moeda honesta.
pub fn frequencia_de_caras(semente: u64, n: usize) -> f64 {
    let mut rng = Rng::novo(semente);
    let caras = (0..n).filter(|_| rng.uniforme(2) == 0).count();
    caras as f64 / n as f64
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn duas_maquinas_concordam_lance_a_lance() {
        assert!(duas_maquinas_concordam(1713, 10_000));
    }

    #[test]
    fn sementes_diferentes_quase_nao_coincidem() {
        let c = coincidencias_com_outra_semente(1713, 100_000);
        assert!(c < 200, "coincidencias: {}", c);
    }

    #[test]
    fn moeda_honesta_concentra_perto_de_meio() {
        for semente in [1713, 1909, 2026] {
            let f = frequencia_de_caras(semente, 100_000);
            assert!((f - 0.5).abs() < 0.01, "frequencia: {}", f);
        }
    }
}
