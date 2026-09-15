//! Struct central to the library, representing a graph and its associated data structures.

use num_traits::{PrimInt, Unsigned};
use oxygraph_derive::serde_feature;
use std::{fmt::Debug, marker::PhantomData};

use crate::{ids::VertexId, storage::GraphEdgeStorage};

#[serde_feature]
pub struct Graph<V, E, I, S>
where
    I: Unsigned + PrimInt,
    S: GraphEdgeStorage<E, I>,
{
    vertices: Vec<V>,
    edge_storage: S,
    _marker: PhantomData<(E, I)>,
}

impl<V, E, I, S> Graph<V, E, I, S>
where
    I: Unsigned + PrimInt,
    S: GraphEdgeStorage<E, I>,
{
    pub(crate) fn new_with(vertices: Vec<V>, edge_storage: S) -> Self {
        Self {
            vertices,
            edge_storage,
            _marker: PhantomData,
        }
    }

    pub fn add_vertex(&mut self, vertex: V) -> VertexId<I> {
        self.vertices.push(vertex);
        VertexId::new(self.vertices.len() - 1)
    }

    pub fn get_vertex(&self, id: VertexId<I>) -> Option<&V> {
        self.vertices.get(id.id())
    }

    pub fn run_with<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Vec<V>, &mut S) -> R,
    {
        f(&mut self.vertices, &mut self.edge_storage)
    }
}

#[serde_feature]
pub struct GraphView<'a, V, E, I, S>
where
    I: Unsigned + PrimInt,
    S: GraphEdgeStorage<E, I>,
{
    vertices: &'a [V],
    edge_storage: &'a S,
    _marker: PhantomData<(E, I)>,
}
