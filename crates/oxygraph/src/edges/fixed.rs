use oxygraph_derive::serde_feature;

use crate::{
    directionality::Directed,
    edges::{GraphEdgeIter, GraphEdgeStorage},
    ids::{EdgeView, UnsignedId, VertexId},
};

#[serde_feature]
pub struct Edge<E, I: UnsignedId> {
    to: VertexId<I>,
    data: E,
}

#[serde_feature]
pub struct AdjListFixed<E: 'static, I: UnsignedId, const N: usize> {
    edges: Vec<[Option<Edge<E, I>>; N]>,
}
impl<E: 'static, I: UnsignedId, const N: usize> AdjListFixed<E, I, N> {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            edges: Vec::with_capacity(capacity),
        }
    }
}

impl<E: 'static, I: UnsignedId, const N: usize> Default for AdjListFixed<E, I, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: 'static, I: UnsignedId, const N: usize> GraphEdgeStorage<E, I> for AdjListFixed<E, I, N> {
    type Directionality = Directed;
    type Index<J: UnsignedId> = AdjListFixed<E, J, N>;

    fn add_edge(&mut self, from: VertexId<I>, to: VertexId<I>, data: E) {
        let from = from.id();
        self.edges
            .resize_with(from + 1, || std::array::from_fn(|_| None));
        let edge = self.edges[from]
            .iter_mut()
            .find(|edge| edge.is_none())
            .expect("maximum degree exceeded");
        *edge = Some(Edge { to, data });
    }

    fn remove_edge(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E> {
        let edges = self.edges.get_mut(from.id())?;
        let edge = edges
            .iter_mut()
            .find(|edge| edge.as_ref().is_some_and(|edge| edge.to == to))?;
        edge.take().map(|edge| edge.data)
    }

    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool {
        self.edges.get(from.id()).is_some_and(|edges| {
            edges
                .iter()
                .any(|edge| edge.as_ref().is_some_and(|edge| edge.to == *to))
        })
    }
}

impl<E: 'static, I: UnsignedId, const N: usize> GraphEdgeIter<E, I> for AdjListFixed<E, I, N> {
    fn edges_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        let vert_id = id.id();
        self.edges
            .get(vert_id)
            .into_iter()
            .flatten()
            .filter_map(move |edge| {
                edge.as_ref()
                    .map(|edge| EdgeView::new(id, edge.to, &edge.data))
            })
    }

    fn edges(&self) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.edges.iter().enumerate().flat_map(|(from_id, edges)| {
            let from_vertex = VertexId::new(from_id);
            edges.iter().filter_map(move |edge| {
                edge.as_ref()
                    .map(|edge| EdgeView::new(from_vertex, edge.to, &edge.data))
            })
        })
    }
}
