//! Validation by intervention in genetics, with and without the **second line**.
//!
//! The research record says, literally: "The replicate shows *one gene in one mutant*, not an effect measured with
//! resolution. Each mutant is a single-line environment, and the smallest possible p-value there is 1/1 000." This module
//! measures what the second line changes — and it measures in design, not in rhetoric: the two numbers that matter are
//! **how many mutants pass** and **what the floor of the design is** in each arrangement.
//!
//! # The design
//!
//! \`G\` mutants, \`L\` lines per mutant and \`n\` individuals per line. The phenotype is
//! \`base + mutant effect + line effect + noise\`. Two mutants are planted on purpose:
//!
//! - **the true mover**: the effect appears in both of its lines;
//! - **the line artifact**: the effect appears in **one** line only — it is the case that the single-line validation
//!   cannot distinguish from a mover, and that the second line catches.
//!
//! The validation, in each arrangement:
//!
//! - **without the second line**: uses the first line of each mutant and accepts whoever stays below `alpha/G`
//!   (Bonferroni over the mutants). The effect is **a point**, and the p comes from the tail of the mutant mean in
//!   the line distribution, **held at the floor of the design** of `1/(B+1)` — the floor is what the `B` draws
//!   resolve, and it is what decides when the effect is large enough to touch the ground.
//! - **with the second line**: it demands the effect in **both** lines, and the combined p is the product — the floor drops to
//!   \`(1/(B+1))^2\` and every effect gains an **interval** (the range between the two line estimates).
//!
//! Declared and not implemented: the reading of the real yeast data (the base left with the Python scaffolding) and
//! the design with more than two lines, which the instrument accepts as a parameter but is not the declared case.

use stochology_core::Rng;

/// The arrangement of the experiment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Design {
    /// How many deleted genes enter the sweep.
    pub mutants: usize,
    /// How many independent lines per mutant (1 is today's design).
    pub lines_per_mutant: usize,
    /// How many individuals per line.
    pub individuals_per_line: usize,
    /// How many permutation draws per test.
    pub permutations: usize,
    /// Declared level before Bonferroni.
    pub alpha: f64,
}

impl Default for Design {
    fn default() -> Self {
        Design {
            mutants: 40,
            lines_per_mutant: 1,
            individuals_per_line: 60,
            permutations: 999,
            alpha: 0.05,
        }
    }
}

impl Design {
    /// The floor of the design: the smallest p-value the validation can produce. It falls with the power of the number of
    /// lines, because the lines are independent and the combined p is the product.
    pub fn floor(&self) -> f64 {
        (1.0 / (self.permutations as f64 + 1.0)).powi(self.lines_per_mutant as i32)
    }

    /// The floor of the design when the second line enters: the product of the two line floors.
    pub fn floor_with_second_line(&self) -> f64 {
        (1.0 / (self.permutations as f64 + 1.0)).powi(2)
    }

    /// The level of each test after Bonferroni over the mutants.
    pub fn corrected_alpha(&self) -> f64 {
        self.alpha / self.mutants as f64
    }
}

/// What the validation returns for a mutant.
#[derive(Debug, Clone, PartialEq)]
pub struct Verdict {
    pub mutant: usize,
    /// The estimate of the effect in each line.
    pub effect_per_line: Vec<f64>,
    /// The combined p-value of the tested lines.
    pub p: f64,
    /// The range between the largest and the smallest line estimate; zero when there is a single line.
    pub interval_width: f64,
    pub passes: bool,
}

/// The result of the two arrangements, side by side.
#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub without_second_line: Vec<Verdict>,
    pub with_second_line: Vec<Verdict>,
    pub floor_without: f64,
    pub floor_with: f64,
    pub corrected_alpha: f64,
    /// The planted mutant whose effect reproduces in both lines.
    pub true_mover: usize,
    /// The planted mutant whose effect appears in a single line.
    pub line_artifact: usize,
}

impl Comparison {
    pub fn how_many_pass_without(&self) -> usize {
        self.without_second_line.iter().filter(|v| v.passes).count()
    }
    pub fn how_many_pass_with(&self) -> usize {
        self.with_second_line.iter().filter(|v| v.passes).count()
    }
}

/// A synthetic world: the mutants, the lines and the individuals, with the two planted cases.
struct World {
    design: Design,
    /// phenotype[mutant][line][individual]
    phenotype: Vec<Vec<Vec<f64>>>,
    /// the mutant whose effect reproduces in both lines
    true_mover: usize,
    /// the mutant whose effect appears in a single line
    line_artifact: usize,
}

fn generate(design: Design, seed: u64) -> World {
    let mut rng = Rng::new(seed);
    let normal = |rng: &mut Rng| {
        let u1 = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
        let u2 = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    };
    let true_mover = 7 % design.mutants;
    let line_artifact = 23 % design.mutants;
    let mut phenotype = Vec::with_capacity(design.mutants);
    for m in 0..design.mutants {
        let mutant_effect = if m == true_mover { 0.9 } else { 0.0 };
        let mut lines = Vec::with_capacity(design.lines_per_mutant);
        for l in 0..design.lines_per_mutant {
            let line_effect = if m == line_artifact && l == 0 {
                0.9
            } else {
                0.0
            };
            let shift = 0.15 * normal(&mut rng);
            let mut individuals = Vec::with_capacity(design.individuals_per_line);
            for _ in 0..design.individuals_per_line {
                individuals.push(5.0 + mutant_effect + line_effect + shift + normal(&mut rng));
            }
            lines.push(individuals);
        }
        phenotype.push(lines);
    }
    World {
        design,
        phenotype,
        true_mover,
        line_artifact,
    }
}

/// The effect of each mutant in a line and the p of the line: the mutant mean against the line distribution
/// (normal tail of the mean, with the pooled deviation), **held at the floor of the design** — which is what the 999 draws
/// resolve. It is that floor that the second line changes.
fn line_effects(world: &World, line: usize, _seed: u64) -> Vec<(f64, f64)> {
    let n = world.design.individuals_per_line as f64;
    let means: Vec<f64> = (0..world.design.mutants)
        .map(|m| {
            let values = &world.phenotype[m][line];
            values.iter().sum::<f64>() / n
        })
        .collect();
    let reference = means.iter().sum::<f64>() / means.len() as f64;
    // pooled deviation inside the line, across the individuals of all mutants
    let mut sum = 0.0;
    let mut degrees: f64 = 0.0;
    for m in 0..world.design.mutants {
        let shift = means[m] - reference;
        for v in &world.phenotype[m][line] {
            sum += (v - reference - shift).powi(2);
            degrees += 1.0;
        }
    }
    let sd = (sum / degrees.max(1.0)).sqrt().max(1e-9);
    let floor = 1.0 / (world.design.permutations as f64 + 1.0);
    means
        .iter()
        .map(|mean| {
            let effect = mean - reference;
            let z = effect.abs() * n.sqrt() / sd;
            let p = 2.0 * (0.5 * (1.0 - erf(z / std::f64::consts::SQRT_2))).max(0.0);
            (effect, p.max(floor))
        })
        .collect()
}

/// The error function, by Abramowitz-Stegun 7.1.26 (error < 1.5e-7).
fn erf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let polynomial = t
        * (0.254_829_592
            + t * (-0.284_496_736
                + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    sign * (1.0 - polynomial * (-x * x).exp())
}

/// Runs both arrangements over the same synthetic world and returns what each one decides.
pub fn compare(design: Design, seed: u64) -> Comparison {
    let world = generate(design, seed);
    let corrected_alpha = design.corrected_alpha();
    let first = line_effects(&world, 0, seed ^ 11);
    let second = line_effects(&world, 1.min(design.lines_per_mutant - 1), seed ^ 977);
    let mut without = Vec::with_capacity(design.mutants);
    let mut with = Vec::with_capacity(design.mutants);
    for m in 0..design.mutants {
        let (effect, p) = first[m];
        without.push(Verdict {
            mutant: m,
            effect_per_line: vec![effect],
            p,
            interval_width: 0.0,
            passes: p < corrected_alpha,
        });
        let (effect_2, p_2) = second[m];
        // with the second line the effect must be in BOTH: the combined p is the product (and falls to the new floor),
        // and approval is conjunctive — that is what separates the mover from the line artifact
        let combined_p = (p * p_2).max(design.floor_with_second_line());
        with.push(Verdict {
            mutant: m,
            effect_per_line: vec![effect, effect_2],
            p: combined_p,
            interval_width: (effect - effect_2).abs(),
            passes: p < corrected_alpha && p_2 < corrected_alpha,
        });
    }
    Comparison {
        without_second_line: without,
        with_second_line: with,
        floor_without: Design {
            lines_per_mutant: 1,
            ..design
        }
        .floor(),
        floor_with: design.floor_with_second_line(),
        corrected_alpha,
        true_mover: world.true_mover,
        line_artifact: world.line_artifact,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_falls_with_the_power_of_the_lines() {
        let d = Design::default();
        let without = Design {
            lines_per_mutant: 1,
            ..d
        }
        .floor();
        let with = Design {
            lines_per_mutant: 2,
            ..d
        }
        .floor();
        assert!(
            (without - 1.0 / 1000.0).abs() < 1e-12,
            "floor without: {without}"
        );
        assert!(
            (with - 1.0 / 1_000_000.0).abs() < 1e-15,
            "floor with: {with}"
        );
        assert!(with < without);
    }

    #[test]
    fn without_the_second_line_the_interval_is_a_point() {
        let c = compare(Design::default(), 2019);
        assert!(
            c.without_second_line
                .iter()
                .all(|v| v.effect_per_line.len() == 1)
        );
        assert!(
            c.without_second_line
                .iter()
                .all(|v| v.interval_width == 0.0)
        );
    }

    #[test]
    fn with_the_second_line_every_effect_has_an_interval() {
        let c = compare(
            Design {
                lines_per_mutant: 2,
                ..Design::default()
            },
            2019,
        );
        assert!(
            c.with_second_line
                .iter()
                .all(|v| v.effect_per_line.len() == 2)
        );
        let with_interval = c
            .with_second_line
            .iter()
            .filter(|v| v.interval_width > 0.0)
            .count();
        assert!(
            with_interval > c.with_second_line.len() / 2,
            "almost all with a range: {with_interval}"
        );
    }

    #[test]
    fn the_line_artifact_passes_without_the_second_line() {
        let c = compare(Design::default(), 2019);
        let artifact = c.line_artifact;
        let v = &c.without_second_line[artifact];
        assert!(
            v.passes,
            "the line artifact passes the single-line validation: {v:?}"
        );
    }

    #[test]
    fn the_second_line_separates_the_artifact_from_the_mover() {
        let c = compare(
            Design {
                lines_per_mutant: 2,
                ..Design::default()
            },
            2019,
        );
        let artifact = c.line_artifact;
        let mover = c.true_mover;
        let a = &c.with_second_line[artifact];
        assert!(
            a.interval_width > 0.3,
            "the artifact has a wide range between lines: {a:?}"
        );
        assert!(!a.passes, "and therefore it does not pass: {a:?}");
        let m = &c.with_second_line[mover];
        assert!(m.passes, "the true mover passes: {m:?}");
    }

    #[test]
    fn the_design_is_deterministic() {
        let d = Design::default();
        assert_eq!(compare(d, 2019), compare(d, 2019));
        assert_ne!(compare(d, 2019), compare(d, 2020));
    }
}
