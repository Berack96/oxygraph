//! Struct central to the library, representing a graph and its associated data structures.

use oxygraph_derive::serde_feature;

use crate::{edges::GraphEdgeStorage, vertices::VertexId, views::GraphFilteredView};

pub trait GraphView<V: 'static, S: GraphEdgeStorage> {
    fn vertex(&self, id: VertexId<S::Id>) -> Option<&V>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn edges(&self) -> &S;

    /// Ids of every vertex the view exposes, i.e. exactly those for which
    /// [`vertex`](Self::vertex) returns `Some`. Used by whole-graph visits (connected
    /// components, SCC, ...) that need to enumerate vertices rather than start from one.
    fn ids(&self) -> impl Iterator<Item = VertexId<S::Id>>;
}

#[serde_feature]
pub struct Graph<V: 'static, S: GraphEdgeStorage> {
    pub(crate) vertices: Vec<V>,
    pub(crate) edge_storage: S,
}

impl<V, S: GraphEdgeStorage> Graph<V, S> {
    pub(crate) fn new_with(vertices: Vec<V>, edge_storage: S) -> Self {
        Self {
            vertices,
            edge_storage,
        }
    }

    pub fn add_vertex(&mut self, vertex: V) -> VertexId<S::Id> {
        self.vertices.push(vertex);
        VertexId::new(self.vertices.len() - 1)
    }

    pub fn edges_mut(&mut self) -> &mut S {
        &mut self.edge_storage
    }

    pub fn vertex_mut(&mut self, id: VertexId<S::Id>) -> Option<&mut V> {
        self.vertices.get_mut(id.id())
    }

    pub fn get_view(&self) -> GraphFilteredView<'_, V, S> {
        self.get_filtered_view(None, None)
    }

    pub fn get_filtered_view(
        &self,
        filter_vertices: Option<fn(&V) -> bool>,
        filter_edges: Option<fn(&S::Edge) -> bool>,
    ) -> GraphFilteredView<'_, V, S> {
        GraphFilteredView::new(self, filter_vertices, filter_edges)
    }
}

impl<V, S: GraphEdgeStorage> GraphView<V, S> for Graph<V, S> {
    fn len(&self) -> usize {
        self.vertices.len()
    }

    fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    fn vertex(&self, id: VertexId<S::Id>) -> Option<&V> {
        self.vertices.get(id.id())
    }

    fn edges(&self) -> &S {
        &self.edge_storage
    }

    fn ids(&self) -> impl Iterator<Item = VertexId<S::Id>> {
        (0..self.vertices.len()).map(VertexId::new)
    }
}
