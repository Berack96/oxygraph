use crate::ids::{Edge, VertexId};
use oxygraph_derive::serde_feature;

#[serde_feature]
pub trait GraphDirectionality {
    fn is_directed(&self) -> bool;
}

#[serde_feature]
pub trait GraphEdgeStorage<E, D>
where
    D: GraphDirectionality,
{
    fn add_edge(&mut self, from: VertexId, to: VertexId, edge: E);
    fn remove_edge(&mut self, from: VertexId, to: VertexId) -> Option<E>;
    fn has_edge(&self, from: VertexId, to: VertexId) -> bool;
    fn edges_of<'a>(&'a self, id: &VertexId) -> impl Iterator<Item = &'a Edge<E>>
    where
        E: 'a;
    fn edges<'a>(&'a self) -> impl Iterator<Item = &'a Edge<E>>
    where
        E: 'a;
}

pub trait GraphView {
    type EdgeType;

    fn get_adjacent(&self, id: &VertexId) -> impl Iterator<Item = &Self::EdgeType>;
}
