use crate::{
    VertexId,
    edges::{Edge, EdgeView, GraphEdgeStorage, GraphEdgeStorageDirected},
};

pub struct EdgeFilteredView<'a, S: GraphEdgeStorage> {
    edge_storage: &'a S,
    filter_edges: Option<fn(&S::Edge) -> bool>,
}

impl<'a, S: GraphEdgeStorage> EdgeFilteredView<'a, S> {
    pub(crate) fn new(edge_storage: &'a S, filter_edges: Option<fn(&S::Edge) -> bool>) -> Self {
        Self {
            edge_storage,
            filter_edges,
        }
    }
}

impl<'a, S: GraphEdgeStorage> GraphEdgeStorage for EdgeFilteredView<'a, S> {
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
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
    }

    fn count(&self) -> usize {
        self.edge_storage
            .get_all()
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
            .count()
    }

    fn of(&self, id: VertexId<S::Id>) -> impl Iterator<Item = EdgeView<'_, S::Edge, S::Id>> {
        self.edge_storage
            .of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
    }

    fn count_of(&self, id: VertexId<S::Id>) -> usize {
        self.edge_storage
            .of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
            .count()
    }

    fn add(&mut self, _from: VertexId<S::Id>, _to: VertexId<S::Id>, _data: S::Edge) {
        panic!("EdgeFilteredView cannot add edges");
    }

    fn remove(&mut self, _from: VertexId<S::Id>, _to: VertexId<S::Id>) -> Option<S::Edge> {
        panic!("EdgeFilteredView cannot remove edges");
    }

    fn get(&self, from: &VertexId<S::Id>, to: &VertexId<S::Id>) -> Option<&S::Edge> {
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

impl<'a, S: GraphEdgeStorageDirected> GraphEdgeStorageDirected for EdgeFilteredView<'a, S> {
    fn children_of(
        &self,
        id: VertexId<S::Id>,
    ) -> impl Iterator<Item = EdgeView<'_, S::Edge, S::Id>> {
        self.edge_storage
            .children_of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
    }

    fn parents_of(
        &self,
        id: VertexId<S::Id>,
    ) -> impl Iterator<Item = EdgeView<'_, S::Edge, S::Id>> {
        self.edge_storage
            .parents_of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
    }

    fn count_incoming(&self, id: VertexId<S::Id>) -> usize {
        self.edge_storage
            .parents_of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
            .count()
    }

    fn count_outgoing(&self, id: VertexId<S::Id>) -> usize {
        self.edge_storage
            .children_of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
            .count()
    }
}
