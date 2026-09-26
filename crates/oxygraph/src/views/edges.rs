use crate::{
    VertexId,
    edges::{Edge, EdgeView, GraphEdgeStorage, GraphEdgeStorageDirected},
};

pub struct EdgeFilteredView<'a, V: 'static, S: GraphEdgeStorage> {
    edge_storage: &'a S,
    filter_edges: Option<fn(&S::Edge) -> bool>,
    vertices: &'a [V],
    filter_vertices: Option<fn(&V) -> bool>,
}

impl<'a, V: 'static, S: GraphEdgeStorage> EdgeFilteredView<'a, V, S> {
    pub(crate) fn new(
        edge_storage: &'a S,
        filter_edges: Option<fn(&S::Edge) -> bool>,
        vertices: &'a [V],
        filter_vertices: Option<fn(&V) -> bool>,
    ) -> Self {
        Self {
            edge_storage,
            filter_edges,
            vertices,
            filter_vertices,
        }
    }

    /// Whether `id` names a vertex both present in `vertices` and accepted by
    /// `filter_vertices`: an edge reaching outside this is no more "in the view" than one
    /// `filter_edges` rejects.
    fn vertex_in_view(&self, id: VertexId<S::Id>) -> bool {
        self.vertices
            .get(id.id())
            .is_some_and(|v| self.filter_vertices.is_none_or(|filter| filter(v)))
    }

    fn edge_in_view(&self, edge: &EdgeView<'_, S::Edge, S::Id>) -> bool {
        self.filter_edges.is_none_or(|filter| filter(edge.data))
            && self.vertex_in_view(edge.from)
            && self.vertex_in_view(edge.to)
    }
}

impl<'a, V: 'static, S: GraphEdgeStorage> GraphEdgeStorage for EdgeFilteredView<'a, V, S> {
    type Edge = S::Edge;
    type Id = S::Id;

    fn with_capacity(_capacity: usize) -> Self {
        panic!("EdgeFilteredView cannot be created with capacity");
    }

    fn with_edges(_edges: Vec<Edge<S::Edge, S::Id>>) -> Self {
        panic!("EdgeFilteredView cannot be created with edges");
    }

    fn get_all(&self) -> impl Iterator<Item = EdgeView<'_, S::Edge, S::Id>> {
        self.edge_storage
            .get_all()
            .filter(|edge| self.edge_in_view(edge))
    }

    fn count(&self) -> usize {
        self.get_all().count()
    }

    fn of(&self, id: VertexId<S::Id>) -> impl Iterator<Item = EdgeView<'_, S::Edge, S::Id>> {
        self.edge_storage
            .of(id)
            .filter(|edge| self.edge_in_view(edge))
    }

    fn count_of(&self, id: VertexId<S::Id>) -> usize {
        self.of(id).count()
    }

    fn add_edge(&mut self, _from: VertexId<S::Id>, _to: VertexId<S::Id>, _data: S::Edge)
    where
        S::Edge: Clone,
    {
        panic!("EdgeFilteredView cannot add edges");
    }

    fn remove_edge(&mut self, _from: VertexId<S::Id>, _to: VertexId<S::Id>) -> Option<S::Edge> {
        panic!("EdgeFilteredView cannot remove edges");
    }

    fn get(&self, from: &VertexId<S::Id>, to: &VertexId<S::Id>) -> Option<&S::Edge> {
        if !self.vertex_in_view(*from) || !self.vertex_in_view(*to) {
            return None;
        }
        self.edge_storage
            .get(from, to)
            .filter(|data| self.filter_edges.is_none_or(|filter| filter(data)))
    }

    fn has_edge(&self, from: &VertexId<S::Id>, to: &VertexId<S::Id>) -> bool {
        self.get(from, to).is_some()
    }

    fn add_all(
        &mut self,
        _from: VertexId<S::Id>,
        _edges: impl IntoIterator<Item = (VertexId<S::Id>, S::Edge)>,
    ) {
        panic!("EdgeFilteredView cannot add edges");
    }

    fn remove_all(&mut self, _from: VertexId<S::Id>) -> Vec<(VertexId<S::Id>, S::Edge)> {
        panic!("EdgeFilteredView cannot remove edges");
    }
}

impl<'a, V: 'static, S: GraphEdgeStorageDirected> GraphEdgeStorageDirected
    for EdgeFilteredView<'a, V, S>
{
    fn children_of(
        &self,
        id: VertexId<S::Id>,
    ) -> impl Iterator<Item = EdgeView<'_, S::Edge, S::Id>> {
        self.edge_storage
            .children_of(id)
            .filter(|edge| self.edge_in_view(edge))
    }

    fn parents_of(
        &self,
        id: VertexId<S::Id>,
    ) -> impl Iterator<Item = EdgeView<'_, S::Edge, S::Id>> {
        self.edge_storage
            .parents_of(id)
            .filter(|edge| self.edge_in_view(edge))
    }

    fn count_incoming(&self, id: VertexId<S::Id>) -> usize {
        self.parents_of(id).count()
    }

    fn count_outgoing(&self, id: VertexId<S::Id>) -> usize {
        self.children_of(id).count()
    }

    fn add_edge_directed(&mut self, _from: VertexId<S::Id>, _to: VertexId<S::Id>, _data: S::Edge) {
        panic!("EdgeFilteredView cannot add edges");
    }

    fn remove_edge_directed(
        &mut self,
        _from: VertexId<S::Id>,
        _to: VertexId<S::Id>,
    ) -> Option<S::Edge> {
        panic!("EdgeFilteredView cannot remove edges");
    }
}
