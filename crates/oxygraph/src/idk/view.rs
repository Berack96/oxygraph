use std::marker::PhantomData;

use num_traits::{PrimInt, Unsigned};
use oxygraph_derive::serde_feature;

use crate::storage::GraphEdgeStorage;

#[serde_feature]
pub struct GraphFilters<'a, V, E> {
    vertices: Option<fn(&'a V) -> bool>,
    edges: Option<fn(&'a E) -> bool>,
}

#[serde_feature]
pub struct GraphView<'a, V, E, I, S>
where
    I: Unsigned + PrimInt,
    S: GraphEdgeStorage<E, I>,
{
    graph: &'a Graph<V, E, I, S>,
    filters: GraphFilters<'a, V, E>,
    _marker: PhantomData<(E, I)>,
}

impl GraphView<'_, V, E, I, S>
where
    I: Unsigned + PrimInt,
    S: GraphEdgeStorage<E, I>,
{
    pub fn new<'a>(graph: &'a Graph<V, E, I, S>) -> GraphView<'a, V, E, I, S> {
        Self::new_with_filters(
            graph,
            GraphFilters {
                vertices: None,
                edges: None,
            },
        )
    }

    pub fn new_with_filters<'a>(
        graph: &'a Graph<V, E, I, S>,
        filters: GraphFilters<'a, V, E>,
    ) -> GraphView<'a, V, E, I, S> {
        Self {
            graph,
            filters,
            _marker: PhantomData,
        }
    }

    pub fn get_vertex(&self, id: VertexId<I>) -> Option<&V> {
        let v = self.graph.(id);
    }
}
