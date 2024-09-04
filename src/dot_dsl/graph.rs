
use std::collections::HashMap;

use super::graph_items::{edge::Edge, node::Node};

pub fn add_attrs(attributes: &mut HashMap<String, String>, attrs: &[(&str, &str)]) {
    for (k, v) in attrs {
        attributes.insert(k.to_string(), v.to_string());
    }
}

pub fn get_attr<'a>(atributes: &'a HashMap<String, String>, name: &str) -> Option<&'a str> {
    let v = atributes.get(name);
    if v == None { None }
    else { Some(v.unwrap().as_str()) }
}

#[derive(PartialEq, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new()
        }
    }

    pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
        self.nodes.extend(nodes.iter().cloned());
        self
    }

    pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
        self.edges.extend(edges.iter().cloned());
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        add_attrs(&mut self.attrs, attrs);
        self
    }

    pub fn node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.name() == name)
    }
}