use crate::traits::GraphDirectionality;

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
