//! Real H0 in Rust: the queue with abandonment and the four scaling rules (R0 to R3).
//!
//! It reads `data/bank_calls.csv` (or the base pointed to by `ESTOCOLOGIA_DADOS`), builds the
//! roster on the training window (March to June) and applies it to the vault (July to October) with **queue with
//! abandonment**: exponential service of 4 min and exponential patience of 3 min — the two
//! values of the registry —, real arrivals spread inside each 5-min interval of the base,
//! FCFS and abandonments. No clock, no network, no external dependency: SplitMix64 from `banco.rs`,
//! fixed seed (`SEED = 2003`, the one in the registry).
//!
//! The grid is that of the retired instrument (`h0-real/estocologia/central.py`, still in the git
//! history at commit `4bb7181`): 169 5-min intervals per day (07:00 to 21:00), constant
//! roster inside each group of 6 intervals (half an hour, 29 groups with the tail of one interval),
//! and the same exact FCFS engine — whoever would suffer a wait longer than their own patience gives up, and
//! whoever does not give up takes the agent with the smallest free time; an agent who leaves the shift finishes
//! the call in progress.
//!
//! The four rules, as the registry defines them:
//!
//! | rule | what it is |
//! | --- | --- |
//! | R0 | the claim: Erlang C slice by slice over the mean training profile |
//! | R1 | weekday profile (zero cost) |
//! | R2 | R1 with the demand multiplied by `1 + Z_MARGIN * cv`, `Z_MARGIN = 1.2816` |
//! | R3 | R0 plus `extra` agents across the board, with `extra` such that the cost equals that of R2 |
//!
//! where `cv` is the standard deviation of the residual of the day total (day total divided by the mean of the
//! total of its weekday, in the training window) — the 6% of chapter 12 — and `Z_MARGIN` is the 90%
//! quantile of the normal: it is the "p90" the registry puts in the name of R2.
//!
//! Every measure comes out in the unit of the day: agent-hours per day, service level weighted by
//! call, days below the 80% goal and the wait tail (median and p90). There are two queue
//! engines: `Engine::Exact` (the reconstruction of the retired instrument) and `Engine::Events`
//! (M/M/c+M event by event, Poisson arrivals slice by slice), declared as an independent
//! verification that the decision is not an artifact of the engine.
//!
//! # Divergence declaration (old registry x this instrument)
//!
//! The registry (`h0-real/README.md`, chapter 12) publishes for the 81-day vault: R0 with 2 264
//! agent-hours per day, 86% and 26% of failed days; R1 with 2 261, 90% and 12%; R3 with 2 433, 94% and
//! 7.4%; R2 with 2 428, 96% and 3.7%. This module is a **Rust reconstruction** of that
//! experiment, with the number generator swapped (SplitMix64 against PCG64) and without numpy: the
//! draws of service and patience do not fall in the same places, so that the queue is not
//! identical call by call. What this module asserts is the **design** (the four rules, the
//! grid, the engine) and the **order** (R2 better than R1, R1 better than R0, R3 between the two, R2 and R3
//! at the same cost), with the measured numbers printed by `run()` and checked by generous
//! bands in `registry_bands_and_declared_divergence`. Measured here (seed 2003, one
//! replica per day, Exact engine), with the published value alongside:
//!
//! | rule | agent-hours/day | service level | days below the goal |
//! | --- | --- | --- | --- |
//! | R0 | 2 264 (2 264) | 83.9% (86%) | 30.9% (26%) |
//! | R1 | 2 261 (2 261) | 84.3% (90%) | 17.3% (12%) |
//! | R2 | 2 428 (2 428) | 91.5% (96%) | 4.9% (3.7%) |
//! | R3 | 2 433 (2 433) | 90.2% (94%) | 11.1% (7.4%) |
//!
//! What **reproduces**: the cost of the four rules case by case (2 264, 2 261, 2 428, 2 433), the
//! peak of 234 agents in R0, the 12 agents across the board that R3 adds, the residual CV of
//! 5.98% against the 6% of the chapter, the drop in failed days from R0 to R2 (31% to 5%) and the order
//! R2 < R3 <= R1 <= R0. Two registry numbers that are not from the four rules
//! also reproduce, and therefore count as proof of the engine: the mean wait per call of R0 without abandonment,
//! 1 040 s against the published 956 s, and the world of the claim, 96.3% service level and no
//! failed day in 81, against the published 97% and 0%.
//!
//! What **diverges**: the service level comes out 2 to 6 points below the published one, and the fraction of
//! failed days of R1 and of R3 comes out above. The most distant case is R1: the registry publishes 90% and 12%,
//! and here the two engines measure 84.3% (Exact) and 87.7% (Events), with the drop in failed days from
//! R0 to R1 reproduced (31% to 17%, against 26% to 12%). The gain of R1 over R0 on the mean,
//! which the registry puts at 4 points, comes out at less than 1 point: in the Exact engine the weekday
//! profile trades slack between days and cuts the bad days, without moving the mean — what remains of
//! wait is what no longer turns into abandonment. With three replicas per day the number of R0 stays at
//! 84.7% (standard error of 0.6 point over the 81 days), which does not close the distance. The divergence
//! stays declared, with both values, without rounding one into the other: the number generator is
//! another one (SplitMix64 against PCG64) and the queue is not identical call by call.
//!
//! The retired instrument counted as "served within 20 s" also whoever gave up with patience
//! drawn below 20 s (the wait of the abandoner is their own patience). The
//! reconstruction does the same, so as not to change the target midway; `abandonments_per_100`
//! comes out alongside, for whoever wants to read the account without that bias.

use crate::banco::{self, GOAL};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

/// Fixed seed of the instrument: the one in the registry (Phi1 — simulability).
pub const SEED: u64 = 2003;
/// Exponential service with a mean of 4 minutes, as in the registry.
pub const AHT_QUEUE_S: f64 = 240.0;
/// Exponential patience with a mean of 3 minutes, as in the registry.
pub const PATIENCE_S: f64 = 180.0;
/// Wait target, in seconds.
pub const TARGET_QUEUE_S: f64 = 20.0;
/// Interval of the base, in seconds.
pub const BIN_S: f64 = 300.0;
/// 5-min intervals per day: 07:00 to 21:00, the same cut as the retired instrument.
pub const BINS: usize = 169;
/// Intervals per roster group (half an hour).
pub const PER_GROUP: usize = 6;
/// Roster groups per day: 28 half hours plus the tail of one interval.
pub const GROUPS: usize = BINS.div_ceil(PER_GROUP);
/// Duration of a full roster group, in seconds.
pub const GROUP_S: f64 = BIN_S * PER_GROUP as f64;
/// 90% quantile of the normal: the margin of R2 is `1 + Z_MARGIN * cv`.
pub const Z_MARGIN: f64 = 1.2816;
/// Ceiling of the uniform generator: 2^53, so that it fits exactly in f64.
const TWO53: f64 = 9_007_199_254_740_992.0;

/// The four scaling rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    R0,
    R1,
    R2,
    R3,
}

impl Rule {
    /// The four, in the order of the registry.
    pub const ALL: [Rule; 4] = [Rule::R0, Rule::R1, Rule::R2, Rule::R3];

    pub fn name(self) -> &'static str {
        match self {
            Rule::R0 => "R0",
            Rule::R1 => "R1",
            Rule::R2 => "R2",
            Rule::R3 => "R3",
        }
    }
}

/// Queue engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Engine {
    /// Reconstruction of the retired instrument: exact FCFS with the observed arrivals.
    Exact,
    /// Independent verification: M/M/c+M event by event, Poisson arrivals.
    Events,
}

/// Declared parameters of the queue.
#[derive(Debug, Clone, Copy)]
pub struct Parameters {
    pub aht_s: f64,
    pub patience_s: f64,
    pub target_s: f64,
    pub goal: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            aht_s: AHT_QUEUE_S,
            patience_s: PATIENCE_S,
            target_s: TARGET_QUEUE_S,
            goal: GOAL,
        }
    }
}

/// Resolves the data path: optional base in ESTOCOLOGIA_DADOS, otherwise the repository.
pub fn data_path(default: &str) -> String {
    if let Ok(base) = std::env::var("ESTOCOLOGIA_DADOS") {
        return format!("{}/{}", base.trim_end_matches('/'), default);
    }
    for prefix in ["", "../", "../../"] {
        let path = format!("{prefix}{default}");
        if std::path::Path::new(&path).exists() {
            return path;
        }
    }
    default.to_string()
}

/// Month of an ISO date (YYYY-MM-DD).
pub fn month_of(date: &str) -> u32 {
    date.get(5..7).and_then(|m| m.parse().ok()).unwrap_or(0)
}

/// Weekday of an ISO date: 0 = Monday, 6 = Sunday (Sakamoto's algorithm).
pub fn weekday(date: &str) -> usize {
    let year: i32 = date.get(0..4).and_then(|x| x.parse().ok()).unwrap_or(0);
    let m: i32 = date.get(5..7).and_then(|x| x.parse().ok()).unwrap_or(1);
    let d: i32 = date.get(8..10).and_then(|x| x.parse().ok()).unwrap_or(1);
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if m < 3 { year - 1 } else { year };
    let w = (y + y / 4 - y / 100 + y / 400 + T[(m - 1).clamp(0, 11) as usize] + d).rem_euclid(7);
    // Sakamoto returns 0 = Sunday; here 0 = Monday.
    ((w + 6).rem_euclid(7)) as usize
}

/// One day of the base on the 5-min grid.
#[derive(Debug, Clone)]
pub struct BinnedDay {
    pub date: String,
    pub bins: [f64; BINS],
    pub total: f64,
}

/// Reads the base (rownames,DateTime,Calls) onto the 5-min grid from 07:00 to 21:00.
pub fn load_bins(path: &str) -> Vec<BinnedDay> {
    let text = std::fs::read_to_string(path).expect("could not read the bank CSV");
    let mut by_day: std::collections::BTreeMap<String, [f64; BINS]> =
        std::collections::BTreeMap::new();
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
        if !(7..=21).contains(&hour) {
            continue;
        }
        let i = ((hour - 7) * 60 + minute) / 5;
        if i >= BINS {
            continue;
        }
        let calls: f64 = fields[2].parse().unwrap_or(0.0);
        by_day.entry(date.to_string()).or_insert([0.0; BINS])[i] += calls;
    }
    by_day
        .into_iter()
        .map(|(date, bins)| BinnedDay {
            date,
            bins,
            total: bins.iter().sum(),
        })
        .filter(|d| d.total > 0.0)
        .collect()
}

/// Training (March to June) and vault (July to October), as in the registry.
pub fn split_training_vault(days: &[BinnedDay]) -> (Vec<BinnedDay>, Vec<BinnedDay>) {
    let training = days
        .iter()
        .filter(|d| (3..=6).contains(&month_of(&d.date)))
        .cloned()
        .collect();
    let vault = days
        .iter()
        .filter(|d| (7..=10).contains(&month_of(&d.date)))
        .cloned()
        .collect();
    (training, vault)
}

/// Uniform in (0, 1).
fn uniform01(rng: &mut banco::Rng) -> f64 {
    (rng.uniform(1u64 << 53) as f64 + 0.5) / TWO53
}

/// Exponential with the requested mean (infinity when the mean is infinite: nobody gives up).
fn exponential(rng: &mut banco::Rng, mean: f64) -> f64 {
    if !mean.is_finite() || mean <= 0.0 {
        return f64::INFINITY;
    }
    -mean * uniform01(rng).ln()
}

/// Seed of a day and a replica, by the SplitMix64 finalisation.
fn day_seed(base: u64, day: usize, replica: usize) -> u64 {
    let mut z = (stochology_core::rng::offset_seed(base)
        ^ (day as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (replica as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F))
    .wrapping_sub(0x9E37_79B9_7F4A_7C15);
    stochology_core::rng::splitmix64_step(&mut z)
}

/// First and last interval of each roster group.
fn group_range(j: usize) -> (usize, usize) {
    (j * PER_GROUP, ((j + 1) * PER_GROUP).min(BINS))
}

/// Agent-hours of a day: each group is worth its duration in hours.
pub fn agent_hours(c: &[u32; GROUPS]) -> f64 {
    (0..GROUPS)
        .map(|j| {
            let (a, b) = group_range(j);
            c[j] as f64 * ((b - a) as f64 * BIN_S / 3600.0)
        })
        .sum()
}

/// Smallest roster that meets the goal in each group, by the formula of the claim (Erlang C) over the
/// arrival profile per 5-min interval, with the demand multiplied by `factor`.
pub fn profile_scale(profile: &[f64; BINS], factor: f64, p: &Parameters) -> [u32; GROUPS] {
    let mut c = [1_u32; GROUPS];
    for j in 0..GROUPS {
        let (a, b) = group_range(j);
        let n = (b - a) as f64;
        let per_second = profile[a..b].iter().sum::<f64>() / (n * BIN_S);
        c[j] = banco::agents_for_goal(factor * per_second * 3600.0, p.aht_s, p.target_s, p.goal);
    }
    c
}

/// Mean profile of the training window, in calls per 5-min interval.
pub fn mean_profile(training: &[BinnedDay]) -> [f64; BINS] {
    assert!(!training.is_empty(), "empty training window");
    let mut mean = [0.0_f64; BINS];
    for d in training {
        for i in 0..BINS {
            mean[i] += d.bins[i];
        }
    }
    mean.map(|x| x / training.len() as f64)
}

/// Profile of each weekday (0 = Monday) and how many days support it. A weekday without
/// a day in the training window falls back to the mean profile (declared: the base has neither Saturday nor Sunday).
pub fn profiles_by_weekday(training: &[BinnedDay]) -> ([[f64; BINS]; 7], [usize; 7]) {
    let mut sum = [[0.0_f64; BINS]; 7];
    let mut count = [0usize; 7];
    for d in training {
        let w = weekday(&d.date);
        count[w] += 1;
        for i in 0..BINS {
            sum[w][i] += d.bins[i];
        }
    }
    let mean = mean_profile(training);
    let mut profiles = [[0.0_f64; BINS]; 7];
    for w in 0..7 {
        for i in 0..BINS {
            profiles[w][i] = if count[w] == 0 {
                mean[i]
            } else {
                sum[w][i] / count[w] as f64
            };
        }
    }
    (profiles, count)
}

/// Sample standard deviation.
fn sd(xs: &[f64]) -> f64 {
    if xs.len() < 2 {
        return 0.0;
    }
    let m = xs.iter().sum::<f64>() / xs.len() as f64;
    (xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (xs.len() as f64 - 1.0)).sqrt()
}

/// The residual CV of the registry: standard deviation of the ratio between the day total and the mean of the total
/// of its weekday, in the training window (the "6%" of chapter 12). It is what the margin of R2 scales.
pub fn daily_residual_cv(training: &[BinnedDay]) -> f64 {
    let (profiles, count) = profiles_by_weekday(training);
    let mut ratios = Vec::new();
    for d in training {
        let w = weekday(&d.date);
        if count[w] == 0 {
            continue;
        }
        let base: f64 = profiles[w].iter().sum();
        if base > 0.0 {
            ratios.push(d.total / base);
        }
    }
    sd(&ratios)
}

/// Min-heap of service completion times (event engine).
#[derive(Debug, Clone, Copy, PartialEq)]
struct Time(f64);

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        // inverted: the std BinaryHeap is a max-heap
        other.0.total_cmp(&self.0)
    }
}

/// Min-heap of patience deadlines, with the call identifier (event engine).
#[derive(Debug, Clone, Copy)]
struct Deadline {
    t: f64,
    id: u32,
}

impl PartialEq for Deadline {
    fn eq(&self, other: &Self) -> bool {
        self.t.to_bits() == other.t.to_bits() && self.id == other.id
    }
}

impl Eq for Deadline {}

impl PartialOrd for Deadline {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Deadline {
    fn cmp(&self, other: &Self) -> Ordering {
        other.t.total_cmp(&self.t).then(other.id.cmp(&self.id))
    }
}

/// What one vault day yielded. The waits are those of every offered call: whoever
/// gives up enters with their own patience, as in the retired instrument.
#[derive(Debug, Clone, Copy, Default)]
pub struct DayMeasure {
    pub arrivals: u64,
    pub within_target: u64,
    pub abandonments: u64,
    pub waiting_at_end: u64,
    pub mean_wait_s: f64,
    pub median_wait_s: f64,
    pub p90_wait_s: f64,
    pub mean_wait_of_abandoners_s: f64,
}

impl DayMeasure {
    /// Service level: fraction of the offered calls served within the target.
    pub fn service_level(&self) -> f64 {
        if self.arrivals == 0 {
            1.0
        } else {
            self.within_target as f64 / self.arrivals as f64
        }
    }

    /// Abandonments in every 100 offered calls.
    pub fn abandonments_per_100(&self) -> f64 {
        if self.arrivals == 0 {
            0.0
        } else {
            100.0 * self.abandonments as f64 / self.arrivals as f64
        }
    }
}

/// Nearest-rank quantile, over an already sorted vector.
fn quantile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let rank = ((q * sorted.len() as f64).ceil() as usize).max(1) - 1;
    sorted[rank.min(sorted.len() - 1)]
}

/// Closes the statistics of a day from the waits of every offered call and from the
/// mark of whoever gave up.
fn summarize(
    waits: &mut [f64],
    abandoners: &[f64],
    waiting_at_end: u64,
    p: &Parameters,
) -> DayMeasure {
    let arrivals = waits.len() as u64;
    let inside = waits.iter().filter(|e| **e <= p.target_s).count() as u64;
    let mean = if waits.is_empty() {
        0.0
    } else {
        waits.iter().sum::<f64>() / waits.len() as f64
    };
    waits.sort_by(f64::total_cmp);
    DayMeasure {
        arrivals,
        within_target: inside,
        abandonments: abandoners.len() as u64,
        waiting_at_end,
        mean_wait_s: mean,
        median_wait_s: quantile(waits, 0.5),
        p90_wait_s: quantile(waits, 0.9),
        mean_wait_of_abandoners_s: if abandoners.is_empty() {
            0.0
        } else {
            abandoners.iter().sum::<f64>() / abandoners.len() as f64
        },
    }
}

/// Exact engine of the retired instrument: the observed arrivals of the day, spread inside
/// each 5-min interval, exponential service, exponential patience and FCFS without
/// preemption. Whoever would suffer a wait longer than their own patience gives up and takes no one;
/// whoever does not give up takes the agent with the smallest free time (the lowest index on ties).
pub fn exact_queue(day: &BinnedDay, c: &[u32; GROUPS], p: &Parameters, seed: u64) -> DayMeasure {
    let mut rng = banco::Rng::new(seed);
    let n: usize = day.bins.iter().map(|x| x.round()).sum::<f64>() as usize;

    let mut arrivals = Vec::with_capacity(n);
    for i in 0..BINS {
        let k = day.bins[i].round() as usize;
        for _ in 0..k {
            arrivals.push(i as f64 * BIN_S + uniform01(&mut rng) * BIN_S);
        }
    }
    let service: Vec<f64> = (0..n).map(|_| exponential(&mut rng, p.aht_s)).collect();
    let patience: Vec<f64> = (0..n)
        .map(|_| {
            if p.patience_s.is_finite() {
                exponential(&mut rng, p.patience_s)
            } else {
                f64::INFINITY
            }
        })
        .collect();

    // in which roster group agent k is on shift, starting from each group
    let k_max = c.iter().copied().max().unwrap_or(0) as usize;
    let mut nxt = vec![-1_i32; k_max * GROUPS];
    for k in 0..k_max {
        let mut next = -1_i32;
        for j in (0..GROUPS).rev() {
            if (k as u32) < c[j] {
                next = j as i32;
            }
            nxt[k * GROUPS + j] = next;
        }
    }
    let mut free = vec![0.0_f64; k_max];

    let mut waits = Vec::with_capacity(n);
    let mut abandoners = Vec::new();
    for i in 0..n {
        let a = arrivals[i];
        let mut best = f64::INFINITY;
        let mut chosen = usize::MAX;
        for k in 0..k_max {
            let t0 = if free[k] > a { free[k] } else { a };
            let j = (t0 / GROUP_S) as usize;
            let v = if j >= GROUPS {
                if (k as u32) < c[GROUPS - 1] {
                    t0
                } else {
                    f64::INFINITY
                }
            } else if (k as u32) < c[j] {
                t0
            } else {
                let x = nxt[k * GROUPS + j];
                if x >= 0 {
                    x as f64 * GROUP_S
                } else {
                    f64::INFINITY
                }
            };
            if v < best {
                best = v;
                chosen = k;
            }
            if v == a {
                // agent already free at the instant of arrival: it is them, and nobody cheaper
                break;
            }
        }
        if !best.is_finite() || best - a > patience[i] {
            // whoever gives up waits their own patience; with no agent available in the day, the
            // wait stays infinite (the retired instrument recorded the same, via numpy's -1)
            let e = if best.is_finite() {
                patience[i]
            } else {
                f64::INFINITY
            };
            waits.push(e);
            abandoners.push(e);
        } else {
            waits.push(best - a);
            free[chosen] = best + service[i];
        }
    }
    summarize(&mut waits, &abandoners, 0, p)
}

/// Reads from the front of the queue the next call that still waits (those that gave up stay behind and
/// are discarded on reading).
fn next_in_queue(queue: &mut VecDeque<u32>, state: &[u8]) -> Option<u32> {
    while let Some(id) = queue.pop_front() {
        if state[id as usize] == 0 {
            return Some(id);
        }
    }
    None
}

/// Verification engine: M/M/c+M event by event on the same grid. Poisson arrivals with
/// the observed rate of each 5-min interval, constant roster inside the group, exponential service and
/// patience. The queue crosses the intervals; whoever is still waiting at 21:00 stays
/// declared in `waiting_at_end`.
pub fn event_queue(day: &BinnedDay, c: &[u32; GROUPS], p: &Parameters, seed: u64) -> DayMeasure {
    let mut rng = banco::Rng::new(seed);
    let mut state: Vec<u8> = Vec::new(); // 0 waiting, 1 in service, 2 gave up
    let mut arrival: Vec<f64> = Vec::new();
    let mut waits: Vec<f64> = Vec::new();
    let mut abandoners: Vec<f64> = Vec::new();
    let mut queue: VecDeque<u32> = VecDeque::new();
    let mut services: BinaryHeap<Time> = BinaryHeap::new();
    let mut deadlines: BinaryHeap<Deadline> = BinaryHeap::new();
    let mut t = 0.0_f64;
    let mut next_arrival: f64;

    for i in 0..BINS {
        let capacity = c[i / PER_GROUP] as usize;
        let lam = day.bins[i] / BIN_S;
        next_arrival = t + exponential(&mut rng, if lam > 0.0 { 1.0 / lam } else { f64::INFINITY });
        let end = (i + 1) as f64 * BIN_S;
        loop {
            while services.len() < capacity {
                match next_in_queue(&mut queue, &state) {
                    Some(id) => {
                        services.push(Time(t + exponential(&mut rng, p.aht_s)));
                        state[id as usize] = 1;
                        waits.push(t - arrival[id as usize]);
                    }
                    None => break,
                }
            }
            // next event: 0 service completion, 1 abandonment, 2 arrival, 3 end of interval
            let mut event = 3u8;
            let mut when = end;
            if let Some(s) = services.peek()
                && s.0 < when
            {
                when = s.0;
                event = 0;
            }
            if let Some(d) = deadlines.peek()
                && d.t < when
            {
                when = d.t;
                event = 1;
            }
            if next_arrival < when {
                when = next_arrival;
                event = 2;
            }
            match event {
                0 => {
                    services.pop();
                }
                1 => {
                    let d = deadlines.pop().expect("deadline in the queue");
                    if state[d.id as usize] == 0 {
                        state[d.id as usize] = 2;
                        let e = when - arrival[d.id as usize];
                        waits.push(e);
                        abandoners.push(e);
                    }
                }
                2 => {
                    t = when;
                    let id = state.len() as u32;
                    state.push(0);
                    arrival.push(t);
                    queue.push_back(id);
                    deadlines.push(Deadline {
                        t: t + exponential(&mut rng, p.patience_s),
                        id,
                    });
                    next_arrival = t + exponential(
                        &mut rng,
                        if lam > 0.0 { 1.0 / lam } else { f64::INFINITY },
                    );
                }
                _ => break,
            }
            if event != 2 {
                t = when;
            }
        }
    }
    let waiting_at_end = state.iter().filter(|e| **e == 0).count() as u64;
    summarize(&mut waits, &abandoners, waiting_at_end, p)
}

/// The four rosters, group by group, and the residual margin that R2 uses.
#[derive(Debug, Clone)]
pub struct Scales {
    pub r0: [u32; GROUPS],
    pub r1: [[u32; GROUPS]; 7],
    pub r2: [[u32; GROUPS]; 7],
    pub r3: [u32; GROUPS],
    pub extra_r3: u32,
    pub cv_residual: f64,
    pub factor_r2: f64,
    pub days_per_weekday: [usize; 7],
}

impl Scales {
    /// The roster that the rule applies on that day.
    pub fn scale(&self, rule: Rule, day: &BinnedDay) -> &[u32; GROUPS] {
        let w = weekday(&day.date);
        match rule {
            Rule::R0 => &self.r0,
            Rule::R1 => &self.r1[w],
            Rule::R2 => &self.r2[w],
            Rule::R3 => &self.r3,
        }
    }
}

/// Builds the four rosters: R0 on the mean training profile, R1 on the weekday profile, R2
/// with the demand multiplied by `1 + Z_MARGIN * cv` and R3 with R0 plus the smallest number of
/// agents across the board that matches the cost of R2 in the vault.
pub fn build_scales(training: &[BinnedDay], vault: &[BinnedDay], p: &Parameters) -> Scales {
    let cv = daily_residual_cv(training);
    let factor = 1.0 + Z_MARGIN * cv;
    let r0 = profile_scale(&mean_profile(training), 1.0, p);
    let (profiles, count) = profiles_by_weekday(training);
    let mut r1 = [[1_u32; GROUPS]; 7];
    let mut r2 = [[1_u32; GROUPS]; 7];
    for w in 0..7 {
        r1[w] = profile_scale(&profiles[w], 1.0, p);
        r2[w] = profile_scale(&profiles[w], factor, p);
    }
    let cost_r2 = if vault.is_empty() {
        0.0
    } else {
        vault
            .iter()
            .map(|d| agent_hours(&r2[weekday(&d.date)]))
            .sum::<f64>()
            / vault.len() as f64
    };
    let mut extra = 0_u32;
    while extra < 1_000 && agent_hours(&r0.map(|x| x + extra)) < cost_r2 {
        extra += 1;
    }
    Scales {
        r0,
        r1,
        r2,
        r3: r0.map(|x| x + extra),
        extra_r3: extra,
        cv_residual: cv,
        factor_r2: factor,
        days_per_weekday: count,
    }
}

/// What one rule yielded in the vault.
#[derive(Debug, Clone, Copy)]
pub struct RuleMeasure {
    pub rule: Rule,
    pub engine: Engine,
    pub agent_hours_per_day: f64,
    pub peak: u32,
    pub service_level: f64,
    pub mean_level_of_days: f64,
    pub days_below_goal: f64,
    pub abandonments_per_100: f64,
    pub mean_wait_s: f64,
    pub median_wait_s: f64,
    pub p90_wait_s: f64,
    pub waiting_at_end: f64,
}

/// Measures one rule in the vault: one queue per day and per replica, with a fixed seed.
pub fn measure_rule(
    vault: &[BinnedDay],
    scales: &Scales,
    rule: Rule,
    seed: u64,
    replicas: usize,
    engine: Engine,
    p: &Parameters,
) -> RuleMeasure {
    let replicas = replicas.max(1);
    let mut sum_calls = 0.0;
    let mut sum_inside = 0.0;
    let mut sum_abandonments = 0.0;
    let mut sum_wait = 0.0;
    let mut mean_waits: Vec<f64> = Vec::new();
    let mut median_waits: Vec<f64> = Vec::new();
    let mut waits_p90: Vec<f64> = Vec::new();
    let mut waiting = 0.0;
    let mut sum_day_levels = 0.0;
    let mut days_below = 0.0;
    let mut hours = 0.0;
    let mut peak = 0_u32;

    for (d, day) in vault.iter().enumerate() {
        let scale = scales.scale(rule, day);
        hours += agent_hours(scale);
        peak = peak.max(scale.iter().copied().max().unwrap_or(0));
        let mut day_level = 0.0;
        for r in 0..replicas {
            let seed_day = day_seed(seed, d, r);
            let m = match engine {
                Engine::Exact => exact_queue(day, scale, p, seed_day),
                Engine::Events => event_queue(day, scale, p, seed_day),
            };
            sum_calls += m.arrivals as f64;
            sum_inside += m.within_target as f64;
            sum_abandonments += m.abandonments as f64;
            sum_wait += m.mean_wait_s;
            waiting += m.waiting_at_end as f64;
            mean_waits.push(m.mean_wait_s);
            median_waits.push(m.median_wait_s);
            waits_p90.push(m.p90_wait_s);
            day_level += m.service_level();
        }
        day_level /= replicas as f64;
        sum_day_levels += day_level;
        if day_level < p.goal {
            days_below += 1.0;
        }
    }
    let n = vault.len() as f64;
    RuleMeasure {
        rule,
        engine,
        agent_hours_per_day: hours / n,
        peak,
        service_level: sum_inside / sum_calls,
        mean_level_of_days: sum_day_levels / n,
        days_below_goal: days_below / n,
        abandonments_per_100: 100.0 * sum_abandonments / sum_calls,
        mean_wait_s: sum_wait / (n * replicas as f64),
        median_wait_s: quantile(&median_waits, 0.5),
        p90_wait_s: quantile(&waits_p90, 0.5),
        waiting_at_end: waiting / (n * replicas as f64),
    }
}

/// The complete report of the real H0.
#[derive(Debug, Clone)]
pub struct Report {
    pub training: usize,
    pub vault: usize,
    pub vault_calls: f64,
    pub replicas: usize,
    pub seed: u64,
    pub cv_residual: f64,
    pub factor_r2: f64,
    pub extra_r3: u32,
    pub days_per_weekday: [usize; 7],
    pub engine: Engine,
    pub measures: Vec<RuleMeasure>,
}

/// Loads the base, splits training and vault and measures the four rules on one queue engine.
pub fn measure_rules(
    path: &str,
    seed: u64,
    replicas: usize,
    engine: Engine,
    p: &Parameters,
) -> Report {
    let days = load_bins(path);
    let (training, vault) = split_training_vault(&days);
    assert!(
        !training.is_empty() && !vault.is_empty(),
        "training window or vault empty in the base"
    );
    let scales = build_scales(&training, &vault, p);
    let measures = Rule::ALL
        .iter()
        .map(|r| measure_rule(&vault, &scales, *r, seed, replicas, engine, p))
        .collect();
    Report {
        training: training.len(),
        vault: vault.len(),
        vault_calls: vault.iter().map(|d| d.total).sum(),
        replicas,
        seed,
        cv_residual: scales.cv_residual,
        factor_r2: scales.factor_r2,
        extra_r3: scales.extra_r3,
        days_per_weekday: scales.days_per_weekday,
        engine,
        measures,
    }
}

/// Prints the head of the report, common to the two engines.
fn print_header(rel: &Report, p: &Parameters) {
    println!("== Real H0 :: the queue with abandonment and the four scaling rules (R0 to R3) ==");
    println!(
        "training {} days (Mar-Jun) | vault {} days (Jul-Oct) | {:.0} calls in the vault",
        rel.training, rel.vault, rel.vault_calls
    );
    println!(
        "exponential service {:.0} s | exponential patience {:.0} s | target {:.0} s | goal {:.0}% | seed {} | {} replicas per day",
        p.aht_s,
        p.patience_s,
        p.target_s,
        p.goal * 100.0,
        rel.seed,
        rel.replicas
    );
    println!(
        "residual CV of the day {:.4} | margin of R2 = 1 + {:.4} x CV = {:.4} | R3 = R0 + {} agents across the board",
        rel.cv_residual, Z_MARGIN, rel.factor_r2, rel.extra_r3
    );
    println!(
        "days in the training window per weekday {:?}",
        rel.days_per_weekday
    );
}

/// Prints the table of one engine.
fn print_table(measures: &[RuleMeasure]) {
    println!(
        "{:<5} {:>17} {:>6} {:>17} {:>22} {:>19} {:>15} {:>12} {:>11}",
        "rule",
        "agent-hours/day",
        "peak",
        "service level",
        "days below the goal",
        "abandon/100",
        "mean wait s",
        "median wait s",
        "p90 wait s"
    );
    for m in measures {
        println!(
            "{:<5} {:>17.0} {:>6} {:>16.1}% {:>21.1}% {:>19.1} {:>15.0} {:>12.0} {:>11.0}",
            m.rule.name(),
            m.agent_hours_per_day,
            m.peak,
            m.service_level * 100.0,
            m.days_below_goal * 100.0,
            m.abandonments_per_100,
            m.mean_wait_s,
            m.median_wait_s,
            m.p90_wait_s
        );
    }
}

/// Prints the whole real H0: the four rules on the two queue engines.
pub fn run() {
    let path = data_path("data/bank_calls.csv");
    let p = Parameters::default();
    let exact = measure_rules(&path, SEED, 3, Engine::Exact, &p);
    print_header(&exact, &p);
    println!();
    println!("Exact engine (observed arrivals, FCFS with abandonment) — the reconstruction");
    print_table(&exact.measures);
    println!();
    println!("Events engine (M/M/c+M event by event, Poisson arrivals) — independent verification");
    let events = measure_rules(&path, SEED, 3, Engine::Events, &p);
    print_table(&events.measures);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    /// The repository base; without it the measurement tests have no data and declare themselves skipped.
    fn path() -> Option<String> {
        let c = data_path("data/bank_calls.csv");
        if std::path::Path::new(&c).exists() {
            Some(c)
        } else {
            None
        }
    }

    /// A single measurement, reused by the tests: fixed seed, one replica per day, as the
    /// retired instrument.
    fn report() -> Option<&'static Report> {
        static REL: OnceLock<Option<Report>> = OnceLock::new();
        REL.get_or_init(|| {
            let c = path()?;
            Some(measure_rules(
                &c,
                SEED,
                1,
                Engine::Exact,
                &Parameters::default(),
            ))
        })
        .as_ref()
    }

    /// Synthetic day with the same count in every interval, to force overload.
    fn overloaded_day(per_interval: f64) -> BinnedDay {
        let bins = [per_interval; BINS];
        BinnedDay {
            date: "2003-07-01".to_string(),
            bins,
            total: bins.iter().sum(),
        }
    }

    #[test]
    fn weekday_of_known_dates() {
        assert_eq!(weekday("2000-01-01"), 5, "Saturday");
        assert_eq!(weekday("2024-01-01"), 0, "Monday");
        assert_eq!(
            weekday("2003-03-03"),
            0,
            "the first Monday of the training window"
        );
        assert_eq!(weekday("2003-08-31"), 6, "Sunday");
    }

    #[test]
    fn grid_is_the_cut_of_the_instrument() {
        // 169 5-min intervals from 07:00 to 21:00, 28 half hours plus the tail of one interval
        assert_eq!(BINS, 169);
        assert_eq!(PER_GROUP, 6);
        assert_eq!(GROUPS, 29);
        assert_eq!(group_range(27), (162, 168));
        assert_eq!(group_range(28), (168, 169));
        // agent-hours of a constant roster: 14.0833 h of shift per agent
        let uniform = [10_u32; GROUPS];
        let expected = 10.0 * BINS as f64 * BIN_S / 3600.0;
        assert!(
            (agent_hours(&uniform) - expected).abs() < 1e-9,
            "{}",
            agent_hours(&uniform)
        );
    }

    #[test]
    fn queue_with_abandonment_stabilises() {
        let p = Parameters::default();
        // 1000 arrivals per 5-min interval against 40 agents, who serve 15/hour each:
        // the offered load is twice the capacity, the worst case for a queue without abandonment
        let day = overloaded_day(1000.0);
        let scale = [40_u32; GROUPS];

        // 1. the exact engine: with patience, the wait stops at the patience ceiling
        let with = exact_queue(&day, &scale, &p, 7);
        assert!(
            with.abandonments_per_100() > 50.0,
            "abandonment {}",
            with.abandonments_per_100()
        );
        assert!(
            with.p90_wait_s <= 3.0 * p.patience_s,
            "p90 of the wait {} against patience {}",
            with.p90_wait_s,
            p.patience_s
        );
        let without = Parameters {
            patience_s: f64::INFINITY,
            ..p
        };
        let no_abandonment = exact_queue(&day, &scale, &without, 7);
        assert_eq!(no_abandonment.abandonments, 0);
        assert!(
            no_abandonment.mean_wait_s > 10.0 * with.mean_wait_s,
            "without abandonment {:.0} s against {:.0} s",
            no_abandonment.mean_wait_s,
            with.mean_wait_s
        );

        // 2. the event engine: the queue stabilises with patience and grows without it
        let ev_with = event_queue(&day, &scale, &p, 7);
        let ev_without = event_queue(&day, &scale, &without, 7);
        // the surplus is 3 calls per second against 0.167 served per second, and the queue
        // that stabilises settles at (surplus) x (patience) = about 570 waiting
        assert!(
            (200..=2000).contains(&ev_with.waiting_at_end),
            "queue settled at {}",
            ev_with.waiting_at_end
        );
        assert!(ev_with.abandonments_per_100() > 50.0);
        assert!(
            ev_without.waiting_at_end > 10_000,
            "without abandonment the queue should accumulate, what was left is {}",
            ev_without.waiting_at_end
        );
        // the settled queue is proportional to the patience: with 90 s of patience the equilibrium
        // wait falls to about half, which is the signature of a queue with an exit
        let short = Parameters {
            patience_s: 90.0,
            ..p
        };
        let ev_short = event_queue(&day, &scale, &short, 7);
        let ratio = ev_short.waiting_at_end as f64 / ev_with.waiting_at_end as f64;
        assert!(
            (0.3..=0.7).contains(&ratio),
            "ratio of the settled queues {ratio}"
        );
        assert!(ev_without.waiting_at_end > 5 * ev_with.waiting_at_end);
    }

    #[test]
    fn monotonicity_r2_better_than_r0() {
        let Some(rel) = report() else {
            eprintln!("no base");
            return;
        };
        let (r0, r1, r2, r3) = (
            &rel.measures[0],
            &rel.measures[1],
            &rel.measures[2],
            &rel.measures[3],
        );
        assert!(
            r2.days_below_goal < r0.days_below_goal,
            "R2 {:.3} failed days against R0 {:.3}",
            r2.days_below_goal,
            r0.days_below_goal
        );
        assert!(r2.service_level > r0.service_level);
        assert!(r2.agent_hours_per_day > r0.agent_hours_per_day);
        assert!(
            r1.days_below_goal <= r0.days_below_goal,
            "R1 worse than the claim"
        );
        assert!(
            r3.days_below_goal <= r0.days_below_goal,
            "R3 worse than the claim"
        );
        assert!(
            r2.days_below_goal <= r3.days_below_goal,
            "R2 did not beat R3"
        );
        // R2 and R3 cost almost the same, and that is why the comparison is of allocation, not of spending
        let ratio = r3.agent_hours_per_day / r2.agent_hours_per_day;
        assert!((0.98..=1.02).contains(&ratio), "R3/R2 = {ratio}");
    }

    #[test]
    fn the_r2_margin_is_the_p90_of_the_residual_cv() {
        let Some(c) = path() else {
            return;
        };
        let p = Parameters::default();
        let days = load_bins(&c);
        let (training, vault) = split_training_vault(&days);
        let esc = build_scales(&training, &vault, &p);
        // the residual CV of chapter 12 is 6%; the instrument measures close to it
        assert!(
            esc.cv_residual > 0.04 && esc.cv_residual < 0.08,
            "cv {}",
            esc.cv_residual
        );
        let expected = 1.0 + Z_MARGIN * esc.cv_residual;
        assert!((esc.factor_r2 - expected).abs() < 1e-12);
        assert!(
            (1.05..=1.10).contains(&esc.factor_r2),
            "factor {}",
            esc.factor_r2
        );
        // R3 is "R0 plus agents across the board": the difference is constant, and it is the measured extra
        assert!(
            esc.extra_r3 >= 8 && esc.extra_r3 <= 16,
            "extra {}",
            esc.extra_r3
        );
        for j in 0..GROUPS {
            assert_eq!(esc.r3[j] - esc.r0[j], esc.extra_r3);
        }
        // the margin is allocated where the risk is: the peak of R2 exceeds that of R1, which exceeds that of R0
        let peak = |c: &[u32; GROUPS]| c.iter().copied().max().unwrap_or(0);
        for w in 0..5 {
            assert!(
                peak(&esc.r2[w]) > peak(&esc.r1[w]),
                "R2 without slack at the peak of day {w}"
            );
        }
        assert!(peak(&esc.r1[0]) > peak(&esc.r0));
        // R1 is "zero cost": the mean of the five weekdays stays within 1% of the cost of R0
        let mean_r1: f64 = (0..5).map(|w| agent_hours(&esc.r1[w])).sum::<f64>() / 5.0;
        let zero = (mean_r1 - agent_hours(&esc.r0)).abs() / agent_hours(&esc.r0);
        assert!(
            zero < 0.01,
            "R1 costs {mean_r1:.0} against {:.0}",
            agent_hours(&esc.r0)
        );
        // and the weekday profile is not the mean profile repeated
        assert_ne!(esc.r1[0], esc.r1[3]);
    }

    #[test]
    fn instrument_runs_with_a_fixed_seed() {
        let Some(c) = path() else {
            return;
        };
        let p = Parameters::default();
        let days = load_bins(&c);
        let (training, vault) = split_training_vault(&days);
        let esc = build_scales(&training, &vault, &p);
        let day = &vault[0];
        let scale = esc.scale(Rule::R0, day);
        let a = exact_queue(day, scale, &p, 42);
        let b = exact_queue(day, scale, &p, 42);
        assert_eq!(a.arrivals, b.arrivals);
        assert_eq!(a.within_target, b.within_target);
        assert_eq!(a.abandonments, b.abandonments);
        assert_eq!(a.mean_wait_s.to_bits(), b.mean_wait_s.to_bits());
        assert_eq!(a.p90_wait_s.to_bits(), b.p90_wait_s.to_bits());
        // and the same event engine, too
        let c1 = event_queue(day, scale, &p, 42);
        let c2 = event_queue(day, scale, &p, 42);
        assert_eq!(c1.arrivals, c2.arrivals);
        assert_eq!(c1.within_target, c2.within_target);
        assert_eq!(c1.mean_wait_s.to_bits(), c2.mean_wait_s.to_bits());
        // a neighbouring seed does not change the measure of the day beyond the noise of the instrument itself
        let d1 = exact_queue(day, scale, &p, 43);
        assert!((d1.service_level() - a.service_level()).abs() < 0.15);
        // two runs of the whole report with the same seed give the same number
        let r1 = measure_rules(&c, SEED, 1, Engine::Exact, &p);
        let r2 = measure_rules(&c, SEED, 1, Engine::Exact, &p);
        assert_eq!(
            r1.measures[0].service_level.to_bits(),
            r2.measures[0].service_level.to_bits()
        );
    }

    #[test]
    fn registry_bands_and_declared_divergence() {
        let Some(rel) = report() else {
            eprintln!("no base");
            return;
        };
        // the vault: 81 working days from July to October, 2.65 million offered calls
        assert_eq!(rel.training, 83);
        assert_eq!(rel.vault, 81);
        assert!((2_600_000.0..=2_700_000.0).contains(&rel.vault_calls));
        // the grid of the instrument includes the 21:00 interval; the half-hour reading of
        // banco.rs discards it, and that is why the count here is 5 596 calls larger
        assert!(rel.vault_calls > 2_649_354.0);

        for m in &rel.measures {
            assert!((2100.0..=2600.0).contains(&m.agent_hours_per_day), "{m:?}");
            assert!((0.0..=1.0).contains(&m.service_level));
            assert!((0.0..=1.0).contains(&m.days_below_goal));
        }
        let r0 = rel.measures[0];
        let r1 = rel.measures[1];
        let r2 = rel.measures[2];
        let r3 = rel.measures[3];

        // Agent-hours: the cost of the registry is reproduced rule by rule (2 264, 2 261, 2 428,
        // 2 433), within 1%.
        for (m, published) in [(r0, 2264.0), (r1, 2261.0), (r2, 2428.0), (r3, 2433.0)] {
            let error = (m.agent_hours_per_day - published).abs() / published;
            assert!(
                error < 0.01,
                "{} costs {:.0} against {:.0}",
                m.rule.name(),
                m.agent_hours_per_day,
                published
            );
        }
        assert_eq!(
            rel.extra_r3, 12,
            "R3 is R0 + 12 agents across the board in the registry"
        );

        // Service level and failed days: here the divergence is declared. The registry publishes
        // 86/26 (R0), 90/12 (R1), 96/3.7 (R2) and 94/7.4 (R3); the band below is generous on
        // purpose, because the number generator is another one and the draws fall elsewhere.
        let ns = |m: &RuleMeasure| m.service_level * 100.0;
        assert!((80.0..=90.0).contains(&ns(&r0)), "SL of R0 {:.1}", ns(&r0));
        assert!(
            ns(&r1) >= ns(&r0) - 1.0,
            "R1 {:.1} against R0 {:.1}",
            ns(&r1),
            ns(&r0)
        );
        assert!(ns(&r2) >= 89.0, "SL of R2 {:.1}", ns(&r2));
        assert!(ns(&r3) >= 88.0, "SL of R3 {:.1}", ns(&r3));
        assert!(
            (0.20..=0.42).contains(&r0.days_below_goal),
            "R0 fails {:.3}",
            r0.days_below_goal
        );
        assert!(
            (0.08..=0.28).contains(&r1.days_below_goal),
            "R1 fails {:.3}",
            r1.days_below_goal
        );
        assert!(
            (0.00..=0.10).contains(&r2.days_below_goal),
            "R2 fails {:.3}",
            r2.days_below_goal
        );
        assert!(
            (0.03..=0.18).contains(&r3.days_below_goal),
            "R3 fails {:.3}",
            r3.days_below_goal
        );
    }

    #[test]
    fn working_print() {
        let Some(rel) = report() else {
            eprintln!("no base");
            return;
        };
        let p = Parameters::default();
        print_header(rel, &p);
        println!("Exact engine (observed arrivals, FCFS with abandonment)");
        print_table(&rel.measures);
    }
}
