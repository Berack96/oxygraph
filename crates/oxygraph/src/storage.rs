use oxygraph_derive::serde_feature;

use crate::{
    directionality::Directed,
    ids::{Edge, VertexId},
    traits::GraphEdgeStorage,
};

#[serde_feature]
pub struct Array<E, const N: usize> {
    edges: Vec<[Edge<E>; N]>,
}

impl<E, const N: usize> GraphEdgeStorage<E, Directed> for Array<E, N> {
    fn add_edge(&mut self, from: VertexId, to: VertexId, edge: E) {
        todo!()
    }

    fn remove_edge(&mut self, from: VertexId, to: VertexId) -> Option<E> {
        todo!()
    }

    fn has_edge(&self, from: VertexId, to: VertexId) -> bool {
        todo!()
    }

    fn edges_of<'a>(&'a self, id: &VertexId) -> impl Iterator<Item = &'a Edge<E>>
    where
        E: 'a,
    {
        todo!()
    }

    fn edges<'a>(&'a self) -> impl Iterator<Item = &'a Edge<E>>
    where
        E: 'a,
    {
        todo!()
    }
}

#[serde_feature]
pub struct List<E> {
    edges: Vec<E>,
}
