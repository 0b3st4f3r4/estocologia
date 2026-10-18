//! Real H0 in Rust: the bank call centre, without Python.
//!
//! It reads the base in 5-minute steps, builds the mean profile of the training window, scales it by
//! Erlang C as the industry practice does, and measures what happens when the same roster meets the
//! real day. It also measures the independent unit (the day against the call) and the halves null.

use std::collections::BTreeMap;

/// Half hours per working day: 07:00 to 21:00.
pub const SLICES: usize = 28;
/// Mean handling time and wait target, in seconds.
pub const AHT_S: f64 = 240.0;
pub const TARGET_S: f64 = 20.0;
pub const GOAL: f64 = 0.80;

#[derive(Debug, Clone)]
pub struct Day {
    pub date: String,
    pub slices: [f64; SLICES],
    pub total: f64,
}

/// Probability of waiting (Erlang C) for offered load `a` and `c` agents (delegates to the H0 core).
pub fn erlang_c(c: f64, a: f64) -> f64 {
    if c <= a {
        return 1.0;
    }
    stochology_horizons::h0::erlang_c(c as u32, a)
}

/// Service level: fraction served within the target number of seconds.
pub fn service_level(c: f64, a: f64, aht_s: f64, target_s: f64) -> f64 {
    if c <= a {
        return 0.0;
    }
    let wait = erlang_c(c, a);
    let inside = 1.0 - wait * (-((c - a) * target_s / aht_s)).exp();
    inside.clamp(0.0, 1.0)
}

/// Smallest number of agents that reaches the goal for an arrival rate (calls per hour).
pub fn agents_for_goal(lam_hour: f64, aht_s: f64, target_s: f64, goal: f64) -> u32 {
    let a = lam_hour * aht_s / 3600.0;
    let mut c = a.floor().max(1.0);
    while c < 5_000.0 {
        if service_level(c, a, aht_s, target_s) >= goal {
            return c as u32;
        }
        c += 1.0;
    }
    c as u32
}

/// Reads the base: rownames,DateTime,Calls with 5-minute stamps.
pub fn load(path: &str) -> Vec<Day> {
    let text = std::fs::read_to_string(path).expect("could not read the bank CSV");
    let mut by_day: BTreeMap<String, [f64; SLICES]> = BTreeMap::new();
    for line in text.lines().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 3 {
            continue;
        }
        let stamp = fields[1];
        if stamp.len() < 16 {
            continue;
        }
        let date = &stamp[..10];
        let hour: usize = stamp[11..13].parse().unwrap_or(0);
        let minute: usize = stamp[14..16].parse().unwrap_or(0);
        if !(7..=20).contains(&hour) {
            continue;
        }
        let slice = (hour - 7) * 2 + if minute >= 30 { 1 } else { 0 };
        let calls: f64 = fields[2].parse().unwrap_or(0.0);
        let entry = by_day.entry(date.to_string()).or_insert([0.0; SLICES]);
        entry[slice] += calls;
    }
    by_day
        .into_iter()
        .map(|(date, slices)| Day {
            date,
            slices,
            total: slices.iter().sum(),
        })
        .collect()
}

fn month(date: &str) -> u32 {
    date[5..7].parse().unwrap_or(0)
}

#[derive(Debug, Clone)]
pub struct BankMeasure {
    pub train_days: usize,
    pub vault_days: usize,
    pub vault_calls: f64,
    pub level_on_mean_profile: f64,
    pub mean_level_of_days: f64,
    pub fraction_days_below_goal: f64,
    pub var_over_mean: f64,
    pub var_over_mean_train: f64,
    pub var_over_mean_all: f64,
    pub sd_between_days: f64,
    pub poisson_sd: f64,
    pub agents_at_peak: u32,
    pub agents_per_day: f64,
    pub halves_null_day: f64,
    pub halves_null_call: f64,
}

/// Scales by Erlang C over the mean training rate, slice by slice.
fn mean_profile_scale(training: &[Day]) -> [u32; SLICES] {
    let mut mean = [0.0_f64; SLICES];
    for d in training {
        for i in 0..SLICES {
            mean[i] += d.slices[i];
        }
    }
    let n = training.len() as f64;
    let mut scale = [1_u32; SLICES];
    for i in 0..SLICES {
        // each slice is worth half an hour: the mean is in calls per half hour
        scale[i] = agents_for_goal((mean[i] / n) * 2.0, AHT_S, TARGET_S, GOAL);
    }
    scale
}

fn day_level(scale: &[u32; SLICES], day: &Day) -> f64 {
    let mut weight = 0.0;
    let mut acc = 0.0;
    for i in 0..SLICES {
        let lam = day.slices[i] * 2.0; // calls per hour in the slice
        let a = lam * AHT_S / 3600.0;
        let c = scale[i] as f64;
        let sl = service_level(c, a, AHT_S, TARGET_S);
        acc += sl * day.slices[i];
        weight += day.slices[i];
    }
    if weight == 0.0 { 0.0 } else { acc / weight }
}

/// Measures everything the chapter cites from the real base.
pub fn measure(path: &str, seed: u64, replicas: usize) -> BankMeasure {
    let days = load(path);
    let training: Vec<Day> = days
        .iter()
        .filter(|d| (3..=6).contains(&month(&d.date)))
        .cloned()
        .collect();
    let vault: Vec<Day> = days
        .iter()
        .filter(|d| (7..=10).contains(&month(&d.date)))
        .cloned()
        .collect();
    let scale = mean_profile_scale(&training);

    let mean = {
        let mut m = [0.0_f64; SLICES];
        for d in &training {
            for i in 0..SLICES {
                m[i] += d.slices[i];
            }
        }
        m.map(|x| x / training.len() as f64)
    };
    let mut profile_level = 0.0;
    let mut weight = 0.0;
    for i in 0..SLICES {
        profile_level += service_level(
            scale[i] as f64,
            mean[i] * 2.0 * AHT_S / 3600.0,
            AHT_S,
            TARGET_S,
        ) * mean[i];
        weight += mean[i];
    }
    let profile_level = profile_level / weight;

    let levels: Vec<f64> = vault.iter().map(|d| day_level(&scale, d)).collect();
    let mean_level = levels.iter().sum::<f64>() / levels.len() as f64;
    let below = levels.iter().filter(|x| **x < GOAL).count() as f64 / levels.len() as f64;

    let totals: Vec<f64> = vault.iter().map(|d| d.total).collect();
    let day_mean = totals.iter().sum::<f64>() / totals.len() as f64;
    let var = totals
        .iter()
        .map(|t| (t - day_mean) * (t - day_mean))
        .sum::<f64>()
        / (totals.len() as f64 - 1.0);

    // halves null: 400 random re-splits of the vault days
    let mut rng = crate::banco::Rng::new(seed);
    let mut halves = Vec::with_capacity(replicas);
    let mut shuffled_levels = levels.clone();
    for _ in 0..replicas {
        for i in (1..shuffled_levels.len()).rev() {
            let j = rng.uniform(i as u64 + 1) as usize;
            shuffled_levels.swap(i, j);
        }
        let half = shuffled_levels.len() / 2;
        let a: f64 = shuffled_levels[..half].iter().sum::<f64>() / half as f64;
        let b: f64 =
            shuffled_levels[half..].iter().sum::<f64>() / (shuffled_levels.len() - half) as f64;
        halves.push((a - b).abs());
    }
    let halves_mean = halves.iter().sum::<f64>() / halves.len() as f64;
    let day_null = (halves
        .iter()
        .map(|x| (x - halves_mean) * (x - halves_mean))
        .sum::<f64>()
        / (halves.len() as f64 - 1.0))
        .sqrt();
    let n_calls: f64 = totals.iter().sum();
    // halves null on the call unit: two equal halves of independent calls
    let call_null = (2.0 * mean_level * (1.0 - mean_level) / (n_calls / 2.0)).sqrt();

    let dispersion = |ds: &[Day]| -> f64 {
        let t: Vec<f64> = ds.iter().map(|d| d.total).collect();
        let m = t.iter().sum::<f64>() / t.len() as f64;
        let v = t.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (t.len() as f64 - 1.0);
        v / m
    };
    let var_train = dispersion(&training);
    let var_all = dispersion(&days);

    BankMeasure {
        train_days: training.len(),
        vault_days: vault.len(),
        vault_calls: n_calls,
        level_on_mean_profile: profile_level,
        mean_level_of_days: mean_level,
        fraction_days_below_goal: below,
        var_over_mean: var / day_mean,
        var_over_mean_train: var_train,
        var_over_mean_all: var_all,
        sd_between_days: var.sqrt(),
        poisson_sd: day_mean.sqrt(),
        agents_at_peak: scale.iter().copied().max().unwrap_or(0),
        agents_per_day: scale.iter().map(|c| *c as f64).sum::<f64>() / 2.0,
        halves_null_day: day_null,
        halves_null_call: call_null,
    }
}

/// Deterministic generator re-exported from the core, without duplication.
pub use stochology_core::Rng;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn erlang_c_grows_with_load() {
        let light = erlang_c(20.0, 10.0);
        let heavy = erlang_c(20.0, 18.0);
        assert!(light < heavy, "{light} {heavy}");
        assert!((0.0..=1.0).contains(&heavy));
    }

    #[test]
    fn one_more_agent_improves_the_level() {
        let a = 10.0;
        let worse = service_level(12.0, a, AHT_S, TARGET_S);
        let better = service_level(14.0, a, AHT_S, TARGET_S);
        assert!(better > worse);
        assert!(better > GOAL);
    }

    #[test]
    fn scaling_by_the_goal_returns_the_goal() {
        let c = agents_for_goal(300.0, AHT_S, TARGET_S, GOAL);
        let sl = service_level(c as f64, 300.0 * AHT_S / 3600.0, AHT_S, TARGET_S);
        assert!(sl >= GOAL, "level {sl}");
        let sl_minus_one = service_level(c as f64 - 1.0, 300.0 * AHT_S / 3600.0, AHT_S, TARGET_S);
        assert!(sl_minus_one < GOAL, "level with one fewer: {sl_minus_one}");
    }
}
