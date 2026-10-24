//! Real empirical dataset loaders (`data/*.csv`), empirical card ground-truth auditor,
//! and un-clamped cluster-permutation severe test (`seed = 2026`).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use stochology_core::{
    apply_cromwell_floor, mean, sha256_hex, EpistemologicalCard, ExceedanceDesignEffect,
    NonlinearSummary, Rng, DEFAULT_SEED,
};
use stochology_lenses::{CausalEnvironment, CorpPavDecomposition};
use stochology_siga::{
    run_full_siga_roadmap_empirical, simulate_sbc_beta_binomial_ranks, SigaExecutionReport,
};

/// Paths of the 3 canonical Epistemological Cards on disk (`cards/*.yaml`).
pub const CANONICAL_CARD_PATHS: [&str; 3] = [
    "cards/card_01_atlantic_storms_evacuation.yaml",
    "cards/card_02_bank_queue_staffing.yaml",
    "cards/card_03_nyc_flights_holding.yaml",
];

/// Empirical ground-truth metrics computed directly from a card's declared `data/*.csv` file.
#[derive(Debug, Clone)]
pub struct EmpiricalDatasetAudit {
    pub dataset_path: String,
    pub dataset_sha256: String,
    pub n_nominal: usize,
    pub num_clusters: usize,
    pub m_bar: f64,
    pub m_tilde: f64,
    pub rho_exceedance_90: f64,
    pub deff_exceedance_90: f64,
    pub n_eff_exceedance_90: usize,
    pub jensen_summary: NonlinearSummary,
    pub event_probability_p: f64,
    pub corp_pav: CorpPavDecomposition,
    pub confusion_tpr_fpr_at_p_star: (f64, f64),
    pub sbc_ranks: Vec<f64>,
    pub observed_test_statistic: f64,
    pub null_distribution_samples: Vec<f64>,
    pub ville_e_values: Vec<f64>,
}

/// Loads and parses the 3 canonical YAML cards directly from disk (`cards/*.yaml`).
pub fn load_cards_from_disk(root_dir: &Path) -> Result<Vec<(String, EpistemologicalCard)>, String> {
    let mut out = Vec::with_capacity(CANONICAL_CARD_PATHS.len());
    for rel_path in CANONICAL_CARD_PATHS {
        let full_path = root_dir.join(rel_path);
        let content = fs::read_to_string(&full_path)
            .map_err(|e| format!("Failed to read '{}': {e}", full_path.display()))?;
        let card = EpistemologicalCard::from_yaml(&content)
            .map_err(|e| format!("Failed to parse '{rel_path}': {e}"))?;
        out.push((rel_path.to_string(), card));
    }
    Ok(out)
}

/// Reads `data/SHA256SUMS` and returns a map of `relative_path -> expected_sha256`.
pub fn load_sha256sums_manifest(root_dir: &Path) -> Result<BTreeMap<String, String>, String> {
    let path = root_dir.join("data/SHA256SUMS");
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;
    let mut map = BTreeMap::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        if let (Some(hash), Some(rel)) = (parts.next(), parts.next()) {
            map.insert(rel.to_string(), hash.to_string());
        }
    }
    Ok(map)
}

/// Reads real Danish Fire Insurance losses (`data/danish_fire.csv`).
pub fn load_danish_fire(root_dir: &Path) -> Vec<f64> {
    let path = root_dir.join("data/danish_fire.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/danish_fire.csv");
    content
        .lines()
        .skip(1)
        .filter_map(|line| {
            let (_, loss_str) = line.split_once(',')?;
            loss_str.trim().parse::<f64>().ok()
        })
        .collect()
}

/// Reads bank call counts grouped by operational day (`data/bank_calls.csv`),
/// returning daily clusters of 5-minute call-volume windows.
pub fn load_bank_call_clusters(root_dir: &Path) -> Vec<Vec<f64>> {
    let path = root_dir.join("data/bank_calls.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/bank_calls.csv");
    let mut by_day: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for line in content.lines().skip(1) {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() >= 3 {
            let date = cols[1].split('T').next().unwrap_or("").to_string();
            if let Ok(calls) = cols[2].trim().parse::<f64>() {
                by_day.entry(date).or_default().push(calls);
            }
        }
    }
    by_day.into_values().collect()
}

/// Reads real `re78` earnings from the NSW experiment (`data/nsw_experimental.csv`, treated vs. control)
/// and the observational CPS sample (`data/cps_observational.csv`).
pub fn load_lalonde_nsw_cps(root_dir: &Path) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let read_file = |rel: &str| -> (Vec<f64>, Vec<f64>) {
        let content = fs::read_to_string(root_dir.join(rel)).expect("failed to read LaLonde CSV");
        let mut treated = Vec::new();
        let mut control = Vec::new();
        for line in content.lines().skip(1) {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() >= 12 {
                let is_treated = cols[2].trim() == "1";
                if let Ok(re78) = cols[11].trim().parse::<f64>() {
                    if is_treated {
                        treated.push(re78);
                    } else {
                        control.push(re78);
                    }
                }
            }
        }
        (treated, control)
    };
    let (nsw_t, nsw_c) = read_file("data/nsw_experimental.csv");
    let (_, cps_c) = read_file("data/cps_observational.csv");
    (nsw_t, nsw_c, cps_c)
}

/// Reads daily arithmetic returns $(S_t - S_{t-1}) / S_{t-1}$ of the Dow Jones Industrial Average (`data/djia_1885_2023.csv`)
/// so that ensemble drift $\mu_{\text{ens}}$ and Itô time-average growth $g_{\text{time}} = \mu_{\text{ens}} - \frac{1}{2}\sigma^2$
/// are evaluated without double-subtracting the variance drain.
pub fn load_djia_returns(root_dir: &Path) -> Vec<f64> {
    let path = root_dir.join("data/djia_1885_2023.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/djia_1885_2023.csv");
    let closes: Vec<f64> = content
        .lines()
        .skip(1)
        .filter_map(|l| {
            let cols: Vec<&str> = l.split(',').collect();
            cols.get(2)?.trim().parse::<f64>().ok()
        })
        .filter(|&c| c > 0.0)
        .collect();
    closes.windows(2).map(|w| (w[1] - w[0]) / w[0]).collect()
}

/// Reads all Atlantic storm observations (`data/atlantic_storms.csv`), returning uncalibrated
/// barometric hurricane-intensity probabilities (`wind >= 64` knots) and binary outcomes (`0.0` or `1.0`)
/// used to demonstrate PAV isotonic recalibration in Lab 3.
pub fn load_atlantic_storms_forecasts(root_dir: &Path) -> (Vec<f64>, Vec<f64>) {
    let path = root_dir.join("data/atlantic_storms.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/atlantic_storms.csv");
    let mut probs = Vec::new();
    let mut outcomes = Vec::new();
    for line in content.lines().skip(1) {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() >= 12 {
            if let (Ok(wind), Ok(pressure)) = (
                cols[10].trim().parse::<f64>(),
                cols[11].trim().parse::<f64>(),
            ) {
                let z = (1005.0 - pressure) * 0.12;
                let p = apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001);
                let y = if wind >= 64.0 { 1.0 } else { 0.0 };
                probs.push(p);
                outcomes.push(y);
            }
        }
    }
    (probs, outcomes)
}

/// Splits `data/atlantic_storms.csv` ($n = 20\,778$) into 3 climatological seasonal regimes
/// (`storms_fringe_season`, `storms_shoulder_season_aug_oct`, `storms_peak_season_sep`)
/// to audit nonlinear physical invariance of cyclostrophic barometric pressure deficit (`X0`)
/// vs. a regime-shifted latitude proxy (`X1`) predicting normalized maximum sustained wind (`Y`).
pub fn load_atlantic_storms_decade_environments(root_dir: &Path) -> Vec<CausalEnvironment> {
    let path = root_dir.join("data/atlantic_storms.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/atlantic_storms.csv");
    let mut x_e0 = Vec::new();
    let mut y_e0 = Vec::new();
    let mut x_e1 = Vec::new();
    let mut y_e1 = Vec::new();
    let mut x_e2 = Vec::new();
    let mut y_e2 = Vec::new();

    for line in content.lines().skip(1) {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() >= 12 {
            if let (Ok(month), Ok(lat), Ok(wind), Ok(pressure)) = (
                cols[3].trim().parse::<u32>(),
                cols[6].trim().parse::<f64>(),
                cols[10].trim().parse::<f64>(),
                cols[11].trim().parse::<f64>(),
            ) {
                let x0 = (1010.0 - pressure) / 20.0;
                let y = (wind - 50.0) / 25.0;
                let lat_z = (lat - 25.0) / 10.0;
                if !(8..=10).contains(&month) {
                    x_e0.push(vec![x0, lat_z]);
                    y_e0.push(y);
                } else if month == 8 || month == 10 {
                    x_e1.push(vec![x0, lat_z]);
                    y_e1.push(y);
                } else {
                    x_e2.push(vec![x0, lat_z]);
                    y_e2.push(y);
                }
            }
        }
    }

    vec![
        CausalEnvironment {
            name: "storms_fringe_season".to_string(),
            x: x_e0,
            y: y_e0,
        },
        CausalEnvironment {
            name: "storms_shoulder_season_aug_oct".to_string(),
            x: x_e1,
            y: y_e1,
        },
        CausalEnvironment {
            name: "storms_peak_season_sep".to_string(),
            x: x_e2,
            y: y_e2,
        },
    ]
}

/// Reads all half-hourly electricity demand observations (MW) of Victoria, Australia (`data/vic_elec.csv`).
pub fn load_vic_elec_demand(root_dir: &Path) -> Vec<f64> {
    let path = root_dir.join("data/vic_elec.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/vic_elec.csv");
    content
        .lines()
        .skip(1)
        .filter_map(|line| {
            let cols: Vec<&str> = line.split(',').collect();
            cols.get(2)?.trim().parse::<f64>().ok()
        })
        .collect()
}

/// Reads Toronto arrest records (`data/toronto_arrests.csv`) and returns empirical base rates
/// $(p_A, p_B)$ of non-release (`released == "No"`) across two demographic groups with unequal base rates.
pub fn load_toronto_arrests_prevalences(root_dir: &Path) -> (f64, f64) {
    let path = root_dir.join("data/toronto_arrests.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/toronto_arrests.csv");
    let mut a_pos = 0usize;
    let mut a_tot = 0usize;
    let mut b_pos = 0usize;
    let mut b_tot = 0usize;
    for line in content.lines().skip(1) {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() >= 8 {
            let not_released = cols[1].trim().trim_matches('"') == "No";
            let group = cols[2].trim().trim_matches('"');
            let employed = cols[6].trim().trim_matches('"') == "Yes";
            if group == "Black" && !employed {
                a_tot += 1;
                if not_released {
                    a_pos += 1;
                }
            } else if group == "White" && employed {
                b_tot += 1;
                if not_released {
                    b_pos += 1;
                }
            }
        }
    }
    let p_a = (a_pos as f64) / (a_tot.max(1) as f64);
    let p_b = (b_pos as f64) / (b_tot.max(1) as f64);
    (p_a, p_b)
}

/// Reads standardized distance / arrival-delay features from `data/nyc_flights_2013.csv` (source $P$)
/// and `data/nyc_flights_2023.csv` (target $Q$), systematically stratified across the full annual files.
pub fn load_nyc_flights_2013_2023(root_dir: &Path) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let (x13, y13) = load_flight_stream(root_dir, "data/nyc_flights_2013.csv", 1_500);
    let (x23, y23) = load_flight_stream(root_dir, "data/nyc_flights_2023.csv", 1_500);
    (x13, y13, x23, y23)
}

/// Reads the pre-departure intraday delay-cascade feature (standardized scheduled departure hour plus
/// causally lagged rolling arrival delay of prior flights) and binary delay outcome ($\text{arr\_delay} > 15\text{ min}$)
/// from `data/nyc_flights_2013.csv` (source $P$) and `data/nyc_flights_2023.csv` (target $Q$).
pub fn load_nyc_flights_cascade_2013_2023(
    root_dir: &Path,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let load_cascade = |rel_path: &str, max_rows: usize| -> (Vec<f64>, Vec<f64>) {
        let content =
            fs::read_to_string(root_dir.join(rel_path)).expect("failed to read flight CSV");
        let mut all_x = Vec::new();
        let mut all_y = Vec::new();
        let mut ema_delay = 0.0_f64;
        let mut prev_day = (0u32, 0u32);
        for line in content.lines().skip(1) {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() >= 9 {
                if let (Ok(month), Ok(day), Ok(sched_dep), Ok(arr_delay)) = (
                    cols[0].trim().parse::<u32>(),
                    cols[1].trim().parse::<u32>(),
                    cols[2].trim().parse::<f64>(),
                    cols[8].trim().parse::<f64>(),
                ) {
                    if (month, day) != prev_day {
                        ema_delay = 0.0;
                        prev_day = (month, day);
                    }
                    let hr = (sched_dep / 100.0).floor();
                    let feat = 0.35 * ((hr - 13.0) / 5.0) + 0.65 * (ema_delay / 25.0);
                    all_x.push(feat.clamp(-3.0, 5.0));
                    all_y.push(if arr_delay > 15.0 { 1.0 } else { 0.0 });
                    ema_delay = 0.80 * ema_delay + 0.20 * arr_delay;
                }
            }
        }
        if max_rows == 0 || all_x.len() <= max_rows {
            return (all_x, all_y);
        }
        let n = all_x.len();
        let mut x = Vec::with_capacity(max_rows);
        let mut y = Vec::with_capacity(max_rows);
        for i in 0..max_rows {
            let idx = (i * n) / max_rows;
            x.push(all_x[idx]);
            y.push(all_y[idx]);
        }
        (x, y)
    };
    let (x13, y13) = load_cascade("data/nyc_flights_2013.csv", 1_500);
    let (x23, y23) = load_cascade("data/nyc_flights_2023.csv", 1_500);
    (x13, y13, x23, y23)
}

/// Reads standardized distance $((d - 1000)/1000)$ and binary arrival-delay indicator $(\text{arr\_delay} > 15\text{ min})$
/// from any flight CSV in `data/` (`nyc_flights_2013.csv`, `nyc_flights_2023.csv`, `nyc_flights_2013_h1.csv`,
/// `nyc_flights_2023_h1.csv`, or `houston_flights_2011.csv`), scanning the entire file and taking a deterministic
/// equispaced sample of `max_rows` across the full temporal span.
pub fn load_flight_stream(
    root_dir: &Path,
    rel_path: &str,
    max_rows: usize,
) -> (Vec<f64>, Vec<f64>) {
    let content = fs::read_to_string(root_dir.join(rel_path)).expect("failed to read flight CSV");
    let mut all_x = Vec::new();
    let mut all_y = Vec::new();
    for line in content.lines().skip(1) {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() >= 9 {
            if let (Ok(distance), Ok(arr_delay)) =
                (cols[7].trim().parse::<f64>(), cols[8].trim().parse::<f64>())
            {
                all_x.push((distance - 1_000.0) / 1_000.0);
                all_y.push(if arr_delay > 15.0 { 1.0 } else { 0.0 });
            }
        }
    }
    if max_rows == 0 || all_x.len() <= max_rows {
        return (all_x, all_y);
    }
    let n = all_x.len();
    let mut x = Vec::with_capacity(max_rows);
    let mut y = Vec::with_capacity(max_rows);
    for i in 0..max_rows {
        let idx = (i * n) / max_rows;
        x.push(all_x[idx]);
        y.push(all_y[idx]);
    }
    (x, y)
}

/// Reads the 3 out-of-time / out-of-domain flight reserve datasets (`data/nyc_flights_2013_h1.csv`,
/// `data/nyc_flights_2023_h1.csv`, and `data/houston_flights_2011.csv`).
#[allow(clippy::type_complexity)]
pub fn load_oot_ood_flight_reserves(
    root_dir: &Path,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let (x13_h1, y13_h1) = load_flight_stream(root_dir, "data/nyc_flights_2013_h1.csv", 1_500);
    let (x23_h1, y23_h1) = load_flight_stream(root_dir, "data/nyc_flights_2023_h1.csv", 1_500);
    let (x_hou, y_hou) = load_flight_stream(root_dir, "data/houston_flights_2011.csv", 1_500);
    (x13_h1, y13_h1, x23_h1, y23_h1, x_hou, y_hou)
}

/// Computes the 30-block Ville betting martingale factors $L_1, \dots, L_{30}$ directly from
/// empirical forecasts `probs` and binary outcomes `outcomes` under $H_0: \mathbb{E}[Y_i \mid \hat{p}_i] = \bar{Y}$.
pub fn compute_ville_betting_e_values(
    probs: &[f64],
    outcomes: &[f64],
    num_blocks: usize,
) -> Vec<f64> {
    assert!(!outcomes.is_empty() && probs.len() == outcomes.len() && num_blocks >= 1);
    let n = outcomes.len();
    let p0 = mean(outcomes).clamp(1e-6, 1.0 - 1e-6);
    let var0 = (p0 * (1.0 - p0)).max(1e-6);
    let block_size = (n / num_blocks).max(1);
    let mut factors = Vec::with_capacity(num_blocks);
    for b in 0..num_blocks {
        let start = b * block_size;
        let end = if b + 1 == num_blocks {
            n
        } else {
            ((b + 1) * block_size).min(n)
        };
        if start >= end {
            break;
        }
        let mut sum = 0.0;
        for i in start..end {
            sum += 1.0 + 0.8 * ((probs[i] - p0) * (outcomes[i] - p0)) / var0;
        }
        let lb = (sum / ((end - start) as f64)).max(0.01);
        factors.push(lb);
    }
    factors
}

/// Computes the un-clamped cluster-permutation test statistic $T_{\text{obs}}$ and 200 null
/// replications $T^{(1)}, \dots, T^{(200)}$ (`seed = 2026`) by permuting cluster outcome rates
/// across the independent clusters.
pub fn compute_unclamped_cluster_permutation_null(
    cluster_probs: &[Vec<f64>],
    cluster_outcomes: &[Vec<f64>],
    seed: u64,
    num_replications: usize,
) -> (f64, Vec<f64>) {
    assert!(!cluster_probs.is_empty() && cluster_probs.len() == cluster_outcomes.len());
    let c_len = cluster_probs.len();
    let mut total_n = 0usize;
    let mut sum_p = 0.0;
    let mut sum_y = 0.0;
    let mut cluster_mean_p = Vec::with_capacity(c_len);
    let mut cluster_mean_y = Vec::with_capacity(c_len);
    let mut cluster_weights = Vec::with_capacity(c_len);

    for (cp, cy) in cluster_probs.iter().zip(cluster_outcomes.iter()) {
        let m = cp.len().max(1);
        total_n += m;
        let sp: f64 = cp.iter().sum();
        let sy: f64 = cy.iter().sum();
        sum_p += sp;
        sum_y += sy;
        cluster_mean_p.push(sp / (m as f64));
        cluster_mean_y.push(sy / (m as f64));
        cluster_weights.push(m as f64);
    }

    let n_f = total_n.max(1) as f64;
    let p_bar = sum_p / n_f;
    let y_bar = sum_y / n_f;

    let mut obs_cov = 0.0;
    for (cp, cy) in cluster_probs.iter().zip(cluster_outcomes.iter()) {
        for (&p, &y) in cp.iter().zip(cy.iter()) {
            obs_cov += (p - p_bar) * (y - y_bar);
        }
    }
    let t_obs = obs_cov / n_f;

    let mut rng = Rng::new(seed);
    let mut indices: Vec<usize> = (0..c_len).collect();
    let mut null_samples = Vec::with_capacity(num_replications);

    for _ in 0..num_replications {
        // Fisher-Yates shuffle of cluster outcome indices
        for i in (1..c_len).rev() {
            let j = (rng.next_u64() as usize) % (i + 1);
            indices.swap(i, j);
        }
        let mut null_cov = 0.0;
        for (c_idx, &perm_idx) in indices.iter().enumerate() {
            let w = cluster_weights[c_idx];
            let dp = cluster_mean_p[c_idx] - p_bar;
            let dy = cluster_mean_y[perm_idx] - y_bar;
            null_cov += w * dp * dy;
        }
        null_samples.push(null_cov / n_f);
    }

    (t_obs, null_samples)
}

fn compute_confusion_tpr_fpr(probs: &[f64], outcomes: &[f64], p_star: f64) -> (f64, f64) {
    let mut tp = 0usize;
    let mut fn_c = 0usize;
    let mut fp = 0usize;
    let mut tn = 0usize;
    for (&p, &y) in probs.iter().zip(outcomes.iter()) {
        let alarm = p >= p_star;
        if y >= 0.5 {
            if alarm {
                tp += 1;
            } else {
                fn_c += 1;
            }
        } else if alarm {
            fp += 1;
        } else {
            tn += 1;
        }
    }
    let tpr = (tp as f64) / ((tp + fn_c).max(1) as f64);
    let fpr = (fp as f64) / ((fp + tn).max(1) as f64);
    (tpr, fpr)
}

/// Computes the complete [`EmpiricalDatasetAudit`] directly from the CSV declared in `card.data_and_audit_contract.dataset_path`.
pub fn compute_empirical_dataset_audit(
    root_dir: &Path,
    card: &EpistemologicalCard,
) -> Result<EmpiricalDatasetAudit, String> {
    let rel_csv = card.data_and_audit_contract.dataset_path.trim();
    let full_csv = root_dir.join(rel_csv);
    let raw_bytes = fs::read(&full_csv)
        .map_err(|e| format!("Failed to read dataset '{}': {e}", full_csv.display()))?;
    let actual_sha256 = sha256_hex(&raw_bytes);

    let manifest = load_sha256sums_manifest(root_dir)?;
    let Some(manifest_sha) = manifest.get(rel_csv) else {
        return Err(format!(
            "Dataset '{rel_csv}' is missing from data/SHA256SUMS"
        ));
    };
    if &actual_sha256 != manifest_sha {
        return Err(format!(
            "Dataset '{rel_csv}' SHA-256 ({actual_sha256}) diverges from data/SHA256SUMS ({manifest_sha})"
        ));
    }

    let content = String::from_utf8(raw_bytes)
        .map_err(|e| format!("Dataset '{rel_csv}' is not valid UTF-8: {e}"))?;

    let p_star = card.decision_rule_and_scope.critical_threshold_p_star;

    let (clusters_raw, jensen, probs, outcomes, cluster_probs, cluster_outcomes) = match rel_csv {
        "data/atlantic_storms.csv" => {
            let mut by_storm: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
            let mut raw_pairs = Vec::new();
            for line in content.lines().skip(1) {
                let cols: Vec<&str> = line.split(',').collect();
                if cols.len() >= 12 {
                    if let (Ok(w), Ok(p)) = (
                        cols[10].trim().parse::<f64>(),
                        cols[11].trim().parse::<f64>(),
                    ) {
                        let key = format!("{}_{}", cols[1].trim(), cols[2].trim());
                        by_storm.entry(key).or_default().push((w, p));
                        raw_pairs.push((w, p));
                    }
                }
            }
            let clusters: Vec<Vec<f64>> = by_storm
                .values()
                .map(|v| v.iter().map(|&(w, _)| w).collect())
                .collect();
            let storm_peaks: Vec<f64> = clusters
                .iter()
                .map(|c| c.iter().copied().fold(0.0_f64, f64::max))
                .collect();
            let sp = storm_peaks.len() * 4 / 5;
            let jensen = NonlinearSummary::evaluate_out_of_sample(
                &storm_peaks[..sp],
                &storm_peaks[sp..],
                |x| x * x,
                2.0,
            );
            let mut probs = Vec::with_capacity(raw_pairs.len());
            let mut outcomes = Vec::with_capacity(raw_pairs.len());
            for &(w, p) in &raw_pairs {
                let z = (968.0 - p) * 0.11;
                probs.push(apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001));
                outcomes.push(if w > 85.0 { 1.0 } else { 0.0 });
            }
            let mut c_probs = Vec::with_capacity(by_storm.len());
            let mut c_outs = Vec::with_capacity(by_storm.len());
            for v in by_storm.values() {
                c_probs.push(
                    v.iter()
                        .map(|&(_, p)| {
                            let z = (968.0 - p) * 0.11;
                            apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001)
                        })
                        .collect(),
                );
                c_outs.push(
                    v.iter()
                        .map(|&(w, _)| if w > 85.0 { 1.0 } else { 0.0 })
                        .collect(),
                );
            }
            (clusters, jensen, probs, outcomes, c_probs, c_outs)
        }
        "data/bank_calls.csv" => {
            let clusters = load_bank_call_clusters(root_dir);
            let daily_totals: Vec<f64> = clusters.iter().map(|c| c.iter().sum::<f64>()).collect();
            let sp = daily_totals.len() * 4 / 5;
            let jensen = NonlinearSummary::evaluate_out_of_sample(
                &daily_totals[..sp],
                &daily_totals[sp..],
                |x| x * x,
                2.0,
            );
            let mut probs = Vec::new();
            let mut outcomes = Vec::new();
            let mut c_probs = Vec::with_capacity(clusters.len());
            let mut c_outs = Vec::with_capacity(clusters.len());
            for d in &clusters {
                let mut prev = d[0];
                let mut cp = Vec::with_capacity(d.len());
                let mut cy = Vec::with_capacity(d.len());
                for &x in d {
                    let z = -0.35 + (prev - 286.0) * 0.025;
                    let p = apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001);
                    let y = if x > 286.0 { 1.0 } else { 0.0 };
                    probs.push(p);
                    outcomes.push(y);
                    cp.push(p);
                    cy.push(y);
                    prev = x;
                }
                c_probs.push(cp);
                c_outs.push(cy);
            }
            (clusters, jensen, probs, outcomes, c_probs, c_outs)
        }
        "data/nyc_flights_2013.csv" => {
            let mut by_day: BTreeMap<(u32, u32), Vec<(f64, f64)>> = BTreeMap::new();
            for line in content.lines().skip(1) {
                let cols: Vec<&str> = line.split(',').collect();
                if cols.len() >= 9 {
                    if let (Ok(m), Ok(d), Ok(sched_dep), Ok(delay)) = (
                        cols[0].trim().parse::<u32>(),
                        cols[1].trim().parse::<u32>(),
                        cols[2].trim().parse::<f64>(),
                        cols[8].trim().parse::<f64>(),
                    ) {
                        by_day.entry((m, d)).or_default().push((sched_dep, delay));
                    }
                }
            }
            let clusters: Vec<Vec<f64>> = by_day
                .values()
                .map(|v| v.iter().map(|&(_, delay)| delay).collect())
                .collect();
            let day_pos_delays: Vec<f64> = clusters
                .iter()
                .map(|c| c.iter().map(|&d| d.max(0.0)).sum::<f64>() / (c.len().max(1) as f64))
                .collect();
            let sp = day_pos_delays.len() * 4 / 5;
            let jensen = NonlinearSummary::evaluate_out_of_sample(
                &day_pos_delays[..sp],
                &day_pos_delays[sp..],
                |x| x * x,
                2.0,
            );
            let mut probs = Vec::new();
            let mut outcomes = Vec::new();
            let mut c_probs = Vec::with_capacity(by_day.len());
            let mut c_outs = Vec::with_capacity(by_day.len());
            for day_rows in by_day.values() {
                let mut ema_delay = 0.0;
                let mut cp = Vec::with_capacity(day_rows.len());
                let mut cy = Vec::with_capacity(day_rows.len());
                for &(sched_dep, delay) in day_rows {
                    let hr = sched_dep / 100.0;
                    let z = -3.1 + 0.08 * (hr - 12.0) + 0.035 * ema_delay;
                    let p = apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001);
                    let y = if delay > 41.0 { 1.0 } else { 0.0 };
                    probs.push(p);
                    outcomes.push(y);
                    cp.push(p);
                    cy.push(y);
                    ema_delay = 0.85 * ema_delay + 0.15 * delay;
                }
                c_probs.push(cp);
                c_outs.push(cy);
            }
            (clusters, jensen, probs, outcomes, c_probs, c_outs)
        }
        other => {
            return Err(format!(
                "Unsupported primary card dataset path '{other}' (expected atlantic_storms, bank_calls, or nyc_flights_2013)"
            ));
        }
    };

    let deff = ExceedanceDesignEffect::from_clustered_observations(&clusters_raw, 0.90);
    let n_eff_rounded = deff.n_effective_p.round() as usize;
    let event_prob_p = mean(&outcomes);
    let corp_pav = CorpPavDecomposition::compute(&probs, &outcomes);
    let confusion = compute_confusion_tpr_fpr(&probs, &outcomes, p_star);

    let alpha_prior = (20.0 * p_star.clamp(0.01, 0.99)).max(0.5);
    let beta_prior = (20.0 * (1.0 - p_star.clamp(0.01, 0.99))).max(0.5);
    let sbc_ranks = simulate_sbc_beta_binomial_ranks(
        DEFAULT_SEED,
        200,
        49,
        alpha_prior,
        beta_prior,
        n_eff_rounded.clamp(20, 120),
    );

    let (t_obs, null_samples) = compute_unclamped_cluster_permutation_null(
        &cluster_probs,
        &cluster_outcomes,
        DEFAULT_SEED,
        200,
    );
    let ville_e_values = compute_ville_betting_e_values(&probs, &outcomes, 30);

    Ok(EmpiricalDatasetAudit {
        dataset_path: rel_csv.to_string(),
        dataset_sha256: actual_sha256,
        n_nominal: deff.n_nominal,
        num_clusters: deff.num_clusters_c,
        m_bar: deff.m_bar,
        m_tilde: deff.m_tilde_moulton,
        rho_exceedance_90: deff.rho_exceedance_p,
        deff_exceedance_90: deff.deff_exceedance,
        n_eff_exceedance_90: n_eff_rounded,
        jensen_summary: jensen,
        event_probability_p: event_prob_p,
        corp_pav,
        confusion_tpr_fpr_at_p_star: confusion,
        sbc_ranks,
        observed_test_statistic: t_obs,
        null_distribution_samples: null_samples,
        ville_e_values,
    })
}

/// Audits an [`EpistemologicalCard`] against both structural invariants (`card.audit()`) and the
/// empirical ground truth recomputed directly from its declared `data/*.csv` file, then runs the
/// un-clamped 8-Step SIGA Roadmap.
pub fn verify_card_against_empirical_data(
    root_dir: &Path,
    card: &EpistemologicalCard,
) -> Result<(EmpiricalDatasetAudit, SigaExecutionReport), String> {
    if let Err(violations) = card.audit() {
        return Err(format!(
            "Card '{}' failed structural audit: {violations:?}",
            card.id
        ));
    }
    let emp = compute_empirical_dataset_audit(root_dir, card)?;

    if card.data_and_audit_contract.dataset_sha256.trim() != emp.dataset_sha256 {
        return Err(format!(
            "Card '{}' declared dataset_sha256 '{}' != actual CSV SHA-256 '{}'",
            card.id, card.data_and_audit_contract.dataset_sha256, emp.dataset_sha256
        ));
    }

    let um = &card.epistemological_header.unit_and_margin;
    if um.nominal_sample_size_n != emp.n_nominal {
        return Err(format!(
            "Card '{}' declared nominal_sample_size_n={} != CSV n={}",
            card.id, um.nominal_sample_size_n, emp.n_nominal
        ));
    }
    if (um.effective_sample_size_n_eff as isize - emp.n_eff_exceedance_90 as isize).abs() > 1 {
        return Err(format!(
            "Card '{}' declared effective_sample_size_n_eff={} != CSV n_eff={}",
            card.id, um.effective_sample_size_n_eff, emp.n_eff_exceedance_90
        ));
    }

    let dr = &card.decision_rule_and_scope;
    if (dr.event_probability_p - emp.event_probability_p).abs() > 5e-4 {
        return Err(format!(
            "Card '{}' declared event_probability_p={:.5} != CSV empirical p={:.5}",
            card.id, dr.event_probability_p, emp.event_probability_p
        ));
    }

    let mc = &card.data_and_audit_contract.murphy_corp_decomposition;
    if (mc.total_score_brier_or_crps - emp.corp_pav.brier_total).abs() > 5e-4
        || (mc.miscalibration_rel_mcb - emp.corp_pav.rel_miscalibration).abs() > 5e-4
        || (mc.resolution_res_dsc - emp.corp_pav.res_resolution).abs() > 5e-4
        || (mc.base_uncertainty_unc0 - emp.corp_pav.unc0_uncertainty).abs() > 5e-4
    {
        return Err(format!(
            "Card '{}' declared CORP-PAV ({:.5}, {:.5}, {:.5}, {:.5}) diverges from CSV CORP-PAV ({:.5}, {:.5}, {:.5}, {:.5})",
            card.id,
            mc.total_score_brier_or_crps,
            mc.miscalibration_rel_mcb,
            mc.resolution_res_dsc,
            mc.base_uncertainty_unc0,
            emp.corp_pav.brier_total,
            emp.corp_pav.rel_miscalibration,
            emp.corp_pav.res_resolution,
            emp.corp_pav.unc0_uncertainty
        ));
    }

    let expected_dj_str = format!("{:.2}", emp.jensen_summary.jensen_gap);
    if !card
        .epistemological_header
        .nonlinearity
        .contains(&expected_dj_str)
    {
        return Err(format!(
            "Card '{}' nonlinearity '{}' does not match CSV Jensen gap Delta_J={expected_dj_str}",
            card.id, card.epistemological_header.nonlinearity
        ));
    }

    let plan_payload = card.compute_preregistration_payload();
    let siga_rep = run_full_siga_roadmap_empirical(
        card,
        &plan_payload,
        &emp.sbc_ranks,
        emp.observed_test_statistic,
        &emp.null_distribution_samples,
        &emp.ville_e_values,
        emp.confusion_tpr_fpr_at_p_star,
    );
    if !siga_rep.approved_end_to_end {
        return Err(format!(
            "Card '{}' failed 8-Step SIGA Roadmap on empirical CSV: steps={:?}, p_null={:.4}, E_t={:.2}",
            card.id,
            siga_rep.steps_1_to_8_completed,
            siga_rep.null_p_value,
            siga_rep.e_process.final_e_value
        ));
    }

    Ok((emp, siga_rep))
}

/// Canonical severe-test entry point declared in `EpistemologicalCard::data_and_audit_contract.null_model`
/// (`crates/stochology-cli#evaluate_empirical_card_severe_test`).
///
/// Recomputes the un-clamped cluster-permutation null distribution (`seed = 2026`), verifies all
/// empirical quantities against the declared `data/*.csv` file, and executes the 8-Step SIGA Roadmap.
pub fn evaluate_empirical_card_severe_test(
    root_dir: &Path,
    card: &EpistemologicalCard,
) -> Result<(EmpiricalDatasetAudit, SigaExecutionReport), String> {
    verify_card_against_empirical_data(root_dir, card)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_loaders_read_real_datasets_in_data() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .expect("workspace root");
        let losses = load_danish_fire(root);
        assert!(losses.len() > 2_000);
        let bank = load_bank_call_clusters(root);
        assert!(bank.len() > 150);
        let (nsw_t, nsw_c, cps_c) = load_lalonde_nsw_cps(root);
        assert!(nsw_t.len() + nsw_c.len() > 400 && cps_c.len() > 15_000);
        let vic = load_vic_elec_demand(root);
        assert_eq!(vic.len(), 52_608);
        let (x13_h1, y13_h1, x23_h1, y23_h1, x_hou, y_hou) = load_oot_ood_flight_reserves(root);
        assert!(x13_h1.len() > 1_400 && x13_h1.len() == y13_h1.len());
        assert!(x23_h1.len() > 1_400 && x23_h1.len() == y23_h1.len());
        assert!(x_hou.len() > 1_400 && x_hou.len() == y_hou.len());
    }
}
