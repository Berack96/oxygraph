use std::marker::PhantomData;

use crate::{
    VertexId,
    edges::{Edge, EdgeView, GraphEdgeStorage, GraphEdgeStorageDirected},
    vertices::UnsignedId,
};

pub struct EdgeFilteredView<'a, E: 'static, I: 'a, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    edge_storage: &'a S,
    filter_edges: Option<fn(&E) -> bool>,
    _marker: PhantomData<I>,
}

impl<'a, E, I, S> EdgeFilteredView<'a, E, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    pub(crate) fn new(edge_storage: &'a S, filter_edges: Option<fn(&E) -> bool>) -> Self {
        Self {
            edge_storage,
            filter_edges,
            _marker: PhantomData,
        }
    }
}

impl<'a, E, I, S> GraphEdgeStorage<E, I> for EdgeFilteredView<'a, E, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    fn with_capacity(_capacity: usize) -> Self {
        panic!("EdgeFilteredView cannot be created with capacity");
    }

    fn with_edges(_edges: Vec<Edge<E, I>>) -> Self {
        panic!("EdgeFilteredView cannot be created with edges");
    }

    fn get_all(&self) -> impl Iterator<Item = EdgeView<'_, E, I>> {
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

    fn of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.edge_storage
            .of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
    }

    fn count_of(&self, id: VertexId<I>) -> usize {
        self.edge_storage
            .of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
            .count()
    }

    fn add(&mut self, _from: VertexId<I>, _to: VertexId<I>, _data: E) {
        panic!("EdgeFilteredView cannot add edges");
    }

    fn remove(&mut self, _from: VertexId<I>, _to: VertexId<I>) -> Option<E> {
        panic!("EdgeFilteredView cannot remove edges");
    }

    fn get(&self, from: &VertexId<I>, to: &VertexId<I>) -> Option<&E> {
        self.edge_storage
            .get(from, to)
            .filter(|data| self.filter_edges.is_none_or(|filter| filter(data)))
    }

    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool {
        self.get(from, to).is_some()
    }

    fn add_all(&mut self, _from: VertexId<I>, _edges: impl IntoIterator<Item = (VertexId<I>, E)>) {
        panic!("EdgeFilteredView cannot add edges");
    }

    fn remove_all(&mut self, _from: VertexId<I>) -> Vec<(VertexId<I>, E)> {
        panic!("EdgeFilteredView cannot remove edges");
    }

    fn is_directed(&self) -> bool {
        false
    }
}

impl<'a, E, I, S> GraphEdgeStorageDirected<E, I> for EdgeFilteredView<'a, E, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorageDirected<E, I>,
{
    fn children_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.edge_storage
            .children_of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
    }

    fn parents_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.edge_storage
            .parents_of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
    }

    fn count_incoming(&self, id: VertexId<I>) -> usize {
        self.edge_storage
            .parents_of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
            .count()
    }

    fn count_outgoing(&self, id: VertexId<I>) -> usize {
        self.edge_storage
            .children_of(id)
            .filter(|edge| self.filter_edges.is_none_or(|filter| filter(edge.data)))
            .count()
    }
}
