use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Node {
    pub name: String,
}

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Property {
    pub name: String,
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chronicle {
    pub history: Vec<State>,
    pub nodes: Vec<Node>,
    pub edges: Vec<(Node, Node)>,
}
