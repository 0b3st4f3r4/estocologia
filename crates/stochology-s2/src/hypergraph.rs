//! The System 2 world over the hypergraph: the search arrives where no lens covers.
//!
//! The goal of deliberation over the hypergraph is declared and measured: **to find the node that none of the
//! five lenses covers**, walking the edges of `E_tree` and `E_graph`. The value of a node is
//! `1 - coverage/5`, and the coverage counts **distinct lenses**, not repeated hyperedges.
//!
//! Declared and not implemented: the code synthesis that docs/PLAN.md §4 asks for in the same sentence (F1); and
//! the weighting by edge type (today `Causal` and `Pathology` weigh the same as `Deductive`).

use std::collections::BTreeMap;
use stochology_core::{Layer, UncertaintyType};
use stochology_hypergraph::pathology::PathologyKind;
use stochology_hypergraph::{EdgeKind, Graph, GraphEdge, Hyperedge, Lens, Node, TreeEdge};

use crate::lats::{Lats, World};

/// The separator between the type of the edge and the destination, in the declared action.
const SEPARATOR: char = '|';

pub fn action(edge_type: &str, destination: &str) -> String {
    format!("{edge_type}{SEPARATOR}{destination}")
}

pub fn action_destination(action: &str) -> &str {
    action
        .split_once(SEPARATOR)
        .map(|(_, d)| d)
        .unwrap_or(action)
}

fn type_name(edge_type: EdgeKind) -> &'static str {
    match edge_type {
        EdgeKind::Deductive => "deductive",
        EdgeKind::MathematicalBridge => "bridge",
        EdgeKind::Causal => "causal",
        EdgeKind::Pathology(_) => "pathology",
    }
}

/// The hypergraph as a world: states are nodes, actions are edges, value is lens coverage.
#[derive(Debug, Clone)]
pub struct HypergraphWorld<'a> {
    graph: &'a Graph,
    lenses: BTreeMap<String, Vec<Lens>>,
}

impl<'a> HypergraphWorld<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        let mut lenses: BTreeMap<String, Vec<Lens>> = graph
            .nodes
            .iter()
            .map(|n| (n.id.clone(), Vec::new()))
            .collect();
        for hyperedge in &graph.hyperedges {
            for member in &hyperedge.members {
                if let Some(list) = lenses.get_mut(member)
                    && !list.contains(&hyperedge.lens)
                {
                    list.push(hyperedge.lens);
                }
            }
        }
        HypergraphWorld { graph, lenses }
    }

    pub fn graph(&self) -> &Graph {
        self.graph
    }

    /// Distinct lenses that cover the node.
    pub fn lenses_of_node(&self, id: &str) -> Vec<Lens> {
        self.lenses.get(id).cloned().unwrap_or_default()
    }

    /// How many lenses cover the node; `None` for a node that does not exist in the graph.
    pub fn coverage(&self, id: &str) -> Option<usize> {
        self.lenses.get(id).map(Vec::len)
    }

    /// Nodes that no lens covers, in the declared order of the graph.
    pub fn uncovered_nodes(&self) -> Vec<&str> {
        self.graph
            .nodes
            .iter()
            .filter(|n| self.lens_count(&n.id) == 0)
            .map(|n| n.id.as_str())
            .collect()
    }

    fn lens_count(&self, id: &str) -> usize {
        self.coverage(id).unwrap_or(0)
    }

    /// Value of the state: how much of the node no lens covers. A node absent from the graph is worth zero.
    pub fn value(&self, id: &str) -> f64 {
        match self.coverage(id) {
            Some(count) => 1.0 - count as f64 / Lens::ALL.len() as f64,
            None => 0.0,
        }
    }

    /// The step-by-step search: repeats LATS from the chosen state, up to `steps` times.
    pub fn search(&self, root: &str, lats: &Lats, steps: usize) -> Option<Trail> {
        let mut current = root.to_string();
        let mut path = vec![root.to_string()];
        let mut value = self.value(root);
        for _ in 0..steps.max(1) {
            if self.terminal(&current) {
                break;
            }
            let decision = lats.deliberate(self, &current)?;
            current = action_destination(&decision.action).to_string();
            value = self.value(&current);
            path.push(current.clone());
        }
        Some(Trail {
            path,
            value,
            lens_free: self.coverage(&current) == Some(0),
        })
    }
}

/// The result of a search: the path, the value of the arrival node and whether it stayed without a lens.
#[derive(Debug, Clone, PartialEq)]
pub struct Trail {
    pub path: Vec<String>,
    pub value: f64,
    pub lens_free: bool,
}

impl World for HypergraphWorld<'_> {
    fn actions(&self, state: &str) -> Vec<String> {
        let mut output = Vec::new();
        for edge in &self.graph.tree_edges {
            if edge.from == state {
                output.push(action("tree", &edge.to));
            }
        }
        for edge in &self.graph.graph_edges {
            if edge.from == state {
                output.push(action(type_name(edge.kind), &edge.to));
            }
        }
        output
    }

    fn step(&self, _state: &str, action: &str) -> String {
        action_destination(action).to_string()
    }

    fn evaluate(&self, state: &str) -> f64 {
        self.value(state)
    }

    fn terminal(&self, state: &str) -> bool {
        match self.coverage(state) {
            Some(0) => true,
            None => true,
            Some(_) => self.actions(state).is_empty(),
        }
    }
}

/// The declared graph of the sample: the horizons, the R axis and the front that no lens covers.
///
/// It is a declared graph, not a measured one: the nodes are the fronts of the program, the `E_tree` edges are
/// what depends on what, and the hyperedges say which lens covers which front. The uncovered node is
/// `f1.code-synthesis` — the front that `docs/PLAN.md` §4 declares and that this crate does not implement. The
/// search of System 2 arrives there through the R axis, and not by shortcut.
pub fn sample_graph() -> Graph {
    let node = |id: &str, layer: Layer, uncertainty: UncertaintyType, definition: &str| Node {
        id: id.to_string(),
        layer,
        uncertainty,
        definition: definition.to_string(),
        artifact: Some(format!("crates/stochology-{id}")),
    };
    let tree_edge = |from: &str, to: &str| TreeEdge {
        from: from.to_string(),
        to: to.to_string(),
    };
    let graph_edge = |from: &str, to: &str, edge_type: EdgeKind| GraphEdge {
        from: from.to_string(),
        to: to.to_string(),
        kind: edge_type,
    };
    let hyperedge = |id: &str, lens: Lens, members: &[&str]| Hyperedge {
        id: id.to_string(),
        lens,
        members: members.iter().map(|m| (*m).to_string()).collect(),
    };
    Graph {
        nodes: vec![
            node(
                "root.stochology",
                Layer::Model,
                UncertaintyType::Epistemic,
                "the program",
            ),
            node(
                "h0.queue",
                Layer::World,
                UncertaintyType::Aleatory,
                "the queue and the call center",
            ),
            node(
                "h1.market",
                Layer::World,
                UncertaintyType::Aleatory,
                "market, employment and fire",
            ),
            node(
                "h2.invariance",
                Layer::Model,
                UncertaintyType::Epistemic,
                "the rounds of invariance",
            ),
            node(
                "h3.mesh",
                Layer::World,
                UncertaintyType::Epistemic,
                "the continuous mesh",
            ),
            node(
                "r.resolution",
                Layer::Model,
                UncertaintyType::Epistemic,
                "the resolution of the question",
            ),
            node(
                "s1.triage",
                Layer::Agent,
                UncertaintyType::Aleatory,
                "the deterministic triage",
            ),
            node(
                "f1.code-synthesis",
                Layer::Agent,
                UncertaintyType::Knightian,
                "deep code synthesis, declared and not implemented",
            ),
        ],
        tree_edges: vec![
            tree_edge("root.stochology", "h0.queue"),
            tree_edge("root.stochology", "h1.market"),
            tree_edge("root.stochology", "h2.invariance"),
            tree_edge("root.stochology", "h3.mesh"),
            tree_edge("root.stochology", "r.resolution"),
            tree_edge("r.resolution", "f1.code-synthesis"),
        ],
        graph_edges: vec![
            graph_edge("h3.mesh", "r.resolution", EdgeKind::Causal),
            // a fixed triage rule imposed on a queue whose load oscillates
            graph_edge(
                "s1.triage",
                "h0.queue",
                EdgeKind::Pathology(PathologyKind::FallacyOfTheStableWorld),
            ),
        ],
        hyperedges: vec![
            hyperedge(
                "lens.fluxonomy",
                Lens::Fluxonomy,
                &["h0.queue", "h1.market"],
            ),
            hyperedge(
                "lens.scalonomy",
                Lens::Scalonomy,
                &["h0.queue", "h1.market", "h2.invariance"],
            ),
            hyperedge(
                "lens.chrononomy",
                Lens::Chrononomy,
                &["h3.mesh", "r.resolution"],
            ),
            hyperedge(
                "lens.morphonomy",
                Lens::Morphonomy,
                &["root.stochology", "h2.invariance", "r.resolution"],
            ),
            hyperedge("lens.crisonomy", Lens::Crisonomy, &["s1.triage", "h3.mesh"]),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            layer: Layer::Model,
            uncertainty: UncertaintyType::Epistemic,
            definition: format!("node {id}"),
            artifact: None,
        }
    }

    fn test_graph() -> Graph {
        Graph {
            nodes: vec![
                node("root"),
                node("covered"),
                node("middle"),
                node("uncovered"),
            ],
            tree_edges: vec![
                TreeEdge {
                    from: "root".into(),
                    to: "covered".into(),
                },
                TreeEdge {
                    from: "root".into(),
                    to: "middle".into(),
                },
                TreeEdge {
                    from: "middle".into(),
                    to: "uncovered".into(),
                },
            ],
            graph_edges: vec![GraphEdge {
                from: "middle".into(),
                to: "uncovered".into(),
                kind: EdgeKind::Causal,
            }],
            hyperedges: vec![
                Hyperedge {
                    id: "h1".into(),
                    lens: Lens::Fluxonomy,
                    members: vec!["root".into(), "covered".into()],
                },
                Hyperedge {
                    id: "h2".into(),
                    lens: Lens::Chrononomy,
                    members: vec!["root".into(), "middle".into()],
                },
                Hyperedge {
                    id: "h3".into(),
                    lens: Lens::Scalonomy,
                    members: vec!["middle".into()],
                },
                Hyperedge {
                    id: "h4".into(),
                    lens: Lens::Morphonomy,
                    members: vec!["middle".into()],
                },
            ],
        }
    }

    #[test]
    fn coverage_counts_distinct_lenses() {
        let graph = test_graph();
        let world = HypergraphWorld::new(&graph);
        assert_eq!(world.coverage("root"), Some(2));
        assert_eq!(world.coverage("middle"), Some(3));
        assert_eq!(world.coverage("uncovered"), Some(0));
        assert_eq!(world.coverage("absent"), None);
        assert_eq!(world.lenses_of_node("middle").len(), 3);
        assert_eq!(world.uncovered_nodes(), vec!["uncovered"]);
        assert_eq!(world.value("uncovered"), 1.0);
        assert_eq!(world.value("root"), 0.6);
    }

    #[test]
    fn deliberation_finds_the_lens_free_node_where_the_greedy_agent_stops_earlier() {
        let graph = test_graph();
        let world = HypergraphWorld::new(&graph);
        // the greedy agent looks only at the neighbour: "covered" is worth 0.8 and "middle" is worth 0.4
        let greedy_agent = crate::lats::greedy(&world, "root").expect("decides");
        assert_eq!(action_destination(&greedy_agent.action), "covered");
        assert_eq!(greedy_agent.value, 0.8);
        // LATS looks two steps ahead: from "middle" there is only one way out, and it is worth 1.0
        let lats = Lats {
            depth: 2,
            budget: 40,
            exploration: 1.0,
            seed: 2026,
        };
        let search = world.search("root", &lats, 3).expect("search");
        assert_eq!(search.path, vec!["root", "middle", "uncovered"]);
        assert!(search.lens_free);
        assert_eq!(search.value, 1.0);
    }

    #[test]
    fn the_action_declares_the_type_of_the_edge() {
        let graph = test_graph();
        let world = HypergraphWorld::new(&graph);
        let actions = world.actions("root");
        assert!(actions.contains(&"tree|covered".to_string()));
        assert!(actions.contains(&"tree|middle".to_string()));
        let from_middle = world.actions("middle");
        assert!(from_middle.contains(&"tree|uncovered".to_string()));
        assert!(from_middle.contains(&"causal|uncovered".to_string()));
        assert_eq!(action_destination(&from_middle[0]), "uncovered");
        assert!(world.actions("uncovered").is_empty());
        assert!(world.terminal("uncovered"));
    }

    #[test]
    fn the_search_is_deterministic_in_the_test_graph() {
        let graph = test_graph();
        let world = HypergraphWorld::new(&graph);
        let lats = Lats::default();
        assert_eq!(
            world.search("root", &lats, 3),
            world.search("root", &lats, 3)
        );
    }

    #[test]
    fn the_sample_graph_is_valid_and_has_one_uncovered_node() {
        let graph = sample_graph();
        assert!(
            graph.validate().is_empty(),
            "the declared graph has to be valid: {:?}",
            graph.validate()
        );
        assert!(graph.topological_order().is_ok());
        assert_eq!(graph.nodes.len(), 8);
        assert_eq!(graph.hyperedges.len(), Lens::ALL.len());
        let world = HypergraphWorld::new(&graph);
        assert_eq!(world.uncovered_nodes(), vec!["f1.code-synthesis"]);
        assert_eq!(
            world.lenses_of_node("root.stochology"),
            vec![Lens::Morphonomy]
        );
    }

    #[test]
    fn the_search_in_the_sample_graph_reaches_the_lens_free_front() {
        let graph = sample_graph();
        let world = HypergraphWorld::new(&graph);
        let lats = Lats {
            depth: 2,
            budget: 40,
            exploration: 1.0,
            seed: 2026,
        };
        // the greedy agent ties on the five fronts of value 0.6 and stays on the first
        let greedy_agent = crate::lats::greedy(&world, "root.stochology").expect("decides");
        assert_eq!(action_destination(&greedy_agent.action), "h0.queue");
        assert!((greedy_agent.value - 0.6).abs() < 1e-12);
        // LATS sees that from r.resolution one reaches the front that no lens covers
        let search = world.search("root.stochology", &lats, 3).expect("search");
        assert_eq!(
            search.path,
            vec!["root.stochology", "r.resolution", "f1.code-synthesis"]
        );
        assert!(search.lens_free);
        assert_eq!(search.value, 1.0);
    }

    #[test]
    fn a_node_absent_from_the_graph_is_worth_nothing() {
        let graph = test_graph();
        let world = HypergraphWorld::new(&graph);
        assert_eq!(world.value("does-not-exist"), 0.0);
        assert!(world.terminal("does-not-exist"));
        assert!(world.lenses_of_node("does-not-exist").is_empty());
    }
}
