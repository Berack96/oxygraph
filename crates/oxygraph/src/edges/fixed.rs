use num_traits::{PrimInt, Unsigned};
use oxygraph_derive::serde_feature;

use crate::{
    directionality::Directed,
    edges::{GraphEdgeIter, GraphEdgeStorage},
    ids::{EdgeView, VertexId},
};

#[serde_feature]
pub struct Edge<E, I>
where
    I: Unsigned + PrimInt,
{
    to: VertexId<I>,
    data: E,
}

#[serde_feature]
pub struct AdjListFixed<E, I, const N: usize>
where
    I: Unsigned + PrimInt,
{
    edges: Vec<[Edge<E, I>; N]>,
}
impl<E, I, const N: usize> AdjListFixed<E, I, N>
where
    I: Unsigned + PrimInt,
{
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            edges: Vec::with_capacity(capacity),
        }
    }
}

impl<E: 'static, I, const N: usize> GraphEdgeStorage<E, I> for AdjListFixed<E, I, N>
where
    I: Unsigned + PrimInt,
{
    type Directionality = Directed;

    fn add_edge(&mut self, from: VertexId<I>, to: VertexId<I>, edge: E) {}

    fn remove_edge(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E> {
        todo!()
    }

    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool {
        todo!()
    }
}

impl<E: 'static, I, const N: usize> GraphEdgeIter<E, I> for AdjListFixed<E, I, N>
where
    I: Unsigned + PrimInt,
{
    fn edges_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        let vert_id = id.id();
        self.edges[vert_id]
            .iter()
            .map(move |edge| EdgeView::new(id.clone(), edge.to, &edge.data))
    }

    fn edges(&self) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.edges.iter().enumerate().flat_map(|(from_id, edges)| {
            let from_vertex = VertexId::new(from_id);
            edges
                .iter()
                .map(move |edge| EdgeView::new(from_vertex.clone(), edge.to, &edge.data))
        })
    }
}
