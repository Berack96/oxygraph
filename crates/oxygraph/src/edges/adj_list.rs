use num_traits::{PrimInt, Unsigned};
use oxygraph_derive::serde_feature;

use crate::{
    directionality::Directed,
    edges::{GraphEdgeIter, GraphEdgeStorage},
    ids::{EdgeView, VertexId},
};

struct Edge<E, I>
where
    I: Unsigned + PrimInt,
{
    to: VertexId<I>,
    data: E,
}

#[serde_feature]
pub struct AdjList<E, I: Unsigned + PrimInt> {
    edges: Vec<Vec<Edge<E, I>>>,
}

impl<E, I: Unsigned + PrimInt> AdjList<E, I> {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            edges: Vec::with_capacity(capacity),
        }
    }
}

impl<E: 'static, I> GraphEdgeStorage<E, I> for AdjList<E, I>
where
    I: Unsigned + PrimInt,
{
    type Directionality = Directed;
    type Index<J: Unsigned + PrimInt> = AdjList<E, J>;

    fn add_edge(&mut self, from: VertexId<I>, to: VertexId<I>, data: E) {
        todo!()
    }

    fn remove_edge(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E> {
        todo!()
    }

    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool {
        todo!()
    }
}

impl<E: 'static, I> GraphEdgeIter<E, I> for AdjList<E, I>
where
    I: Unsigned + PrimInt,
{
    fn edges_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        let vert_id = id.id();
        self.edges[vert_id].iter().filter_map(move |edge| {
            if edge.to == id {
                Some(EdgeView::new(id.clone(), edge.to, &edge.data))
            } else {
                None
            }
        })
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
