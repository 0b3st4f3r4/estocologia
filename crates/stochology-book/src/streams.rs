//! The real streams of Chapter 20, rebuilt beside the book (2026-10-19).
//!
//! The research record measured six streams with code that left the tree on 2026-10-16. This instrument redoes
//! them on the same public data, in Rust, from a specification declared here and in the chapter. It is a rebuild,
//! not a copy: where the retired code made a choice that the record does not state, this module makes its own and
//! says so, and its numbers are compared with the record's in Appendix E rather than forced to match them.
//!
//! The clock. Every fact carries the instant at which it becomes knowable. A forecast issued at instant `t` reads
//! only facts with that instant at or before `t`; the canary is a column equal to the label, knowable only when the
//! label is, so under the honest clock it is empty at every emission.
//!
//! The gates are the chapter's: for a binary forecaster `|p̄ − ȳ| ≤ 0.02`, `ECE ≤ 0.03` and Brier skill above zero
//! against the moving climatology; for an interval forecaster, coverage of the 90% interval between 0.86 and 0.94;
//! for the value at risk, violations between 0.5% and 1.5% and a Kupiec p-value above 0.05.
//!
//! Standard library only; no seed, because no step draws a random number.

use std::collections::{BTreeMap, HashMap};

pub const FLIGHTS_2013: &str = "data/nyc_flights_2013.csv";
pub const FLIGHTS_2023: &str = "data/nyc_flights_2023.csv";
pub const ARRESTS: &str = "data/toronto_arrests.csv";
pub const STORMS: &str = "data/atlantic_storms.csv";
pub const ENERGY: &str = "data/vic_elec.csv";
pub const BANK: &str = "data/bank_calls.csv";
pub const MARKET: &str = "data/djia_1885_2023.csv";

const BINS: usize = 10;
const RIDGE: f64 = 1e-3;

// ---------------------------------------------------------------- reading

/// Splits one CSV line, honouring double quotes.
fn split_csv(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    for c in line.chars() {
        match c {
            '"' => quoted = !quoted,
            ',' if !quoted => out.push(std::mem::take(&mut field)),
            _ => field.push(c),
        }
    }
    out.push(field);
    out
}

/// Reads a CSV into a header index and rows of fields.
fn read_csv(path: &str) -> (HashMap<String, usize>, Vec<Vec<String>>) {
    let text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("could not read {path}"));
    let mut lines = text.lines();
    let header: HashMap<String, usize> = split_csv(lines.next().unwrap_or_default())
        .into_iter()
        .enumerate()
        .map(|(i, h)| (h, i))
        .collect();
    let rows = lines.filter(|l| !l.is_empty()).map(split_csv).collect();
    (header, rows)
}

fn num(s: &str) -> Option<f64> {
    if s.is_empty() || s == "NA" {
        None
    } else {
        s.parse().ok()
    }
}

/// Days since 1970-01-01 for a civil date (proleptic Gregorian).
pub fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Day of the week, 0 = Monday.
fn weekday(days: i64) -> usize {
    (days + 3).rem_euclid(7) as usize
}

fn parse_date(s: &str) -> i64 {
    let y: i64 = s[0..4].parse().unwrap_or(1970);
    let m: i64 = s[5..7].parse().unwrap_or(1);
    let d: i64 = s[8..10].parse().unwrap_or(1);
    days_from_civil(y, m, d)
}

// ---------------------------------------------------------------- the binary toolbox

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

fn logit(p: f64) -> f64 {
    let p = p.clamp(1e-9, 1.0 - 1e-9);
    (p / (1.0 - p)).ln()
}

/// Logistic regression by reweighted least squares, ridge outside the intercept (column 0).
pub fn fit_logistic(x: &[Vec<f64>], y: &[f64], ridge: f64) -> Vec<f64> {
    let p = x.first().map_or(0, Vec::len);
    let mut b = vec![0.0; p];
    for _ in 0..50 {
        let mut a = vec![vec![0.0; p + 1]; p];
        for (row, &yi) in x.iter().zip(y) {
            let eta: f64 = row.iter().zip(&b).map(|(v, c)| v * c).sum();
            let mu = sigmoid(eta).clamp(1e-9, 1.0 - 1e-9);
            let w = (mu * (1.0 - mu)).max(1e-9);
            let z = eta + (yi - mu) / w;
            for j in 0..p {
                let wj = w * row[j];
                if wj == 0.0 {
                    continue;
                }
                for k in j..p {
                    a[j][k] += wj * row[k];
                }
                a[j][p] += wj * z;
            }
        }
        for j in 0..p {
            for k in 0..j {
                a[j][k] = a[k][j];
            }
        }
        for j in 1..p {
            a[j][j] += ridge;
        }
        let new = solve(a);
        let step = new
            .iter()
            .zip(&b)
            .map(|(n, o)| (n - o).abs())
            .fold(0.0, f64::max);
        b = new;
        if step < 1e-9 {
            break;
        }
    }
    b
}

/// Gauss–Jordan elimination with partial pivoting on an augmented matrix.
fn solve(mut a: Vec<Vec<f64>>) -> Vec<f64> {
    let p = a.len();
    for col in 0..p {
        let piv = (col..p)
            .max_by(|&r, &s| a[r][col].abs().total_cmp(&a[s][col].abs()))
            .unwrap_or(col);
        a.swap(col, piv);
        let d = a[col][col];
        if d.abs() < 1e-14 {
            continue;
        }
        for k in col..=p {
            a[col][k] /= d;
        }
        for r in 0..p {
            if r != col {
                let f = a[r][col];
                if f != 0.0 {
                    for k in col..=p {
                        a[r][k] -= f * a[col][k];
                    }
                }
            }
        }
    }
    (0..p).map(|j| a[j][p]).collect()
}

fn predict(b: &[f64], row: &[f64]) -> f64 {
    sigmoid(row.iter().zip(b).map(|(v, c)| v * c).sum())
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        0.0
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}

/// Expected calibration error with ten boxes of equal mass that never split a run of equal forecasts.
pub fn ece(p: &[f64], y: &[f64]) -> f64 {
    let n = p.len();
    if n == 0 {
        return 0.0;
    }
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| p[a].total_cmp(&p[b]));
    let mut total = 0.0;
    let mut start = 0;
    for k in 1..=BINS {
        let mut end = (k * n) / BINS;
        if k < BINS {
            while end < n && end > 0 && p[idx[end]] == p[idx[end - 1]] {
                end += 1;
            }
        } else {
            end = n;
        }
        if end <= start {
            continue;
        }
        let bin = &idx[start..end];
        let pb = bin.iter().map(|&i| p[i]).sum::<f64>() / bin.len() as f64;
        let yb = bin.iter().map(|&i| y[i]).sum::<f64>() / bin.len() as f64;
        total += bin.len() as f64 / n as f64 * (pb - yb).abs();
        start = end;
    }
    total
}

/// Simulations and seed of the null ECE, as in Chapter 8.
const NULL_SIMULATIONS: usize = 2000;
const NULL_SEED: u64 = 2026;

/// The 95% quantile of the ECE when each outcome is drawn with the chance its own forecast states. The boxes depend
/// only on the forecasts, so they are built once and each simulation only redraws the outcomes.
pub fn ece_null_q95(p: &[f64], simulations: usize, seed: u64) -> f64 {
    let n = p.len();
    if n == 0 {
        return 0.0;
    }
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| p[a].total_cmp(&p[b]));
    let mut boxes: Vec<(usize, usize, f64)> = Vec::new();
    let mut start = 0;
    for k in 1..=BINS {
        let mut end = (k * n) / BINS;
        if k < BINS {
            while end < n && end > 0 && p[idx[end]] == p[idx[end - 1]] {
                end += 1;
            }
        } else {
            end = n;
        }
        if end > start {
            let mean_p = idx[start..end].iter().map(|&i| p[i]).sum::<f64>() / (end - start) as f64;
            boxes.push((start, end, mean_p));
            start = end;
        }
    }
    let mut state = seed;
    let mut uniform = || {
        let v = stochology_core::rng::splitmix64_step(&mut state);
        (v >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut eces: Vec<f64> = (0..simulations)
        .map(|_| {
            boxes
                .iter()
                .map(|&(s, e, mean_p)| {
                    let hits = idx[s..e].iter().filter(|&&i| uniform() < p[i]).count() as f64;
                    (e - s) as f64 / n as f64 * (mean_p - hits / (e - s) as f64).abs()
                })
                .sum()
        })
        .collect();
    eces.sort_by(f64::total_cmp);
    eces[(0.95 * (eces.len() - 1) as f64).round() as usize]
}

fn brier(p: &[f64], y: &[f64]) -> f64 {
    p.iter().zip(y).map(|(a, b)| (a - b).powi(2)).sum::<f64>() / p.len().max(1) as f64
}

/// The verdict of a binary stream in its vault.
#[derive(Debug, Clone)]
pub struct Binary {
    pub n: usize,
    pub rate: f64,
    pub citl: f64,
    pub ece: f64,
    pub skill: f64,
    /// The 95% quantile of the ECE of a perfectly calibrated forecaster with these same forecasts (Chapter 8).
    pub null_q95: f64,
}

impl Binary {
    /// `clim` is the moving climatology at each emission: the rate of the labels knowable by then.
    pub fn measure(p: &[f64], y: &[f64], clim: &[f64]) -> Self {
        let reference = brier(clim, y);
        Binary {
            n: p.len(),
            rate: mean(y),
            citl: mean(p) - mean(y),
            ece: ece(p, y),
            skill: 1.0 - brier(p, y) / reference,
            null_q95: ece_null_q95(p, NULL_SIMULATIONS, NULL_SEED),
        }
    }

    pub fn passes(&self) -> bool {
        self.citl.abs() <= 0.02 && self.ece <= 0.03 && self.skill > 0.0
    }

    fn line(&self) -> String {
        format!(
            "n = {:>6} | rate {:.4} | CITL {:+.4} | ECE {:.4} (null q95 {:.4}) | skill {:+.4} | gate {}",
            self.n,
            self.rate,
            self.citl,
            self.ece,
            self.null_q95,
            self.skill,
            if self.passes() { "PASSES" } else { "FAILS" }
        )
    }
}

// ---------------------------------------------------------------- flights

/// One flight, with the instants of the clock in minutes since 1970.
#[derive(Debug, Clone)]
struct Flight {
    emission: i64,
    known: i64,
    day: i64,
    carrier: String,
    origin: String,
    dep_hour: i64,
    distance: f64,
    y: f64,
}

fn load_flights(path: &str, year: i64) -> Vec<Flight> {
    let (h, rows) = read_csv(path);
    let col = |r: &Vec<String>, name: &str| r[h[name]].clone();
    let mut out = Vec::new();
    for r in &rows {
        let Some(delay) = num(&col(r, "arr_delay")) else {
            continue;
        };
        let month: i64 = col(r, "month").parse().unwrap_or(1);
        let dday: i64 = col(r, "day").parse().unwrap_or(1);
        let day = days_from_civil(year, month, dday);
        let dep: i64 = col(r, "sched_dep_time").parse().unwrap_or(0);
        let arr: i64 = col(r, "sched_arr_time").parse().unwrap_or(0);
        let dep_min = (dep / 100) * 60 + dep % 100;
        let mut arr_min = (arr / 100) * 60 + arr % 100;
        if arr_min < dep_min {
            arr_min += 1440;
        }
        let base = day * 1440;
        out.push(Flight {
            emission: base + dep_min - 120,
            known: base + arr_min + delay.round() as i64,
            day,
            carrier: col(r, "carrier"),
            origin: col(r, "origin"),
            dep_hour: dep / 100,
            distance: num(&col(r, "distance")).unwrap_or(1.0),
            y: if delay > 15.0 { 1.0 } else { 0.0 },
        });
    }
    out.sort_by_key(|f| (f.emission, f.known));
    out
}

/// The share of delayed arrivals among the flights of the same origin that became known in the three hours
/// before the emission, with one imaginary delayed and one imaginary punctual flight (Laplace).
fn recent_shares(flights: &[Flight]) -> Vec<f64> {
    let mut by_origin: HashMap<&str, Vec<(i64, f64)>> = HashMap::new();
    for f in flights {
        by_origin
            .entry(f.origin.as_str())
            .or_default()
            .push((f.known, f.y));
    }
    let mut prefix: HashMap<&str, (Vec<i64>, Vec<f64>)> = HashMap::new();
    for (k, mut v) in by_origin {
        v.sort_by_key(|e| e.0);
        let times = v.iter().map(|e| e.0).collect();
        let mut acc = vec![0.0];
        for e in &v {
            acc.push(acc.last().copied().unwrap_or(0.0) + e.1);
        }
        prefix.insert(k, (times, acc));
    }
    flights
        .iter()
        .map(|f| {
            let (times, acc) = &prefix[f.origin.as_str()];
            let hi = times.partition_point(|&t| t <= f.emission);
            let lo = times.partition_point(|&t| t <= f.emission - 180);
            let n = (hi - lo) as f64;
            (acc[hi] - acc[lo] + 1.0) / (n + 2.0)
        })
        .collect()
}

struct Design {
    carriers: Vec<String>,
    origins: Vec<String>,
}

impl Design {
    fn new(train: &[&Flight]) -> Self {
        let mut carriers: Vec<String> = train.iter().map(|f| f.carrier.clone()).collect();
        carriers.sort();
        carriers.dedup();
        let mut origins: Vec<String> = train.iter().map(|f| f.origin.clone()).collect();
        origins.sort();
        origins.dedup();
        Design { carriers, origins }
    }

    /// Intercept, carrier and origin dummies (first level dropped), five departure bands, weekday dummies,
    /// log distance, the recent share at the origin, and the canary column.
    fn row(&self, f: &Flight, recent: f64, canary: f64) -> Vec<f64> {
        let mut r = vec![1.0];
        for c in self.carriers.iter().skip(1) {
            r.push(if &f.carrier == c { 1.0 } else { 0.0 });
        }
        for o in self.origins.iter().skip(1) {
            r.push(if &f.origin == o { 1.0 } else { 0.0 });
        }
        let band = match f.dep_hour {
            0..=8 => 0,
            9..=12 => 1,
            13..=16 => 2,
            17..=20 => 3,
            _ => 4,
        };
        for b in 1..5 {
            r.push(if band == b { 1.0 } else { 0.0 });
        }
        let wd = weekday(f.day);
        for d in 1..7 {
            r.push(if wd == d { 1.0 } else { 0.0 });
        }
        r.push(f.distance.max(1.0).ln());
        r.push(logit(recent));
        r.push(canary);
        r
    }
}

/// The verdict of one flights vault: the raw model, the model with the online intercept correction, and the
/// canary under both clocks.
#[derive(Debug, Clone)]
pub struct FlightsVault {
    pub raw: Binary,
    pub corrected: Binary,
    pub canary_honest_available: usize,
    pub canary_honest_gain: f64,
    pub canary_off_gain: f64,
    pub training: usize,
}

/// Rebuilds a flights vault: a logistic model fitted on the 60 days before the vault, applied to every flight of
/// October to December two hours before its scheduled departure, with the intercept moved after each known arrival
/// by `δ ← δ + 0.002 (y − p')`, as the chapter declares.
pub fn flights_vault(path: &str, year: i64) -> FlightsVault {
    let flights = load_flights(path, year);
    let recent = recent_shares(&flights);
    let vault_start = days_from_civil(year, 10, 1) * 1440;
    let train_start = vault_start - 60 * 1440;
    let train: Vec<usize> = (0..flights.len())
        .filter(|&i| flights[i].emission >= train_start && flights[i].emission < vault_start)
        .collect();
    let vault: Vec<usize> = (0..flights.len())
        .filter(|&i| flights[i].emission >= vault_start)
        .collect();
    let design = Design::new(&train.iter().map(|&i| &flights[i]).collect::<Vec<_>>());

    let fit_with = |canary: &dyn Fn(usize, i64) -> f64| -> Vec<f64> {
        let x: Vec<Vec<f64>> = train
            .iter()
            .map(|&i| design.row(&flights[i], recent[i], canary(i, flights[i].emission)))
            .collect();
        let y: Vec<f64> = train.iter().map(|&i| flights[i].y).collect();
        fit_logistic(&x, &y, RIDGE)
    };
    // Under the honest clock the canary is the label only once the arrival is known, which is never before the
    // emission; with the clock switched off it is the label itself.
    let honest = |i: usize, t: i64| {
        if flights[i].known <= t {
            flights[i].y
        } else {
            0.0
        }
    };
    let off = |i: usize, _t: i64| flights[i].y;
    let b_raw = fit_with(&|_, _| 0.0);
    let b_honest = fit_with(&honest);
    let b_off = fit_with(&off);

    // The moving climatology and the online correction, in the order of the clock.
    let mut arrivals: Vec<usize> = (0..flights.len())
        .filter(|&i| flights[i].emission >= train_start)
        .collect();
    arrivals.sort_by_key(|&i| flights[i].known);
    let mut next = 0;
    let (mut known_y, mut known_n) = (0.0, 0.0);
    let mut delta = 0.0;
    let mut issued: HashMap<usize, f64> = HashMap::new();
    let (mut p_raw, mut p_cor, mut p_hon, mut p_off, mut clim, mut ys) = (
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let mut honest_available = 0;
    for &i in &vault {
        let t = flights[i].emission;
        while next < arrivals.len() && flights[arrivals[next]].known <= t {
            let j = arrivals[next];
            known_y += flights[j].y;
            known_n += 1.0;
            if let Some(pj) = issued.remove(&j) {
                delta += 0.002 * (flights[j].y - pj);
            }
            next += 1;
        }
        let raw = predict(&b_raw, &design.row(&flights[i], recent[i], 0.0));
        let corrected = sigmoid(logit(raw) + delta);
        issued.insert(i, corrected);
        let c = honest(i, t);
        if c != 0.0 || flights[i].known <= t {
            honest_available += 1;
        }
        p_raw.push(raw);
        p_cor.push(corrected);
        p_hon.push(predict(&b_honest, &design.row(&flights[i], recent[i], c)));
        p_off.push(predict(
            &b_off,
            &design.row(&flights[i], recent[i], off(i, t)),
        ));
        clim.push((known_y + 1.0) / (known_n + 2.0));
        ys.push(flights[i].y);
    }
    let raw = Binary::measure(&p_raw, &ys, &clim);
    FlightsVault {
        corrected: Binary::measure(&p_cor, &ys, &clim),
        canary_honest_available: honest_available,
        canary_honest_gain: Binary::measure(&p_hon, &ys, &clim).skill - raw.skill,
        canary_off_gain: Binary::measure(&p_off, &ys, &clim).skill - raw.skill,
        training: train.len(),
        raw,
    }
}

// ---------------------------------------------------------------- Toronto, 1997 to 2002

/// The verdict of the Toronto vault, with the fairness audit by perceived colour.
#[derive(Debug, Clone)]
pub struct Arrests {
    pub verdict: Binary,
    pub training: usize,
    pub canary_off_gain: f64,
    /// Per vault year: (year, colour, false positive rate of the marker, false negative rate, cases).
    pub audit: Vec<(i64, String, f64, f64, usize)>,
}

/// The marker flags a detainee when the forecast chance of release falls below this threshold.
pub const ARREST_THRESHOLD: f64 = 0.70;

/// A model blind to colour (employment, citizenship, prior checks, age and its square, sex), fitted on 1997 to 2000
/// and scored on 2001 and 2002; the climatology of a year is the release rate of the years before it.
pub fn arrests() -> Arrests {
    let (h, rows) = read_csv(ARRESTS);
    struct Case {
        year: i64,
        colour: String,
        x: Vec<f64>,
        y: f64,
    }
    let cases: Vec<Case> = rows
        .iter()
        .map(|r| {
            let age = num(&r[h["age"]]).unwrap_or(25.0);
            let a = (age - 25.0) / 10.0;
            Case {
                year: r[h["year"]].parse().unwrap_or(0),
                colour: r[h["colour"]].clone(),
                x: vec![
                    1.0,
                    if r[h["employed"]] == "Yes" { 1.0 } else { 0.0 },
                    if r[h["citizen"]] == "Yes" { 1.0 } else { 0.0 },
                    num(&r[h["checks"]]).unwrap_or(0.0),
                    a,
                    a * a,
                    if r[h["sex"]] == "Male" { 1.0 } else { 0.0 },
                ],
                y: if r[h["released"]] == "Yes" { 1.0 } else { 0.0 },
            }
        })
        .collect();
    let train: Vec<&Case> = cases.iter().filter(|c| c.year <= 2000).collect();
    let vault: Vec<&Case> = cases.iter().filter(|c| c.year >= 2001).collect();
    let x: Vec<Vec<f64>> = train.iter().map(|c| c.x.clone()).collect();
    let y: Vec<f64> = train.iter().map(|c| c.y).collect();
    let b = fit_logistic(&x, &y, RIDGE);
    let xc: Vec<Vec<f64>> = train
        .iter()
        .map(|c| [c.x.clone(), vec![c.y]].concat())
        .collect();
    let b_off = fit_logistic(&xc, &y, RIDGE);
    let clim_of = |year: i64| {
        let past: Vec<f64> = cases
            .iter()
            .filter(|c| c.year < year)
            .map(|c| c.y)
            .collect();
        mean(&past)
    };
    let p: Vec<f64> = vault.iter().map(|c| predict(&b, &c.x)).collect();
    let p_off: Vec<f64> = vault
        .iter()
        .map(|c| predict(&b_off, &[c.x.clone(), vec![c.y]].concat()))
        .collect();
    let ys: Vec<f64> = vault.iter().map(|c| c.y).collect();
    let clim: Vec<f64> = vault.iter().map(|c| clim_of(c.year)).collect();
    let verdict = Binary::measure(&p, &ys, &clim);
    let canary_off_gain = Binary::measure(&p_off, &ys, &clim).skill - verdict.skill;
    let mut audit = Vec::new();
    for year in [2001, 2002] {
        for colour in ["Black", "White"] {
            let (mut fp, mut neg, mut fneg, mut pos, mut n): (f64, f64, f64, f64, usize) =
                (0.0, 0.0, 0.0, 0.0, 0);
            for (c, &pi) in vault.iter().zip(&p) {
                if c.year != year || c.colour != colour {
                    continue;
                }
                n += 1;
                let marked = pi < ARREST_THRESHOLD;
                if c.y == 1.0 {
                    neg += 1.0;
                    if marked {
                        fp += 1.0;
                    }
                } else {
                    pos += 1.0;
                    if !marked {
                        fneg += 1.0;
                    }
                }
            }
            audit.push((
                year,
                colour.to_string(),
                fp / neg.max(1.0),
                fneg / pos.max(1.0),
                n,
            ));
        }
    }
    Arrests {
        verdict,
        training: train.len(),
        canary_off_gain,
        audit,
    }
}

// ---------------------------------------------------------------- Atlantic cyclones

/// One storm's observations by hour: wind, pressure, latitude, longitude and month.
type Track = BTreeMap<i64, (f64, f64, f64, f64, i64)>;

/// The chance that a cyclone gains 30 knots in the next 24 hours, from what is known at the observation: wind,
/// its change over the last 12 and 24 hours, pressure, latitude, longitude and the month; fitted on 1975 to 2014,
/// scored on 2015 to 2022, the climatology of a year being the rate of the years before it.
pub fn cyclones() -> (Binary, usize) {
    let (h, rows) = read_csv(STORMS);
    let mut by_storm: BTreeMap<(String, i64), Track> = BTreeMap::new();
    for r in &rows {
        let year: i64 = r[h["year"]].parse().unwrap_or(0);
        let month: i64 = r[h["month"]].parse().unwrap_or(1);
        let day: i64 = r[h["day"]].parse().unwrap_or(1);
        let hour: i64 = r[h["hour"]].parse().unwrap_or(0);
        let (Some(wind), Some(lat), Some(long)) =
            (num(&r[h["wind"]]), num(&r[h["lat"]]), num(&r[h["long"]]))
        else {
            continue;
        };
        let pressure = num(&r[h["pressure"]]).unwrap_or(1000.0);
        let t = days_from_civil(year, month, day) * 24 + hour;
        by_storm
            .entry((r[h["name"]].clone(), year))
            .or_default()
            .insert(t, (wind, pressure, lat, long, month));
    }
    struct Obs {
        year: i64,
        x: Vec<f64>,
        y: f64,
    }
    let mut obs = Vec::new();
    for ((_, year), track) in &by_storm {
        for (&t, &(wind, pressure, lat, long, month)) in track {
            let Some(&(future, ..)) = track.get(&(t + 24)) else {
                continue;
            };
            let d12 = track.get(&(t - 12)).map_or(0.0, |p| wind - p.0);
            let d24 = track.get(&(t - 24)).map_or(0.0, |p| wind - p.0);
            obs.push(Obs {
                year: *year,
                x: vec![
                    1.0,
                    wind / 50.0,
                    (wind / 50.0).powi(2),
                    d12 / 10.0,
                    d24 / 10.0,
                    (1010.0 - pressure) / 30.0,
                    lat / 20.0,
                    long / 50.0,
                    if (8..=10).contains(&month) { 1.0 } else { 0.0 },
                ],
                y: if future - wind >= 30.0 { 1.0 } else { 0.0 },
            });
        }
    }
    let train: Vec<&Obs> = obs.iter().filter(|o| o.year <= 2014).collect();
    let vault: Vec<&Obs> = obs
        .iter()
        .filter(|o| (2015..=2022).contains(&o.year))
        .collect();
    let b = fit_logistic(
        &train.iter().map(|o| o.x.clone()).collect::<Vec<_>>(),
        &train.iter().map(|o| o.y).collect::<Vec<_>>(),
        RIDGE,
    );
    let p: Vec<f64> = vault.iter().map(|o| predict(&b, &o.x)).collect();
    let ys: Vec<f64> = vault.iter().map(|o| o.y).collect();
    let clim: Vec<f64> = vault
        .iter()
        .map(|o| {
            mean(
                &obs.iter()
                    .filter(|q| q.year < o.year)
                    .map(|q| q.y)
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    (Binary::measure(&p, &ys, &clim), train.len())
}

// ---------------------------------------------------------------- interval streams

fn quantile(mut v: Vec<f64>, q: f64) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(f64::total_cmp);
    let pos = q * (v.len() - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    v[lo] + (pos - lo as f64) * (v[hi] - v[lo])
}

/// Coverage of a day-ahead 90% interval over a set of days, each day a vector of slots. The point forecast of a slot
/// is the mean of the same slot over the last four days of the same type; the interval multiplies it by the 5% and
/// 95% quantiles of the relative errors of that rule over the previous `window` days, all slots pooled (a rolling
/// split-conformal interval).
fn interval_coverage(
    days: &[(i64, bool, Vec<f64>)],
    vault_from: i64,
    vault_to: i64,
    window: usize,
) -> (usize, f64) {
    let point = |k: usize| -> Option<Vec<f64>> {
        let kind = days[k].1;
        let same: Vec<&Vec<f64>> = days[..k]
            .iter()
            .rev()
            .filter(|d| d.1 == kind)
            .take(4)
            .map(|d| &d.2)
            .collect();
        if same.len() < 4 {
            return None;
        }
        Some(
            (0..days[k].2.len())
                .map(|s| same.iter().map(|d| d[s]).sum::<f64>() / 4.0)
                .collect(),
        )
    };
    let mut errors: Vec<Vec<f64>> = vec![Vec::new(); days.len()];
    for k in 0..days.len() {
        if let Some(pt) = point(k) {
            errors[k] = days[k]
                .2
                .iter()
                .zip(&pt)
                .filter(|(_, p)| **p > 0.0)
                .map(|(y, p)| y / p - 1.0)
                .collect();
        }
    }
    let (mut inside, mut total) = (0usize, 0usize);
    for k in 0..days.len() {
        if days[k].0 < vault_from || days[k].0 > vault_to {
            continue;
        }
        let Some(pt) = point(k) else { continue };
        let past: Vec<f64> = errors[k.saturating_sub(window)..k]
            .iter()
            .flatten()
            .copied()
            .collect();
        if past.is_empty() {
            continue;
        }
        let (lo, hi) = (quantile(past.clone(), 0.05), quantile(past, 0.95));
        for (y, p) in days[k].2.iter().zip(&pt) {
            total += 1;
            if *y >= p * (1.0 + lo) && *y <= p * (1.0 + hi) {
                inside += 1;
            }
        }
    }
    (total, inside as f64 / total.max(1) as f64)
}

/// Victoria: half-hourly demand, the next day forecast the day before; working days and the rest (weekends and
/// holidays) are the two types; the vault is 2014.
pub fn energy() -> (usize, f64) {
    let (h, rows) = read_csv(ENERGY);
    let mut by_day: BTreeMap<i64, (bool, Vec<f64>)> = BTreeMap::new();
    for r in &rows {
        let day = parse_date(&r[h["Date"]]);
        let holiday = r[h["Holiday"]] == "TRUE";
        let working = !holiday && weekday(day) < 5;
        let demand = num(&r[h["Demand"]]).unwrap_or(0.0);
        by_day
            .entry(day)
            .or_insert((working, Vec::new()))
            .1
            .push(demand);
    }
    let days: Vec<(i64, bool, Vec<f64>)> = by_day
        .into_iter()
        .filter(|(_, (_, v))| v.len() == 48)
        .map(|(d, (w, v))| (d, w, v))
        .collect();
    interval_coverage(
        &days,
        days_from_civil(2014, 1, 1),
        days_from_civil(2014, 12, 31),
        56,
    )
}

/// The bank: calls per 5 minutes of the next working day; one type (every day is a working day); vault September
/// and October 2003.
pub fn bank() -> (usize, f64) {
    let (h, rows) = read_csv(BANK);
    let mut by_day: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for r in &rows {
        let stamp = &r[h["DateTime"]];
        by_day
            .entry(parse_date(stamp))
            .or_default()
            .push(num(&r[h["Calls"]]).unwrap_or(0.0));
    }
    let width = by_day.values().map(Vec::len).max().unwrap_or(0);
    let days: Vec<(i64, bool, Vec<f64>)> = by_day
        .into_iter()
        .filter(|(_, v)| v.len() == width)
        .map(|(d, v)| (d, true, v))
        .collect();
    interval_coverage(
        &days,
        days_from_civil(2003, 9, 1),
        days_from_civil(2003, 10, 31),
        20,
    )
}

// ---------------------------------------------------------------- the market

/// Complementary error function (Numerical Recipes, Chebyshev fit, relative error below 1.2e-7).
fn erfc(x: f64) -> f64 {
    let z = x.abs();
    let t = 1.0 / (1.0 + 0.5 * z);
    let r = t
        * (-z * z - 1.265_512_23
            + t * (1.000_023_68
                + t * (0.374_091_96
                    + t * (0.096_784_18
                        + t * (-0.186_288_06
                            + t * (0.278_868_07
                                + t * (-1.135_203_98
                                    + t * (1.488_515_87
                                        + t * (-0.822_152_23 + t * 0.170_872_77)))))))))
            .exp();
    if x >= 0.0 { r } else { 2.0 - r }
}

/// Survival function of a chi-square with one degree of freedom.
pub fn chi2_1_sf(x: f64) -> f64 {
    erfc((x.max(0.0) / 2.0).sqrt())
}

fn xlogy(x: f64, y: f64) -> f64 {
    if x == 0.0 { 0.0 } else { x * y.ln() }
}

/// Kupiec's unconditional coverage test: p-value for `violations` in `n` sessions at level `level`.
pub fn kupiec(violations: usize, n: usize, level: f64) -> f64 {
    let (x, n) = (violations as f64, n as f64);
    let rate = x / n;
    let lr = -2.0
        * (xlogy(n - x, 1.0 - level) + xlogy(x, level) - xlogy(n - x, 1.0 - rate) - xlogy(x, rate));
    chi2_1_sf(lr)
}

/// Christoffersen's independence test: p-value from the transitions between violation and no violation.
pub fn christoffersen(hits: &[bool]) -> f64 {
    let (mut n00, mut n01, mut n10, mut n11): (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);
    for w in hits.windows(2) {
        match (w[0], w[1]) {
            (false, false) => n00 += 1.0,
            (false, true) => n01 += 1.0,
            (true, false) => n10 += 1.0,
            (true, true) => n11 += 1.0,
        }
    }
    let p01 = n01 / (n00 + n01).max(1.0);
    let p11 = n11 / (n10 + n11).max(1.0);
    let p = (n01 + n11) / (n00 + n01 + n10 + n11).max(1.0);
    let null = xlogy(n00 + n10, 1.0 - p) + xlogy(n01 + n11, p);
    let alt = xlogy(n00, 1.0 - p01) + xlogy(n01, p01) + xlogy(n10, 1.0 - p11) + xlogy(n11, p11);
    chi2_1_sf(-2.0 * (null - alt))
}

/// The 99% value at risk of the next session by historical simulation over the last 250 returns; the vault is
/// every session from 2000 on.
pub fn market() -> (usize, f64, f64, f64) {
    let (h, rows) = read_csv(MARKET);
    let series: Vec<(i64, f64)> = rows
        .iter()
        .filter_map(|r| Some((parse_date(&r[h["date"]]), num(&r[h["value"]])?)))
        .collect();
    let returns: Vec<(i64, f64)> = series
        .windows(2)
        .map(|w| (w[1].0, w[1].1 / w[0].1 - 1.0))
        .collect();
    let start = days_from_civil(2000, 1, 1);
    let mut hits = Vec::new();
    for k in 250..returns.len() {
        if returns[k].0 < start {
            continue;
        }
        let window: Vec<f64> = returns[k - 250..k].iter().map(|r| r.1).collect();
        let var = quantile(window, 0.01);
        hits.push(returns[k].1 < var);
    }
    let x = hits.iter().filter(|&&b| b).count();
    (
        hits.len(),
        x as f64 / hits.len() as f64,
        kupiec(x, hits.len(), 0.01),
        christoffersen(&hits),
    )
}

// ---------------------------------------------------------------- the instrument

pub fn run() {
    println!("== Chapter 20 :: the real streams, rebuilt beside the book ==");
    println!(
        "no step draws a random number | gates: |CITL| <= 0.02, ECE <= 0.03, skill > 0 | coverage 0.86 to 0.94 | VaR 0.5% to 1.5% and Kupiec p > 0.05"
    );
    let (n, cov) = energy();
    println!(
        "energy (Victoria 2014, day ahead, 90% interval):  n = {n:>6} | coverage {cov:.3} | gate {}",
        if (0.86..=0.94).contains(&cov) {
            "PASSES"
        } else {
            "FAILS"
        }
    );
    let (n, cov) = bank();
    println!(
        "bank (Sep-Oct 2003, next working day, 90%):       n = {n:>6} | coverage {cov:.3} | gate {}",
        if (0.86..=0.94).contains(&cov) {
            "PASSES"
        } else {
            "FAILS"
        }
    );
    let (n, rate, pk, pc) = market();
    println!(
        "market (Dow Jones 2000-2023, 99% VaR, 250 days):  n = {n:>6} | violations {:.2}% | Kupiec p = {pk:.2} | Christoffersen p = {pc:.2} | gate {}",
        100.0 * rate,
        if (0.005..=0.015).contains(&rate) && pk > 0.05 {
            "PASSES"
        } else {
            "FAILS"
        }
    );
    for (label, path, year) in [
        ("flights 2013 (Oct-Dec)", FLIGHTS_2013, 2013),
        ("flights 2023 (Oct-Dec)", FLIGHTS_2023, 2023),
    ] {
        let v = flights_vault(path, year);
        println!(
            "{label}, trained on {} flights of the 60 days before:",
            v.training
        );
        println!("   raw model:             {}", v.raw.line());
        println!("   intercept correction:  {}", v.corrected.line());
        println!(
            "   canary: honest clock available in {} emissions, skill gain {:+.4} | clock switched off, skill gain {:+.4}",
            v.canary_honest_available, v.canary_honest_gain, v.canary_off_gain
        );
    }
    let (v, training) = cyclones();
    println!(
        "cyclones (2015-2022, +30 kt in 24 h), trained on {training} observations of 1975-2014:"
    );
    println!("   model:                 {}", v.line());
    let a = arrests();
    println!(
        "Toronto (2001-2002, chance of release, model blind to colour), trained on {} cases of 1997-2000:",
        a.training
    );
    println!("   model:                 {}", a.verdict.line());
    println!(
        "   canary: clock switched off, skill gain {:+.4}",
        a.canary_off_gain
    );
    println!(
        "   marker (chance of release below {ARREST_THRESHOLD:.2}): false positive and false negative rates by perceived colour"
    );
    for (year, colour, fpr, fnr, n) in &a.audit {
        println!("   {year} {colour:<5}  FPR {fpr:.3}  FNR {fnr:.3}  ({n} cases)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_civil_calendar_is_right() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(weekday(days_from_civil(2013, 10, 1)), 1); // a Tuesday
        assert_eq!(weekday(days_from_civil(1987, 10, 19)), 0); // a Monday
    }

    #[test]
    fn ece_is_zero_for_boxes_that_match_their_rate() {
        let p = vec![0.2; 10]
            .into_iter()
            .chain(vec![0.8; 10])
            .collect::<Vec<_>>();
        let y = [vec![1.0, 1.0], vec![0.0; 8], vec![1.0; 8], vec![0.0, 0.0]].concat();
        assert!(ece(&p, &y) < 1e-12);
    }

    #[test]
    fn kupiec_accepts_the_nominal_rate_and_rejects_ten_times_it() {
        assert!(kupiec(10, 1000, 0.01) > 0.99);
        assert!(kupiec(100, 1000, 0.01) < 1e-6);
    }

    #[test]
    fn christoffersen_rejects_clustered_violations() {
        let mut hits = vec![false; 1000];
        for h in hits.iter_mut().skip(500).take(10) {
            *h = true;
        }
        assert!(christoffersen(&hits) < 0.01);
    }

    #[test]
    fn the_chi_square_tail_is_right_at_its_quantiles() {
        assert!((chi2_1_sf(3.841_458_8) - 0.05).abs() < 1e-6);
        assert!((chi2_1_sf(6.634_896_6) - 0.01).abs() < 1e-6);
    }

    #[test]
    fn the_null_of_the_ece_shrinks_with_the_size_of_the_vault() {
        let small: Vec<f64> = (0..1_000)
            .map(|i| 0.1 + 0.8 * (i % 10) as f64 / 9.0)
            .collect();
        let large: Vec<f64> = (0..100_000)
            .map(|i| 0.1 + 0.8 * (i % 10) as f64 / 9.0)
            .collect();
        let (qs, ql) = (ece_null_q95(&small, 400, 7), ece_null_q95(&large, 400, 7));
        assert!(qs > 0.0 && ql > 0.0 && ql < qs / 5.0, "{qs} {ql}");
    }

    #[test]
    fn logistic_recovers_a_known_coefficient() {
        let x: Vec<Vec<f64>> = (0..2000).map(|i| vec![1.0, (i % 2) as f64]).collect();
        let y: Vec<f64> = (0..2000)
            .map(|i| {
                if (i % 2 == 1 && i % 4 != 1) || i % 8 == 0 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();
        let b = fit_logistic(&x, &y, 0.0);
        let p1 = sigmoid(b[0] + b[1]);
        let p0 = sigmoid(b[0]);
        assert!((p1 - 0.5).abs() < 1e-6 && (p0 - 0.25).abs() < 1e-6);
    }
}
