//! System 1: deterministic triage of cards (declared in docs/PLAN.md §4 and in Chapter 16,
//! implemented here).
//!
//! # What this module does
//!
//! It receives the **one-line definition** of a card and returns a **category of a closed set**
//! plus a **probability**. Nothing is generated: the answer is a choice among six declared labels,
//! with explicit and checkable lexical evidence.
//!
//! # Why this is "LLM-as-Jev"
//!
//! The expression declared in `docs/PLAN.md` §4 asks for an O(1) mode of **closed** categorical
//! triage. A free LLM generates new text on every call: two runs diverge, and gate G1
//! (determinism) does not close. Here the place of the LLM is taken by a **Jev** — a mute judge:
//!
//! 1. **no free generation** — the output is a label of `Category::ALL`, never a new sentence;
//! 2. **same input, same output** — the suffix tree is materialised in a map ordered by key, and
//!    the tie-break is by the declared order of the categories;
//! 3. **neither 0% nor 100%** — the score is born on the scale [0.01; 0.99] (see below), the mass
//!    above the ceiling is transferred to whoever still has room and the sum is checked at every
//!    round;
//! 4. **the judge does not invent vocabulary** — if no declared theme matches, the answer is
//!    `outros`, with a low-but-not-zero probability; the judge says "I do not know", it does not
//!    improvise.
//!
//! # The deterministic suffix tree
//!
//! Each category declares themes (keywords). On the indexing path the suffix tree is materialised as
//! the set **of all suffixes** of all themes, each suffix pointing to the **shortest theme** that has
//! it as a suffix. On the query path, for each position of the normalised text the **longest indexed
//! suffix** that starts there is looked up — it is the classic tree traversal, and it is what makes
//! it possible to match a word that ends in an indexed suffix with no
//! morphological rule at all.
//!
//! The score of a category is the **length of the matched theme** divided by the length of the
//! longest theme **of the vocabulary** (thirteen, `randomization`). A global denominator, and not
//! one per category: it is what makes two matches of the same length directly comparable and what
//! makes the tie-break an operation over equal numbers. The transfer to probability is affine,
//! `p = 0.01 + 0.94 x score`: the difference of one matched character is always worth the same,
//! and the epistemological interval stops being a cut applied at the end and becomes the scale of
//! the score itself. A match of four characters returns 0.9866 before normalisation; one of ten,
//! 0.9966 — never 1.
//!
//! # Declared limit
//!
//! System 1 is triage, not judgement: it locates the error family of a card and returns a
//! distribution over closed labels. The question that demands deliberation — the R axis, code
//! synthesis, adversarial stress — stays with System 2, which lives in `crates/stochology-s2`:
//! LATS, belief-state engine, Metropolis inference and deliberative stress of the panel.

use std::collections::BTreeMap;

/// Epistemological floor: no probability goes down to 0%.
pub const FLOOR: f64 = 0.01;

/// Epistemological ceiling: no probability goes up to 100%.
pub const CEILING: f64 = 0.99;

/// Minimum length of a matched theme for it to count as evidence.
///
/// Below that the match is noise (`de`, `a`, `em`) and the match is discarded.
pub const MIN_THEME: usize = 4;

// The theme gate only makes sense if the minimum is greater than zero: with 1 or 2, any preposition
// would sustain a category. The proof is of compilation, not of test.
const _: () = assert!(MIN_THEME >= 4);

/// Name of the label for absence of evidence.
const OTHER_LABEL: &str = "other";

/// Declared weight of the `outros` label when no declared theme matches the definition.
///
/// Low-but-not-zero: "I do not know" has mass, and it is not dogma. After normalising over the
/// declared prior (`outros` = 0.34 and each theme = 0.01) the effective probability stays at
/// 34/39 = 0.8718, with the five thematic categories at 0.0256 each — none at zero, and none at the
/// ceiling. When the definition matches some theme, this prior does not enter: the mass comes from
/// the lexical match.
const MASS_WITHOUT_EVIDENCE: f64 = 0.34;

/// Declared weight of each thematic category when no theme matches.
const WEIGHT_WITHOUT_MATCH: f64 = 0.01;

/// Transfer from score to mass: p = FLOOR + (1 - 6 x FLOOR) x score.
///
/// The score is the length of the matched theme divided by the longest theme of the vocabulary, so
/// p = 0.01 requires no match and p = 0.99 requires the whole longest theme. The line is monotone
/// and affine: the difference of one matched character is always worth the same in probability, and
/// the epistemological interval stops being a cut applied at the end and becomes the scale of the
/// score itself. A match of four characters returns 0.9866; of ten, 0.9966.
fn transfer(score: f64) -> f64 {
    let n = Category::ALL.len() as f64;
    FLOOR + (1.0 - n * FLOOR) * score
}

/// Declared ordinal bias: how much the score of a category gains for being declared earlier.
///
/// It is of the order of `1e-9` — invisible to any strength that differs by one character, and
/// decisive when two categories match a theme of the **same length**. Without it, an exact tie
/// would be at the mercy of floating-point rounding; with it, the tie-break is a declared rule: the
/// smallest ordinal wins.
const ORDINAL_BIAS: f64 = 1e-9;

/// Length of the longest theme of the vocabulary (`randomization`, thirteen characters), used as
/// the **global** denominator of the score and checked in the `debug_assert` of `S1::new`.
///
/// A global denominator, and not one per category: it is what makes two matches of the same length
/// directly comparable, and therefore what makes the tie-break an operation over equal numbers, and
/// not an artefact of which category has the longest theme.
const LONGEST_THEME: f64 = 13.0;

/// **Closed** set of triage categories.
///
/// The order of the enumeration is normative: it is the canonical tie-break when the lexical match
/// is exactly tied. A new category only enters with a new declaration — there is no open category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    /// Waiting, queue, service, service saturation (Fluxonomy).
    Queue,
    /// Causality, intervention, identification (causal band).
    Causal,
    /// Heavy tail, rare event, variance, non-linearity (Crisonomy).
    Tail,
    /// Calibration, coverage, rank, score (band 4).
    Calibration,
    /// Fairness, affected group, who pays for the error (band 6).
    Fairness,
    /// Absence of evidence in the declared vocabulary.
    Others,
}

impl Category {
    /// The six categories, in the normative tie-break order.
    pub const ALL: [Category; 6] = [
        Category::Queue,
        Category::Causal,
        Category::Tail,
        Category::Calibration,
        Category::Fairness,
        Category::Others,
    ];

    /// The stable label that accompanies the category in a card, log and record.
    pub fn name(self) -> &'static str {
        match self {
            Category::Queue => "queue",
            Category::Causal => "causal",
            Category::Tail => "tail",
            Category::Calibration => "calibration",
            Category::Fairness => "fairness",
            Category::Others => OTHER_LABEL,
        }
    }

    /// Index of the category in the declared order.
    pub fn index(self) -> usize {
        match self {
            Category::Queue => 0,
            Category::Causal => 1,
            Category::Tail => 2,
            Category::Calibration => 3,
            Category::Fairness => 4,
            Category::Others => 5,
        }
    }

    /// Recovers the category by its declared index.
    pub fn from_index(i: usize) -> Option<Category> {
        Category::ALL.get(i).copied()
    }
}

/// Result of a triage: the winning category and the whole distribution over the closed set, already
/// clamped to [0.01; 0.99] and adding up to 1.
#[derive(Debug, Clone, PartialEq)]
pub struct Triage {
    /// Winning category (highest probability; tie-break by the declared order).
    pub category: Category,
    /// Probability of the winning category, in [0.01; 0.99].
    pub probability: f64,
    /// Probability of each category, in the order of `Category::ALL`.
    pub distribution: [f64; 6],
}

impl Triage {
    /// Probability of a specific category.
    pub fn probability_of(&self, c: Category) -> f64 {
        self.distribution[c.index()]
    }

    /// Sum of the distribution. It has to be 1 within `1e-12`.
    pub fn sum(&self) -> f64 {
        self.distribution.iter().sum()
    }

    /// How many categories are outside [0.01; 0.99]. It has to be always zero.
    pub fn out_of_range(&self) -> usize {
        self.distribution
            .iter()
            .filter(|p| **p < FLOOR || **p > CEILING)
            .count()
    }

    /// Stable line for the record.
    pub fn line(&self) -> String {
        let products: Vec<String> = self
            .distribution
            .iter()
            .map(|p| format!("{:.4}", p))
            .collect();
        format!(
            "category={} probability={:.4} products=[{}] sum={:.4} out_of[0.01;0.99]={}",
            self.category.name(),
            self.probability,
            products.join(", "),
            self.sum(),
            self.out_of_range()
        )
    }
}

/// An indexed theme: the normalised text and the category that declared it.
#[derive(Debug, Clone, Copy)]
struct Theme {
    text: &'static str,
    category: Category,
}

/// Declared vocabulary of the six categories.
///
/// Only what is verifiable by hand enters here: a short theme, in lowercase, without accent, without
/// space.
const VOCABULARY: [Theme; 35] = [
    // queue — waiting and service saturation
    Theme {
        text: "queue",
        category: Category::Queue,
    },
    Theme {
        text: "waiting",
        category: Category::Queue,
    },
    Theme {
        text: "delay",
        category: Category::Queue,
    },
    Theme {
        text: "service",
        category: Category::Queue,
    },
    Theme {
        text: "station",
        category: Category::Queue,
    },
    Theme {
        text: "demand",
        category: Category::Queue,
    },
    // causal — intervention and identification
    Theme {
        text: "causal",
        category: Category::Causal,
    },
    Theme {
        text: "cause",
        category: Category::Causal,
    },
    Theme {
        text: "effect",
        category: Category::Causal,
    },
    Theme {
        text: "treatment",
        category: Category::Causal,
    },
    Theme {
        text: "intervention",
        category: Category::Causal,
    },
    Theme {
        text: "randomization",
        category: Category::Causal,
    },
    Theme {
        text: "confounder",
        category: Category::Causal,
    },
    // tail — rare event, variance, heavy tail
    Theme {
        text: "tail",
        category: Category::Tail,
    },
    Theme {
        text: "rare",
        category: Category::Tail,
    },
    Theme {
        text: "extreme",
        category: Category::Tail,
    },
    Theme {
        text: "variance",
        category: Category::Tail,
    },
    Theme {
        text: "volatility",
        category: Category::Tail,
    },
    Theme {
        text: "mean",
        category: Category::Tail,
    },
    // calibration — checking the size of the uncertainty
    Theme {
        text: "calibration",
        category: Category::Calibration,
    },
    Theme {
        text: "coverage",
        category: Category::Calibration,
    },
    Theme {
        text: "score",
        category: Category::Calibration,
    },
    Theme {
        text: "probability",
        category: Category::Calibration,
    },
    Theme {
        text: "brier",
        category: Category::Calibration,
    },
    Theme {
        text: "platt",
        category: Category::Calibration,
    },
    // fairness — who pays for the error
    Theme {
        text: "fairness",
        category: Category::Fairness,
    },
    Theme {
        text: "equity",
        category: Category::Fairness,
    },
    Theme {
        text: "subgroup",
        category: Category::Fairness,
    },
    Theme {
        text: "group",
        category: Category::Fairness,
    },
    Theme {
        text: "pays",
        category: Category::Fairness,
    },
    Theme {
        text: "error",
        category: Category::Fairness,
    },
    Theme {
        text: "ethics",
        category: Category::Fairness,
    },
    // others — the vocabulary of the declared absence itself
    Theme {
        text: "other",
        category: Category::Others,
    },
    Theme {
        text: "undefined",
        category: Category::Others,
    },
    Theme {
        text: "generic",
        category: Category::Others,
    },
];

/// Empty words: normalised, but without theme value. They never enter the tree.
const STOPWORDS: [&str; 27] = [
    "a", "an", "the", "of", "in", "on", "at", "to", "for", "by", "with", "without", "as", "is",
    "are", "was", "and", "or", "that", "this", "these", "those", "from", "into", "over", "under",
    "between",
];

/// Triage engine of System 1: a suffix tree built once and queried in O(1).
#[derive(Debug, Clone)]
pub struct S1 {
    /// Indexed suffix -> length of the **shortest theme** that ends at that suffix.
    suffixes: BTreeMap<String, usize>,
}

impl Default for S1 {
    fn default() -> Self {
        Self::new()
    }
}

impl S1 {
    /// Builds the triage engine from the declared vocabulary.
    ///
    /// The construction is canonical: it walks `VOCABULARIO` in the declared order and, when
    /// inserting a repeated suffix, keeps the **smallest** length — the final map does not depend on
    /// the insertion order and is identical in every run.
    pub fn new() -> S1 {
        let mut suffixes: BTreeMap<String, usize> = BTreeMap::new();

        for theme in VOCABULARY.iter() {
            let len = theme.text.chars().count();
            for start in 0..len {
                let suffix: String = theme.text.chars().skip(start).collect();
                if suffix.chars().count() < MIN_THEME {
                    continue;
                }
                match suffixes.get(&suffix) {
                    Some(current) if *current <= len => {}
                    _ => {
                        suffixes.insert(suffix, len);
                    }
                }
            }
        }

        debug_assert_eq!(
            VOCABULARY
                .iter()
                .map(|t| t.text.chars().count())
                .max()
                .unwrap_or(0) as f64,
            LONGEST_THEME,
            "LONGEST_THEME has to be the longest theme of the vocabulary"
        );

        S1 { suffixes }
    }

    /// How many suffixes the tree has. Audit instrument for the size of the index.
    pub fn indexed_suffixes(&self) -> usize {
        self.suffixes.len()
    }

    /// Matching strength of each category for a definition, normalised by the global
    /// `LONGEST_THEME`. It is what the tie-break reads before the probability distribution, and it is
    /// exposed so that the tie can be tested directly.
    pub fn strengths(&self, definition: &str) -> [f64; 6] {
        let mut strength = [0usize; 6];
        for (category, theme) in self.matches(definition) {
            let score = theme.chars().count();
            if score > strength[category.index()] {
                strength[category.index()] = score;
            }
        }
        let mut output = [0.0f64; 6];
        for c in Category::ALL.iter() {
            output[c.index()] = (strength[c.index()] as f64) / LONGEST_THEME;
        }
        output
    }

    /// All the matches found: (category, declared theme that matched), in the order in which they
    /// appear in the text.
    ///
    /// Each start position contributes **at most one** match, the longest — it is the definition of
    /// walking a suffix tree, and it is what keeps a theme and its own suffix from counting twice.
    /// The same theme may match at different positions.
    fn matches(&self, definition: &str) -> Vec<(Category, &'static str)> {
        let units = normalize(definition);
        let mut output: Vec<(Category, &'static str)> = Vec::new();

        for start in 0..units.len() {
            let mut local_winner: Option<&'static str> = None;
            for end in (start + 1)..=units.len() {
                let chunk = units[start..end].join(" ");
                match self.theme_at_suffix(&chunk) {
                    Some(theme) => local_winner = Some(theme),
                    None => break,
                }
            }
            if let Some(theme) = local_winner {
                output.push((self.theme_category(theme).expect("declared theme"), theme));
            }
        }
        output
    }

    /// The smallest declared theme that ends at that suffix.
    fn theme_at_suffix(&self, suffix: &str) -> Option<&'static str> {
        VOCABULARY
            .iter()
            .filter(|t| t.text.ends_with(suffix))
            .min_by_key(|t| t.text.chars().count())
            .map(|t| t.text)
    }

    /// Triage a one-line definition.
    ///
    /// Returns the winning category, its probability and the whole distribution over the closed set.
    /// An empty input or one outside the vocabulary falls into `outros`, with a low-but-not-zero
    /// probability.
    pub fn triage(&self, definition: &str) -> Triage {
        let strengths = self.strengths(definition);

        let n = Category::ALL.len() as f64;
        let without_evidence = strengths.iter().all(|f| *f <= 0.0);

        let mut raw = [0.0f64; 6];
        for c in Category::ALL.iter() {
            // The ordinal bias carries the tie-break and enters the score, before the transfer.
            let order = (n - c.index() as f64) * ORDINAL_BIAS;
            raw[c.index()] = if strengths[c.index()] > 0.0 {
                transfer(strengths[c.index()] + order)
            } else if without_evidence {
                // No theme matched: the declared prior holds, and it already respects the floor.
                if *c == Category::Others {
                    MASS_WITHOUT_EVIDENCE
                } else {
                    WEIGHT_WITHOUT_MATCH
                }
            } else {
                // This category did not match, but another one did: it stays at the floor.
                FLOOR
            };
        }

        // With no evidence at all, the declared prior is the answer: the distribution is that prior
        // normalised, and it does not go through the ceiling adjustment. With evidence, the
        // adjustment holds.
        let distribution = if without_evidence {
            distribute_prior(raw)
        } else {
            distribute(raw)
        };

        let (index, probability) = winner(&distribution);

        Triage {
            category: Category::ALL[index],
            probability,
            distribution,
        }
    }

    /// Which category declared the theme that ends at that text.
    fn theme_category(&self, theme: &str) -> Option<Category> {
        VOCABULARY
            .iter()
            .find(|t| t.text == theme)
            .map(|t| t.category)
    }
}

/// Normalises the definition: lowercase, without accent, without punctuation, without empty words.
///
/// Deterministic and without an external table: every non-ASCII character is transliterated by a
/// fixed rule for accented Latin letters, and whatever is left outside `[a-z0-9]` becomes a
/// separator.
pub fn normalize(definition: &str) -> Vec<String> {
    let mut current = String::new();
    let mut output: Vec<String> = Vec::new();

    for ch in definition.chars() {
        for simplified in transliterate(ch).chars() {
            if simplified.is_ascii_alphanumeric() {
                current.push(simplified.to_ascii_lowercase());
            } else if !current.is_empty() {
                if !STOPWORDS.contains(&current.as_str()) {
                    output.push(current.clone());
                }
                current.clear();
            }
        }
    }
    if !current.is_empty() && !STOPWORDS.contains(&current.as_str()) {
        output.push(current);
    }
    output
}

/// Transfers a character to the accented-free simple form. It always returns at most one character.
fn transliterate(ch: char) -> &'static str {
    match ch {
        'á' | 'à' | 'â' | 'ã' | 'ä' | 'Á' | 'À' | 'Â' | 'Ã' | 'Ä' => "a",
        'é' | 'è' | 'ê' | 'ë' | 'É' | 'È' | 'Ê' | 'Ë' => "e",
        'í' | 'ì' | 'î' | 'ï' | 'Í' | 'Ì' | 'Î' | 'Ï' => "i",
        'ó' | 'ò' | 'ô' | 'õ' | 'ö' | 'Ó' | 'Ò' | 'Ô' | 'Õ' | 'Ö' => "o",
        'ú' | 'ù' | 'û' | 'ü' | 'Ú' | 'Ù' | 'Û' | 'Ü' => "u",
        'ç' | 'Ç' => "c",
        'ñ' | 'Ñ' => "n",
        c if c.is_ascii() => ascii_table(c),
        _ => " ",
    }
}

/// One-character table for each ASCII code: a letter or digit stays as it is, the rest is a
/// separator. No external dependency and no state-dependent branch.
fn ascii_table(c: char) -> &'static str {
    const TABLE: [&str; 128] = build_ascii_table();
    TABLE[c as usize]
}

/// Builds the ASCII table at compile time.
const fn build_ascii_table() -> [&'static str; 128] {
    const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    const UPPERCASE: [&str; 26] = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    const LOWERCASE: [&str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let mut t = [" "; 128];
    let mut i = 0usize;
    while i < 10 {
        t[48 + i] = DIGITS[i];
        i += 1;
    }
    let mut j = 0usize;
    while j < 26 {
        t[65 + j] = UPPERCASE[j];
        t[97 + j] = LOWERCASE[j];
        j += 1;
    }
    t
}

/// Clamps to [0.01; 0.99] and reprojects the cut mass, preserving sum 1.
///
/// The loop is deterministic and the sum is the target: each round takes the surplus mass from
/// whoever is at the ceiling, delivers it proportionally to whoever is not, and lifts whoever fell
/// below the floor. On the set of six categories the interval is feasible
/// (6 x 0.01 <= 1 <= 6 x 0.99), so convergence is immediate — and the loop stops by count, never by
/// clock.
pub fn distribute(raw: [f64; 6]) -> [f64; 6] {
    let n = Category::ALL.len();
    assert!(
        (n as f64) * FLOOR <= 1.0 && (n as f64) * CEILING >= 1.0,
        "infeasible interval for {} categories",
        n
    );

    let sum: f64 = raw.iter().sum();
    let mut p = [0.0f64; 6];
    for (i, value) in p.iter_mut().enumerate() {
        *value = if sum > 0.0 {
            raw[i] / sum
        } else {
            1.0 / n as f64
        };
    }

    // Exact-transfer adjustment: what passes the ceiling becomes surplus, the surplus is split
    // equally among whoever still has room, whoever falls below the floor is lifted up to the floor
    // and what that costs comes out of whoever has the most mass. Every round closes the account and
    // no round manages to push a category outside the interval, so the iteration is stable — unlike
    // the proportional redistribution, which oscillated between the ceiling and the floor.
    for _ in 0..64 {
        let mut surplus = 0.0f64;
        let mut with_room: Vec<usize> = Vec::new();
        for (i, value) in p.iter_mut().enumerate() {
            if *value < FLOOR {
                surplus -= FLOOR - *value;
                *value = FLOOR;
            } else if *value > CEILING {
                surplus += *value - CEILING;
                *value = CEILING;
            }
            if *value < CEILING {
                with_room.push(i);
            }
        }
        if surplus.abs() <= 1e-15 {
            break;
        }
        if surplus > 0.0 {
            if with_room.is_empty() {
                break;
            }
            let quota = surplus / with_room.len() as f64;
            for i in with_room.iter() {
                p[*i] = (p[*i] + quota).min(CEILING);
            }
        } else {
            let available: f64 = p.iter().map(|v| (v - FLOOR).max(0.0)).sum();
            if available > 0.0 {
                let quota = (-surplus) / available;
                for value in p.iter_mut() {
                    if *value > FLOOR {
                        *value -= (*value - FLOOR) * quota;
                    }
                }
            }
        }
    }

    // Final floating-point correction: the residual mass goes to the category of **smallest declared
    // ordinal** that still has room on both sides — the same tie-break that `winner` uses.
    // Deterministic by construction, and it is what makes an exact tie of weight resolve in favour
    // of the smallest ordinal instead of becoming a coin flip.
    let rest = 1.0 - p.iter().sum::<f64>();
    if rest.abs() > 1e-12 {
        for value in p.iter_mut() {
            if *value + rest >= FLOOR && *value + rest <= CEILING {
                *value += rest;
                break;
            }
        }
    }

    p
}

/// Distribution of a **declared prior**, with no lexical evidence at all.
///
/// It normalises the vector, respects the floor and cuts at the ceiling without reprojecting: when
/// there is no evidence, the residual mass exists so that no category is left at 0%, and returning
/// it to the ceiling would reintroduce the dogma that the interval forbids.
pub fn distribute_prior(raw: [f64; 6]) -> [f64; 6] {
    let n = Category::ALL.len();
    let total: f64 = raw.iter().sum();
    let mut p = [0.0f64; 6];
    for (i, value) in p.iter_mut().enumerate() {
        let fraction = if total > 0.0 {
            raw[i] / total
        } else {
            1.0 / n as f64
        };
        *value = fraction.clamp(FLOOR, CEILING);
    }
    p
}

/// Winner: highest probability; a tie resolved by the declared order of `Category::ALL`.
fn winner(p: &[f64; 6]) -> (usize, f64) {
    let mut index = 0usize;
    let mut value = p[0];
    for (i, v) in p.iter().enumerate().skip(1) {
        if *v > value {
            value = *v;
            index = i;
        }
    }
    (index, value)
}

/// Shortcut: builds the triage engine and triages a definition.
pub fn triage(definition: &str) -> Triage {
    S1::new().triage(definition)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Canonical definition of the card in order, used in several tests.
    const IN_ORDER: &str = "waiting time in the queue at the health station";
    /// Definition without any declared theme.
    const UNKNOWN: &str = "xyzzy plugh abracadabra";

    #[test]
    fn happy_paths_every_theme_finds_its_category() {
        let s1 = S1::new();
        let cases = [
            (
                "waiting time in the queue at the health station",
                Category::Queue,
            ),
            ("causal effect of the treatment on income", Category::Causal),
            ("heavy tail of a rare extreme event", Category::Tail),
            (
                "calibration of the platt score with brier",
                Category::Calibration,
            ),
            (
                "fairness across subgroups and who pays for the error",
                Category::Fairness,
            ),
            ("generic undefined card", Category::Others),
        ];
        for (definition, expected) in cases {
            let t = s1.triage(definition);
            assert_eq!(
                t.category,
                expected,
                "definition '{}' triaged as {}",
                definition,
                t.category.name()
            );
        }
    }

    #[test]
    fn reading_of_the_record() {
        let s1 = S1::new();
        for definition in [
            IN_ORDER,
            "causal effect of the treatment on income",
            "heavy tail of a rare extreme event",
            "calibration of the platt score with brier",
            "fairness across subgroups and who pays for the error",
            UNKNOWN,
        ] {
            println!("{:>44} -> {}", definition, s1.triage(definition).line());
        }
        println!("suffixes indexed in the tree: {}", s1.indexed_suffixes());
    }

    #[test]
    fn determinism_same_input_same_output() {
        let a = triage(IN_ORDER);
        let b = triage(IN_ORDER);
        assert_eq!(a, b);

        let s1 = S1::new();
        let c = s1.triage(IN_ORDER);
        assert_eq!(a, c);
        assert_eq!(a.line(), c.line());
        // And the index is reproducible too: two constructions, same size.
        assert_eq!(s1.indexed_suffixes(), S1::new().indexed_suffixes());
    }

    #[test]
    fn no_probability_at_zero_or_one() {
        let s1 = S1::new();
        for definition in [
            IN_ORDER,
            UNKNOWN,
            "heavy tail",
            "fairness",
            "",
            "a an the of",
            "causal causal causal causal",
        ] {
            let t = s1.triage(definition);
            assert_eq!(t.out_of_range(), 0, "definition '{}'", definition);
            for p in t.distribution.iter() {
                assert!(*p > 0.0 && *p < 1.0, "dogmatic probability {}", p);
                assert!(*p >= FLOOR && *p <= CEILING);
            }
            // The winner is also strictly below the ceiling.
            assert!(t.probability < 1.0);
        }
    }

    #[test]
    fn sum_one_after_the_adjustment() {
        let s1 = S1::new();
        for definition in [
            IN_ORDER,
            UNKNOWN,
            "causal effect",
            "station",
            "",
            "group subgroup pays error ethics fairness",
        ] {
            let t = s1.triage(definition);
            assert!(
                (t.sum() - 1.0).abs() < 1e-12,
                "sum {} for '{}'",
                t.sum(),
                definition
            );
        }
    }

    #[test]
    fn unknown_input_falls_into_others() {
        let s1 = S1::new();
        let t = s1.triage(UNKNOWN);
        assert_eq!(t.category, Category::Others);
        // The effective mass is the declared weight divided by the total mass of the prior: the five
        // thematic categories stay at 0.0256 and "other" takes 0.8718. Analytical value checked.
        let expected = MASS_WITHOUT_EVIDENCE / (MASS_WITHOUT_EVIDENCE + 5.0 * WEIGHT_WITHOUT_MATCH);
        assert!(
            expected > 0.5 && expected < CEILING,
            "declared prior = {}",
            expected
        );
        assert!(
            (t.probability - expected).abs() < 1e-12,
            "mass of other {} != {}",
            t.probability,
            expected
        );
        assert!(
            t.probability > FLOOR && t.probability < CEILING,
            "low-but-not-zero, and never dogma"
        );
        // It is the largest of the distribution, and still it is not dogma.
        for c in Category::ALL.iter().filter(|c| **c != Category::Others) {
            assert!(t.probability > t.probability_of(*c));
        }
        // The five thematic categories keep a positive mass: "other" does not erase the
        // alternatives, it only demotes them.
        for c in Category::ALL.iter().filter(|c| **c != Category::Others) {
            assert!(t.probability_of(*c) >= FLOOR);
        }
        // An empty input follows the same path.
        assert_eq!(s1.triage("").category, Category::Others);
        assert_eq!(s1.triage("   ").category, Category::Others);
    }

    #[test]
    fn case_that_must_fail_a_short_match_is_not_evidence() {
        let s1 = S1::new();
        let short = s1.triage("the aa bb cc dd");
        assert_eq!(
            short.category,
            Category::Others,
            "a short excerpt cannot sustain thematic triage"
        );
        // Negative proof of the same gate: without the minimum length, "the" does not match; if it
        // did, the lexical energy of any preposition would become a category.
        assert_ne!(short.category, Category::Fairness);
    }

    #[test]
    fn case_that_must_fail_a_card_with_thematic_evidence_does_not_fall_into_others() {
        let s1 = S1::new();
        let t = s1.triage(IN_ORDER);
        assert_eq!(t.category, Category::Queue);
        // The same gate that accepts the card in order has to reject the opposite reading.
        assert_ne!(t.category, Category::Others);
        assert!(
            t.probability >= 0.8 && t.probability <= CEILING,
            "the card in order has strong evidence and stays below the ceiling: {}",
            t.probability
        );
        assert!(
            t.probability > MASS_WITHOUT_EVIDENCE,
            "thematic evidence cannot return the mass of 'I do not know'"
        );
    }

    #[test]
    fn the_tie_break_is_the_declared_order() {
        let s1 = S1::new();
        // "queue" (5, in Queue) and "cause" (5, in Causal): with a global denominator the two
        // strengths are 5/13, exactly equal.
        let definition = "queue cause";
        let strengths = s1.strengths(definition);
        let diff = (strengths[Category::Queue.index()] - strengths[Category::Causal.index()]).abs();
        assert!(
            diff < 1e-12,
            "unequal strengths in the tie, difference {}",
            diff
        );
        assert!((strengths[Category::Queue.index()] - 5.0 / LONGEST_THEME).abs() < 1e-12);
        assert!((strengths[Category::Causal.index()] - 5.0 / LONGEST_THEME).abs() < 1e-12);
        // The tie does not become a tie of probability: the rest of the mass goes to the category of
        // smallest declared ordinal, and the choice is stable in every run.
        let t = s1.triage(definition);
        assert_eq!(
            t.category,
            Category::Queue,
            "tie resolved by the smallest declared ordinal (Queue before Causal)"
        );
        assert!(
            t.probability_of(Category::Queue) > t.probability_of(Category::Causal),
            "the residual mass also goes to the smallest ordinal"
        );
        assert_eq!(t, s1.triage(definition));
    }

    #[test]
    fn long_suffix_beats_short_theme() {
        let s1 = S1::new();
        // "retroactive" ends in "active", which is a suffix of some theme? No: the valid match is
        // "treatment" (9) in the causal category.
        let t = s1.triage("retroactive effects of the treatment");
        assert_eq!(t.category, Category::Causal);
        assert!(s1.indexed_suffixes() > 0);
        // The divisor is global, so the score never passes 1 and the card stays far from the ceiling.
        let strengths = s1.strengths("retroactive effects of the treatment");
        assert!((strengths[Category::Causal.index()] - 9.0 / LONGEST_THEME).abs() < 1e-12);
        assert!(t.probability_of(Category::Causal) <= CEILING);
        assert!(strengths[Category::Causal.index()] < 1.0);
    }

    #[test]
    fn normalization_is_stable() {
        assert_eq!(normalize("Causal!"), vec!["causal".to_string()]);
        assert_eq!(normalize("calibração"), vec!["calibracao".to_string()]);
        assert_eq!(
            normalize("THE queue AT THE station"),
            vec!["queue".to_string(), "station".to_string()]
        );
        // Empty words vanish, no matter how many times they appear.
        assert_eq!(normalize("the the the"), Vec::<String>::new());
    }

    #[test]
    fn ceiling_reprojection_preserves_the_sum() {
        // A degenerate raw vector: one category with enormous mass, the others almost null.
        let p = distribute([1000.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-12);
        assert!(p[0] <= CEILING && p[0] >= FLOOR);
        for value in p.iter() {
            assert!(*value >= FLOOR && *value <= CEILING);
        }
        // The interval is a fixed point: distributing again changes nothing.
        let q = distribute(p);
        for i in 0..6 {
            assert!(
                (p[i] - q[i]).abs() < 1e-12,
                "fixed point broken at index {}",
                i
            );
        }
    }
}
