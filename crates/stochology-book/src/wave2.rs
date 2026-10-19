//! Declaration 2 of point 23 (empirical/p23-onda/DECLARACAO_2.md, sealed before this module first read a dataset):
//! the two refinements that the first run's report proposed, each counted as K = 2.
//!
//! **T1, K = 2.** The same arrow, encoding, null and rules as declaration 1, with three changes chosen from what the
//! first run showed: spikes from excursions at or above 1.0 instead of 1.5 (the engine abstained in 41% of the
//! weeks), a memory of six hours (tau = 12 half-hours) and a horizon of a day (48 half-hours); the rival's lags go to
//! the same horizon. The first report's idea of learning from pairs in the same half-hour is dropped: a pair with no
//! order carries no direction.
//!
//! **T2, K = 2.** The same conflict, systems, rival and rules, with the phasor of each level taken from the deviation
//! of its hourly log-odds from the common hourly profile of all training flights: the evening peak every source
//! shares is taken out before the phase is read, as the anomaly of T1 takes out the daily cycle.
//!
//! The sealed module of declaration 1 (`wave.rs`) is not changed; what this module needs from it, it calls, and what
//! it changes, it carries.

use std::collections::{BTreeMap, HashMap};

use stochology_core::Rng;
use stochology_wave::channel::{ChannelBus, Event};
use stochology_wave::cycle::{Point, first_harmonic, phase_of};
use stochology_wave::phasor::{Coupling, Regime, Thresholds, superpose};
use stochology_wave::seal::wrap_pi;
use stochology_wave::spike::StdpConfig;

use crate::streams::{fit_logistic, logit, predict};
use crate::wave::{
    ADDS_MIN_GAIN, ArrowReading, BOOTSTRAP, CELL_PRIOR, Case, ConflictReading, Day, ENERGY,
    FLIGHTS_2013, FLIGHTS_2023, MIN_HOUR, MIN_LEVEL, MIN_STRATUM, NULL_SHIFT_WEEKS, P1_ALPHA,
    P1_MIN_RATE, P2_MIN_GAP, P3_MAX_LOSS, RIDGE, SECONDARY_MARGIN, SEED, SLOTS, Tally, WEEKS,
    anomaly, binomial_upper, conflict_test, flight_cases, load_energy, synthetic_energy,
};

// ------------------------------------------------------------------ T1, K = 2

pub const THRESHOLD_K2: f64 = 1.0;
pub const TAU_K2: f64 = 12.0;
pub const HORIZON_K2: u64 = 48;
pub const XCORR_LAGS_K2: usize = 48;

/// The STDP rule of declaration 1 with six hours of memory and a day of horizon.
pub fn stdp_config_k2() -> StdpConfig {
    StdpConfig {
        q: 0.5,
        a_plus: 0.18,
        a_minus: 0.12,
        tau_plus_us: TAU_K2,
        tau_minus_us: TAU_K2,
        lr_alpha: 0.30,
        oja: 0.04,
        horizon_us: HORIZON_K2,
    }
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

/// The spikes of declaration 1 with the threshold of declaration 2.
pub fn events_k2(z: &[f64], channel: usize, absolute: bool) -> Vec<Event> {
    let score = |x: f64| if absolute { x.abs() } else { x };
    let mut out = Vec::new();
    let mut t = 0;
    while t < z.len() {
        if score(z[t]) >= THRESHOLD_K2 {
            let start = t;
            let mut peak = score(z[t]);
            while t < z.len() && score(z[t]) >= THRESHOLD_K2 {
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

pub fn engine_arrow_k2(zt: &[f64], zd: &[f64]) -> (f64, f64) {
    let cfg = stdp_config_k2();
    let mut bus = ChannelBus::new(2, 0.25);
    let mut all = events_k2(zt, 0, true);
    all.extend(events_k2(zd, 1, false));
    bus.publish_all(all, &cfg);
    let phase = wrap_pi(2.0 * std::f64::consts::PI * cfg.q * bus.alpha[0][1]);
    (
        bus.asymmetry(0, 1),
        phase / (2.0 * std::f64::consts::PI) * 24.0,
    )
}

pub fn xcorr_arrow_k2(zt: &[f64], zd: &[f64]) -> f64 {
    let x: Vec<f64> = zt.iter().map(|v| v.abs()).collect();
    let n = x.len();
    let mut a = 0.0;
    for k in 1..=XCORR_LAGS_K2 {
        a += corr(&x[..n - k], &zd[k..]);
        a -= corr(&x[k..], &zd[..n - k]);
    }
    a
}

fn add(t: &mut Tally, a: f64, margin: f64) {
    if a > margin {
        t.correct += 1;
    } else if a < -margin {
        t.wrong += 1;
    } else {
        t.none += 1;
    }
}

fn rate(t: &Tally, n: usize) -> f64 {
    t.correct as f64 / n.max(1) as f64
}

/// T1 under declaration 2: the rules of declaration 1, word for word.
pub fn arrow_test_k2(demand: &[Day], temperature: &[Day]) -> ArrowReading {
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
        let (a, lag) = engine_arrow_k2(&zt, &zd);
        add(&mut engine, a, 0.0);
        add(&mut engine_margin, a, SECONDARY_MARGIN);
        if a > 0.0 {
            lags.push(lag);
        }
        add(&mut engine_null, engine_arrow_k2(&zt_null, &zd).0, 0.0);
        add(&mut rival, xcorr_arrow_k2(&zt, &zd), 0.0);
        add(&mut rival_null, xcorr_arrow_k2(&zt_null, &zd), 0.0);
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
    let p1 = engine.correct > engine.wrong
        && binomial_p < P1_ALPHA
        && rate(&engine, weeks) >= P1_MIN_RATE;
    let p2 = rate(&engine, weeks) - rate(&engine_null, weeks) >= P2_MIN_GAP;
    let p3 = rate(&rival, weeks) - rate(&engine, weeks) <= P3_MAX_LOSS;
    let adds = rate(&engine, weeks) - rate(&rival, weeks) >= ADDS_MIN_GAIN
        && rate(&engine_null, weeks) <= rate(&rival_null, weeks);
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

// ------------------------------------------------------------------ T2, K = 2

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

#[derive(Debug, Clone, Default)]
struct Level {
    n: usize,
    k: f64,
    by_hour: BTreeMap<i64, (usize, f64)>,
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

pub fn conflict_test_deviation(train: &[Case], vault: &[Case], seed: u64) -> ConflictReading {
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
    // The common daily profile: the hourly log-odds of all training flights.
    let mut common: BTreeMap<i64, (usize, f64)> = BTreeMap::new();
    for c in train {
        let e = common.entry(c.hour).or_default();
        e.0 += 1;
        e.1 += c.y;
    }
    let common: BTreeMap<i64, f64> = common
        .into_iter()
        .map(|(h, (n, k))| (h, logit((k + 1.0) / (n as f64 + 2.0))))
        .collect();
    // Phasor of each eligible level: first harmonic of the DEVIATION of its hourly log-odds from the common profile,
    // over a 24-hour cycle (declaration 2: the evening peak that every source shares is taken out first).
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
                    value: logit((k + 1.0) / (n as f64 + 2.0))
                        - common.get(&h).copied().unwrap_or(0.0),
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

/// The synthetic flights of declaration 1 with a common evening peak added to every flight, the shape the first run
/// found in the real data: with the phase of the raw profile every source falls in phase; with the phase of the
/// deviation from the common profile the planted antiphase should come back.
pub fn synthetic_flights_common(seed: u64, bonus: bool, n: usize) -> Vec<Case> {
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
            let mut z = -1.5
                + 2.5 * (tau * (h - 19.0) / 24.0).cos()
                + (tau * (h - pc) / 24.0).cos()
                + (tau * (h - po) / 24.0).cos();
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
        rate(t, n)
    )
}

fn print_arrow(label: &str, r: &ArrowReading) {
    println!("\n-- T1 (K = 2) {label}: {} weeks --", r.weeks);
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
    println!("\n-- T2 (K = 2) {label} --");
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
    println!(
        "== The wave engine against data, declaration 2, K = 2 (empirical/p23-onda/DECLARACAO_2.md) =="
    );
    println!("controls on synthetic data, seed {SEED}:");
    let (d, t) = synthetic_energy(SEED, true);
    print_arrow(
        "control, planted arrow",
        &arrow_test_k2(&anomaly(&d), &anomaly(&t)),
    );
    let (d, t) = synthetic_energy(SEED, false);
    print_arrow(
        "control, independent",
        &arrow_test_k2(&anomaly(&d), &anomaly(&t)),
    );
    for (label, bonus) in [("planted interaction", true), ("no interaction", false)] {
        let base = synthetic_flights_common(SEED, bonus, 60_000);
        let vault = synthetic_flights_common(SEED + 1, bonus, 60_000);
        print_conflict(
            &format!(
                "control, common evening peak, {label}, phase of the raw profile (declaration 1)"
            ),
            &conflict_test(&base, &vault, SEED),
        );
        print_conflict(
            &format!("control, common evening peak, {label}, phase of the deviation"),
            &conflict_test_deviation(&base, &vault, SEED),
        );
    }

    println!("\nreal data:");
    let (d, t) = load_energy(ENERGY);
    let arrow = arrow_test_k2(&anomaly(&d), &anomaly(&t));
    print_arrow("Victoria 2012-2014, temperature -> demand", &arrow);
    let (tr, va) = flight_cases(FLIGHTS_2013, 2013);
    let c13 = conflict_test_deviation(&tr, &va, SEED);
    print_conflict("New York 2013, vault October to December", &c13);
    let (tr, va) = flight_cases(FLIGHTS_2023, 2023);
    let c23 = conflict_test_deviation(&tr, &va, SEED);
    print_conflict("New York 2023, vault October to December", &c23);

    let testable = c13.testable && c23.testable;
    let works = testable && c13.ci.0 > 0.0 && c23.delta > 0.0;
    let adds = testable && c13.delta > c13.delta_rival && c23.delta > c23.delta_rival;
    println!("\n== verdicts of declaration 2 (K = 2), by the rules sealed before the run ==");
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
    fn the_planted_arrow_survives_the_wider_horizon() {
        let (d, t) = synthetic_energy(SEED, true);
        let planted = arrow_test_k2(&anomaly(&d), &anomaly(&t));
        assert!(
            rate(&planted.engine, planted.weeks) >= 0.80,
            "{:?}",
            planted.engine
        );
        let (d, t) = synthetic_energy(SEED, false);
        let indep = arrow_test_k2(&anomaly(&d), &anomaly(&t));
        let decided = (indep.engine.correct + indep.engine.wrong) as f64;
        let imbalance = (indep.engine.correct as f64 - indep.engine.wrong as f64).abs();
        assert!(imbalance <= 0.3 * decided, "{:?}", indep.engine);
    }

    #[test]
    fn a_common_peak_hides_the_conflict_from_the_raw_phase_and_not_from_the_deviation() {
        let base = synthetic_flights_common(SEED, true, 60_000);
        let vault = synthetic_flights_common(SEED + 1, true, 60_000);
        let raw = conflict_test(&base, &vault, SEED);
        let dev = conflict_test_deviation(&base, &vault, SEED);
        assert!(
            dev.regimes[3] > 2 * raw.regimes[3].max(1),
            "{:?} vs {:?}",
            raw.regimes,
            dev.regimes
        );
        assert!(dev.testable, "{:?}", dev.regimes);
    }
}
