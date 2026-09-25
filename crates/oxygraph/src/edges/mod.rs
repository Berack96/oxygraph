mod adj_list;
mod fixed;

pub use adj_list::AdjList;
pub use fixed::AdjListFixed;

use oxygraph_derive::serde_feature;

use crate::vertices::{UnsignedId, VertexId};

/// Edge data usable as a Dijkstra edge weight.
/// `weight` must be non-negative: `Dijkstra` does not terminate on negative-weight cycles.
pub trait Weighted {
    fn weight(&self) -> f64;
}

#[serde_feature]
pub trait GraphEdgeStorage<E: 'static, I: UnsignedId> {
    fn with_capacity(capacity: usize) -> Self;
    fn with_edges(edges: Vec<Edge<E, I>>) -> Self;

    fn get_all(&self) -> impl Iterator<Item = EdgeView<'_, E, I>>;
    fn count(&self) -> usize;

    fn of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>>;
    fn count_of(&self, id: VertexId<I>) -> usize;

    fn add(&mut self, from: VertexId<I>, to: VertexId<I>, data: E);
    fn remove(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E>;
    fn get(&self, from: &VertexId<I>, to: &VertexId<I>) -> Option<&E>;
    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool;

    fn add_all(&mut self, from: VertexId<I>, edges: impl IntoIterator<Item = (VertexId<I>, E)>);
    fn remove_all(&mut self, from: VertexId<I>) -> Vec<(VertexId<I>, E)>;

    fn is_directed(&self) -> bool {
        false
    }
}

pub trait GraphEdgeStorageDirected<E: 'static, I: UnsignedId>: GraphEdgeStorage<E, I> {
    fn children_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>>;
    fn parents_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>>;

    fn count_incoming(&self, id: VertexId<I>) -> usize;
    fn count_outgoing(&self, id: VertexId<I>) -> usize;

    fn is_directed(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde_feature]
pub struct Edge<E, I: UnsignedId> {
    pub from: VertexId<I>,
    pub to: VertexId<I>,
    pub data: E,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde_feature]
pub struct EdgeSimple<E, I: UnsignedId> {
    to: VertexId<I>,
    data: E,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde_feature]
pub struct EdgeView<'a, E, I: UnsignedId> {
    pub from: VertexId<I>,
    pub to: VertexId<I>,
    pub data: &'a E,
}
impl<'a, E, I: UnsignedId> EdgeView<'a, E, I> {
    pub fn new(from: VertexId<I>, to: VertexId<I>, data: &'a E) -> Self {
        Self { from, to, data }
    }
}
