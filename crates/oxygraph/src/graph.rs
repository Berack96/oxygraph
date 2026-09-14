//! Struct central to the library, representing a graph and its associated data structures.

use std::{fmt::Debug, marker::PhantomData};

use crate::{ids::VertexId, traits::EdgeStorage};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Graph<V, E, S>
where
    V: Clone + Debug,
    E: Clone + Debug,
    S: EdgeStorage<E>,
{
    vertices: Vec<V>,
    edges: Vec<S>,
    _marker: PhantomData<E>,
}

impl<V, E, S> Graph<V, E, S>
where
    V: Clone + Debug,
    E: Clone + Debug,
    S: Clone + Debug + EdgeStorage<E>,
{
    pub(crate) fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(capacity),
            edges: Vec::with_capacity(capacity),
            _marker: PhantomData,
        }
    }

    pub fn add_vertex(&mut self, vertex: V) -> VertexId {
        self.vertices.push(vertex);
        VertexId(self.vertices.len() - 1)
    }

    pub fn get_vertex(&self, id: usize) -> Option<&V> {
        self.vertices.get(id)
    }

    pub fn get_edges(&self, id: usize) -> Option<&S> {
        self.edges.get(id)
    }
}
