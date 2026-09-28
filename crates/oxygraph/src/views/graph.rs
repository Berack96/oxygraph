use crate::{
    Graph, VertexId,
    edges::GraphEdgeStorage,
    graph::GraphView,
    views::{EdgeFilter, VertexFilter, edges::EdgeFilteredView},
};

pub struct GraphFilteredView<'a, V: 'static, S: GraphEdgeStorage> {
    graph: &'a Graph<V, S>,
    edges: EdgeFilteredView<'a, V, S>,
}

impl<'a, V: 'static, S: GraphEdgeStorage> GraphFilteredView<'a, V, S> {
    pub fn new(
        graph: &'a Graph<V, S>,
        filter_vertices: Option<VertexFilter<'a, V>>,
        filter_edges: Option<EdgeFilter<'a, S::Edge>>,
    ) -> GraphFilteredView<'a, V, S> {
        Self {
            graph,
            edges: EdgeFilteredView::new(
                &graph.edge_storage,
                filter_edges,
                &graph.vertices,
                filter_vertices,
            ),
        }
    }
}

impl<'a, V: 'static, S: GraphEdgeStorage> GraphView<V, EdgeFilteredView<'a, V, S>>
    for GraphFilteredView<'a, V, S>
{
    fn vertex(&self, id: VertexId<S::Id>) -> Option<&V> {
        self.graph
            .vertex(id)
            .filter(|_| self.edges.vertex_in_view(id))
    }

    fn len(&self) -> usize {
        self.ids().count()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn edges(&self) -> &EdgeFilteredView<'a, V, S> {
        &self.edges
    }

    fn ids(&self) -> impl Iterator<Item = VertexId<S::Id>> {
        (0..self.graph.vertices.len())
            .map(VertexId::new)
            .filter(move |&id| self.vertex(id).is_some())
    }
}
