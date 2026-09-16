mod adj_list;
mod fixed;

pub use adj_list::AdjList;
pub use fixed::AdjListFixed;

use oxygraph_derive::serde_feature;

use crate::{
    directionality::GraphDirectionality,
    ids::{EdgeView, UnsignedId, VertexId},
};

pub trait GraphEdgeIter<E: 'static, I: UnsignedId> {
    fn edges_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>>;
    fn edges(&self) -> impl Iterator<Item = EdgeView<'_, E, I>>;
}

#[serde_feature]
pub trait GraphEdgeStorage<E: 'static, I: UnsignedId>: GraphEdgeIter<E, I> {
    type Directionality: GraphDirectionality;
    type Index<J: UnsignedId>: GraphEdgeStorage<E, J>;

    fn add_edge(&mut self, from: VertexId<I>, to: VertexId<I>, data: E);
    fn remove_edge(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E>;
    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool;
}
