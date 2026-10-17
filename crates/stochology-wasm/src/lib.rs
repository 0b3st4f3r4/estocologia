//! stochology-wasm — o playground: os instrumentos no navegador, sem servidor.
//!
//! O crate exporta a funcao pura em `api` e a ponte C em `estocologia_*`, a unica coisa que o
//! navegador precisa. Sem dependencia externa: o alvo e `wasm32-unknown-unknown`.
//!
//! Comando declarado:
//! `cargo build --release --target wasm32-unknown-unknown -p stochology-wasm`
//! e, para abrir, `crates/stochology-wasm/www/index.html`.
//!
//! Declarado e nao implementado: a interface de memoria (hoje tudo passa por inteiros) e o
//! carregamento do hipergrafo no navegador.

// A ponte C exporta com `#[unsafe(no_mangle)]`, que o lint de unsafe_code conta como
// declaracao insegura. Nao ha bloco unsafe neste crate: o allow e pelo atributo de export,
// e o resto da arvore continua com unsafe_code = "deny".
#![allow(unsafe_code)]

use stochology_core::Rng;

/// As funcoes puras que a ponte C e o navegador usam.
pub mod api {
    use super::*;

    /// Versao do playground, para o HTML saber com o que esta falando.
    pub fn versao() -> u32 {
        1
    }

    /// Cara ou coroa deterministico: quantas caras em `lances`, com a probabilidade em centesimos.
    pub fn moeda(semente: u64, lances: u32, centesimos_de_cara: u64) -> u64 {
        let p = (centesimos_de_cara.min(100) as f64) / 100.0;
        let mut rng = Rng::novo(semente);
        let mut caras = 0u64;
        for _ in 0..lances {
            let u = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            if u < p {
                caras += 1;
            }
        }
        caras
    }

    /// Erlang C em decimilessimos (0 a 10 000): a mesma formula do H0.
    pub fn erlang_c_decimilessimos(agentes: u32, carga_centesimos: u64) -> u64 {
        let carga = carga_centesimos as f64 / 100.0;
        (stochology_h0::erlang_c(agentes, carga) * 10_000.0).round() as u64
    }

    /// Indice de dispersao do nulo de Poisson, em milesimos. Poisson fica perto de 1000.
    pub fn dispersao_do_nulo(semente: u64, passos: u32, media_centesimos: u64) -> u64 {
        let nulo = stochology_h0::NuloPoisson::novo(media_centesimos as f64 / 100.0);
        let serie = nulo.serie(semente, passos.max(2) as usize);
        (stochology_h0::indice_de_dispersao(&serie) * 1000.0).round() as u64
    }

    /// A regra do contrato em codigo: nenhuma probabilidade bate em 0 nem em 1.
    pub fn probabilidade_presa(centesimos: u64) -> u64 {
        centesimos.clamp(1, 99)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn estocologia_versao() -> u32 {
    api::versao()
}

#[unsafe(no_mangle)]
pub extern "C" fn estocologia_moeda(semente: u64, lances: u32, centesimos_de_cara: u64) -> u64 {
    api::moeda(semente, lances, centesimos_de_cara)
}

#[unsafe(no_mangle)]
pub extern "C" fn estocologia_erlang_c(agentes: u32, carga_centesimos: u64) -> u64 {
    api::erlang_c_decimilessimos(agentes, carga_centesimos)
}

#[unsafe(no_mangle)]
pub extern "C" fn estocologia_dispersao_do_nulo(
    semente: u64,
    passos: u32,
    media_centesimos: u64,
) -> u64 {
    api::dispersao_do_nulo(semente, passos, media_centesimos)
}

#[unsafe(no_mangle)]
pub extern "C" fn estocologia_probabilidade_presa(centesimos: u64) -> u64 {
    api::probabilidade_presa(centesimos)
}

#[cfg(test)]
mod testes {
    use super::api;

    #[test]
    fn a_moeda_bate_a_probabilidade_e_nao_depende_do_relogio() {
        let caras = api::moeda(2026, 200_000, 50);
        let fracao = caras as f64 / 200_000.0;
        assert!((fracao - 0.5).abs() < 0.01, "moeda justa: {fracao}");
        assert_eq!(caras, api::moeda(2026, 200_000, 50));
        assert_ne!(caras, api::moeda(2027, 200_000, 50));
        let viciada = api::moeda(2026, 200_000, 90);
        assert!(viciada as f64 / 200_000.0 > 0.88);
    }

    #[test]
    fn a_ponte_entrega_os_mesmos_numeros_da_funcao_pura() {
        assert_eq!(api::versao(), 1);
        assert_eq!(api::erlang_c_decimilessimos(10, 500), 361);
        assert_eq!(api::erlang_c_decimilessimos(3, 200), 4444);
        let dispersao = api::dispersao_do_nulo(7, 20_000, 400);
        assert!(
            (990..=1010).contains(&dispersao),
            "Poisson fica em 1000: {dispersao}"
        );
    }

    #[test]
    fn a_probabilidade_nunca_bate_nos_extremos() {
        assert_eq!(api::probabilidade_presa(0), 1);
        assert_eq!(api::probabilidade_presa(100), 99);
        assert_eq!(api::probabilidade_presa(37), 37);
    }
}
