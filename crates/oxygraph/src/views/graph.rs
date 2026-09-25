use crate::{
    Graph, VertexId, edges::GraphEdgeStorage, graph::GraphView, views::edges::EdgeFilteredView,
};

pub struct GraphFilteredView<'a, V: 'static, S: GraphEdgeStorage> {
    graph: &'a Graph<V, S>,
    edges: EdgeFilteredView<'a, S>,
    filter_vertices: Option<fn(&V) -> bool>,
}

impl<'a, V, S: GraphEdgeStorage> GraphFilteredView<'a, V, S> {
    pub fn new(
        graph: &'a Graph<V, S>,
        filter_vertices: Option<fn(&V) -> bool>,
        filter_edges: Option<fn(&S::Edge) -> bool>,
    ) -> GraphFilteredView<'a, V, S> {
        Self {
            graph,
            edges: EdgeFilteredView::new(&graph.edge_storage, filter_edges),
            filter_vertices,
        }
    }
}

impl<'a, V: 'static, S: GraphEdgeStorage> GraphView<V, EdgeFilteredView<'a, S>>
    for GraphFilteredView<'a, V, S>
{
    fn vertex(&self, id: VertexId<S::Id>) -> Option<&V> {
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

    fn edges(&self) -> &EdgeFilteredView<'a, S> {
        &self.edges
    }
}
