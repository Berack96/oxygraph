//! Struct central to the library, representing a graph and its associated data structures.

use oxygraph_derive::serde_feature;
use std::marker::PhantomData;

use crate::{
    edges::GraphEdgeStorage,
    vertices::{UnsignedId, VertexId},
    views::GraphFilteredView,
};

pub trait GraphView<V: 'static, E: 'static, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    fn vertex(&self, id: VertexId<I>) -> Option<&V>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn edges(&self) -> &S;
}

#[serde_feature]
pub struct Graph<V: 'static, E: 'static, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    pub(crate) vertices: Vec<V>,
    pub(crate) edge_storage: S,
    _marker: PhantomData<(E, I)>,
}

impl<V, E, I, S> Graph<V, E, I, S>
where
    I: UnsignedId,
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

    pub fn edges_mut(&mut self) -> &mut S {
        &mut self.edge_storage
    }

    pub fn run_with<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Vec<V>, &mut S) -> R,
    {
        f(&mut self.vertices, &mut self.edge_storage)
    }

    pub fn get_filtered_view(
        &self,
        filter_vertices: Option<fn(&V) -> bool>,
        filter_edges: Option<fn(&E) -> bool>,
    ) -> GraphFilteredView<'_, V, E, I, S> {
        GraphFilteredView::new(self, filter_vertices, filter_edges)
    }
}

impl<V, E, I, S> GraphView<V, E, I, S> for Graph<V, E, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    fn len(&self) -> usize {
        self.vertices.len()
    }

    fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    fn vertex(&self, id: VertexId<I>) -> Option<&V> {
        self.vertices.get(id.id())
    }

    fn edges(&self) -> &S {
        &self.edge_storage
    }
}
