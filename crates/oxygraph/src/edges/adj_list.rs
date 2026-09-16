use oxygraph_derive::serde_feature;

use crate::{
    directionality::Directed,
    edges::{GraphEdgeIter, GraphEdgeStorage},
    ids::{EdgeView, UnsignedId, VertexId},
};

struct Edge<E, I: UnsignedId> {
    to: VertexId<I>,
    data: E,
}

#[serde_feature]
pub struct AdjList<E: 'static, I: UnsignedId> {
    edges: Vec<Vec<Edge<E, I>>>,
}

impl<E: 'static, I: UnsignedId> AdjList<E, I> {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            edges: Vec::with_capacity(capacity),
        }
    }
}

impl<E: 'static, I: UnsignedId> Default for AdjList<E, I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: 'static, I: UnsignedId> GraphEdgeStorage<E, I> for AdjList<E, I> {
    type Directionality = Directed;
    type Index<J: UnsignedId> = AdjList<E, J>;

    fn add_edge(&mut self, from: VertexId<I>, to: VertexId<I>, data: E) {
        let from = from.id();
        self.edges.resize_with(from + 1, Vec::new);
        self.edges[from].push(Edge { to, data });
    }

    fn remove_edge(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E> {
        let edges = self.edges.get_mut(from.id())?;
        let index = edges.iter().position(|edge| edge.to == to)?;
        Some(edges.remove(index).data)
    }

    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool {
        self.edges
            .get(from.id())
            .is_some_and(|edges| edges.iter().any(|edge| edge.to == *to))
    }
}

impl<E: 'static, I: UnsignedId> GraphEdgeIter<E, I> for AdjList<E, I> {
    fn edges_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        let vert_id = id.id();
        self.edges
            .get(vert_id)
            .into_iter()
            .flatten()
            .map(move |edge| EdgeView::new(id, edge.to, &edge.data))
    }

    fn edges(&self) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.edges.iter().enumerate().flat_map(|(from_id, edges)| {
            let from_vertex = VertexId::new(from_id);
            edges
                .iter()
                .map(move |edge| EdgeView::new(from_vertex, edge.to, &edge.data))
        })
    }
}
