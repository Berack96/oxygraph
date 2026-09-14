use crate::traits::GraphDirectionality;

#[derive(Default, Clone, Debug)]
pub struct Directed;
impl GraphDirectionality for Directed {
    fn is_directed(&self) -> bool {
        true
    }
}

#[derive(Default, Clone, Debug)]
pub struct Undirected;
impl GraphDirectionality for Undirected {
    fn is_directed(&self) -> bool {
        false
    }
}
