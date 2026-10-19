//! The spread of the seed over whole laboratories: what moves, and whether any verdict moves with it.
//!
//! `labs --mc-error` audits, number by number, the simulated quantities that a reading of the book was known to depend
//! on. This instrument audits the rest by brute force. It reruns an instrument under twenty seed offsets
//! (`STOCHOLOGY_SEED_OFFSET` = 1 to 20, which every generator adds to its declared seed) and compares each run with
//! the published one, line by line. A line has numbers and words. The numbers are expected to move: that is the Monte
//! Carlo error. The words are not: when the words of a line change under another seed ("passes" becomes "fails", "true"
//! becomes "false", one decision becomes the other), a verdict printed by the laboratory depends on the draw, and the
//! book cannot read it as a property of the world.
//!
//! The instrument spawns the `labs` binary itself, so it is slow (several minutes on two cores), and it is not in the
//! verification suite; its output is recorded in `docs/SEED_SPREAD.md` with the commit it was run on.

use std::process::Command;

pub const OFFSETS: u64 = 20;

/// The instruments whose generators the register of seeds listed as not audited.
pub fn targets() -> Vec<(String, Vec<String>)> {
    let mut v: Vec<(String, Vec<String>)> = [4, 5, 6, 9, 11, 12, 15, 17, 19, 20]
        .iter()
        .map(|n| {
            (
                format!("--chapter {n}"),
                vec!["--chapter".to_string(), n.to_string()],
            )
        })
        .collect();
    for flag in ["--bank", "--queue", "--rehearsal-shelf"] {
        v.push((flag.to_string(), vec![flag.to_string()]));
    }
    v
}

/// The numbers of a line, and the line with every number replaced by '#' and its spaces collapsed.
pub fn split_line(line: &str) -> (Vec<String>, String) {
    let mut numbers = Vec::new();
    let mut words = String::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let starts_number = chars[i].is_ascii_digit()
            || ((chars[i] == '-' || chars[i] == '+' || chars[i] == '.')
                && i + 1 < chars.len()
                && chars[i + 1].is_ascii_digit()
                && (i == 0 || !chars[i - 1].is_ascii_alphanumeric()));
        if starts_number {
            let start = i;
            i += 1;
            while i < chars.len()
                && (chars[i].is_ascii_digit()
                    || chars[i] == '.'
                    || ((chars[i] == 'e' || chars[i] == 'E')
                        && i + 1 < chars.len()
                        && (chars[i + 1].is_ascii_digit() || chars[i + 1] == '-')))
            {
                if (chars[i] == 'e' || chars[i] == 'E') && chars[i + 1] == '-' {
                    i += 1;
                }
                i += 1;
            }
            numbers.push(chars[start..i].iter().collect());
            words.push('#');
        } else {
            words.push(chars[i]);
            i += 1;
        }
    }
    // column padding moves with the width of the numbers; it is layout, not a verdict
    let words = words.split_whitespace().collect::<Vec<_>>().join(" ");
    (numbers, words)
}

fn run_with(args: &[String], offset: u64) -> Vec<String> {
    let exe = std::env::current_exe().expect("the labs binary");
    let out = Command::new(exe)
        .args(args)
        .env("STOCHOLOGY_SEED_OFFSET", offset.to_string())
        .output()
        .expect("the instrument runs");
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_string)
        .collect()
}

/// The comparison of one instrument: lines, numbers, numbers that moved, lines whose words moved and in how many seeds,
/// and examples of the lines whose words moved: (published, one other).
#[derive(Debug, Default)]
pub struct Spread {
    pub lines: usize,
    pub numbers: usize,
    pub moved: usize,
    pub structure_changed: usize,
    pub word_lines: Vec<(usize, usize, String, String)>,
}

pub fn compare(published: &[String], others: &[Vec<String>]) -> Spread {
    let mut s = Spread {
        lines: published.len(),
        ..Spread::default()
    };
    let parts: Vec<(Vec<String>, String)> = published.iter().map(|l| split_line(l)).collect();
    s.numbers = parts.iter().map(|p| p.0.len()).sum();
    let mut moved = vec![vec![false; 0]; parts.len()];
    for (i, p) in parts.iter().enumerate() {
        moved[i] = vec![false; p.0.len()];
    }
    let mut word_seeds = vec![0usize; parts.len()];
    let mut example: Vec<Option<String>> = vec![None; parts.len()];
    for other in others {
        if other.len() != published.len() {
            s.structure_changed += 1;
            continue;
        }
        for (i, line) in other.iter().enumerate() {
            let (nums, words) = split_line(line);
            if words != parts[i].1 {
                word_seeds[i] += 1;
                if example[i].is_none() {
                    example[i] = Some(line.clone());
                }
                continue;
            }
            for (j, n) in nums.iter().enumerate() {
                if *n != parts[i].0[j] {
                    moved[i][j] = true;
                }
            }
        }
    }
    s.moved = moved.iter().map(|m| m.iter().filter(|x| **x).count()).sum();
    for (i, count) in word_seeds.iter().enumerate() {
        if *count > 0 {
            s.word_lines.push((
                i + 1,
                *count,
                published[i].clone(),
                example[i].clone().unwrap_or_default(),
            ));
        }
    }
    s
}

fn clip(line: &str) -> String {
    let t = line.trim();
    if t.chars().count() > 150 {
        format!("{}...", t.chars().take(150).collect::<String>())
    } else {
        t.to_string()
    }
}

pub fn run() {
    println!(
        "== The spread of the seed over whole laboratories: what moves, and whether a verdict moves with it =="
    );
    println!(
        "each instrument rerun under seed offsets 1 to {OFFSETS} and compared line by line with the published run (offset 0)"
    );
    let mut total_lines = 0;
    let mut total_words = 0;
    for (label, args) in targets() {
        let published = run_with(&args, 0);
        let others: Vec<Vec<String>> = (1..=OFFSETS).map(|k| run_with(&args, k)).collect();
        let s = compare(&published, &others);
        total_lines += s.lines;
        total_words += s.word_lines.len();
        println!(
            "\n-- labs {label} --\n  {} lines | {} numbers, {} of them move under another seed | lines whose words move: {} | runs with another number of lines: {}",
            s.lines,
            s.numbers,
            s.moved,
            s.word_lines.len(),
            s.structure_changed
        );
        for (line, seeds, published, other) in &s.word_lines {
            println!("  line {line}, words differ in {seeds} of {OFFSETS} seeds");
            println!("    published: {}", clip(published));
            println!("    another:   {}", clip(other));
        }
    }
    println!(
        "\n{total_lines} lines in {} instruments; lines whose words move under another seed: {total_words}",
        targets().len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_splits_into_its_numbers_and_its_words() {
        let (n, w) = split_line("ECE 0.0121 (null q95 -0.0046), gate PASSES, 1.2e-5 at x = 8");
        // the digits of a label (q95) are numbers too; they never move, so they cost nothing
        assert_eq!(n, vec!["0.0121", "95", "-0.0046", "1.2e-5", "8"]);
        assert_eq!(w, "ECE # (null q# #), gate PASSES, # at x = #");
        assert_eq!(
            split_line("  7 or 9     9%").1,
            split_line("  7 or 9    10%").1
        );
        let (n, w) = split_line("R2-hora and A5-governance");
        assert_eq!(n, vec!["2", "5"]);
        assert_eq!(w, "R#-hora and A#-governance");
    }

    #[test]
    fn a_verdict_that_moves_is_found_and_a_number_that_moves_is_counted() {
        let published = vec!["gate PASSES at 0.031".to_string(), "mean 1.00".to_string()];
        let others = vec![
            vec!["gate FAILS at 0.029".to_string(), "mean 1.00".to_string()],
            vec!["gate PASSES at 0.030".to_string(), "mean 1.01".to_string()],
        ];
        let s = compare(&published, &others);
        assert_eq!(s.numbers, 2);
        assert_eq!(s.moved, 2);
        assert_eq!(s.word_lines.len(), 1);
        assert_eq!(s.word_lines[0].1, 1);
    }
}
