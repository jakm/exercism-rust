use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    node1: String,
    node2: String,
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(node1: &str, node2: &str) -> Self {
        Edge {
            node1: node1.to_string(),
            node2: node2.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(self: &Self, attrs: &[(&str, &str)]) -> Self {
        let mut edge = self.clone();
        edge.attrs = attrs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        edge
    }
}
