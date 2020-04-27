pub mod graph_items;

use std::collections::HashMap;

use graph_items::edge::Edge;
use graph_items::node::Node;

#[derive(Clone, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(self: &Self, nodes: &[Node]) -> Self {
        let mut graph = self.clone();
        graph.nodes = Vec::from(nodes);
        graph
    }

    pub fn with_edges(self: &Self, edges: &[Edge]) -> Self {
        let mut graph = self.clone();
        graph.edges = Vec::from(edges);
        graph
    }

    pub fn with_attrs(self: &Self, attrs: &[(&str, &str)]) -> Self {
        let mut graph = self.clone();
        graph.attrs = attrs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        graph
    }

    pub fn get_node(self: &Self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|&n| n.name == name)
    }
}
