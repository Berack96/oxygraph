mod adj_list;
mod csr;
mod fixed;
mod matrix;

pub use adj_list::AdjList;
pub use csr::AdjCsr;
pub use fixed::AdjListFixed;
pub use matrix::AdjMatrix;

#[cfg(feature = "serde")]
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

/// Contract every edge storage must satisfy, independent of whether it is also capable of
/// directed navigation ([`GraphEdgeStorageDirected`]): every write here is undirected.
/// `add_edge`/`remove_edge` always act on both endpoints, so generic code written against
/// this trait alone can never end up with an asymmetric edge by accident — only
/// [`GraphEdgeStorageDirected::add_edge_directed`] can create one, on a storage that opts
/// into it. `with_edges`/`add_all`/`remove_all` are the literal, non-mirrored bulk-load
/// counterparts (one arc per tuple, exactly as given).
#[cfg_attr(feature = "serde", serde_feature)]
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

    /// Inserts an undirected edge: stored on both endpoints, so it is reachable from either
    /// side regardless of navigation direction. A self-loop (`from == to`) is stored once.
    /// Requires `Self::Edge: Clone` to populate both sides; storages whose edge data isn't
    /// `Clone` can still be built one direction at a time via
    /// [`GraphEdgeStorageDirected::add_edge_directed`].
    fn add_edge(&mut self, from: VertexId<Self::Id>, to: VertexId<Self::Id>, data: Self::Edge)
    where
        Self::Edge: Clone;
    /// Removes an undirected edge, the mirror of [`add_edge`](Self::add_edge): clears both
    /// endpoints (best-effort on each side) and returns the data removed from the
    /// `from -> to` side.
    fn remove_edge(
        &mut self,
        from: VertexId<Self::Id>,
        to: VertexId<Self::Id>,
    ) -> Option<Self::Edge>;
    fn get(&self, from: &VertexId<Self::Id>, to: &VertexId<Self::Id>) -> Option<&Self::Edge>;
    fn has_edge(&self, from: &VertexId<Self::Id>, to: &VertexId<Self::Id>) -> bool;

    fn add_all(
        &mut self,
        from: VertexId<Self::Id>,
        edges: impl IntoIterator<Item = (VertexId<Self::Id>, Self::Edge)>,
    );
    fn remove_all(&mut self, from: VertexId<Self::Id>) -> Vec<(VertexId<Self::Id>, Self::Edge)>;
}

/// Extends [`GraphEdgeStorage`] with direction-aware navigation and single-direction writes.
/// Implementing this trait is an opt-in promise that `add_edge_directed`/`remove_edge_directed`
/// touch only the given direction, while the inherited `add_edge`/`remove_edge` must still
/// keep behaving as fully undirected (double arc in, double arc out) — a storage cannot use
/// its directed capability to cut corners on the base trait's contract.
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

    /// Inserts a single directed arc `from -> to`; the reverse direction is left untouched.
    fn add_edge_directed(
        &mut self,
        from: VertexId<Self::Id>,
        to: VertexId<Self::Id>,
        data: Self::Edge,
    );
    /// Removes a single directed arc `from -> to`; the reverse direction, if present, is left untouched.
    fn remove_edge_directed(
        &mut self,
        from: VertexId<Self::Id>,
        to: VertexId<Self::Id>,
    ) -> Option<Self::Edge>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", serde_feature)]
pub struct Edge<E, I: UnsignedId> {
    pub from: VertexId<I>,
    pub to: VertexId<I>,
    pub data: E,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", serde_feature)]
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
