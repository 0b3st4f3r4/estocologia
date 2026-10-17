//! Gerador determinístico SplitMix64 (semente fixa, sem dependências externas).

/// Um passo de SplitMix64 sobre o estado mutável: único lugar onde residem as constantes de mistura.
pub fn passo_splitmix64(estado: &mut u64) -> u64 {
    *estado = estado.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *estado;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    estado: u64,
}

impl Rng {
    pub fn novo(semente: u64) -> Self {
        Rng { estado: semente }
    }

    pub fn proximo(&mut self) -> u64 {
        passo_splitmix64(&mut self.estado)
    }

    /// Inteiro em [0, limite).
    pub fn uniforme(&mut self, limite: u64) -> u64 {
        if limite == 0 {
            return 0;
        }
        self.proximo() % limite
    }

    /// Grandeza da cauda pesada na faixa [inicio, fim) em centesimos.
    pub fn centesimos(&mut self, inicio: u64, fim: u64) -> u64 {
        inicio + self.uniforme(fim.saturating_sub(inicio).max(1))
    }
}

#[cfg(test)]
mod testes {
    use super::Rng;

    #[test]
    fn mesma_semente_mesma_sequencia() {
        let mut a = Rng::novo(2026);
        let mut b = Rng::novo(2026);
        for _ in 0..64 {
            assert_eq!(a.proximo(), b.proximo());
        }
    }

    #[test]
    fn sementes_diferentes_divergem() {
        let mut a = Rng::novo(1);
        let mut b = Rng::novo(2);
        assert_ne!(a.proximo(), b.proximo());
    }
}
