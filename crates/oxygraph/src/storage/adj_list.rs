use num_traits::{PrimInt, Unsigned};
use oxygraph_derive::serde_feature;

use crate::{
    directionality::Directed,
    ids::{EdgeView, VertexId},
    storage::GraphEdgeStorage,
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

impl<E, I> GraphEdgeStorage<E, I> for AdjList<E, I>
where
    I: Unsigned + PrimInt,
{
    type Directionality = Directed;

    fn add_edge(&mut self, from: VertexId<I>, to: VertexId<I>, data: E) {
        todo!()
    }

    fn remove_edge(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E> {
        todo!()
    }

    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool {
        todo!()
    }

    fn edges_of<'a>(&'a self, id: &VertexId<I>) -> impl Iterator<Item = EdgeView<'a, E, I>>
    where
        E: 'a,
    {
        let vert_id = id.id();
        self.edges[vert_id].iter().filter_map(move |edge| {
            if edge.to == *id {
                Some(EdgeView::new(id.clone(), edge.to, &edge.data))
            } else {
                None
            }
        })
    }

    fn edges<'a>(&'a self) -> impl Iterator<Item = EdgeView<'a, E, I>>
    where
        E: 'a,
    {
        self.edges.iter().enumerate().flat_map(|(from_id, edges)| {
            let from_vertex = VertexId::new(from_id);
            edges
                .iter()
                .map(move |edge| EdgeView::new(from_vertex.clone(), edge.to, &edge.data))
        })
    }
}
