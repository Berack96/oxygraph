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

#[serde_feature]
pub trait GraphEdgeStorage<E, I>
where
    I: Unsigned + PrimInt,
{
    type Directionality: GraphDirectionality;

    fn add_edge(&mut self, from: VertexId<I>, to: VertexId<I>, data: E);
    fn remove_edge(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E>;
    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool;
    fn edges_of<'a>(&'a self, id: &VertexId<I>) -> impl Iterator<Item = EdgeView<'a, E, I>>
    where
        E: 'a;
    fn edges<'a>(&'a self) -> impl Iterator<Item = EdgeView<'a, E, I>>
    where
        E: 'a;
}
