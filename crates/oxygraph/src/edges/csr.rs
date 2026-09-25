use oxygraph_derive::serde_feature;

use crate::{
    edges::{Edge, EdgeView, GraphEdgeStorage, GraphEdgeStorageDirected, MaybeSerde},
    vertices::{UnsignedId, VertexId},
};

/// Compressed Sparse Row edge storage: outgoing arcs are packed into two flat, contiguous
/// arrays (`targets`/`data`), grouped by `from` and located via `offsets`. No per-vertex
/// `Vec` indirection, so [`get_all`](GraphEdgeStorage::get_all) is a single linear scan with
/// good cache locality — the format's whole point.
///
/// The trade-off is mutation: [`with_edges`](GraphEdgeStorage::with_edges) builds the packed
/// layout in one O(V + E) counting sort, but a single
/// [`add_edge_directed`](GraphEdgeStorageDirected::add_edge_directed) or
/// [`remove_edge_directed`](GraphEdgeStorageDirected::remove_edge_directed) call is O(V + E),
/// since it has to shift the packed arrays. CSR fits graphs that are mostly built once (or in
/// batches) and then read many times, not ones mutated edge-by-edge.
#[serde_feature]
pub struct AdjCsr<E: 'static, I: UnsignedId> {
    offsets: Vec<usize>,
    targets: Vec<VertexId<I>>,
    data: Vec<E>,
}

impl<E: 'static, I: UnsignedId> AdjCsr<E, I> {
    pub fn new() -> Self {
        Self {
            offsets: vec![0],
            targets: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(capacity + 1);
        offsets.push(0);
        Self {
            offsets,
            targets: Vec::new(),
            data: Vec::new(),
        }
    }

    fn ensure_vertex(&mut self, id: usize) {
        if id + 2 > self.offsets.len() {
            let last = *self.offsets.last().unwrap();
            self.offsets.resize(id + 2, last);
        }
    }

    fn range_of(&self, id: usize) -> std::ops::Range<usize> {
        match (self.offsets.get(id), self.offsets.get(id + 1)) {
            (Some(&start), Some(&end)) => start..end,
            _ => 0..0,
        }
    }
}

impl<E: 'static, I: UnsignedId> Default for AdjCsr<E, I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: 'static + MaybeSerde, I: UnsignedId + MaybeSerde> GraphEdgeStorage for AdjCsr<E, I> {
    type Edge = E;
    type Id = I;

    fn of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.range_of(id.id())
            .map(move |i| EdgeView::new(id, self.targets[i], &self.data[i]))
    }

    fn get_all(&self) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.offsets
            .windows(2)
            .enumerate()
            .flat_map(move |(from, w)| {
                let from_vertex = VertexId::new(from);
                (w[0]..w[1])
                    .map(move |i| EdgeView::new(from_vertex, self.targets[i], &self.data[i]))
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
        self.range_of(from.id())
            .find(|&i| self.targets[i] == *to)
            .map(|i| &self.data[i])
    }

    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool {
        self.get(from, to).is_some()
    }

    fn count(&self) -> usize {
        self.targets.len()
    }

    fn with_capacity(capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(capacity + 1);
        offsets.push(0);
        Self {
            offsets,
            targets: Vec::new(),
            data: Vec::new(),
        }
    }

    fn with_edges(edges: Vec<Edge<E, I>>) -> Self {
        let vertex_count = edges
            .iter()
            .map(|edge| edge.from.id().max(edge.to.id()) + 1)
            .max()
            .unwrap_or(0);

        let mut offsets = vec![0usize; vertex_count + 1];
        for edge in &edges {
            offsets[edge.from.id() + 1] += 1;
        }
        for i in 1..offsets.len() {
            offsets[i] += offsets[i - 1];
        }

        let mut cursor = offsets.clone();
        let mut slots: Vec<Option<(VertexId<I>, E)>> = (0..edges.len()).map(|_| None).collect();
        for edge in edges {
            let pos = cursor[edge.from.id()];
            cursor[edge.from.id()] += 1;
            slots[pos] = Some((edge.to, edge.data));
        }

        let (targets, data) = slots
            .into_iter()
            .map(|slot| slot.expect("csr construction fills every slot exactly once"))
            .unzip();

        Self {
            offsets,
            targets,
            data,
        }
    }

    fn count_of(&self, id: VertexId<I>) -> usize {
        self.range_of(id.id()).len()
    }

    fn add_all(&mut self, from: VertexId<I>, edges: impl IntoIterator<Item = (VertexId<I>, E)>) {
        for (to, data) in edges {
            self.add_edge_directed(from, to, data);
        }
    }

    fn remove_all(&mut self, from: VertexId<I>) -> Vec<(VertexId<I>, E)> {
        self.range_of(from.id())
            .map(|i| self.targets[i])
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|to| self.remove_edge_directed(from, to).map(|data| (to, data)))
            .collect()
    }
}

impl<E: 'static + MaybeSerde, I: UnsignedId + MaybeSerde> GraphEdgeStorageDirected
    for AdjCsr<E, I>
{
    fn children_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.of(id)
    }

    fn parents_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.get_all().filter(move |edge| edge.to == id)
    }

    fn count_incoming(&self, id: VertexId<I>) -> usize {
        self.parents_of(id).count()
    }

    fn count_outgoing(&self, id: VertexId<I>) -> usize {
        self.count_of(id)
    }

    fn add_edge_directed(&mut self, from: VertexId<I>, to: VertexId<I>, data: E) {
        let from = from.id();
        self.ensure_vertex(from);
        let insert_at = self.offsets[from + 1];
        self.targets.insert(insert_at, to);
        self.data.insert(insert_at, data);
        for offset in &mut self.offsets[from + 1..] {
            *offset += 1;
        }
    }

    fn remove_edge_directed(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E> {
        let pos = self.range_of(from.id()).find(|&i| self.targets[i] == to)?;
        self.targets.remove(pos);
        let data = self.data.remove(pos);
        for offset in &mut self.offsets[from.id() + 1..] {
            *offset -= 1;
        }
        Some(data)
    }
}
