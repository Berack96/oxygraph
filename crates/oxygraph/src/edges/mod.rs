mod adj_list;
mod fixed;

pub use adj_list::AdjList;
pub use fixed::AdjListFixed;

use oxygraph_derive::serde_feature;

use crate::vertices::{UnsignedId, VertexId};

/// No-op bound when `serde` is disabled, `Serialize + DeserializeOwned` when it's enabled.
/// A concrete storage's `impl GraphEdgeStorage` must satisfy `Edge`/`Id`'s own bound (see
/// below); Rust has no way to `#[cfg]` a single predicate inside one `where` clause, so this
/// lets that requirement be written once, unconditionally, instead of duplicating the impl.
#[cfg(feature = "serde")]
pub(crate) trait MaybeSerde: serde::Serialize + serde::de::DeserializeOwned {}
#[cfg(feature = "serde")]
impl<T: serde::Serialize + serde::de::DeserializeOwned> MaybeSerde for T {}

#[cfg(not(feature = "serde"))]
pub(crate) trait MaybeSerde {}
#[cfg(not(feature = "serde"))]
impl<T: ?Sized> MaybeSerde for T {}

/// Edge data usable as a Dijkstra edge weight.
/// `weight` must be non-negative: `Dijkstra` does not terminate on negative-weight cycles.
pub trait Weighted {
    fn weight(&self) -> f64;
}

#[serde_feature]
pub trait GraphEdgeStorage {
    type Edge: 'static;
    type Id: UnsignedId;

    fn with_capacity(capacity: usize) -> Self;
    fn with_edges(edges: Vec<Edge<Self::Edge, Self::Id>>) -> Self;

    fn get_all(&self) -> impl Iterator<Item = EdgeView<'_, Self::Edge, Self::Id>>;
    fn count(&self) -> usize;

    fn of(
        &self,
        id: VertexId<Self::Id>,
    ) -> impl Iterator<Item = EdgeView<'_, Self::Edge, Self::Id>>;
    fn count_of(&self, id: VertexId<Self::Id>) -> usize;

    fn add(&mut self, from: VertexId<Self::Id>, to: VertexId<Self::Id>, data: Self::Edge);
    fn remove(&mut self, from: VertexId<Self::Id>, to: VertexId<Self::Id>) -> Option<Self::Edge>;
    fn get(&self, from: &VertexId<Self::Id>, to: &VertexId<Self::Id>) -> Option<&Self::Edge>;
    fn has_edge(&self, from: &VertexId<Self::Id>, to: &VertexId<Self::Id>) -> bool;

    fn add_all(
        &mut self,
        from: VertexId<Self::Id>,
        edges: impl IntoIterator<Item = (VertexId<Self::Id>, Self::Edge)>,
    );
    fn remove_all(&mut self, from: VertexId<Self::Id>) -> Vec<(VertexId<Self::Id>, Self::Edge)>;
}

pub trait GraphEdgeStorageDirected: GraphEdgeStorage {
    fn children_of(
        &self,
        id: VertexId<Self::Id>,
    ) -> impl Iterator<Item = EdgeView<'_, Self::Edge, Self::Id>>;
    fn parents_of(
        &self,
        id: VertexId<Self::Id>,
    ) -> impl Iterator<Item = EdgeView<'_, Self::Edge, Self::Id>>;

    fn count_incoming(&self, id: VertexId<Self::Id>) -> usize;
    fn count_outgoing(&self, id: VertexId<Self::Id>) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde_feature]
pub struct Edge<E, I: UnsignedId> {
    pub from: VertexId<I>,
    pub to: VertexId<I>,
    pub data: E,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde_feature]
pub struct EdgeSimple<E, I: UnsignedId> {
    to: VertexId<I>,
    data: E,
}

// Not `#[serde_feature]`: `data` borrows (`&'a E`), and a borrowed view can't implement
// `Deserialize` (nothing to borrow from during deserialization). `Edge<E, I>` is the
// serializable, owned counterpart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeView<'a, E, I: UnsignedId> {
    pub from: VertexId<I>,
    pub to: VertexId<I>,
    pub data: &'a E,
}
impl<'a, E, I: UnsignedId> EdgeView<'a, E, I> {
    pub fn new(from: VertexId<I>, to: VertexId<I>, data: &'a E) -> Self {
        Self { from, to, data }
    }
}
