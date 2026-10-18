//! Chapter 21 — the verifier of the final project card.
//!
//! Self-contained laboratory, no external dependencies: a verifier that rejects a card without the
//! decider's question, without a seed and without an exit clause, and that shows every rule rejecting at
//! least one card (a gate must know how to reject).
//! Run from the repository root:
//!     cargo run --release --bin labs -- --chapter 21

/// The seven rules, in the order in which they are applied.
const RULES: [&str; 7] = [
    "R1 generative process declared",
    "R2 independent unit declared",
    "R3 null model with the same seed",
    "R4 decider question (who decides, which action, question)",
    "R5 fixed numeric seed",
    "R6 pre-registration hash with 64 hexadecimal digits",
    "R7 exit clause with an objective trigger",
];

/// Generic names that the decider rule rejects.
const GENERIC: [&str; 9] = [
    "the decider",
    "the decision makers",
    "the user",
    "the client",
    "the model",
    "the data",
    "the decision",
    "who decides",
    "nobody",
];

struct Card {
    name: &'static str,
    generator: &'static str,
    unit: &'static str,
    null_model: &'static str,
    decider: &'static str,
    action: &'static str,
    question: &'static str,
    seed: &'static str,
    hash: &'static str,
    exit_clause: &'static str,
}

fn hex64(s: &str) -> bool {
    s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// The verifier. Returns the list of problems; empty means the card is accepted.
fn verify(f: &Card) -> Vec<String> {
    let mut p = Vec::new();

    // R1
    if f.generator.trim().len() < 10 {
        p.push("R1 generator field empty or too short".to_string());
    }
    // R2
    if f.unit.trim().len() < 5 {
        p.push("R2 unit field empty".to_string());
    }
    // R3
    if f.null_model.trim().len() < 5 || !f.null_model.contains(f.seed) && !f.seed.is_empty() {
        p.push("R3 null model missing or without the same seed".to_string());
    }
    // R4 — the decider question, in three parts
    let t = f.question;
    if t.trim().is_empty() {
        p.push("R4 field decider_question missing".to_string());
    } else {
        if t.chars().count() < 60 {
            p.push("R4 question too short (fewer than 60 characters)".to_string());
        }
        match t.find("Who decides:") {
            None => p.push("R4 the text does not name who decides".to_string()),
            Some(i) => {
                let rest = &t[i + "Who decides:".len()..];
                let stop = rest
                    .find("Action the decision changes:")
                    .unwrap_or(rest.len())
                    .min(120);
                let name = rest[..stop].trim();
                if name.chars().count() < 3 {
                    p.push("R4 the text does not name who decides".to_string());
                } else if GENERIC.iter().any(|g| name.to_lowercase().starts_with(g)) {
                    p.push("R4 who decides is generic".to_string());
                }
            }
        }
        let label = format!("Action the decision changes: {}", f.action);
        if !t.contains(&label) {
            p.push("R4 the action of the decision does not appear literally".to_string());
        }
        if !t.trim_end().ends_with('?') {
            p.push("R4 the text does not end in a question".to_string());
        }
    }
    // R5
    if f.seed.parse::<u64>().is_err() {
        p.push("R5 no fixed numeric seed".to_string());
    }
    // R6
    if !hex64(f.hash) {
        p.push("R6 pre-registration hash missing or out of format".to_string());
    }
    // R7
    if f.exit_clause.trim().is_empty() {
        p.push("R7 no exit clause".to_string());
    } else if !f.exit_clause.chars().any(|c| c.is_ascii_digit()) {
        p.push("R7 exit clause without an objective trigger".to_string());
    }
    p
}

pub fn run() {
    println!("== Chapter 21 :: the verifier of the final project card ==");
    println!("seven rules, and each one has to reject at least one card:\n");
    for (i, r) in RULES.iter().enumerate() {
        println!("  {}. {}", i + 1, r);
    }

    let base = Card {
        name: "01 complete",
        generator: "school lottery, fixed seed 2026, gaussian noise",
        unit: "the school, not the student",
        null_model: "the same world with seed 2026 and without the effect",
        decider: "the municipal education secretariat",
        action: "cut the class sizes of the early grades to 15 students",
        question: "Who decides: the municipal education secretariat. Action the decision changes: cut the class sizes of the early grades to 15 students. The decider question is: does cutting the class to 15 students improve the reading score one year later?",
        seed: "2026",
        hash: "0f1e2d3c4b5a69788796a5b4c3d2e1f00f1e2d3c4b5a69788796a5b4c3d2e1f0",
        exit_clause: "valid while the measured effect stays above 0.10 sd for three consecutive years",
    };
    let cases: [Card; 12] = [
        Card { ..base },
        Card {
            name: "02 without the decider question",
            question: "",
            ..base
        },
        Card {
            name: "03 generic who decides",
            question: "Who decides: the decider. Action the decision changes: cut the class sizes of the early grades to 15 students. The decider question is: does cutting them improve reading?",
            ..base
        },
        Card {
            name: "04 the action does not appear literally",
            question: "Who decides: the municipal education secretariat. Action the decision changes: hire more teachers. The decider question is: does that improve reading one year later?",
            ..base
        },
        Card {
            name: "05 without a question mark",
            question: "Who decides: the municipal education secretariat. Action the decision changes: cut the class sizes of the early grades to 15 students. The decider question is: this improves reading one year later.",
            ..base
        },
        Card {
            name: "06 without a seed",
            seed: "",
            ..base
        },
        Card {
            name: "07 without an exit clause",
            exit_clause: "",
            ..base
        },
        Card {
            name: "08 clause without a trigger",
            exit_clause: "valid while the model keeps being good and nobody complains",
            ..base
        },
        Card {
            name: "09 without pre-registration",
            hash: "",
            ..base
        },
        Card {
            name: "10 without a null model",
            null_model: "",
            ..base
        },
        Card {
            name: "11 without a generator",
            generator: "no idea",
            ..base
        },
        Card {
            name: "12 without a unit",
            unit: "?",
            ..base
        },
    ];

    let mut accepted = 0usize;
    let mut rejections = vec![0usize; 7];
    for f in &cases {
        let problems = verify(f);
        if problems.is_empty() {
            accepted += 1;
            println!("\n{:<28} ACCEPTED (who decides: {})", f.name, f.decider);
        } else {
            println!("\n{:<28} REJECTED", f.name);
            for p in &problems {
                println!("    {}", p);
                if let Some(n) = p.get(1..2).and_then(|d| d.parse::<usize>().ok()) {
                    rejections[n - 1] += 1;
                }
            }
        }
    }
    println!(
        "\n{} cards: {} accepted, {} rejected",
        cases.len(),
        accepted,
        cases.len() - accepted
    );
    let covered = rejections.iter().filter(|n| **n > 0).count();
    println!(
        "rules that rejected at least one card: {} of {} | rejections per rule: {:?}",
        covered,
        RULES.len(),
        rejections
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifier_accepts_a_complete_card_and_rejects_a_generic_decider() {
        let mut f = Card {
            name: "01 complete",
            generator: "school lottery, fixed seed 2026, gaussian noise",
            unit: "the school, not the student",
            null_model: "the same world with seed 2026 and without the effect",
            decider: "the municipal education secretariat",
            action: "cut the class sizes of the early grades to 15 students",
            question: "Who decides: the municipal education secretariat. Action the decision changes: cut the class sizes of the early grades to 15 students. The decider question is: does cutting the class to 15 students improve the reading score one year later?",
            seed: "2026",
            hash: "0f1e2d3c4b5a69788796a5b4c3d2e1f00f1e2d3c4b5a69788796a5b4c3d2e1f0",
            exit_clause: "valid while the measured effect stays above 0.10 sd for three consecutive years",
        };
        assert!(verify(&f).is_empty());
        f.question = "Who decides: the decider. Action the decision changes: cut the class sizes of the early grades to 15 students. The decider question is: does cutting them improve reading?";
        assert!(!verify(&f).is_empty());
    }
}
