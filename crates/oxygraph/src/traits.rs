use crate::ids::{Edge, VertexId};
use oxygraph_derive::base_model;

/// In teoria questa è quella giusta, ma manca da definire bene l'interfaccia e come indicare se è direzionato o meno.
/// Avevo in mente di passarlo come parametro di compilazione, in modo che anche le viste possano essere ottimizzate in base ad esso.
/// Non saprei come farlo però, dato che è da pensarci sopra.
#[base_model]
pub trait GraphEdgeStorage<E, D>
where
    D: GraphDirectionality,
{
    fn add_edge(&mut self, from: VertexId, to: VertexId, edge: E);
    fn remove_edge(&mut self, from: VertexId, to: VertexId) -> Option<E>;
    fn has_edge(&self, from: VertexId, to: VertexId) -> bool;
    fn edges_of<'a>(&'a self, id: &VertexId) -> impl Iterator<Item = &'a Edge<E>>
    where
        E: 'a;
    fn edges<'a>(&'a self) -> impl Iterator<Item = &'a Edge<E>>
    where
        E: 'a;
}

pub trait EdgeStorage<E>: Clone + std::fmt::Debug + std::marker::Sized
where
    E: Clone + std::fmt::Debug,
{
    fn add_to(&mut self, edge: Edge<E>);
    fn remove(&mut self, id: &VertexId) -> Option<E>;
    fn has_edge(&self, id: &VertexId) -> bool;
    fn get_all(&self) -> &[Edge<E>];
}

pub trait GraphView {
    type EdgeType;

    fn get_adjacent(&self, id: &VertexId) -> impl Iterator<Item = &Self::EdgeType>;
}
