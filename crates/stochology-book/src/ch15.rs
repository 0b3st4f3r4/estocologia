//! Chapter 15 laboratory — the map of knowledge.
//!
//! Mini typed hypergraph G = (V, E_tree, E_graph, H, Phi): four structural
//! proofs and three measured lenses (Scalonomy by Hill, Crisonomy by EVPI,
//! Morphonomy by invariance across regimes).
//!
//! Fixed seed. No external dependencies. SplitMix64 built in.

use stochology_hypergraph::lenses::evpi_from_actions;

fn splitmix64(state: &mut u64) -> u64 {
    stochology_core::rng::splitmix64_step(state)
}

fn uniform(state: &mut u64, limit: u64) -> u64 {
    splitmix64(state) % limit
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layer {
    L1,
    L2,
    L3,
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    id: String,
    layer: Layer,
    definition: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeType {
    Deductive,
    MathematicalBridge,
    Causal,
    Pathology,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lens {
    Fluxonomy,
    Chrononomy,
    Scalonomy,
    Morphonomy,
    Crisonomy,
}

impl Lens {
    const ALL: [Lens; 5] = [
        Lens::Fluxonomy,
        Lens::Chrononomy,
        Lens::Scalonomy,
        Lens::Morphonomy,
        Lens::Crisonomy,
    ];

    fn name(self) -> &'static str {
        match self {
            Lens::Fluxonomy => "Fluxonomy",
            Lens::Chrononomy => "Chrononomy",
            Lens::Scalonomy => "Scalonomy",
            Lens::Morphonomy => "Morphonomy",
            Lens::Crisonomy => "Crisonomy",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Hyperedge {
    id: String,
    lens: Lens,
    members: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct Edge {
    from: String,
    to: String,
    kind: EdgeType,
    label: String,
}

#[derive(Debug, Clone, Default)]
struct Graph {
    nodes: Vec<Node>,
    tree: Vec<Edge>,
    graph: Vec<Edge>,
    hyperedges: Vec<Hyperedge>,
}

fn node(id: &str, layer: Layer, definition: &str) -> Node {
    Node {
        id: id.to_string(),
        layer,
        definition: definition.to_string(),
    }
}

fn edge(from: &str, to: &str, kind: EdgeType, label: &str) -> Edge {
    Edge {
        from: from.to_string(),
        to: to.to_string(),
        kind,
        label: label.to_string(),
    }
}

impl Graph {
    /// Structure: unique and complete ids, existing endpoints, tree without a cycle,
    /// hyperedges with two members or more.
    fn validate(&self) -> Vec<String> {
        let mut p: Vec<String> = Vec::new();
        let mut seen: Vec<&str> = Vec::new();
        for n in &self.nodes {
            if n.id.trim().is_empty() || n.definition.trim().is_empty() {
                p.push(format!("incomplete node: {}", n.id));
            }
            if seen.contains(&n.id.as_str()) {
                p.push(format!("duplicate node: {}", n.id));
            }
            seen.push(n.id.as_str());
        }
        let exists = |id: &str| self.nodes.iter().any(|n| n.id == id);
        for a in self.tree.iter().chain(self.graph.iter()) {
            if !exists(&a.from) || !exists(&a.to) {
                p.push(format!("edge to absent node: {} -> {}", a.from, a.to));
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
        if let Some(cycle) = self.cycle_in_tree() {
            p.push(format!("cycle in E_tree: {:?}", cycle));
        }
        p
    }

    fn topological_order(&self) -> Result<Vec<String>, Vec<String>> {
        let mut order: Vec<String> = Vec::new();
        let mut visiting: Vec<String> = Vec::new();
        for n in &self.nodes {
            if !self.dfs(&n.id, &mut order, &mut visiting) {
                return Err(visiting);
            }
        }
        Ok(order)
    }

    fn dfs(&self, id: &str, order: &mut Vec<String>, visiting: &mut Vec<String>) -> bool {
        if order.iter().any(|x| x == id) {
            return true;
        }
        if visiting.iter().any(|x| x == id) {
            return false;
        }
        visiting.push(id.to_string());
        for a in &self.tree {
            if a.from == id && !self.dfs(&a.to, order, visiting) {
                return false;
            }
        }
        visiting.retain(|x| x != id);
        order.push(id.to_string());
        true
    }

    fn cycle_in_tree(&self) -> Option<Vec<String>> {
        self.topological_order().err()
    }

    fn nodes_without_lens(&self) -> Vec<String> {
        self.nodes
            .iter()
            .filter(|n| !self.hyperedges.iter().any(|h| h.members.contains(&n.id)))
            .map(|n| n.id.clone())
            .collect()
    }

    /// Deductive depth: length of the longest path to a leaf.
    fn depths(&self) -> Vec<(String, usize)> {
        match self.topological_order() {
            Ok(order) => order
                .iter()
                .map(|id| {
                    let mut d = 1usize;
                    let mut current = id.clone();
                    while let Some(a) = self.tree.iter().find(|a| a.from == current) {
                        d += 1;
                        current = a.to.clone();
                    }
                    (id.clone(), d)
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    fn by_layer(&self, c: Layer) -> usize {
        self.nodes.iter().filter(|n| n.layer == c).count()
    }
}

// ---------------------------------------------------------------- the lenses

/// Scalonomy: Hill estimator over the upper tail (delegates to ~stochology-hypergraph~).
fn hill_index(values: &[f64], k: usize) -> f64 {
    stochology_hypergraph::lenses::hill_index(values, k)
}

fn wild_tail(alpha: f64) -> bool {
    stochology_hypergraph::lenses::wild_tail(alpha)
}

fn slope(xs: &[f64], ys: &[f64]) -> f64 {
    stochology_hypergraph::lenses::slope(xs, ys)
}

fn invariant(slopes: &[f64], tolerance: f64) -> bool {
    stochology_hypergraph::lenses::invariant_across_regimes(slopes, tolerance)
}

// ------------------------------------------------------------- the bench
fn valid_graph() -> Graph {
    Graph {
        nodes: vec![
            node(
                "L1_demand",
                Layer::L1,
                "call counts every 5 minutes of the real 2003 support desk",
            ),
            node(
                "L2_erlang_c",
                Layer::L2,
                "SL(c,lambda) = 1 - C(c,lambda*AHT)*exp(-(c-lambda*AHT)t/AHT), stationary, without abandonment",
            ),
            node(
                "L3_manager",
                Layer::L3,
                "staffing decided by Erlang C over the mean rate of the history",
            ),
        ],
        tree: vec![
            edge(
                "L2_erlang_c",
                "L3_manager",
                EdgeType::Deductive,
                "card that the decision uses",
            ),
            edge(
                "L1_demand",
                "L2_erlang_c",
                EdgeType::Deductive,
                "input of the model",
            ),
        ],
        graph: vec![
            edge(
                "L2_erlang_c",
                "L1_demand",
                EdgeType::Pathology,
                "Fallacy of the Mean: SL(E[L]) in place of E[SL(L)]",
            ),
            edge(
                "L3_manager",
                "L2_erlang_c",
                EdgeType::Pathology,
                "decision by the average day, without the tail of the days",
            ),
            edge(
                "L1_demand",
                "L2_erlang_c",
                EdgeType::Causal,
                "do(staffing) over the real demand",
            ),
            edge(
                "L2_erlang_c",
                "L3_manager",
                EdgeType::MathematicalBridge,
                "Jensen: E[f(X)] >= f(E[X])",
            ),
        ],
        hyperedges: vec![
            Hyperedge {
                id: "H1".to_string(),
                lens: Lens::Chrononomy,
                members: vec!["L1_demand".to_string(), "L2_erlang_c".to_string()],
            },
            Hyperedge {
                id: "H2".to_string(),
                lens: Lens::Scalonomy,
                members: vec!["L1_demand".to_string(), "L3_manager".to_string()],
            },
            Hyperedge {
                id: "H3".to_string(),
                lens: Lens::Crisonomy,
                members: vec![
                    "L1_demand".to_string(),
                    "L2_erlang_c".to_string(),
                    "L3_manager".to_string(),
                ],
            },
        ],
    }
}

/// A regime: its own base, the effect of the treatment and the chance of being treated.
struct Regime {
    p_treated: u64,
    base: f64,
    effect: f64,
    n: usize,
}

struct Measure {
    marginal: f64,
    treated_fraction: f64,
    by_regime: Vec<f64>,
}

/// Each regime receives n units with the requested treatment fraction and a uniform
/// residual in [-2, 2]. The marginal comparison mixes the regimes; the comparison
/// within the regime isolates the effect.
fn measure(seed: u64, regimes: &[Regime]) -> Measure {
    let mut rng = stochology_core::rng::offset_seed(seed);
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    let mut by_regime: Vec<f64> = Vec::new();
    for r in regimes {
        let n = r.n;
        let mut rx: Vec<f64> = Vec::new();
        let mut ry: Vec<f64> = Vec::new();
        for _ in 0..n {
            let t = if uniform(&mut rng, 100) < r.p_treated {
                1.0
            } else {
                0.0
            };
            let residual = (uniform(&mut rng, 4_001) as f64 - 2_000.0) / 1_000.0;
            let out = r.base + r.effect * t + residual;
            rx.push(t);
            ry.push(out);
            x.push(t);
            y.push(out);
        }
        by_regime.push(slope(&rx, &ry));
    }
    Measure {
        marginal: slope(&x, &y),
        treated_fraction: x.iter().sum::<f64>() / x.len() as f64,
        by_regime,
    }
}
pub fn run() {
    println!("== Chapter 15 :: the map of knowledge ==");

    let g = valid_graph();
    let order = g.topological_order().unwrap_or_default();
    let depth = g.depths();
    let leaves = depth.iter().filter(|(_, d)| *d == 1).count();
    let roots = depth.iter().filter(|(_, d)| *d > 1).count();
    println!(
        "graph: nodes={} (L1={} L2={} L3={}) E_tree={} E_graph={} hyperedges={} validation={:?}",
        g.nodes.len(),
        g.by_layer(Layer::L1),
        g.by_layer(Layer::L2),
        g.by_layer(Layer::L3),
        g.tree.len(),
        g.graph.len(),
        g.hyperedges.len(),
        g.validate()
    );
    println!(
        "topological order = {:?} | leaves={} roots={} | nodes without lens = {:?}",
        order,
        leaves,
        roots,
        g.nodes_without_lens()
    );

    let mut g1 = g.clone();
    g1.tree.push(edge(
        "L2_erlang_c",
        "L2_ghost_node",
        EdgeType::Deductive,
        "broken bridge",
    ));
    println!("proof 1 (edge to an absent node): {:?}", g1.validate());

    let mut g2 = g.clone();
    g2.tree.push(edge(
        "L3_manager",
        "L2_erlang_c",
        EdgeType::Deductive,
        "return",
    ));
    println!("proof 2 (deductive cycle): {:?}", g2.validate());

    let mut g3 = g.clone();
    g3.hyperedges.push(Hyperedge {
        id: "H4".to_string(),
        lens: Lens::Fluxonomy,
        members: vec!["L1_demand".to_string()],
    });
    println!("proof 3 (hyperedge with one member): {:?}", g3.validate());

    let mut g4 = g.clone();
    g4.nodes
        .push(node("L2_orphan", Layer::L2, "node that nobody looks at"));
    println!(
        "proof 4 (node outside every lens): {:?} validation={:?}",
        g4.nodes_without_lens(),
        g4.validate()
    );

    // Which lenses the graph triggers and which stay unused.
    let mut used: Vec<&str> = g.hyperedges.iter().map(|h| h.lens.name()).collect();
    used.sort();
    let not_triggered: Vec<&str> = Lens::ALL
        .iter()
        .map(|l| l.name())
        .filter(|n| !used.contains(n))
        .collect();
    println!(
        "lenses triggered = {:?} | not triggered = {:?}",
        used, not_triggered
    );

    // Lens 1 — Scalonomy.
    let mut rng = stochology_core::rng::offset_seed(1909);
    let series: Vec<f64> = (0..20_000)
        .map(|_| {
            let u = (splitmix64(&mut rng) as f64) / (u64::MAX as f64);
            (1.0 - u).powf(-1.0 / 1.81)
        })
        .collect();
    for k in [250usize, 500, 1_000, 2_000] {
        let a = hill_index(&series, k);
        println!(
            "Scalonomy: hill_alpha(k={}) = {:.4} wild_tail={} values_above_1000={}",
            k,
            a,
            wild_tail(a),
            series.iter().filter(|x| **x > 1_000.0).count()
        );
    }

    // Lens 2 — Crisonomy. EVPI = E_state[max_act payoff] - max_act E_state[payoff].
    // Until 2026-10-19 this block passed the payoffs of one act where the best payoff of each state
    // was due, and printed an EVPI of zero next to a value with information of 7 against 5.8.
    let probabilities = [0.70_f64, 0.30];
    let (profit, loss) = (10.0_f64, 4.0_f64);
    let full = [vec![profit, -loss], vec![0.0, 0.0]];
    let (without, with, value) = evpi_from_actions(&full, &probabilities);
    println!(
        "Crisonomy: act (+{profit:.0} in a good year, -{loss:.0} in a bad one) or not act: best without information = {without:.4} | with perfect information = {with:.4} | EVPI = {value:.4}"
    );
    let half = [vec![0.5 * profit, -0.5 * loss], vec![0.0, 0.0]];
    let (without_h, with_h, value_h) = evpi_from_actions(&half, &probabilities);
    println!(
        "Crisonomy: only half the stake available: best without information = {without_h:.4} | with perfect information = {with_h:.4} | EVPI = {value_h:.4}"
    );
    let flat = [vec![profit, profit], vec![0.0, 0.0]];
    println!(
        "Crisonomy: useless sensor (same payoff in both states): EVPI = {:.4}",
        evpi_from_actions(&flat, &probabilities).2
    );

    // Lens 3 — Morphonomy.
    // Regimes 1, 2 and 3 with bases 100, 140 and 120 and true effect 10 in all three.
    let regimes = [
        Regime {
            p_treated: 25,
            base: 100.0,
            effect: 10.0,
            n: 240,
        },
        Regime {
            p_treated: 75,
            base: 140.0,
            effect: 10.0,
            n: 240,
        },
        Regime {
            p_treated: 50,
            base: 120.0,
            effect: 10.0,
            n: 240,
        },
    ];
    let m = measure(2026, &regimes);
    let rounded = |v: &[f64]| {
        v.iter()
            .map(|x| (x * 100.0).round() / 100.0)
            .collect::<Vec<f64>>()
    };
    println!(
        "Morphonomy: rule 1 -> marginal slope = {:.2} (treated fraction = {:.3}) | by regime = {:?} | invariant(tolerance 0.5) = {}",
        m.marginal,
        m.treated_fraction,
        rounded(&m.by_regime),
        invariant(&m.by_regime, 0.5)
    );
    // Same true effect, swapped allocation: the marginal changes sign.
    let swapped_regimes = [
        Regime {
            p_treated: 75,
            base: 100.0,
            effect: 10.0,
            n: 240,
        },
        Regime {
            p_treated: 25,
            base: 140.0,
            effect: 10.0,
            n: 240,
        },
        Regime {
            p_treated: 50,
            base: 120.0,
            effect: 10.0,
            n: 240,
        },
    ];
    let t = measure(2026, &swapped_regimes);
    println!(
        "Morphonomy: rule 1 with the allocation swapped -> marginal slope = {:.2} | by regime = {:?} | invariant(tolerance 0.5) = {}",
        t.marginal,
        rounded(&t.by_regime),
        invariant(&t.by_regime, 0.5)
    );
    // An effect that breaks the invariance: the middle regime responds 40, not 10.
    let broken_regimes = [
        Regime {
            p_treated: 25,
            base: 100.0,
            effect: 10.0,
            n: 240,
        },
        Regime {
            p_treated: 75,
            base: 140.0,
            effect: 40.0,
            n: 240,
        },
        Regime {
            p_treated: 50,
            base: 120.0,
            effect: 10.0,
            n: 240,
        },
    ];
    let q = measure(2026, &broken_regimes);
    println!(
        "Morphonomy: rule 2, middle regime with effect 40 -> marginal slope = {:.2} | by regime = {:?} | invariant(tolerance 0.5) = {} | step within the regime = {:.2}",
        q.marginal,
        rounded(&q.by_regime),
        invariant(&q.by_regime, 0.5),
        q.by_regime[1] - q.by_regime[0]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_graph_passes_and_lenses_agree_with_hypergraph() {
        let g = valid_graph();
        assert!(g.validate().is_empty());
        assert!(g.topological_order().is_ok());
        let xs = [1.0, 2.0, 4.0, 8.0, 16.0, 32.0];
        assert_eq!(
            hill_index(&xs, 3),
            stochology_hypergraph::lenses::hill_index(&xs, 3)
        );
        assert_eq!(
            wild_tail(1.8),
            stochology_hypergraph::lenses::wild_tail(1.8)
        );
    }
}
