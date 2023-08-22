use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Node {
    pub name: String,
}

impl From<String> for Node {
    fn from(name: String) -> Self {
        Node { name }
    }
}

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Property {
    pub name: String,
}

impl From<String> for Property {
    fn from(name: String) -> Self {
        Property { name }
    }
}

/// The system state at a given point in time
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    pub temperature: f32,
    pub energy: f32,
    pub route: Vec<Node>,
    pub used: bool,
    // #[serde(default)]
    // pub meta: HashMap<String, String>,
    // #[serde(default)]
    // pub properties: HashMap<Node, Vec<Property>>,
}

/// The system state over the runtime of the application
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chronicle {
    pub history: Vec<State>,
    pub nodes: Vec<Node>,
    pub edges: Vec<(Node, Node)>,
    /// The node positions, mapped to a `[-1, 1]` coordinate plane with `(0, 0)` at the center of the screen
    #[serde_as(as = "Vec<(_, _)>")]
    pub positions: HashMap<Node, (f32, f32)>,
}

impl Chronicle {
    /// Returns the position of the given node, otherwise `(0.0, 0.0)` if it has no associated position data
    pub(crate) fn position<'a>(&'a self, node: &Node) -> &'a (f32, f32) {
        self.positions.get(node).unwrap_or(&(0.0, 0.0))
    }
}
