use std::fmt;
use crate::ast::nodeid::NodeId;


#[derive(Debug, Clone, Eq)]
pub struct Identifier {
    pub name: String,
    pub node_id: NodeId,
}

impl Identifier {
    pub fn new(name: String, node_id: NodeId) -> Self {
        Self { name, node_id }
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::hash::Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl std::ops::Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}