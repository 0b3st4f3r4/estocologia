//! The wave engine against data (empirical/p23-onda/DECLARACAO.md, sealed before this instrument first read a
//! dataset).
//!
//! Two claims of the engine, each with a truth it can miss.
//!
//! **T1, the arrow (layer 3, phasor STDP).** In Victoria, temperature drives electricity demand and not the
//! reverse. Each of 156 weeks of half-hours becomes two spike streams, thermal stress and demand surges, taken from
//! anomalies against the same half-hour of the six neighbouring days (which removes the daily cycle both series
//! share), with the phase of a spike set to its time of day. The engine learns one bus per week; its arrow is the
//! sign of eta(temperature -> demand) - eta(demand -> temperature). The rival is the lagged cross-correlation over
//! the same horizon; the null pairs each week's demand with the temperature of the week thirteen weeks later, which
//! keeps the time of day and the weekday and breaks the link.
//!
//! **T2, the conflict (layer 1, superposition, phase as time).** For a flight out of New York, each of its carrier,
//! origin and destination has a daily profile of delay risk learnt on the 60 days before the vault; its first
//! harmonic is a phasor whose phase is the hour of peak risk. The three superpose, and the engine's regime says
//! whether the fast additive forecast (System 1, the superposition itself read as an additive model) suffices or
//! deliberation (System 2) is needed. The claim: the gain of a model with interactions over the additive one is
//! larger in the conflict regime than in resonance. The rival
//! stratifies the same flights by the agreement of the three sources' levels, without phase.
//!
//! Deterministic: the bootstrap and the synthetic controls use SplitMix64 with seed 2026.

use std::collections::{BTreeMap, HashMap};

use stochology_core::Rng;
use stochology_wave::channel::{ChannelBus, Event};
use stochology_wave::cycle::{Point, first_harmonic, phase_of};
use stochology_wave::phasor::{Coupling, Regime, Thresholds, superpose};
use stochology_wave::seal::wrap_pi;
use stochology_wave::spike::StdpConfig;

use crate::streams::{
    days_from_civil, fit_logistic, load_flights, logit, num, parse_date, predict, read_csv,
};

pub const ENERGY: &str = "data/vic_elec.csv";
pub const FLIGHTS_2013: &str = "data/nyc_flights_2013.csv";
pub const FLIGHTS_2023: &str = "data/nyc_flights_2023.csv";
pub const SEED: u64 = 2026;

// ------------------------------------------------------------------ T1, the arrow

pub const SLOTS: usize = 48;
pub const WEEKS: usize = 156;
pub const PROFILE_DAYS: i64 = 3;
pub const THRESHOLD: f64 = 1.5;
pub const NULL_SHIFT_WEEKS: usize = 13;
pub const XCORR_LAGS: usize = 12;
pub const P1_ALPHA: f64 = 0.01;
pub const P1_MIN_RATE: f64 = 0.60;
pub const P2_MIN_GAP: f64 = 0.20;
pub const P3_MAX_LOSS: f64 = 0.10;
pub const ADDS_MIN_GAIN: f64 = 0.10;
pub const SECONDARY_MARGIN: f64 = 0.05;

/// The STDP rule on a half-hour clock: three hours of memory each way, six hours of horizon.
pub fn stdp_config() -> StdpConfig {
    StdpConfig {
        q: 0.5,
        a_plus: 0.18,
        a_minus: 0.12,
        tau_plus_us: 6.0,
        tau_minus_us: 6.0,
        lr_alpha: 0.30,
        oja: 0.04,
        horizon_us: 12,
    }
}

pub type Day = [f64; SLOTS];

/// Demand and temperature, one row of 48 half-hours per local date, in date order.
pub fn load_energy(path: &str) -> (Vec<Day>, Vec<Day>) {
    let (h, rows) = read_csv(path);
    let mut by_day: BTreeMap<i64, (Vec<f64>, Vec<f64>)> = BTreeMap::new();
    for r in &rows {
        let day = parse_date(&r[h["Date"]]);
        let e = by_day.entry(day).or_default();
        e.0.push(num(&r[h["Demand"]]).unwrap_or(0.0));
        e.1.push(num(&r[h["Temperature"]]).unwrap_or(0.0));
    }
    let mut demand = Vec::new();
    let mut temperature = Vec::new();
    for (_, (d, t)) in by_day {
        if d.len() == SLOTS {
            let mut a = [0.0; SLOTS];
            let mut b = [0.0; SLOTS];
            a.copy_from_slice(&d);
            b.copy_from_slice(&t);
            demand.push(a);
            temperature.push(b);
        }
    }
    (demand, temperature)
}

/// The anomaly of each half-hour against the mean of the same half-hour on the three days before and the three
/// after (fewer at the edges of the series).
pub fn anomaly(days: &[Day]) -> Vec<Day> {
    let n = days.len() as i64;
    (0..n)
        .map(|d| {
            let mut out = [0.0; SLOTS];
            for (s, o) in out.iter_mut().enumerate() {
                let mut sum = 0.0;
                let mut k = 0.0;
                for e in (d - PROFILE_DAYS)..=(d + PROFILE_DAYS) {
                    if e != d && e >= 0 && e < n {
                        sum += days[e as usize][s];
                        k += 1.0;
                    }
                }
                *o = days[d as usize][s] - if k > 0.0 { sum / k } else { 0.0 };
            }
            out
        })
        .collect()
}

fn window(anom: &[Day], week: usize) -> Vec<f64> {
    anom[week * 7..week * 7 + 7]
        .iter()
        .flat_map(|d| d.iter().copied())
        .collect()
}

fn zscore(v: &[f64]) -> Vec<f64> {
    let n = v.len().max(1) as f64;
    let m = v.iter().sum::<f64>() / n;
    let sd = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / n).sqrt();
    if sd <= 0.0 {
        return vec![0.0; v.len()];
    }
    v.iter().map(|x| (x - m) / sd).collect()
}

/// Spikes at the first half-hour of each excursion of the score at or above the threshold: for temperature the
/// score is |z| (heat or cold), for demand it is z (surges only). Amplitude min(1, peak / 3); phase the time of
/// day of the spike.
pub fn events(z: &[f64], channel: usize, absolute: bool) -> Vec<Event> {
    let score = |x: f64| if absolute { x.abs() } else { x };
    let mut out = Vec::new();
    let mut t = 0;
    while t < z.len() {
        if score(z[t]) >= THRESHOLD {
            let start = t;
            let mut peak = score(z[t]);
            while t < z.len() && score(z[t]) >= THRESHOLD {
                peak = peak.max(score(z[t]));
                t += 1;
            }
            out.push(Event::new(
                channel,
                start as u64,
                (peak / 3.0).min(1.0),
                phase_of((start % SLOTS) as f64, SLOTS as f64),
            ));
        } else {
            t += 1;
        }
    }
    out
}

/// The engine's reading of one week: the asymmetry temperature -> demand and the learnt lag, in hours.
pub fn engine_arrow(zt: &[f64], zd: &[f64]) -> (f64, f64) {
    let cfg = stdp_config();
    let mut bus = ChannelBus::new(2, 0.25);
    let mut all = events(zt, 0, true);
    all.extend(events(zd, 1, false));
    bus.publish_all(all, &cfg);
    let phase = wrap_pi(2.0 * std::f64::consts::PI * cfg.q * bus.alpha[0][1]);
    (
        bus.asymmetry(0, 1),
        phase / (2.0 * std::f64::consts::PI) * 24.0,
    )
}

fn corr(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    if n < 3 {
        return 0.0;
    }
    let (ma, mb) = (
        a[..n].iter().sum::<f64>() / n as f64,
        b[..n].iter().sum::<f64>() / n as f64,
    );
    let (mut sab, mut saa, mut sbb) = (0.0, 0.0, 0.0);
    for i in 0..n {
        let (x, y) = (a[i] - ma, b[i] - mb);
        sab += x * y;
        saa += x * x;
        sbb += y * y;
    }
    if saa <= 0.0 || sbb <= 0.0 {
        0.0
    } else {
        sab / (saa * sbb).sqrt()
    }
}

/// The rival: sum over lags 1..12 of corr(|z_T|_t, z_D,t+k) minus the same with the roles swapped.
pub fn xcorr_arrow(zt: &[f64], zd: &[f64]) -> f64 {
    let x: Vec<f64> = zt.iter().map(|v| v.abs()).collect();
    let n = x.len();
    let mut a = 0.0;
    for k in 1..=XCORR_LAGS {
        a += corr(&x[..n - k], &zd[k..]);
        a -= corr(&x[k..], &zd[..n - k]);
    }
    a
}

/// P(X >= k) for X ~ Binomial(n, 1/2).
pub fn binomial_upper(k: usize, n: usize) -> f64 {
    if k == 0 {
        return 1.0;
    }
    let ln_c = |n: usize, i: usize| -> f64 {
        (1..=n).map(|j| (j as f64).ln()).sum::<f64>()
            - (1..=i).map(|j| (j as f64).ln()).sum::<f64>()
            - (1..=(n - i)).map(|j| (j as f64).ln()).sum::<f64>()
    };
    (k..=n)
        .map(|i| (ln_c(n, i) - n as f64 * 2f64.ln()).exp())
        .sum::<f64>()
        .min(1.0)
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Tally {
    pub correct: usize,
    pub wrong: usize,
    pub none: usize,
}

impl Tally {
    fn add(&mut self, a: f64, margin: f64) {
        if a > margin {
            self.correct += 1;
        } else if a < -margin {
            self.wrong += 1;
        } else {
            self.none += 1;
        }
    }
    fn rate(&self, n: usize) -> f64 {
        self.correct as f64 / n.max(1) as f64
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowReading {
    pub weeks: usize,
    pub engine: Tally,
    pub engine_null: Tally,
    pub rival: Tally,
    pub rival_null: Tally,
    pub engine_margin: Tally,
    pub binomial_p: f64,
    pub median_lag_hours: f64,
    pub p1: bool,
    pub p2: bool,
    pub p3: bool,
    pub works: bool,
    pub adds: bool,
}

/// The whole of T1 over per-day anomalies of demand and temperature.
pub fn arrow_test(demand: &[Day], temperature: &[Day]) -> ArrowReading {
    let weeks = (demand.len().min(temperature.len()) / 7).min(WEEKS);
    let (mut engine, mut engine_null, mut rival, mut rival_null, mut engine_margin) = (
        Tally::default(),
        Tally::default(),
        Tally::default(),
        Tally::default(),
        Tally::default(),
    );
    let mut lags = Vec::new();
    for w in 0..weeks {
        let zd = zscore(&window(demand, w));
        let zt = zscore(&window(temperature, w));
        let zt_null = zscore(&window(temperature, (w + NULL_SHIFT_WEEKS) % weeks));
        let (a, lag) = engine_arrow(&zt, &zd);
        engine.add(a, 0.0);
        engine_margin.add(a, SECONDARY_MARGIN);
        if a > 0.0 {
            lags.push(lag);
        }
        engine_null.add(engine_arrow(&zt_null, &zd).0, 0.0);
        rival.add(xcorr_arrow(&zt, &zd), 0.0);
        rival_null.add(xcorr_arrow(&zt_null, &zd), 0.0);
    }
    lags.sort_by(f64::total_cmp);
    let median_lag_hours = if lags.is_empty() {
        f64::NAN
    } else if lags.len() % 2 == 1 {
        lags[lags.len() / 2]
    } else {
        0.5 * (lags[lags.len() / 2 - 1] + lags[lags.len() / 2])
    };
    let decided = engine.correct + engine.wrong;
    let binomial_p = binomial_upper(engine.correct, decided);
    let p1 =
        engine.correct > engine.wrong && binomial_p < P1_ALPHA && engine.rate(weeks) >= P1_MIN_RATE;
    let p2 = engine.rate(weeks) - engine_null.rate(weeks) >= P2_MIN_GAP;
    let p3 = rival.rate(weeks) - engine.rate(weeks) <= P3_MAX_LOSS;
    let adds = engine.rate(weeks) - rival.rate(weeks) >= ADDS_MIN_GAIN
        && engine_null.rate(weeks) <= rival_null.rate(weeks);
    ArrowReading {
        weeks,
        engine,
        engine_null,
        rival,
        rival_null,
        engine_margin,
        binomial_p,
        median_lag_hours,
        p1,
        p2,
        p3,
        works: p1 && p2 && p3,
        adds,
    }
}

fn gaussian(rng: &mut Rng) -> f64 {
    let u = |r: &mut Rng| ((r.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
    let (a, b) = (u(rng), u(rng));
    (-2.0 * a.ln()).sqrt() * (2.0 * std::f64::consts::PI * b).cos()
}

/// A synthetic Victoria: temperature with a season, a day and a persistent weather noise; demand with its own day
/// and a response to the thermal stress two hours earlier (planted) or to an unrelated noise (independent).
pub fn synthetic_energy(seed: u64, planted: bool) -> (Vec<Day>, Vec<Day>) {
    let mut rng = Rng::new(seed);
    let days = WEEKS * 7;
    let (mut e, mut f) = (0.0f64, 0.0f64);
    let mut weather = Vec::with_capacity(days * SLOTS);
    let mut other = Vec::with_capacity(days * SLOTS);
    for _ in 0..days * SLOTS {
        e = 0.97 * e + 0.5 * gaussian(&mut rng);
        f = 0.97 * f + 0.5 * gaussian(&mut rng);
        weather.push(e);
        other.push(f);
    }
    let tau = 2.0 * std::f64::consts::PI;
    let mut demand = Vec::with_capacity(days);
    let mut temperature = Vec::with_capacity(days);
    for d in 0..days {
        let mut dd = [0.0; SLOTS];
        let mut tt = [0.0; SLOTS];
        for s in 0..SLOTS {
            let t = d * SLOTS + s;
            let season = 6.0 * (tau * d as f64 / 365.0).sin();
            tt[s] = 18.0 + season + 4.0 * (tau * (s as f64 - 30.0) / 48.0).cos() + weather[t];
            let driver = if t >= 4 {
                if planted {
                    weather[t - 4]
                } else {
                    other[t - 4]
                }
            } else {
                0.0
            };
            dd[s] = 4500.0
                + 600.0 * (tau * (s as f64 - 36.0) / 48.0).cos()
                + 120.0 * driver.abs()
                + 40.0 * gaussian(&mut rng);
        }
        demand.push(dd);
        temperature.push(tt);
    }
    (demand, temperature)
}

// ------------------------------------------------------------------ T2, the conflict

pub const MIN_LEVEL: usize = 200;
pub const MIN_HOUR: usize = 10;
pub const CELL_PRIOR: f64 = 50.0;
pub const RIDGE: f64 = 1.0;
pub const MIN_STRATUM: usize = 500;
pub const BOOTSTRAP: usize = 2000;

/// One case: its day, its hour, its three sources (carrier, origin, destination) and its outcome.
#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    pub day: i64,
    pub hour: i64,
    pub levels: [String; 3],
    pub y: f64,
}

#[derive(Debug, Clone, Default)]
struct Level {
    n: usize,
    k: f64,
    by_hour: BTreeMap<i64, (usize, f64)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConflictReading {
    pub train: usize,
    pub included: usize,
    pub regimes: [usize; 4],
    pub brier_s1: f64,
    pub brier_s2: f64,
    pub gain_conflict: f64,
    pub gain_resonance: f64,
    pub delta: f64,
    pub ci: (f64, f64),
    pub delta_rival: f64,
    pub testable: bool,
}

fn band(hour: i64) -> usize {
    match hour {
        0..=8 => 0,
        9..=12 => 1,
        13..=16 => 2,
        17..=20 => 3,
        _ => 4,
    }
}

fn regime_index(r: Regime) -> usize {
    match r {
        Regime::Quiescent => 0,
        Regime::Resonance => 1,
        Regime::Beating => 2,
        Regime::Conflict => 3,
    }
}

/// The whole of T2 for one vault.
pub fn conflict_test(train: &[Case], vault: &[Case], seed: u64) -> ConflictReading {
    let k_all: f64 = train.iter().map(|c| c.y).sum();
    let global = (k_all + 1.0) / (train.len() as f64 + 2.0);

    let mut sources: [BTreeMap<String, Level>; 3] = Default::default();
    for c in train {
        for (s, level) in c.levels.iter().enumerate() {
            let l = sources[s].entry(level.clone()).or_default();
            l.n += 1;
            l.k += c.y;
            let h = l.by_hour.entry(c.hour).or_default();
            h.0 += 1;
            h.1 += c.y;
        }
    }
    // Phasor of each eligible level: first harmonic of its hourly log-odds over a 24-hour cycle.
    let mut phasors: [BTreeMap<String, (f64, f64)>; 3] = Default::default();
    let mut amplitudes = Vec::new();
    for s in 0..3 {
        for (name, l) in &sources[s] {
            if l.n < MIN_LEVEL {
                continue;
            }
            let points: Vec<Point> = l
                .by_hour
                .iter()
                .filter(|(_, (n, _))| *n >= MIN_HOUR)
                .map(|(&h, &(n, k))| Point {
                    position: h as f64,
                    value: logit((k + 1.0) / (n as f64 + 2.0)),
                    weight: n as f64,
                })
                .collect();
            let (a, phi) = first_harmonic(&points, 24.0);
            phasors[s].insert(name.clone(), (a, phi));
            amplitudes.push(a);
        }
    }
    amplitudes.sort_by(f64::total_cmp);
    let median = match amplitudes.len() {
        0 => 1.0,
        n if n % 2 == 1 => amplitudes[n / 2],
        n => 0.5 * (amplitudes[n / 2 - 1] + amplitudes[n / 2]),
    };
    let median = if median > 0.0 { median } else { 1.0 };

    // System 1: the superposition read as an additive model. Level offsets, five departure bands, and for each
    // source the harmonic of the case's level at the case's hour, A cos(2 pi h / 24 - phi), with its own
    // coefficient: what the phasors carry, and nothing that crosses two sources.
    let eligible: Vec<Vec<String>> = (0..3)
        .map(|s| phasors[s].keys().cloned().collect())
        .collect();
    let omega = 2.0 * std::f64::consts::PI / 24.0;
    let row = |c: &Case| -> Vec<f64> {
        let mut r = vec![1.0];
        for s in 0..3 {
            for name in eligible[s].iter().skip(1) {
                r.push(if &c.levels[s] == name { 1.0 } else { 0.0 });
            }
        }
        for b in 1..5 {
            r.push(if band(c.hour) == b { 1.0 } else { 0.0 });
        }
        for s in 0..3 {
            let (a, phi) = phasors[s].get(&c.levels[s]).copied().unwrap_or((0.0, 0.0));
            r.push(a * (omega * c.hour as f64 - phi).cos());
        }
        r
    };
    let x: Vec<Vec<f64>> = train.iter().map(&row).collect();
    let y: Vec<f64> = train.iter().map(|c| c.y).collect();
    let beta = fit_logistic(&x, &y, RIDGE);

    // System 2: the (carrier, origin, hour) cell, shrunk toward System 1 with 50 imaginary flights.
    let mut cells: HashMap<(String, String, i64), (f64, f64)> = HashMap::new();
    for c in train {
        let e = cells
            .entry((c.levels[0].clone(), c.levels[1].clone(), c.hour))
            .or_default();
        e.0 += 1.0;
        e.1 += c.y;
    }

    let thresholds = Thresholds::default();
    let mut regimes = [0usize; 4];
    let mut rows: Vec<(usize, i64, f64, f64, f64, f64)> = Vec::new(); // regime, day, gain, rival D, s1 sq, s2 sq
    for c in vault {
        let legs: Option<Vec<(f64, f64)>> = (0..3)
            .map(|s| phasors[s].get(&c.levels[s]).copied())
            .collect();
        let Some(legs) = legs else { continue };
        let couplings: Vec<Coupling> = legs
            .iter()
            .map(|&(a, phi)| Coupling::new(a / median, phi, 1.0, 0.0))
            .collect();
        let r = regime_index(superpose(&couplings, &thresholds).regime);
        regimes[r] += 1;
        let p1 = predict(&beta, &row(c));
        let (n, k) = cells
            .get(&(c.levels[0].clone(), c.levels[1].clone(), c.hour))
            .copied()
            .unwrap_or((0.0, 0.0));
        let p2 = (k + CELL_PRIOR * p1) / (n + CELL_PRIOR);
        let (s1, s2) = ((p1 - c.y).powi(2), (p2 - c.y).powi(2));
        let d: Vec<f64> = (0..3)
            .map(|s| {
                let l = &sources[s][&c.levels[s]];
                logit((l.k + 1.0) / (l.n as f64 + 2.0)) - logit(global)
            })
            .collect();
        let total: f64 = d.iter().map(|v| v.abs()).sum();
        let coherence = if total > 0.0 {
            d.iter().sum::<f64>().abs() / total
        } else {
            1.0
        };
        rows.push((r, c.day, s1 - s2, coherence, s1, s2));
    }

    let mean_gain = |pick: &dyn Fn(usize) -> bool| -> f64 {
        let v: Vec<f64> = rows
            .iter()
            .enumerate()
            .filter(|(i, _)| pick(*i))
            .map(|(_, r)| r.2)
            .collect();
        if v.is_empty() {
            f64::NAN
        } else {
            v.iter().sum::<f64>() / v.len() as f64
        }
    };
    let gain_conflict = mean_gain(&|i| rows[i].0 == 3);
    let gain_resonance = mean_gain(&|i| rows[i].0 == 1);
    let delta = gain_conflict - gain_resonance;

    // Rival: the same numbers of flights, the least and the most agreeing by level, without phase.
    let mut order: Vec<usize> = (0..rows.len()).collect();
    order.sort_by(|&a, &b| rows[a].3.total_cmp(&rows[b].3));
    let (nc, nr) = (regimes[3], regimes[1]);
    let low: Vec<usize> = order.iter().take(nc).copied().collect();
    let high: Vec<usize> = order.iter().rev().take(nr).copied().collect();
    let avg = |ix: &[usize]| -> f64 {
        if ix.is_empty() {
            f64::NAN
        } else {
            ix.iter().map(|&i| rows[i].2).sum::<f64>() / ix.len() as f64
        }
    };
    let delta_rival = avg(&low) - avg(&high);

    // Bootstrap by day.
    let mut by_day: BTreeMap<i64, (f64, f64, f64, f64)> = BTreeMap::new();
    for r in &rows {
        let e = by_day.entry(r.1).or_default();
        if r.0 == 3 {
            e.0 += r.2;
            e.1 += 1.0;
        } else if r.0 == 1 {
            e.2 += r.2;
            e.3 += 1.0;
        }
    }
    let days: Vec<(f64, f64, f64, f64)> = by_day.into_values().collect();
    let mut rng = Rng::new(seed);
    let mut draws = Vec::with_capacity(BOOTSTRAP);
    for _ in 0..BOOTSTRAP {
        let mut acc = (0.0, 0.0, 0.0, 0.0);
        for _ in 0..days.len() {
            let d = days[rng.uniform(days.len() as u64) as usize];
            acc.0 += d.0;
            acc.1 += d.1;
            acc.2 += d.2;
            acc.3 += d.3;
        }
        if acc.1 > 0.0 && acc.3 > 0.0 {
            draws.push(acc.0 / acc.1 - acc.2 / acc.3);
        }
    }
    draws.sort_by(f64::total_cmp);
    let ci = if draws.is_empty() {
        (f64::NAN, f64::NAN)
    } else {
        let b = draws.len();
        (
            draws[(0.025 * b as f64) as usize],
            draws[((0.975 * b as f64).ceil() as usize).saturating_sub(1)],
        )
    };

    let included = rows.len();
    let brier = |j: usize| -> f64 {
        if included == 0 {
            f64::NAN
        } else {
            rows.iter()
                .map(|r| if j == 1 { r.4 } else { r.5 })
                .sum::<f64>()
                / included as f64
        }
    };
    ConflictReading {
        train: train.len(),
        included,
        regimes,
        brier_s1: brier(1),
        brier_s2: brier(2),
        gain_conflict,
        gain_resonance,
        delta,
        ci,
        delta_rival,
        testable: regimes[3] >= MIN_STRATUM && regimes[1] >= MIN_STRATUM,
    }
}

/// The flights of a year as cases: the 60 days before 1 October to learn, October to December as the vault, both
/// split by the instant of emission as in Chapter 20.
pub fn flight_cases(path: &str, year: i64) -> (Vec<Case>, Vec<Case>) {
    let flights = load_flights(path, year);
    let vault_start = days_from_civil(year, 10, 1) * 1440;
    let train_start = vault_start - 60 * 1440;
    let case = |f: &crate::streams::Flight| Case {
        day: f.day,
        hour: f.dep_hour,
        levels: [f.carrier.clone(), f.origin.clone(), f.dest.clone()],
        y: f.y,
    };
    let train = flights
        .iter()
        .filter(|f| f.emission >= train_start && f.emission < vault_start)
        .map(case)
        .collect();
    let vault = flights
        .iter()
        .filter(|f| f.emission >= vault_start)
        .map(case)
        .collect();
    (train, vault)
}

/// Synthetic flights: twelve carriers whose daily risk peaks every two hours around the clock and three origins
/// that peak at 8, 14 and 20 hours (three phasors 120 degrees apart, so that the marginal profile of each level
/// keeps its own peak), six destinations with no daily profile. With `bonus`, the three carrier-origin pairs that
/// peak exactly twelve hours apart are riskier in the evening: an interaction concentrated where the sources are in
/// antiphase, which is the world in which the engine's claim is true.
pub fn synthetic_flights(seed: u64, bonus: bool, n: usize) -> Vec<Case> {
    let mut rng = Rng::new(seed);
    let origins = [8.0, 14.0, 20.0];
    let tau = 2.0 * std::f64::consts::PI;
    (0..n)
        .map(|i| {
            let c = rng.uniform(12) as usize;
            let o = rng.uniform(3) as usize;
            let d = rng.uniform(6) as usize;
            let (pc, po) = (2.0 * c as f64, origins[o]);
            let hour = 5 + rng.uniform(19) as i64;
            let h = hour as f64;
            let mut z = -1.5 + (tau * (h - pc) / 24.0).cos() + (tau * (h - po) / 24.0).cos();
            if bonus && (pc - po).abs() == 12.0 && (17..=21).contains(&hour) {
                z += 1.5;
            }
            let p = 1.0 / (1.0 + (-z).exp());
            let u = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            Case {
                day: (i / 700) as i64,
                hour,
                levels: [format!("C{c:02}"), format!("O{o}"), format!("D{d}")],
                y: if u < p { 1.0 } else { 0.0 },
            }
        })
        .collect()
}

// ------------------------------------------------------------------ the instrument

fn tally_line(name: &str, t: &Tally, n: usize) -> String {
    format!(
        "  {name:<34} temperature -> demand {:>3} | demand -> temperature {:>3} | none {:>3} | rate {:.3}",
        t.correct,
        t.wrong,
        t.none,
        t.rate(n)
    )
}

fn print_arrow(label: &str, r: &ArrowReading) {
    println!("\n-- T1 {label}: {} weeks --", r.weeks);
    println!("{}", tally_line("engine", &r.engine, r.weeks));
    println!(
        "{}",
        tally_line("engine, null pairing (+13 weeks)", &r.engine_null, r.weeks)
    );
    println!(
        "{}",
        tally_line("rival (lagged cross-correlation)", &r.rival, r.weeks)
    );
    println!(
        "{}",
        tally_line("rival, null pairing", &r.rival_null, r.weeks)
    );
    println!(
        "{}",
        tally_line("engine, margin 0.05 (secondary)", &r.engine_margin, r.weeks)
    );
    println!(
        "  binomial P(X >= {} | n = {}) = {:.3e} | median learnt lag {:.2} h",
        r.engine.correct,
        r.engine.correct + r.engine.wrong,
        r.binomial_p,
        r.median_lag_hours
    );
    println!(
        "  P1 {} | P2 {} | P3 {} | works: {} | adds over the rival: {}",
        r.p1, r.p2, r.p3, r.works, r.adds
    );
}

fn print_conflict(label: &str, r: &ConflictReading) {
    println!("\n-- T2 {label} --");
    println!(
        "  training {} | vault flights with three eligible sources {} | quiescent {} resonance {} beating {} conflict {}",
        r.train, r.included, r.regimes[0], r.regimes[1], r.regimes[2], r.regimes[3]
    );
    println!(
        "  Brier System 1 (additive) {:.5} | System 2 (cells) {:.5}",
        r.brier_s1, r.brier_s2
    );
    println!(
        "  gain of System 2: conflict {:+.5} | resonance {:+.5} | delta {:+.5}, 95% CI by day [{:+.5}; {:+.5}]",
        r.gain_conflict, r.gain_resonance, r.delta, r.ci.0, r.ci.1
    );
    println!(
        "  rival delta (level agreement, no phase) {:+.5} | testable: {}",
        r.delta_rival, r.testable
    );
}

pub fn run() {
    println!("== The wave engine against data (empirical/p23-onda/DECLARACAO.md) ==");
    println!("controls on synthetic data, seed {SEED}:");
    let (d, t) = synthetic_energy(SEED, true);
    print_arrow(
        "control, planted arrow",
        &arrow_test(&anomaly(&d), &anomaly(&t)),
    );
    let (d, t) = synthetic_energy(SEED, false);
    print_arrow(
        "control, independent",
        &arrow_test(&anomaly(&d), &anomaly(&t)),
    );
    let base = synthetic_flights(SEED, true, 60_000);
    let vault = synthetic_flights(SEED + 1, true, 60_000);
    print_conflict(
        "control, planted interaction",
        &conflict_test(&base, &vault, SEED),
    );
    let base = synthetic_flights(SEED, false, 60_000);
    let vault = synthetic_flights(SEED + 1, false, 60_000);
    print_conflict(
        "control, no interaction",
        &conflict_test(&base, &vault, SEED),
    );

    println!("\nreal data:");
    let (d, t) = load_energy(ENERGY);
    let arrow = arrow_test(&anomaly(&d), &anomaly(&t));
    print_arrow("Victoria 2012-2014, temperature -> demand", &arrow);
    let (tr, va) = flight_cases(FLIGHTS_2013, 2013);
    let c13 = conflict_test(&tr, &va, SEED);
    print_conflict("New York 2013, vault October to December", &c13);
    let (tr, va) = flight_cases(FLIGHTS_2023, 2023);
    let c23 = conflict_test(&tr, &va, SEED);
    print_conflict("New York 2023, vault October to December", &c23);

    let testable = c13.testable && c23.testable;
    let works = testable && c13.ci.0 > 0.0 && c23.delta > 0.0;
    let adds = testable && c13.delta > c13.delta_rival && c23.delta > c23.delta_rival;
    println!("\n== verdicts, by the rules sealed before the run ==");
    println!(
        "T1 the arrow: {} | adds over the cross-correlation: {}",
        if arrow.works {
            "SUPPORTED"
        } else {
            "NOT SUPPORTED"
        },
        if arrow.adds { "yes" } else { "no" }
    );
    println!(
        "T2 the conflict: {} | adds over level agreement: {}",
        if !testable {
            "NOT TESTABLE under this encoding"
        } else if works {
            "SUPPORTED"
        } else {
            "NOT SUPPORTED"
        },
        if adds { "yes" } else { "no" }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_planted_arrow_is_found_and_the_independent_one_is_not() {
        let (d, t) = synthetic_energy(SEED, true);
        let planted = arrow_test(&anomaly(&d), &anomaly(&t));
        assert!(
            planted.engine.rate(planted.weeks) >= 0.80,
            "{:?}",
            planted.engine
        );
        let (d, t) = synthetic_energy(SEED, false);
        let indep = arrow_test(&anomaly(&d), &anomaly(&t));
        let decided = (indep.engine.correct + indep.engine.wrong) as f64;
        let imbalance = (indep.engine.correct as f64 - indep.engine.wrong as f64).abs();
        assert!(imbalance <= 0.3 * decided, "{:?}", indep.engine);
    }

    #[test]
    fn the_anomaly_removes_a_shared_daily_cycle() {
        let days: Vec<Day> = (0..10)
            .map(|_| {
                let mut d = [0.0; SLOTS];
                for (s, v) in d.iter_mut().enumerate() {
                    *v = (s as f64 / 48.0 * std::f64::consts::TAU).sin() * 10.0;
                }
                d
            })
            .collect();
        assert!(anomaly(&days).iter().flatten().all(|v| v.abs() < 1e-9));
    }

    #[test]
    fn events_open_once_per_excursion_with_the_time_of_day_as_phase() {
        let mut z = vec![0.0; 96];
        z[50] = 2.0;
        z[51] = 3.5;
        z[52] = 1.0;
        z[60] = -2.0;
        let e = events(&z, 0, true);
        assert_eq!(e.len(), 2);
        assert_eq!(e[0].t, 50);
        assert!((e[0].amplitude - 1.0).abs() < 1e-12);
        assert!((e[0].phase - phase_of(2.0, 48.0)).abs() < 1e-12);
        assert_eq!(events(&z, 1, false).len(), 1);
    }

    #[test]
    fn the_binomial_tail_is_exact_at_small_n() {
        assert!((binomial_upper(3, 3) - 0.125).abs() < 1e-12);
        assert!((binomial_upper(2, 3) - 0.5).abs() < 1e-12);
        assert_eq!(binomial_upper(0, 5), 1.0);
    }

    #[test]
    fn a_planted_interaction_in_conflict_raises_the_gain_of_system_2_there() {
        let with = conflict_test(
            &synthetic_flights(SEED, true, 60_000),
            &synthetic_flights(SEED + 1, true, 60_000),
            SEED,
        );
        let without = conflict_test(
            &synthetic_flights(SEED, false, 60_000),
            &synthetic_flights(SEED + 1, false, 60_000),
            SEED,
        );
        assert!(with.testable, "{:?}", with.regimes);
        assert!(with.ci.0 > 0.0, "{:?}", with.ci);
        assert!(
            with.delta > without.delta,
            "{} vs {}",
            with.delta,
            without.delta
        );
        assert!(
            without.ci.0 <= 0.0 && without.ci.1 >= 0.0,
            "{:?}",
            without.ci
        );
    }
}
