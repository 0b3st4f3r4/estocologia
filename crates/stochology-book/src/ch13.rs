//! Chapter 13 — H1, part 1: the volatility drag in the Dow Jones and the LaLonde experiment.
//!
//! Self-contained, no external dependencies: reads the public datasets of the repository and reproduces
//! the published numbers of the real H1 — time growth, drag, 20-year cohorts with a monthly start, the
//! 1987 shock, the resampled-years null and the raw decomposition against the randomised NSW experiment.
//! Run from the repository root:
//!     cargo run --release --bin labs -- --chapter 16
//! (accepts three optional paths when run on its own: the Dow CSV and the NSW and CPS CSVs)

const TRAIN_END_YEAR: i32 = 1999; // the training set is 1885-1999; the vault is 2000-2023
const HORIZON: i32 = 20; // years per cohort ("whoever holds for decades")
const CLAIMED_LEVERAGE: f64 = 3.0;
const N_NULL: usize = 2000; // resamplings of years for the claim world

#[derive(Clone)]
struct Sm64 {
    state: u64,
}

impl Sm64 {
    fn new(s: u64) -> Self {
        Sm64 { state: s }
    }
    fn next_u64(&mut self) -> u64 {
        stochology_core::rng::splitmix64_step(&mut self.state)
    }
    fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / 9007199254740992.0)
    }
}

fn quantile(mut v: Vec<f64>, p: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let i = ((p * v.len() as f64).ceil() as usize).max(1).min(v.len()) - 1;
    v[i]
}

fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}

fn sd(x: &[f64]) -> f64 {
    let m = mean(x);
    (x.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / (x.len() as f64 - 1.0)).sqrt()
}

/// Days since 1970-01-01 (Howard Hinnant, days_from_civil).
fn days_since_1970(y0: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y0 - 1 } else { y0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn is_leap(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn days_in_month(y: i64, m: i64) -> i64 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if is_leap(y) {
                29
            } else {
                28
            }
        }
    }
}

/// Reads the Dow Jones: date labels and closes. Columns: rownames,date,value.
fn read_dow(path: &str) -> (Vec<String>, Vec<f64>) {
    let text = std::fs::read_to_string(path).expect("did not read the Dow CSV");
    let mut labels = Vec::new();
    let mut values = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = line.split(',').collect();
        if c.len() < 3 {
            continue;
        }
        labels.push(c[1].trim().to_string());
        values.push(c[2].trim().parse().unwrap());
    }
    (labels, values)
}

/// Cumulative log-growth: c[k] is the sum of ln(1 + L r[i]) for i < k.
fn cumulative(r: &[f64], l: f64) -> Vec<f64> {
    let mut c = vec![0.0f64; r.len() + 1];
    for i in 0..r.len() {
        let y = 1.0 + l * r[i];
        c[i + 1] = c[i] + if y > 0.0 { y.ln() } else { f64::NEG_INFINITY };
    }
    c
}

/// Cohorts that start on the first trading session of each month and end on the last session before (start + 20 years).
/// Returns the ln of the final wealth of each cohort. Uses only sessions up to `end` (the training set).
#[allow(clippy::needless_range_loop)] // explicit indices: the cohort window is the subject of the code
fn monthly_cohorts(dates: &[i64], labels: &[String], r: &[f64], end: usize, l: f64) -> Vec<f64> {
    let c = cumulative(r, l);
    let mut out = Vec::new();
    for s in 1..end {
        if labels[s - 1][..7] == labels[s][..7] {
            continue; // only the first trading session of the month
        }
        let year: i64 = labels[s][..4].parse().unwrap();
        let month: i64 = labels[s][5..7].parse().unwrap();
        let day: i64 = labels[s][8..10].parse().unwrap();
        let target_year = year + HORIZON as i64;
        let target = days_since_1970(
            target_year,
            month,
            day.min(days_in_month(target_year, month)),
        );
        if dates[end - 1] < target {
            continue; // the cohort would not fit in the training set
        }
        let mut f = s;
        for j in s..end {
            if dates[j] <= target {
                f = j;
            } else {
                break;
            }
        }
        out.push(c[f] - c[s]);
    }
    out
}

/// 20-year moving sums of an annual series: the cohorts of the severe test.
fn annual_cohorts(series: &[f64]) -> Vec<f64> {
    let h = HORIZON as usize;
    (0..series.len().saturating_sub(h - 1))
        .map(|s| series[s..s + h].iter().sum())
        .collect()
}

fn fraction_above(cohorts: &[f64]) -> f64 {
    cohorts.iter().filter(|x| **x > 0.0).count() as f64 / cohorts.len() as f64
}

/// World of the claim: the years of the training set are resampled and wealth grows 3 times the annual
/// arithmetic sum, without drag. Returns (mean fraction, 1% quantile, p-value against the observed).
fn claim_world(
    annual_arithmetic: &[f64],
    observed: f64,
    reps: usize,
    seed: u64,
) -> (f64, f64, f64) {
    let mut rng = Sm64::new(seed);
    let n = annual_arithmetic.len();
    let mut fracs = Vec::with_capacity(reps);
    for _ in 0..reps {
        let series: Vec<f64> = (0..n)
            .map(|_| CLAIMED_LEVERAGE * annual_arithmetic[(rng.uniform() * n as f64) as usize % n])
            .collect();
        fracs.push(fraction_above(&annual_cohorts(&series)));
    }
    let p = fracs.iter().filter(|x| **x <= observed).count() as f64 / reps as f64;
    (mean(&fracs), quantile(fracs, 0.01), p)
}

/// One person of the NSW or the CPS: the columns used are treat (2), re75 (10) and re78 (11).
struct Person {
    treated: bool,
    re75: f64,
    re78: f64,
}

fn read_lalonde(path: &str) -> Vec<Person> {
    let text = std::fs::read_to_string(path).expect("did not read the NSW/CPS CSV");
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = line.split(',').collect();
        if c.len() < 12 {
            continue;
        }
        out.push(Person {
            treated: c[2].trim() == "1",
            re75: c[10].trim().parse().unwrap(),
            re78: c[11].trim().parse().unwrap(),
        });
    }
    out
}

fn income(v: &[&Person]) -> f64 {
    v.iter().map(|p| p.re78).sum::<f64>() / v.len() as f64
}

fn employment(v: &[&Person], year: u8) -> f64 {
    let n = v
        .iter()
        .filter(|p| {
            if year == 75 {
                p.re75 > 0.0
            } else {
                p.re78 > 0.0
            }
        })
        .count();
    n as f64 / v.len() as f64
}

/// Resolves the data path: optional base in ESTOCOLOGIA_DADOS, otherwise the repository.
fn data_path(default: &str) -> String {
    match std::env::var("ESTOCOLOGIA_DADOS") {
        Ok(base) => format!("{}/{}", base.trim_end_matches('/'), default),
        Err(_) => default.to_string(),
    }
}

pub fn run() {
    let dow_csv = data_path("data/djia_1885_2023.csv");
    let nsw_csv = data_path("data/nsw_experimental.csv");
    let cps_csv = data_path("data/cps_observational.csv");

    println!("== Chapter 13 :: volatility drag and the selection bias ==");
    let (labels, prices) = read_dow(&dow_csv);
    let dates: Vec<i64> = labels
        .iter()
        .map(|s| {
            days_since_1970(
                s[..4].parse().unwrap(),
                s[5..7].parse().unwrap(),
                s[8..10].parse().unwrap(),
            )
        })
        .collect();
    let r: Vec<f64> = (1..prices.len())
        .map(|i| prices[i] / prices[i - 1] - 1.0)
        .collect();
    let years: Vec<i32> = labels[1..]
        .iter()
        .map(|s| s[..4].parse().unwrap())
        .collect();
    let end = years
        .iter()
        .position(|y| *y > TRAIN_END_YEAR)
        .unwrap_or(years.len());
    let mut years_u: Vec<i32> = years[..end].to_vec();
    years_u.dedup();
    let n_years = years_u.len() as f64;
    let rt = &r[..end];

    println!(
        "training 1885-1999: {} sessions in {:.0} years; whole series 1885-2023: {} sessions",
        rt.len(),
        n_years,
        r.len()
    );
    let mu_day = mean(rt);
    println!(
        "daily arithmetic mean = {:.9} ({:.4} per year), daily sd = {:.6} ({:.4} per year)",
        mu_day,
        mu_day * rt.len() as f64 / n_years,
        sd(rt),
        sd(rt) * (rt.len() as f64 / n_years).sqrt()
    );

    println!();
    println!("time growth per year and drag (cohorts and trajectories of the training set)");
    let mut g = Vec::new();
    for l in [1.0f64, 2.0, CLAIMED_LEVERAGE] {
        let gl = rt.iter().map(|x| (1.0 + l * x).ln()).sum::<f64>() / n_years;
        g.push(gl);
        println!(
            "  L={:.0}: the arithmetic sum would give {:.4} per year; the realised growth is {:.4}; drag {:.4}",
            l,
            l * mu_day * rt.len() as f64 / n_years,
            gl,
            l * mu_day * rt.len() as f64 / n_years - gl
        );
    }
    let (mut worst, mut iworst) = (0.0f64, 0usize);
    for (i, x) in rt.iter().enumerate() {
        if *x < worst {
            worst = *x;
            iworst = i;
        }
    }
    println!(
        "worst session: {:.4} ({:.2}%) on {}; loss at L=1 {:.3}, L=2 {:.3}, L=3 {:.3}; daily ruin at L >= {:.2}",
        worst,
        100.0 * worst,
        labels[iworst + 1],
        -worst,
        -2.0 * worst,
        -CLAIMED_LEVERAGE * worst,
        -1.0 / worst
    );

    for (name, from, to) in [
        ("calendar year 1929-1932", "1929-01-01", "1932-12-31"),
        (
            "peak to trough, 1929-09-03 to 1932-07-08",
            "1929-09-03",
            "1932-07-08",
        ),
    ] {
        let i0 = labels.iter().position(|s| s.as_str() >= from).unwrap();
        let i1 = labels.iter().rposition(|s| s.as_str() <= to).unwrap();
        for l in [1.0f64, CLAIMED_LEVERAGE] {
            let w: f64 = (i0..i1).map(|i| 1.0 + l * r[i]).product();
            println!(
                "  {} at L={:.0}: the capital ends at {:.5} of the initial one ({:.1}% destroyed)",
                name,
                l,
                w,
                100.0 * (1.0 - w)
            );
        }
    }

    println!();
    println!("20-year cohorts starting on the first trading session of each month (training set):");
    println!(
        "{:<6} {:>12} {:>14} {:>14} {:>16}",
        "L", "cohorts", "median (x)", "mean (x)", "above the start"
    );
    for l in [1.0f64, 2.0, CLAIMED_LEVERAGE] {
        let co = monthly_cohorts(&dates, &labels, &r, end, l);
        let wealths: Vec<f64> = co.iter().map(|x| x.exp()).collect();
        let n = co.len();
        println!(
            "{:<6.0} {:>12} {:>14.2} {:>14.1} {:>16.4}",
            l,
            n,
            quantile(co.clone(), 0.5).exp(),
            mean(&wealths),
            fraction_above(&co)
        );
    }

    // ln-growth per unit-year (calendar year): the unit of the severe test.
    let mut year_series: Vec<i32> = Vec::new();
    let mut g1: Vec<f64> = Vec::new();
    let mut g3: Vec<f64> = Vec::new();
    let mut a_arith: Vec<f64> = Vec::new();
    for i in 0..end {
        if year_series.last() != Some(&years[i]) {
            year_series.push(years[i]);
            g1.push(0.0);
            g3.push(0.0);
            a_arith.push(0.0);
        }
        let j = g1.len() - 1;
        g1[j] += (1.0 + r[i]).ln();
        g3[j] += (1.0 + CLAIMED_LEVERAGE * r[i]).ln();
        a_arith[j] += r[i];
    }
    println!();
    println!(
        "cohorts per unit-year ({} years, moving sum of {}): above the start at 1x = {:.4}, at 3x = {:.4} ({} cohorts)",
        year_series.len(),
        HORIZON,
        fraction_above(&annual_cohorts(&g1)),
        fraction_above(&annual_cohorts(&g3)),
        annual_cohorts(&g3).len()
    );
    let obs = fraction_above(&annual_cohorts(&g3));
    let (null_mean, q01, p) = claim_world(&a_arith, obs, N_NULL, 17_001);
    println!(
        "claim world (training years resampled, growth 3*annual arithmetic sum, without drag):"
    );
    println!(
        "  {} replicas: mean fraction of the cohorts above the start = {:.4}, 1% quantile = {:.4}, p(fraction <= {:.4}) = {:.4}",
        N_NULL, null_mean, q01, obs, p
    );

    let nsw = read_lalonde(&nsw_csv);
    let cps = read_lalonde(&cps_csv);
    let treated: Vec<&Person> = nsw.iter().filter(|p| p.treated).collect();
    let control: Vec<&Person> = nsw.iter().filter(|p| !p.treated).collect();
    let all_cps: Vec<&Person> = cps.iter().collect();
    println!();
    println!(
        "LaLonde: NSW with {} treated and {} randomised controls; CPS with {} observations",
        treated.len(),
        control.len(),
        cps.len()
    );
    println!(
        "{:<40} {:>10} {:>14}",
        "comparison (1978 outcome)", "employment", "income (US$)"
    );
    println!(
        "{:<40} {:>10.3} {:>14.0}",
        "randomised experiment (NSW treat. - NSW ctrl)",
        employment(&treated, 78) - employment(&control, 78),
        income(&treated) - income(&control)
    );
    println!(
        "{:<40} {:>10.3} {:>14.0}",
        "raw comparison (NSW treat. - CPS)",
        employment(&treated, 78) - employment(&all_cps, 78),
        income(&treated) - income(&all_cps)
    );
    println!(
        "{:<40} {:>10.3} {:>14.0}",
        "implied selection bias (raw - experiment)",
        (employment(&treated, 78) - employment(&all_cps, 78))
            - (employment(&treated, 78) - employment(&control, 78)),
        (income(&treated) - income(&all_cps)) - (income(&treated) - income(&control))
    );
    println!(
        "employment levels in 1978: NSW treated {:.3}, NSW controls {:.3}, CPS {:.3}; in 100 people: {:.0}, {:.0} and {:.0}",
        employment(&treated, 78),
        employment(&control, 78),
        employment(&all_cps, 78),
        100.0 * employment(&treated, 78),
        100.0 * employment(&control, 78),
        100.0 * employment(&all_cps, 78)
    );
    println!(
        "mean income of 1978 (US$): NSW treated {:.0}, NSW controls {:.0}, CPS {:.0}",
        income(&treated),
        income(&control),
        income(&all_cps)
    );
    println!(
        "1975 placebo (employment): NSW treated {:.3}, NSW controls {:.3}, CPS {:.3}; raw difference treated - CPS = {:.3}",
        employment(&treated, 75),
        employment(&control, 75),
        employment(&all_cps, 75),
        employment(&treated, 75) - employment(&all_cps, 75)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leverage_drag_and_selection_bias_match_stochology_horizons_h1() {
        let r = [-0.10, 0.10];
        let ac1 = cumulative(&r, 1.0);
        let ac3 = cumulative(&r, 3.0);
        assert!(ac3.last().unwrap() < ac1.last().unwrap());
        assert!(stochology_horizons::h1::drag(3.0, &r) > stochology_horizons::h1::drag(1.0, &r));
        assert!(is_leap(2000) && !is_leap(1900));
    }
}
