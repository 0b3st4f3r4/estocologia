//! stochology-wasm — the playground: the instruments in the browser, with no server.
//!
//! The crate exports the pure function in `api` and the C bridge in `stochology_*`, the only thing
//! the browser needs; the historical `estocologia_*` symbols stay as aliases. No external dependency: the target is `wasm32-unknown-unknown`.
//!
//! Declared command:
//! `cargo build --release --target wasm32-unknown-unknown -p stochology-wasm`
//! and, to open it, `crates/stochology-wasm/www/index.html`.
//!
//! Declared and not implemented: the memory interface (today everything goes through integers) and
//! loading the hypergraph in the browser.

// The C bridge exports with `#[unsafe(no_mangle)]`, which the unsafe_code lint counts as an unsafe
// declaration. There is no unsafe block in this crate: the allow is for the export attribute,
// and the rest of the tree keeps unsafe_code = "deny".
#![allow(unsafe_code)]

use stochology_core::Rng;

/// The pure functions that the C bridge and the browser use.
pub mod api {
    use super::*;

    /// Playground version, so the HTML knows what it is talking to.
    pub fn version() -> u32 {
        1
    }

    /// Deterministic coin toss: how many heads in `tosses`, with the probability in hundredths.
    pub fn coin(seed: u64, tosses: u32, heads_hundredths: u64) -> u64 {
        let p = (heads_hundredths.min(100) as f64) / 100.0;
        let mut rng = Rng::new(seed);
        let mut heads = 0u64;
        for _ in 0..tosses {
            let u = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            if u < p {
                heads += 1;
            }
        }
        heads
    }

    /// Erlang C in ten-thousandths (0 to 10 000): the same formula as H0.
    pub fn erlang_c_ten_thousandths(agents: u32, load_hundredths: u64) -> u64 {
        let load = load_hundredths as f64 / 100.0;
        (stochology_horizons::h0::erlang_c(agents, load) * 10_000.0).round() as u64
    }

    /// Dispersion index of the Poisson null, in thousandths. Poisson stays close to 1000.
    pub fn null_dispersion(seed: u64, steps: u32, mean_hundredths: u64) -> u64 {
        let null = stochology_horizons::h0::PoissonNull::new(mean_hundredths as f64 / 100.0);
        let series = null.series(seed, steps.max(2) as usize);
        (stochology_horizons::h0::dispersion_index(&series) * 1000.0).round() as u64
    }

    /// The contract rule in code: no probability hits 0 or 1.
    pub fn trapped_probability(hundredths: u64) -> u64 {
        hundredths.clamp(1, 99)
    }
}

// The C bridge carries the name of the repository since 2026-10-20 (`stochology_*`, in English like the rest of the
// tree). The historical symbols (`estocologia_*`) were part of the contract, because the playground page and any
// external caller link against them, so they stay as aliases that call the same functions: nothing that linked before
// breaks, and the playground uses the new names.

#[unsafe(no_mangle)]
pub extern "C" fn stochology_version() -> u32 {
    api::version()
}

#[unsafe(no_mangle)]
pub extern "C" fn stochology_coin(seed: u64, tosses: u32, heads_hundredths: u64) -> u64 {
    api::coin(seed, tosses, heads_hundredths)
}

#[unsafe(no_mangle)]
pub extern "C" fn stochology_erlang_c(agents: u32, load_hundredths: u64) -> u64 {
    api::erlang_c_ten_thousandths(agents, load_hundredths)
}

#[unsafe(no_mangle)]
pub extern "C" fn stochology_null_dispersion(seed: u64, steps: u32, mean_hundredths: u64) -> u64 {
    api::null_dispersion(seed, steps, mean_hundredths)
}

#[unsafe(no_mangle)]
pub extern "C" fn stochology_trapped_probability(hundredths: u64) -> u64 {
    api::trapped_probability(hundredths)
}

/// Historical alias of [`stochology_version`].
#[unsafe(no_mangle)]
pub extern "C" fn estocologia_versao() -> u32 {
    stochology_version()
}

/// Historical alias of [`stochology_coin`].
#[unsafe(no_mangle)]
pub extern "C" fn estocologia_moeda(seed: u64, tosses: u32, heads_hundredths: u64) -> u64 {
    stochology_coin(seed, tosses, heads_hundredths)
}

/// Historical alias of [`stochology_erlang_c`].
#[unsafe(no_mangle)]
pub extern "C" fn estocologia_erlang_c(agents: u32, load_hundredths: u64) -> u64 {
    stochology_erlang_c(agents, load_hundredths)
}

/// Historical alias of [`stochology_null_dispersion`].
#[unsafe(no_mangle)]
pub extern "C" fn estocologia_dispersao_do_nulo(
    seed: u64,
    steps: u32,
    mean_hundredths: u64,
) -> u64 {
    stochology_null_dispersion(seed, steps, mean_hundredths)
}

/// Historical alias of [`stochology_trapped_probability`].
#[unsafe(no_mangle)]
pub extern "C" fn estocologia_probabilidade_presa(hundredths: u64) -> u64 {
    stochology_trapped_probability(hundredths)
}

#[cfg(test)]
mod tests {
    use super::api;

    #[test]
    fn coin_matches_the_probability_and_does_not_depend_on_the_clock() {
        let heads = api::coin(2026, 200_000, 50);
        let fraction = heads as f64 / 200_000.0;
        assert!((fraction - 0.5).abs() < 0.01, "fair coin: {fraction}");
        assert_eq!(heads, api::coin(2026, 200_000, 50));
        assert_ne!(heads, api::coin(2027, 200_000, 50));
        let biased = api::coin(2026, 200_000, 90);
        assert!(biased as f64 / 200_000.0 > 0.88);
    }

    #[test]
    fn the_bridge_delivers_the_same_numbers_as_the_pure_function() {
        assert_eq!(api::version(), 1);
        assert_eq!(api::erlang_c_ten_thousandths(10, 500), 361);
        assert_eq!(api::erlang_c_ten_thousandths(3, 200), 4444);
        let dispersion = api::null_dispersion(7, 20_000, 400);
        assert!(
            (990..=1010).contains(&dispersion),
            "Poisson sits at 1000: {dispersion}"
        );
    }

    #[test]
    fn the_new_symbols_and_the_historical_aliases_give_the_same_numbers() {
        use super::*;
        assert_eq!(stochology_version(), estocologia_versao());
        assert_eq!(
            stochology_coin(2026, 1000, 50),
            estocologia_moeda(2026, 1000, 50)
        );
        assert_eq!(stochology_erlang_c(10, 500), estocologia_erlang_c(10, 500));
        assert_eq!(
            stochology_null_dispersion(7, 2000, 400),
            estocologia_dispersao_do_nulo(7, 2000, 400)
        );
        assert_eq!(
            stochology_trapped_probability(0),
            estocologia_probabilidade_presa(0)
        );
    }

    #[test]
    fn probability_never_hits_the_extremes() {
        assert_eq!(api::trapped_probability(0), 1);
        assert_eq!(api::trapped_probability(100), 99);
        assert_eq!(api::trapped_probability(37), 37);
    }
}
