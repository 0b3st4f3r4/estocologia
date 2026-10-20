//! The pathology edges, typed, and the predicates that fire on a card.
//!
//! Chapter 15 names six pathologies and the checklist adds ten more. Until this module the graph had a
//! single untyped `Pathology` edge. Each kind below is now a name the validator can read. What the code
//! can *detect* is declared by [`reach`]: only the kinds marked [`Reach::OnCard`] have a predicate over the
//! fields of today's card; the others say whether the field exists without a predicate yet, which field is
//! still missing, or which data they would need, and
//! return no finding. A kind that cannot be seen on the card is not pretended to be seen.

use stochology_core::{Card, Resolution};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PathologyKind {
    // The six of Chapter 15.
    FallacyOfTheMean,
    FallacyOfTheStableWorld,
    InspectionParadox,
    TranspositionOfTheConditional,
    RussellsTurkey,
    CaseForPopulation,
    // The ten of the review.
    SelectionMeng,
    Overbet,
    BlindEce,
    EnsembleForTime,
    WildTail,
    Pseudoreplication,
    AggregatedResolution,
    PerformativeLoop,
    ColliderAdjustment,
    DependentDecibans,
}

impl PathologyKind {
    pub const ALL: [PathologyKind; 16] = [
        PathologyKind::FallacyOfTheMean,
        PathologyKind::FallacyOfTheStableWorld,
        PathologyKind::InspectionParadox,
        PathologyKind::TranspositionOfTheConditional,
        PathologyKind::RussellsTurkey,
        PathologyKind::CaseForPopulation,
        PathologyKind::SelectionMeng,
        PathologyKind::Overbet,
        PathologyKind::BlindEce,
        PathologyKind::EnsembleForTime,
        PathologyKind::WildTail,
        PathologyKind::Pseudoreplication,
        PathologyKind::AggregatedResolution,
        PathologyKind::PerformativeLoop,
        PathologyKind::ColliderAdjustment,
        PathologyKind::DependentDecibans,
    ];

    pub fn name(self) -> &'static str {
        match self {
            PathologyKind::FallacyOfTheMean => "fallacy-of-the-mean",
            PathologyKind::FallacyOfTheStableWorld => "fallacy-of-the-stable-world",
            PathologyKind::InspectionParadox => "inspection-paradox",
            PathologyKind::TranspositionOfTheConditional => "transposition-of-the-conditional",
            PathologyKind::RussellsTurkey => "russells-turkey",
            PathologyKind::CaseForPopulation => "case-for-population",
            PathologyKind::SelectionMeng => "selection-meng",
            PathologyKind::Overbet => "overbet",
            PathologyKind::BlindEce => "blind-ece",
            PathologyKind::EnsembleForTime => "ensemble-for-time",
            PathologyKind::WildTail => "wild-tail",
            PathologyKind::Pseudoreplication => "pseudoreplication",
            PathologyKind::AggregatedResolution => "aggregated-resolution",
            PathologyKind::PerformativeLoop => "performative-loop",
            PathologyKind::ColliderAdjustment => "collider-adjustment",
            PathologyKind::DependentDecibans => "dependent-decibans",
        }
    }
}

/// What it takes for the code to see a pathology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reach {
    /// A predicate over the fields of the card exists in this module.
    OnCard,
    /// The card carries the fields since contract 1.6, but no predicate over them is written yet.
    FieldsExist(&'static [&'static str]),
    /// It would need fields that the card does not carry yet.
    NeedsFields(&'static [&'static str]),
    /// It lives in the data or the simulation, not in the card; the instrument is in the book's laboratories.
    NeedsData,
}

pub fn reach(kind: PathologyKind) -> Reach {
    use PathologyKind::*;
    match kind {
        AggregatedResolution | Pseudoreplication => Reach::OnCard,
        SelectionMeng => Reach::FieldsExist(&["correlacao_de_selecao"]),
        BlindEce => Reach::FieldsExist(&["corte_de_ece"]),
        ColliderAdjustment | DependentDecibans => Reach::FieldsExist(&["grafo_causal"]),
        Overbet | EnsembleForTime => Reach::NeedsFields(&["fracao_apostada"]),
        PerformativeLoop => Reach::NeedsFields(&["canario", "contrafactual"]),
        FallacyOfTheMean
        | FallacyOfTheStableWorld
        | InspectionParadox
        | RussellsTurkey
        | WildTail
        | CaseForPopulation
        | TranspositionOfTheConditional => Reach::NeedsData,
    }
}

/// A predicate over the card that fires for one kind of pathology.
pub trait Detect {
    fn kind(&self) -> PathologyKind;
    /// `Some(detail)` when the card shows the pathology.
    fn detect(&self, card: &Card) -> Option<String>;
}

/// The question is asked of the average (R0) while the card declares an independent unit: the
/// unit is finer than the question. Phi5 refuses the day-or-coarser case; this marks the edge for any unit.
pub struct AggregatedResolution;

impl Detect for AggregatedResolution {
    fn kind(&self) -> PathologyKind {
        PathologyKind::AggregatedResolution
    }
    fn detect(&self, card: &Card) -> Option<String> {
        let unit = card.independent_unit.as_deref()?;
        (card.resolution == Resolution::R0Aggregated).then(|| {
            format!(
                "question at {} against independent unit \"{}\"",
                card.resolution.code(),
                unit.trim()
            )
        })
    }
}

/// A margin is declared on a unit named as a record, a row or an hour: neighbouring records of one
/// unit are not independent. Syntactic, like the type III lock: it reads the words of the declared unit.
pub struct Pseudoreplication;

impl Detect for Pseudoreplication {
    fn kind(&self) -> PathologyKind {
        PathologyKind::Pseudoreplication
    }
    fn detect(&self, card: &Card) -> Option<String> {
        const RECORD_WORDS: [&str; 7] = [
            "linha", "registro", "observa", "row", "record", "chamada", "hora",
        ];
        card.margin?;
        let unit = card.independent_unit.as_deref()?.trim().to_lowercase();
        RECORD_WORDS
            .iter()
            .any(|w| unit.contains(w))
            .then(|| format!("margin declared on \"{unit}\", a record grain: neighbouring records are not independent"))
    }
}

/// The detectors that exist today, one per kind marked [`Reach::OnCard`].
pub fn detectors() -> Vec<Box<dyn Detect>> {
    vec![Box::new(AggregatedResolution), Box::new(Pseudoreplication)]
}

/// Every pathology the card shows, with the detail.
pub fn detect_all(card: &Card) -> Vec<(PathologyKind, String)> {
    detectors()
        .iter()
        .filter_map(|d| d.detect(card).map(|detail| (d.kind(), detail)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::{Layer, UncertaintyType};

    fn card() -> Card {
        Card::new("c", Layer::Model, UncertaintyType::Aleatory)
    }

    #[test]
    fn there_are_sixteen_distinct_kinds_and_the_reach_table_is_total() {
        let mut names: Vec<_> = PathologyKind::ALL.iter().map(|k| k.name()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 16);
        let on_card = PathologyKind::ALL
            .iter()
            .filter(|k| reach(**k) == Reach::OnCard)
            .count();
        assert_eq!(on_card, detectors().len());
        for d in detectors() {
            assert_eq!(reach(d.kind()), Reach::OnCard);
        }
    }

    #[test]
    fn aggregated_resolution_fires_on_r0_with_a_unit_and_not_on_r1() {
        let mut c = card();
        c.independent_unit = Some("dia".into());
        c.resolution = Resolution::R0Aggregated;
        assert_eq!(detect_all(&c).len(), 1);
        assert_eq!(detect_all(&c)[0].0, PathologyKind::AggregatedResolution);
        c.resolution = Resolution::R1Day;
        assert!(detect_all(&c).is_empty());
        // without a declared unit there is nothing to compare the question with
        c.resolution = Resolution::R0Aggregated;
        c.independent_unit = None;
        assert!(detect_all(&c).is_empty());
    }

    #[test]
    fn pseudoreplication_fires_on_a_record_grain_with_a_margin_and_not_on_a_day() {
        let mut c = card();
        c.resolution = Resolution::R1Day;
        c.margin = Some(0.1);
        c.independent_unit = Some("cada chamada".into());
        let found = detect_all(&c);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].0, PathologyKind::Pseudoreplication);
        c.independent_unit = Some("dia de operação".into());
        assert!(detect_all(&c).is_empty());
        // oracle for the limit: no margin, nothing to defend, nothing fires
        c.independent_unit = Some("cada chamada".into());
        c.margin = None;
        assert!(detect_all(&c).is_empty());
    }

    #[test]
    fn a_kind_the_card_cannot_show_is_never_reported() {
        // The card carries Meng's selection correlation since 1.6, but no predicate over it is written: the code says
        // so and the finding list stays empty.
        assert!(matches!(
            reach(PathologyKind::SelectionMeng),
            Reach::FieldsExist(_)
        ));
        assert!(matches!(
            reach(PathologyKind::Overbet),
            Reach::NeedsFields(_)
        ));
        assert!(detect_all(&card()).is_empty());
    }
}
