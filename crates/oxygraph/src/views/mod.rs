mod edges;
mod graph;

pub use {edges::EdgeFilteredView, graph::GraphFilteredView};

/// A vertex predicate for [`GraphFilteredView`]/[`EdgeFilteredView`], boxed so it can capture
/// runtime state instead of being limited to a capture-free `fn` pointer.
pub type VertexFilter<'a, V> = Box<dyn Fn(&V) -> bool + 'a>;

/// An edge predicate for [`GraphFilteredView`]/[`EdgeFilteredView`], boxed so it can capture
/// runtime state instead of being limited to a capture-free `fn` pointer.
pub type EdgeFilter<'a, E> = Box<dyn Fn(&E) -> bool + 'a>;
