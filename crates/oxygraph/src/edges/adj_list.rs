use oxygraph_derive::serde_feature;

use crate::{
    edges::{Edge, EdgeSimple, EdgeView, GraphEdgeStorage, GraphEdgeStorageDirected, MaybeSerde},
    vertices::{UnsignedId, VertexId},
};

#[serde_feature]
pub struct AdjList<E: 'static, I: UnsignedId> {
    edges: Vec<Vec<EdgeSimple<E, I>>>,
    edge_count: usize,
}

impl<E: 'static, I: UnsignedId> AdjList<E, I> {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            edge_count: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            edges: Vec::with_capacity(capacity),
            edge_count: 0,
        }
    }
}

impl<E: 'static, I: UnsignedId> Default for AdjList<E, I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: 'static + MaybeSerde, I: UnsignedId + MaybeSerde> GraphEdgeStorage for AdjList<E, I> {
    type Edge = E;
    type Id = I;

    fn of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        let vert_id = id.id();
        self.edges
            .get(vert_id)
            .into_iter()
            .flatten()
            .map(move |edge| EdgeView::new(id, edge.to, &edge.data))
    }

    fn get_all(&self) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.edges.iter().enumerate().flat_map(|(from_id, edges)| {
            let from_vertex = VertexId::new(from_id);
            edges
                .iter()
                .map(move |edge| EdgeView::new(from_vertex, edge.to, &edge.data))
        })
    }

    fn add_edge(&mut self, from: VertexId<I>, to: VertexId<I>, data: E)
    where
        E: Clone,
    {
        if from == to {
            self.add_edge_directed(from, to, data);
        } else {
            self.add_edge_directed(from, to, data.clone());
            self.add_edge_directed(to, from, data);
        }
    }

    fn remove_edge(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E> {
        let removed = self.remove_edge_directed(from, to);
        if from != to {
            self.remove_edge_directed(to, from);
        }
        removed
    }

    fn get(&self, from: &VertexId<I>, to: &VertexId<I>) -> Option<&E> {
        self.edges
            .get(from.id())
            .and_then(|edges| edges.iter().find(|edge| edge.to == *to))
            .map(|edge| &edge.data)
    }

    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool {
        self.edges
            .get(from.id())
            .is_some_and(|edges| edges.iter().any(|edge| edge.to == *to))
    }

    fn count(&self) -> usize {
        self.edge_count
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            edges: Vec::with_capacity(capacity),
            edge_count: 0,
        }
    }

    fn with_edges(edges: Vec<Edge<E, I>>) -> Self {
        let mut adj_list = Self::with_capacity(edges.len());
        for edge in edges {
            adj_list.add_edge_directed(edge.from, edge.to, edge.data);
        }
        adj_list
    }

    fn count_of(&self, id: VertexId<I>) -> usize {
        self.edges.get(id.id()).map_or(0, |edges| edges.len())
    }

    fn add_all(&mut self, from: VertexId<I>, edges: impl IntoIterator<Item = (VertexId<I>, E)>) {
        for (to, data) in edges {
            self.add_edge_directed(from, to, data);
        }
    }

    fn remove_all(&mut self, from: VertexId<I>) -> Vec<(VertexId<I>, E)> {
        if let Some(edges) = self.edges.get_mut(from.id()) {
            self.edge_count -= edges.len();
            edges.drain(..).map(|edge| (edge.to, edge.data)).collect()
        } else {
            Vec::new()
        }
    }
}

impl<E: 'static + MaybeSerde, I: UnsignedId + MaybeSerde> GraphEdgeStorageDirected
    for AdjList<E, I>
{
    fn children_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.of(id)
    }

    fn parents_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.get_all().filter(move |edge| id == edge.to)
    }

    fn count_incoming(&self, id: VertexId<I>) -> usize {
        self.parents_of(id).count()
    }

    fn count_outgoing(&self, id: VertexId<I>) -> usize {
        self.count_of(id)
    }

    fn add_edge_directed(&mut self, from: VertexId<I>, to: VertexId<I>, data: E) {
        let from = from.id();
        self.edges.resize_with(from + 1, Vec::new);
        self.edges[from].push(EdgeSimple { to, data });
        self.edge_count += 1;
    }

    fn remove_edge_directed(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E> {
        let edges = self.edges.get_mut(from.id())?;
        let index = edges.iter().position(|edge| edge.to == to)?;
        self.edge_count -= 1;
        Some(edges.swap_remove(index).data)
    }
}
