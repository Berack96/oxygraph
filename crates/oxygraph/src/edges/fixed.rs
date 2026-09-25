use crate::{
    edges::{Edge, EdgeSimple, EdgeView, GraphEdgeStorage, GraphEdgeStorageDirected, MaybeSerde},
    vertices::{UnsignedId, VertexId},
};

// Not `#[serde_feature]`: `[Option<EdgeSimple<E, I>>; N]` has a const-generic size N, and
// serde's derive can't generate (de)serialize impls for an array whose size is generic.
pub struct AdjListFixed<E: 'static, I: UnsignedId, const N: usize> {
    edges: Vec<[Option<EdgeSimple<E, I>>; N]>,
    edge_count: usize,
}
impl<E: 'static, I: UnsignedId, const N: usize> AdjListFixed<E, I, N> {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            edge_count: 0,
        }
    }
}

impl<E: 'static, I: UnsignedId, const N: usize> Default for AdjListFixed<E, I, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: 'static + MaybeSerde, I: UnsignedId + MaybeSerde, const N: usize> GraphEdgeStorage
    for AdjListFixed<E, I, N>
{
    type Edge = E;
    type Id = I;

    fn of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
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

    fn get_all(&self) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.edges.iter().enumerate().flat_map(|(from_id, edges)| {
            let from_vertex = VertexId::new(from_id);
            edges.iter().filter_map(move |edge| {
                edge.as_ref()
                    .map(|edge| EdgeView::new(from_vertex, edge.to, &edge.data))
            })
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
            .and_then(|edges| {
                edges
                    .iter()
                    .find(|edge| edge.as_ref().is_some_and(|edge| edge.to == *to))
            })
            .map(|edge| &edge.as_ref().unwrap().data)
    }

    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool {
        self.edges.get(from.id()).is_some_and(|edges| {
            edges
                .iter()
                .any(|edge| edge.as_ref().is_some_and(|edge| edge.to == *to))
        })
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
        self.edges
            .get(id.id())
            .map(|edges| edges.iter().filter(|edge| edge.is_some()).count())
            .unwrap_or(0)
    }

    fn add_all(&mut self, from: VertexId<I>, edges: impl IntoIterator<Item = (VertexId<I>, E)>) {
        for (to, data) in edges {
            self.add_edge_directed(from, to, data);
        }
    }

    fn remove_all(&mut self, from: VertexId<I>) -> Vec<(VertexId<I>, E)> {
        let edges = self.edges.get_mut(from.id());
        if let Some(edges) = edges {
            let mut removed_edges = Vec::new();
            for edge in edges.iter_mut() {
                if let Some(edge) = edge.take() {
                    self.edge_count -= 1;
                    removed_edges.push((edge.to, edge.data));
                }
            }
            removed_edges
        } else {
            Vec::new()
        }
    }
}

impl<E: 'static + MaybeSerde, I: UnsignedId + MaybeSerde, const N: usize> GraphEdgeStorageDirected
    for AdjListFixed<E, I, N>
{
    fn children_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.of(id)
    }

    fn parents_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.get_all().filter(move |edge| edge.to == id)
    }

    fn count_incoming(&self, id: VertexId<I>) -> usize {
        self.get_all().filter(|edge| edge.to == id).count()
    }

    fn count_outgoing(&self, id: VertexId<I>) -> usize {
        self.count_of(id)
    }

    fn add_edge_directed(&mut self, from: VertexId<I>, to: VertexId<I>, data: E) {
        let from = from.id();
        self.edges
            .resize_with(from + 1, || std::array::from_fn(|_| None));
        let edge = self.edges[from]
            .iter_mut()
            .find(|edge| edge.is_none())
            .expect("maximum degree exceeded");
        *edge = Some(EdgeSimple { to, data });
        self.edge_count += 1;
    }

    fn remove_edge_directed(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E> {
        let edges = self.edges.get_mut(from.id())?;
        let edge = edges
            .iter_mut()
            .find(|edge| edge.as_ref().is_some_and(|edge| edge.to == to))?;
        let data = edge.take().map(|edge| edge.data);
        if data.is_some() {
            self.edge_count -= 1;
        }
        data
    }
}
