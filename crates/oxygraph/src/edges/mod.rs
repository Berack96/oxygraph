mod adj_list;
mod fixed;

pub use adj_list::AdjList;
pub use fixed::AdjListFixed;

use num_traits::{PrimInt, Unsigned};
use oxygraph_derive::serde_feature;

use crate::{
    directionality::GraphDirectionality,
    ids::{EdgeView, VertexId},
};

pub trait GraphEdgeIter<E: 'static, I>
where
    I: Unsigned + PrimInt,
{
    fn edges_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>>;
    fn edges(&self) -> impl Iterator<Item = EdgeView<'_, E, I>>;
}

#[serde_feature]
pub trait GraphEdgeStorage<E: 'static, I>: GraphEdgeIter<E, I>
where
    I: Unsigned + PrimInt,
{
    type Directionality: GraphDirectionality;

    fn add_edge(&mut self, from: VertexId<I>, to: VertexId<I>, data: E);
    fn remove_edge(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E>;
    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool;
}
