pub mod edge {
    use std::collections::HashMap;
    use crate::dot_dsl::graph::{add_attrs, get_attr};
    
    #[derive(PartialEq, Debug, Clone)]
    pub struct Edge {
        node1: String,
        node2: String,
        attrs: HashMap<String, String>
    }

    impl Edge {
        pub fn new(node1: &str, node2: &str) -> Self {
            Edge {
                node1: node1.into(),
                node2: node2.into(),
                attrs: HashMap::new()
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            add_attrs(&mut self.attrs, attrs);
            self
        }
    
        pub fn attr(&self, name: &str) -> Option<&str> {
            get_attr(&self.attrs, name)
        }
    }
}

pub mod node {
    use std::collections::HashMap;
    use crate::dot_dsl::graph::{add_attrs, get_attr};

    #[derive(Eq, PartialEq, Debug, Clone)]
    pub struct Node {
        pub name: String,
        attrs: HashMap<String, String>
    }

    impl Node {
        pub fn new(name: &str) -> Self {
            Node {
                name: name.into(),
                attrs: HashMap::new()
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            add_attrs(&mut self.attrs, attrs);
            self
        }
    
        pub fn attr(&self, name: &str) -> Option<&str> {
            get_attr(&self.attrs, name)
        }   
    }
}