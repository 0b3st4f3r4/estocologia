//! The Erlang formula and the minimal staffing that delivers a service target.
//!
//! It lives in the core because two horizons need it: H0 (the queue and the call centre, which
//! re-exports it) and the R axis (predictions 3 and 5, which size the average-day staffing and the
//! day staffing).

/// Erlang B: blocking probability of `agents` servers with load `load = lambda/mu`.
/// Stable recurrence, clamped to [0, 1].
pub fn erlang_b(agents: u32, load: f64) -> f64 {
    if load <= 0.0 || agents == 0 {
        return 0.0;
    }
    let mut b = 1.0_f64;
    for k in 1..=agents {
        let kf = k as f64;
        b = load * b / (kf + load * b);
    }
    b.clamp(0.0, 1.0)
}

/// Erlang C: probability of waiting, with `agents` servers and load `load`.
pub fn erlang_c(agents: u32, load: f64) -> f64 {
    if agents == 0 {
        return 1.0;
    }
    if load >= agents as f64 {
        return 1.0;
    }
    let b = erlang_b(agents, load);
    let rho = load / agents as f64;
    (b / (1.0 - rho * (1.0 - b))).clamp(0.0, 1.0)
}

/// Smallest number of servers that makes immediate service cover `target` of the load: Erlang C <= 1 - target.
pub fn minimal_scale(lambda: f64, mu: f64, target: f64) -> u32 {
    if mu <= 0.0 || lambda <= 0.0 {
        return 0;
    }
    let load = lambda / mu;
    let mut agents = load.ceil().max(1.0) as u32;
    while erlang_c(agents, load) > 1.0 - target && agents < 100_000 {
        agents += 1;
    }
    agents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_scale_meets_the_target_and_one_less_does_not() {
        for (lambda, mu, target) in [
            (8.0_f64, 1.0_f64, 0.8_f64),
            (234.0, 1.0, 0.8),
            (5.0, 2.0, 0.9),
        ] {
            let agents = minimal_scale(lambda, mu, target);
            let load = lambda / mu;
            assert!(erlang_c(agents, load) <= 1.0 - target + 1e-12);
            if (agents as f64) > load.ceil() {
                assert!(erlang_c(agents - 1, load) > 1.0 - target - 1e-12);
            }
        }
    }

    #[test]
    fn erlang_b_and_erlang_c_stay_in_range() {
        for load in [0.5_f64, 1.0, 4.0, 8.0, 40.0] {
            for agents in 1..40u32 {
                let b = erlang_b(agents, load);
                let c = erlang_c(agents, load);
                assert!((0.0..=1.0).contains(&b), "B out of range: {b}");
                assert!((0.0..=1.0).contains(&c), "C out of range: {c}");
            }
        }
        assert!(erlang_c(0, 1.0) == 1.0);
        assert!(erlang_c(3, 4.0) == 1.0, "load above capacity always waits");
    }
}
