//! Typed hypergraph G = (V, E_tree, E_graph, H, Phi) and the five lenses.

pub mod lenses;
pub mod pathology;

use pathology::PathologyKind;
use stochology_core::{Layer, UncertaintyType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub layer: Layer,
    pub uncertainty: UncertaintyType,
    pub definition: String,
    pub artifact: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeEdge {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeKind {
    Deductive,
    MathematicalBridge,
    Causal,
    /// A structural misalignment between the Model and the World, typed by the pathology it marks.
    Pathology(PathologyKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub kind: EdgeKind,
}

/// The five operational lenses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lens {
    Fluxonomy,
    Chrononomy,
    Scalonomy,
    Morphonomy,
    Crisonomy,
}

impl Lens {
    pub const ALL: [Lens; 5] = [
        Lens::Fluxonomy,
        Lens::Chrononomy,
        Lens::Scalonomy,
        Lens::Morphonomy,
        Lens::Crisonomy,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Lens::Fluxonomy => "Fluxonomy",
            Lens::Chrononomy => "Chrononomy",
            Lens::Scalonomy => "Scalonomy",
            Lens::Morphonomy => "Morphonomy",
            Lens::Crisonomy => "Crisonomy",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hyperedge {
    pub id: String,
    pub lens: Lens,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub tree_edges: Vec<TreeEdge>,
    pub graph_edges: Vec<GraphEdge>,
    pub hyperedges: Vec<Hyperedge>,
}

impl Graph {
    /// Validates structure: unique ids, existing endpoints, cycle-free tree,
    /// hyperedges with at least two members.
    pub fn validate(&self) -> Vec<String> {
        let mut problems = Vec::new();
        let mut seen: Vec<&str> = Vec::new();
        for node in &self.nodes {
            if node.id.trim().is_empty() || node.definition.trim().is_empty() {
                problems.push(format!("incomplete node: {}", node.id));
            }
            if seen.contains(&node.id.as_str()) {
                problems.push(format!("duplicate node: {}", node.id));
            }
            seen.push(node.id.as_str());
        }

        let exists = |id: &str| self.nodes.iter().any(|n| n.id == id);

        for a in &self.tree_edges {
            if !exists(&a.from) || !exists(&a.to) {
                problems.push(format!("tree edge to missing node: {} -> {}", a.from, a.to));
            }
        }
        for a in &self.graph_edges {
            if !exists(&a.from) || !exists(&a.to) {
                problems.push(format!(
                    "graph edge to missing node: {} -> {}",
                    a.from, a.to
                ));
            }
        }
        for h in &self.hyperedges {
            if h.members.len() < 2 {
                problems.push(format!("hyperedge with fewer than two members: {}", h.id));
            }
            for m in &h.members {
                if !exists(m) {
                    problems.push(format!("hyperedge {} cites missing node: {}", h.id, m));
                }
            }
        }

        if let Some(cycle) = self.cycle_in_tree() {
            problems.push(format!("cycle in E_tree: {:?}", cycle));
        }

        problems
    }

    /// Topological ordering of the prerequisites; detects a deductive cycle.
    pub fn topological_order(&self) -> Result<Vec<String>, Vec<String>> {
        let mut order = Vec::new();
        let mut visiting: Vec<String> = Vec::new();
        for node in &self.nodes {
            if !self.dfs_ok(&node.id, &mut order, &mut visiting) {
                return Err(visiting);
            }
        }
        Ok(order)
    }

    fn dfs_ok(&self, id: &str, order: &mut Vec<String>, visiting: &mut Vec<String>) -> bool {
        if order.iter().any(|x| x == id) {
            return true;
        }
        if visiting.iter().any(|x| x == id) {
            return false;
        }
        visiting.push(id.to_string());
        for a in &self.tree_edges {
            if a.from == id && !self.dfs_ok(&a.to, order, visiting) {
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

    /// Coverage: every node must sit in at least one hyperedge (lens).
    pub fn nodes_without_lens(&self) -> Vec<String> {
        self.nodes
            .iter()
            .filter(|n| !self.hyperedges.iter().any(|h| h.members.contains(&n.id)))
            .map(|n| n.id.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Graph, Hyperedge, Lens, Node, TreeEdge};
    use stochology_core::{Layer, UncertaintyType};

    fn node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            layer: Layer::Model,
            uncertainty: UncertaintyType::Epistemic,
            definition: format!("short definition of {}", id),
            artifact: Some(format!("artifact/{}", id)),
        }
    }

    fn ok_graph() -> Graph {
        Graph {
            nodes: vec![node("jensen"), node("queue"), node("wait")],
            tree_edges: vec![TreeEdge {
                from: "jensen".to_string(),
                to: "wait".to_string(),
            }],
            graph_edges: vec![],
            hyperedges: vec![Hyperedge {
                id: "H1".to_string(),
                lens: Lens::Scalonomy,
                members: vec!["queue".to_string(), "wait".to_string()],
            }],
        }
    }

    #[test]
    fn complete_graph_validates() {
        assert!(ok_graph().validate().is_empty());
    }

    #[test]
    fn edge_to_missing_node_fails() {
        let mut g = ok_graph();
        g.tree_edges.push(TreeEdge {
            from: "queue".to_string(),
            to: "ghost-node".to_string(),
        });
        assert!(g.validate().iter().any(|p| p.contains("missing node")));
    }

    #[test]
    fn deductive_cycle_fails() {
        let mut g = ok_graph();
        g.tree_edges.push(TreeEdge {
            from: "wait".to_string(),
            to: "jensen".to_string(),
        });
        assert!(g.validate().iter().any(|p| p.contains("cycle")));
    }

    #[test]
    fn node_without_lens_is_flagged() {
        let g = ok_graph();
        assert_eq!(g.nodes_without_lens(), vec!["jensen".to_string()]);
    }
}
