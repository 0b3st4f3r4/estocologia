//! Chapter 19 — the fine resolution of yeast genetics: what the **second line** changes.
//!
//! The registry says, in plain words, that the replicate found *a gene in a mutant*, not an effect measured at
//! resolution: each mutant is a single-line environment and the smallest possible p-value there is 1/1 000.
//! This laboratory measures what the second line changes in the design of validation by intervention, with
//! two cases planted by the instrument — the **true mover** (the effect shows up in both of its lines)
//! and the **line artifact** (the effect shows up in a single line). The question is which of the two
//! validation can tell apart.
//!
//! Run: ~labs --genetics~

use stochology_horizons::h2::genetics::{Comparison, Design, Verdict, compare};

/// The seed of the chapter, the same as the ICP laboratory.
const SEED: u64 = 2019;

fn marker(comparison: &Comparison, m: usize) -> &'static str {
    if m == comparison.true_mover {
        " <- true mover (effect on both lines)"
    } else if m == comparison.line_artifact {
        " <- line artifact (effect on a single line)"
    } else {
        ""
    }
}

fn single_line(c: &Comparison, design: &Design) -> Vec<String> {
    c.without_second_line
        .iter()
        .enumerate()
        .filter(|(_, v)| v.passes)
        .map(|(m, v)| {
            format!(
                "  mutant {:>2} | effect {:>+7.3} | p {:>9.6} | {}{}",
                m,
                v.effect_per_line[0],
                v.p,
                if v.p <= (1.0 / (design.permutations as f64 + 1.0)) + 1e-12 {
                    "passes at the floor"
                } else {
                    "passes"
                },
                marker(c, m)
            )
        })
        .collect()
}

fn two_lines(c: &Comparison, which: &[usize], alpha: f64) -> Vec<String> {
    which
        .iter()
        .map(|&m| {
            let v: &Verdict = &c.with_second_line[m];
            format!(
                "  mutant {:>2} | effect l1 {:>+7.3} | effect l2 {:>+7.3} | span {:>6.3} | p {:>9.6} | {}{}",
                m,
                v.effect_per_line[0],
                v.effect_per_line[1],
                v.interval_width,
                v.p,
                if v.passes { "passes" } else { "rejected" },
                if v.passes && v.p >= alpha {
                    " (but at the floor)"
                } else {
                    ""
                }
            ) + marker(c, v.mutant)
        })
        .collect()
}

pub fn run() {
    println!("== Chapter 19 :: the fine resolution of validation by intervention ==");
    let one = Design::default();
    let two = Design {
        lines_per_mutant: 2,
        ..Design::default()
    };
    let c_one = compare(one, SEED);
    let c_two = compare(two, SEED);
    println!(
        "design: {} mutants | {} individuals per line | {} draws per test | alpha = {:.2} | Bonferroni alpha/{} = {:.6}",
        one.mutants,
        one.individuals_per_line,
        one.permutations,
        one.alpha,
        one.mutants,
        c_one.corrected_alpha
    );
    println!(
        "planted by the instrument: true mover = mutant {} | line artifact = mutant {}",
        c_one.true_mover, c_one.line_artifact
    );

    println!("\n-- today's validation: every mutant is a single-line environment --");
    println!(
        "  {} mutants x {} individuals | design floor = {:.6} | threshold = {:.6}",
        one.mutants, one.individuals_per_line, c_one.floor_without, c_one.corrected_alpha
    );
    for line in single_line(&c_one, &one) {
        println!("{line}");
    }
    println!(
        "  passing: {} of {}",
        c_one.how_many_pass_without(),
        one.mutants
    );

    println!("\n-- the same world, with the second line: the effect must be on both --");
    println!(
        "  {} mutants x 2 lines x {} individuals | design floor = {:.6} | threshold = {:.6}",
        two.mutants, two.individuals_per_line, c_two.floor_with, c_two.corrected_alpha
    );
    let mut look = vec![c_two.true_mover, c_two.line_artifact];
    for (m, v) in c_two.with_second_line.iter().enumerate() {
        if v.passes && !look.contains(&m) {
            look.push(m);
        }
    }
    look.sort_unstable();
    for line in two_lines(&c_two, &look, c_two.corrected_alpha) {
        println!("{line}");
    }
    println!(
        "  passing: {} of {}",
        c_two.how_many_pass_with(),
        two.mutants
    );
    println!(
        "  effect on the second line: artifact {:>+7.3} (does not reproduce) | mover {:>+7.3} (does reproduce) | in single-line validation the estimate is a point and the span is zero",
        c_two.with_second_line[c_two.line_artifact].effect_per_line[1],
        c_two.with_second_line[c_two.true_mover].effect_per_line[1]
    );

    println!(
        "\nreading: with one line the floor of {:.6} and the threshold of {:.6} let {} of the {} mutants through — the true mover, the line artifact and two mutants that are just noise. All of them sit on the design floor, and the p-value distinguishes none of the three from what we are looking for. With the second line the artifact leaves the list, because its effect on the second line is zero, and the floor drops to {:.9}: {} of the {} mutants pass. The p was never the problem: the problem was there being a single line.",
        c_one.floor_without,
        c_one.corrected_alpha,
        c_one.how_many_pass_without(),
        one.mutants,
        c_two.floor_with,
        c_two.how_many_pass_with(),
        two.mutants
    );
}
