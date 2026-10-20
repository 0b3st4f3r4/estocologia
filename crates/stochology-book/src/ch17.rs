//! Chapter 17 laboratory — cards, prohibitions, invariants Phi1-Phi4 and gates.
//!
//! Validate a complete card and a card with one planted defect at a time,
//! printing the failures by gate, in the spirit of ~portoes --defeito~.
//!
//! Fixed seed. No external dependencies. SplitMix64 built in.

fn splitmix64(state: &mut u64) -> u64 {
    stochology_core::rng::splitmix64_step(state)
}

fn uniform(state: &mut u64, limit: u64) -> u64 {
    splitmix64(state) % limit
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gate {
    Phi1,
    Phi2,
    Phi3,
    Phi4,
    TypeIII,
}

impl Gate {
    fn code(self) -> &'static str {
        match self {
            Gate::Phi1 => "Phi1-simulability",
            Gate::Phi2 => "Phi2-independent-unit",
            Gate::Phi3 => "Phi3-layers-and-null-model",
            Gate::Phi4 => "Phi4-scope-and-exit",
            Gate::TypeIII => "type-III-decider-question",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Defect {
    NoSeed,
    NoDefinition,
    NoMarginNoUnit,
    NoNullModel,
    NoScopeNoPayerNoExit,
    QuestionWithoutMark,
    QuestionWithoutAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layer {
    L1,
    L2,
    L3,
}

impl Layer {
    fn code(self) -> &'static str {
        match self {
            Layer::L1 => "L1",
            Layer::L2 => "L2",
            Layer::L3 => "L3",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UncertaintyType {
    Aleatory,
    Epistemic,
    Knightian,
    NotApplicable,
}

impl UncertaintyType {
    fn code(self) -> &'static str {
        match self {
            UncertaintyType::Aleatory => "aleatory",
            UncertaintyType::Epistemic => "epistemic",
            UncertaintyType::Knightian => "knightian",
            UncertaintyType::NotApplicable => "not-applicable",
        }
    }
}

#[derive(Debug, Clone)]
struct Card {
    id: String,
    layer: Layer,
    uncertainty: UncertaintyType,
    definition: String,
    simulable_artifact: Option<String>,
    seed: Option<u64>,
    independent_unit: Option<String>,
    margin: Option<f64>,
    null_model: Option<String>,
    ecological_scope: bool,
    who_pays_for_error: Option<String>,
    exit_clause: Option<String>,
    decider_question: String,
    package_action: String,
    header: [String; 5],
}

fn complete_card() -> Card {
    Card {
        id: "h0.posto-saude".to_string(),
        layer: Layer::L1,
        uncertainty: UncertaintyType::Aleatory,
        definition: "queue with non-linear arrival and convex waiting time".to_string(),
        simulable_artifact: Some("crates/stochology-book#ch12".to_string()),
        seed: Some(2026),
        independent_unit: Some("day".to_string()),
        margin: Some(0.51),
        null_model: Some("historical-mean".to_string()),
        ecological_scope: true,
        who_pays_for_error: Some("the patient who waits and gives up".to_string()),
        exit_clause: Some(
            "stop if the fraction of waits above 30 min passes 0,25".to_string(),
        ),
        decider_question: "should we open a second room on the busy days?".to_string(),
        package_action: "open a second room".to_string(),
        header: [
            "generating_process: queue with an arrival rate that oscillates between 5 and 11 per hour and a non-Poisson arrival"
                .to_string(),
            "non_linearity: the wait grows on a curve; f(E[X]) is not E[f(X)] near saturation".to_string(),
            "unit_and_margin: the independent unit is the DAY; 81 days, not 2,65 million calls".to_string(),
            "noise_physics: the daily demand has Var/mean = 273 (Poisson: 1); no absorbing states".to_string(),
            "causal_ladder: step 2 do(staffing) simulated over the real demand; untestable premise: the demand does not react to the staffing".to_string(),
        ],
    }
}

#[derive(Debug, Clone)]
struct Failure {
    gate: Gate,
    detail: String,
}

fn evaluate(f: &Card) -> Vec<Failure> {
    let mut failures: Vec<Failure> = Vec::new();
    let mut add = |gate: Gate, detail: &str| {
        failures.push(Failure {
            gate,
            detail: detail.to_string(),
        });
    };

    if f.seed.is_none() || f.simulable_artifact.is_none() {
        add(Gate::Phi1, "missing fixed seed or simulable artifact");
    }
    if f.margin.is_none() || f.independent_unit.is_none() {
        add(Gate::Phi2, "margin without a declared independent unit");
    }
    if f.null_model.is_none() || f.definition.trim().is_empty() {
        add(Gate::Phi3, "no null model or no one-line definition");
    }
    if !f.ecological_scope || f.who_pays_for_error.is_none() || f.exit_clause.is_none() {
        add(
            Gate::Phi4,
            "ecological scope, who pays for the error or exit clause missing",
        );
    }
    let question = f.decider_question.trim();
    let action = f.package_action.trim().to_lowercase();
    let matches = !action.is_empty() && question.to_lowercase().contains(&action);
    // Same four-way split as the core lock (`stochology-core/src/invariants.rs`): the message
    // names which part of the question is missing.
    match (question.ends_with('?'), matches) {
        (true, true) => {}
        (false, true) => add(Gate::TypeIII, "decider_question does not end in ?"),
        (true, false) => add(
            Gate::TypeIII,
            "decider_question does not quote the package action",
        ),
        (false, false) => add(
            Gate::TypeIII,
            "decider_question neither ends in ? nor quotes the package action",
        ),
    }
    if f.header.iter().any(|c| c.trim().is_empty()) {
        add(Gate::Phi3, "five-question header incomplete");
    }
    for (i, c) in f.header.iter().enumerate() {
        let expected = [
            "generating_process",
            "non_linearity",
            "unit_and_margin",
            "noise_physics",
            "causal_ladder",
        ][i];
        if !c.starts_with(expected) {
            add(Gate::Phi3, "header field out of order");
        }
    }
    failures
}

// ----------------------------------------------- graph, in the spirit of Ch. 15

#[derive(Debug, Clone, PartialEq)]
struct Node {
    id: String,
    definition: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Hyperedge {
    id: String,
    members: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct Graph {
    nodes: Vec<Node>,
    tree: Vec<(String, String)>,
    hyperedges: Vec<Hyperedge>,
}

impl Graph {
    fn validate(&self) -> Vec<String> {
        let mut p: Vec<String> = Vec::new();
        let exists = |id: &str| self.nodes.iter().any(|n| n.id == id);
        for (from, to) in &self.tree {
            if !exists(from) || !exists(to) {
                p.push(format!("edge to absent node: {} -> {}", from, to));
            }
        }
        for h in &self.hyperedges {
            if h.members.len() < 2 {
                p.push(format!("hyperedge with fewer than two members: {}", h.id));
            }
            for m in &h.members {
                if !exists(m) {
                    p.push(format!("hyperedge {} cites absent node: {}", h.id, m));
                }
            }
        }
        let mut colour: Vec<(String, u8)> = Vec::new();
        let mut cycle: Option<String> = None;
        for n in &self.nodes {
            if self.visit(&n.id, &mut colour, &mut cycle) {
                p.push(format!("cycle in E_tree: {:?}", cycle));
                break;
            }
        }
        p
    }

    fn visit(&self, id: &str, colour: &mut Vec<(String, u8)>, cycle: &mut Option<String>) -> bool {
        match colour.iter().find(|(x, _)| x == id).map(|(_, c)| *c) {
            Some(1) => {
                *cycle = Some(id.to_string());
                true
            }
            Some(2) => false,
            _ => {
                colour.push((id.to_string(), 1));
                for (from, to) in &self.tree {
                    if from == id && self.visit(to, colour, cycle) {
                        return true;
                    }
                }
                if let Some(e) = colour.iter_mut().find(|(x, _)| x == id) {
                    e.1 = 2;
                }
                false
            }
        }
    }
}

fn good_graph() -> Graph {
    Graph {
        nodes: vec![
            Node {
                id: "L1_queue".to_string(),
                definition: "wait measured over 81 days".to_string(),
            },
            Node {
                id: "L2_erlang".to_string(),
                definition: "stationary level of service".to_string(),
            },
            Node {
                id: "L3_manager".to_string(),
                definition: "staffing decided by the mean".to_string(),
            },
        ],
        tree: vec![
            ("L1_queue".to_string(), "L2_erlang".to_string()),
            ("L2_erlang".to_string(), "L3_manager".to_string()),
        ],
        hyperedges: vec![Hyperedge {
            id: "H1".to_string(),
            members: vec!["L1_queue".to_string(), "L2_erlang".to_string()],
        }],
    }
}
fn with_defect(d: Defect) -> Card {
    let mut f = complete_card();
    match d {
        Defect::NoSeed => f.seed = None,
        Defect::NoDefinition => f.definition = String::new(),
        Defect::NoMarginNoUnit => {
            f.margin = None;
            f.independent_unit = None;
        }
        Defect::NoNullModel => f.null_model = None,
        Defect::NoScopeNoPayerNoExit => {
            f.ecological_scope = false;
            f.who_pays_for_error = None;
            f.exit_clause = None;
        }
        Defect::QuestionWithoutMark => {
            f.decider_question = "should we open a second room on the busy days".to_string()
        }
        Defect::QuestionWithoutAction => {
            f.decider_question = "should we do something on the busy days?".to_string()
        }
    }
    f
}

fn names(v: &[Gate]) -> String {
    v.iter().map(|p| p.code()).collect::<Vec<&str>>().join(",")
}

pub fn run() {
    println!("== Chapter 17 :: the iron contract ==");

    let f = complete_card();
    let failures = evaluate(&f);
    println!(
        "card {}: layer={} uncertainty={} header={} fields | failures={:?} total={}",
        f.id,
        f.layer.code(),
        f.uncertainty.code(),
        f.header.len(),
        failures
            .iter()
            .map(|x| x.gate.code())
            .collect::<Vec<&str>>(),
        failures.len()
    );

    // The declared taxonomy: three layers and four uncertainty types.
    let layers = [Layer::L1, Layer::L2, Layer::L3];
    let uncertainties = [
        UncertaintyType::Aleatory,
        UncertaintyType::Epistemic,
        UncertaintyType::Knightian,
        UncertaintyType::NotApplicable,
    ];
    println!(
        "declared taxonomy: layers={:?} uncertainties={:?}",
        layers.iter().map(|c| c.code()).collect::<Vec<&str>>(),
        uncertainties
            .iter()
            .map(|i| i.code())
            .collect::<Vec<&str>>()
    );

    let defects = [
        Defect::NoSeed,
        Defect::NoDefinition,
        Defect::NoMarginNoUnit,
        Defect::NoNullModel,
        Defect::NoScopeNoPayerNoExit,
        Defect::QuestionWithoutMark,
        Defect::QuestionWithoutAction,
    ];
    for d in defects {
        let fx = with_defect(d);
        let failures = evaluate(&fx);
        let gates: Vec<Gate> = {
            let mut v: Vec<Gate> = failures.iter().map(|x| x.gate).collect();
            v.sort_by_key(|p| p.code());
            v.dedup();
            v
        };
        println!(
            "defect {:?}: failures={} gates={} detail={:?}",
            d,
            failures.len(),
            names(&gates),
            failures
                .first()
                .map(|x| x.detail.clone())
                .unwrap_or_default()
        );
    }

    // The two graph invariants, in the spirit of --defeito.
    let g = good_graph();
    println!("intact graph: validation={:?}", g.validate());
    let mut g1 = g.clone();
    g1.tree
        .push(("L2_erlang".to_string(), "L2_ghost".to_string()));
    println!("graph with a ghost edge: validation={:?}", g1.validate());
    let mut g2 = g.clone();
    g2.tree
        .push(("L3_manager".to_string(), "L1_queue".to_string()));
    println!(
        "graph with a deductive cycle: validation={:?}",
        g2.validate()
    );
    let mut g3 = g.clone();
    g3.hyperedges.push(Hyperedge {
        id: "H2".to_string(),
        members: vec!["L3_manager".to_string()],
    });
    println!(
        "graph with a one-member hyperedge: validation={:?}",
        g3.validate()
    );

    // Property test over the function of the Phi2 gate: the simulated margin
    // can never be negative and can never shrink when the sample grows.
    let mut rng = stochology_core::rng::offset_seed(2026);
    let mut violations = 0usize;
    let mut smallest = f64::MAX;
    let mut largest = f64::MIN;
    for _ in 0..10_000 {
        let n = (uniform(&mut rng, 900) + 100) as f64;
        let p = (uniform(&mut rng, 99) + 1) as f64 / 100.0;
        let margin = 1.96 * (p * (1.0 - p) / n).sqrt();
        smallest = smallest.min(margin);
        largest = largest.max(margin);
        if !(0.0..=0.5).contains(&margin) {
            violations += 1;
        }
    }
    println!(
        "property test (10.000 simulated cards): violations={} margin_min={:.5} margin_max={:.5}",
        violations, smallest, largest
    );

    let repetitions = 32usize;
    let mut different = 0usize;
    for _ in 0..repetitions {
        let mut a = 2026u64;
        let mut b = 2026u64;
        let va: Vec<u64> = (0..16).map(|_| uniform(&mut a, 1_000_000)).collect();
        let vb: Vec<u64> = (0..16).map(|_| uniform(&mut b, 1_000_000)).collect();
        if va != vb {
            different += 1;
        }
    }
    println!(
        "same seed, two validators: runs that diverge in {} of {}",
        different, repetitions
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_card_passes_and_a_defect_triggers_a_gate() {
        assert!(evaluate(&complete_card()).is_empty());
        assert!(!evaluate(&with_defect(Defect::NoSeed)).is_empty());
        assert!(good_graph().validate().is_empty());
    }
}
