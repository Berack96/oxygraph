//! Struct central to the library, representing a graph and its associated data structures.

use oxygraph_derive::serde_feature;
use std::marker::PhantomData;

use crate::{
    EdgeView,
    edges::{GraphEdgeIter, GraphEdgeStorage},
    ids::{UnsignedId, VertexId},
};

#[serde_feature]
pub struct Graph<V: 'static, E: 'static, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    vertices: Vec<V>,
    edge_storage: S,
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
pub struct GraphFilters<V: 'static, E: 'static> {
    vertices: Option<fn(&V) -> bool>,
    edges: Option<fn(&E) -> bool>,
}

#[serde_feature]
pub struct GraphView<'a, V: 'static, E: 'static, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    graph: &'a Graph<V, E, I, S>,
    filters: GraphFilters<V, E>,
    _marker: PhantomData<(E, I)>,
}

impl<'a, V, E, I, S> GraphView<'a, V, E, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    pub fn new(graph: &'a Graph<V, E, I, S>) -> GraphView<'a, V, E, I, S> {
        Self {
            graph,
            filters: GraphFilters {
                vertices: None,
                edges: None,
            },
            _marker: PhantomData,
        }
    }

    pub fn get_vertex(&self, id: VertexId<I>) -> Option<&V> {
        let vertex = self.graph.vertices.get(id.id());
        if let Some(v) = vertex {
            self.filters
                .vertices
                .is_none_or(|filter| filter(v))
                .then_some(v)
        } else {
            None
        }
    }
}

impl<V: 'static, E: 'static, I, S> GraphEdgeIter<E, I> for GraphView<'_, V, E, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    fn edges_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        let vertex = self.graph.vertices.get(id.id());
        let edges = self.graph.edge_storage.edges_of(id);
        let filter_edg = self.filters.edges;
        let filter_ver = self.filters.vertices;

        edges
            .filter(move |_| !vertex.is_none())
            .filter(move |e| match filter_ver {
                Some(f) => self.graph.get_vertex(e.to).is_some_and(f),
                None => true,
            })
            .filter(move |e| match filter_edg {
                Some(f) => f(e.data),
                None => true,
            })
    }

    fn edges(&self) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.graph
            .vertices
            .iter()
            .enumerate()
            .filter_map(move |(i, _)| self.graph.edge_storage.edges_of(VertexId::new(i)).next())
    }
}
