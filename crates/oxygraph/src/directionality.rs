use oxygraph_derive::serde_feature;

#[serde_feature]
pub trait GraphDirectionality {
    fn is_directed(&self) -> bool;
}

pub struct Directed;
impl GraphDirectionality for Directed {
    fn is_directed(&self) -> bool {
        true
    }
}

pub struct Undirected;
impl GraphDirectionality for Undirected {
    fn is_directed(&self) -> bool {
        false
    }
}
