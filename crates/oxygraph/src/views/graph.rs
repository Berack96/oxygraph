use std::marker::PhantomData;

use oxygraph_derive::serde_feature;

use crate::{
    Graph, VertexId, edges::GraphEdgeStorage, graph::GraphView, vertices::UnsignedId,
    views::edges::EdgeFilteredView,
};

#[serde_feature]
pub struct GraphFilteredView<'a, V: 'static, E: 'static, I: 'a, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    graph: &'a Graph<V, E, I, S>,
    edges: EdgeFilteredView<'a, E, I, S>,
    filter_vertices: Option<fn(&V) -> bool>,
    _marker: PhantomData<(E, I)>,
}

impl<'a, V, E, I, S> GraphFilteredView<'a, V, E, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    pub fn new(
        graph: &'a Graph<V, E, I, S>,
        filter_vertices: Option<fn(&V) -> bool>,
        filter_edges: Option<fn(&E) -> bool>,
    ) -> GraphFilteredView<'a, V, E, I, S> {
        Self {
            graph,
            edges: EdgeFilteredView::new(&graph.edge_storage, filter_edges),
            filter_vertices,
            _marker: PhantomData,
        }
    }
}

impl<'a, V, E, I, S> GraphView<V, E, I, EdgeFilteredView<'a, E, I, S>>
    for GraphFilteredView<'a, V, E, I, S>
where
    V: 'static,
    E: 'static,
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    fn vertex(&self, id: VertexId<I>) -> Option<&V> {
        let vertex = self.graph.vertex(id);
        if let Some(v) = vertex {
            self.filter_vertices
                .is_none_or(|filter| filter(v))
                .then_some(v)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.graph
            .vertices
            .iter()
            .filter(|v| self.filter_vertices.is_none_or(|filter| filter(v)))
            .count()
    }

    fn is_empty(&self) -> bool {
        !self
            .graph
            .vertices
            .iter()
            .any(|v| self.filter_vertices.is_some_and(|filter| !filter(v)))
    }

    fn edges(&self) -> &EdgeFilteredView<'a, E, I, S> {
        &self.edges
    }
}
