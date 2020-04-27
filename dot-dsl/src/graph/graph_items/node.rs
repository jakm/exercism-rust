use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(self: &Self, attrs: &[(&str, &str)]) -> Self {
        let mut node = self.clone();
        node.attrs = attrs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        node
    }

    pub fn get_attr(self: &Self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|s| s.as_str())
    }
}
