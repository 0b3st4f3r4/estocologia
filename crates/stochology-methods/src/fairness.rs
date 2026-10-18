//! stochology-methods — subgroup fairness: parity, opportunity, and calibration.
//!
//! The contract demands that a decision declare whom it weighs on. Here live the three measures
//! the program uses: demographic parity (positive rate per group), equal opportunity
//! (sensitivity per group), and calibration by region, which reuses the ECE of H3 — including the
//! declared limitation that the ECE tolerance depends on n.
//!
//! Declared and not implemented: the damage weights (what a false negative costs in each group) and
//! the listening to those who pay for the error, which is field work (`governance/GRUPOS.md`).

pub fn ece(confidence: &[f64], correct: &[bool], buckets: usize) -> f64 {
    if confidence.len() != correct.len() || confidence.is_empty() || buckets == 0 {
        return f64::NAN;
    }
    let mut sum = vec![0.0_f64; buckets];
    let mut hits = vec![0.0_f64; buckets];
    let mut count = vec![0usize; buckets];
    for (p, y) in confidence.iter().zip(correct.iter()) {
        let index = ((p.clamp(0.0, 1.0) * buckets as f64).floor() as usize).min(buckets - 1);
        sum[index] += p;
        hits[index] += if *y { 1.0 } else { 0.0 };
        count[index] += 1;
    }
    let n = confidence.len() as f64;
    let mut ece = 0.0;
    for index in 0..buckets {
        if count[index] == 0 {
            continue;
        }
        let mean_confidence = sum[index] / count[index] as f64;
        let mean_hit = hits[index] / count[index] as f64;
        ece += (count[index] as f64 / n) * (mean_hit - mean_confidence).abs();
    }
    ece
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub name: String,
    pub n: usize,
    pub positives: usize,
    pub true_positives: usize,
    pub false_negatives: usize,
}

impl Group {
    pub fn new(
        name: &str,
        n: usize,
        positives: usize,
        true_positives: usize,
        false_negatives: usize,
    ) -> Self {
        Group {
            name: name.to_string(),
            n,
            positives,
            true_positives,
            false_negatives,
        }
    }

    /// Positive rate in the group: who was treated as a case.
    pub fn positive_rate(&self) -> f64 {
        if self.n == 0 {
            return 0.0;
        }
        self.positives as f64 / self.n as f64
    }

    /// Sensitivity: among those who really were cases, how many were served.
    pub fn sensitivity(&self) -> f64 {
        let cases = self.true_positives + self.false_negatives;
        if cases == 0 {
            return 0.0;
        }
        self.true_positives as f64 / cases as f64
    }
}

fn amplitude(values: Vec<f64>) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let largest = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let smallest = values.iter().cloned().fold(f64::INFINITY, f64::min);
    largest - smallest
}

/// Demographic parity: the distance between the largest and the smallest positive rate.
pub fn demographic_parity(groups: &[Group]) -> f64 {
    amplitude(groups.iter().map(Group::positive_rate).collect())
}

/// Equal opportunity: the distance between the largest and the smallest sensitivity.
pub fn equal_opportunity(groups: &[Group]) -> f64 {
    amplitude(groups.iter().map(Group::sensitivity).collect())
}

/// ECE of a declared region of the sample (the subgroup that pays for the error).
pub fn ece_of_group(confidence: &[f64], correct: &[bool], buckets: usize, region: &[bool]) -> f64 {
    if confidence.len() != correct.len() || confidence.len() != region.len() {
        return f64::NAN;
    }
    let mut p = Vec::new();
    let mut y = Vec::new();
    for i in 0..confidence.len() {
        if region[i] {
            p.push(confidence[i]);
            y.push(correct[i]);
        }
    }
    ece(&p, &y, buckets)
}

/// Declared ECE tolerance for n points: it falls with the square root of n. There is no single
/// number: there is this rule, which is what chapters 15 and 20 leave in place of a fixed value.
pub fn ece_tolerance(n: usize) -> f64 {
    if n == 0 {
        return f64::INFINITY;
    }
    1.5 / (n as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::Rng;

    #[test]
    fn equal_groups_have_no_amplitude() {
        let a = Group::new("a", 100, 30, 24, 6);
        let b = Group::new("b", 200, 60, 48, 12);
        assert!(demographic_parity(&[a.clone(), b.clone()]).abs() < 1e-12);
        assert!(equal_opportunity(&[a, b]).abs() < 1e-12);
        assert_eq!(demographic_parity(&[]), 0.0);
    }

    #[test]
    fn parity_measures_the_injected_gap() {
        let a = Group::new("a", 100, 40, 30, 10);
        let b = Group::new("b", 100, 20, 10, 10);
        assert!((demographic_parity(&[a.clone(), b.clone()]) - 0.20).abs() < 1e-12);
        assert!((equal_opportunity(&[a, b]) - 0.25).abs() < 1e-12);
    }

    #[test]
    fn calibration_finds_the_miscalibrated_region() {
        let mut rng = Rng::new(2026);
        let mut confidence = Vec::new();
        let mut correct = Vec::new();
        let mut region = Vec::new();
        for i in 0..20_000 {
            let u = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            let p = u;
            let inside = i % 2 == 0;
            let draw = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            // the inner group hits less than it promises; the outer one is calibrated
            let hit = if inside { draw < p * 0.6 } else { draw < p };
            confidence.push(p);
            correct.push(hit);
            region.push(inside);
        }
        let inner = ece_of_group(&confidence, &correct, 20, &region);
        let outside: Vec<bool> = region.iter().map(|r| !r).collect();
        let outer = ece_of_group(&confidence, &correct, 20, &outside);
        assert!(outer < 0.03, "outside is calibrated: {outer}");
        assert!(inner > 0.08, "inside is miscalibrated: {inner}");
    }

    #[test]
    fn the_ece_tolerance_falls_with_the_square_root_of_n() {
        let base = ece_tolerance(100);
        assert!((ece_tolerance(400) - base / 2.0).abs() < 1e-12);
        assert!((ece_tolerance(1_600) - base / 4.0).abs() < 1e-12);
        assert!(ece_tolerance(0).is_infinite());
    }
}
