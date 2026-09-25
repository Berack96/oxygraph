use std::marker::PhantomData;

use oxygraph_derive::serde_feature;

use crate::{
    edges::{Edge, EdgeView, GraphEdgeStorage, GraphEdgeStorageDirected, MaybeSerde},
    vertices::{UnsignedId, VertexId},
};

/// Dense adjacency matrix: a flattened, row-major `n * n` grid of cells, one per ordered
/// vertex pair. `get`/`has_edge`/`add_edge_directed`/`remove_edge_directed` are O(1) — direct
/// cell access, no scanning. `of`/`count_of` are O(n) (a row scan), and, unlike `AdjList`,
/// `parents_of`/`count_incoming` are O(n) too (a column scan) rather than O(E).
///
/// The trade-off is memory: O(V²) regardless of how many edges actually exist, and growing
/// to a higher vertex id reallocates the whole grid (O(n²)). Good fit for dense graphs with a
/// known-ish vertex count; wasteful for large sparse graphs, where `AdjList`/`AdjCsr` fit
/// better.
#[serde_feature]
pub struct AdjMatrix<E: 'static, I: UnsignedId> {
    cells: Vec<Option<E>>,
    size: usize,
    edge_count: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    _marker: PhantomData<I>,
}

impl<E: 'static, I: UnsignedId> AdjMatrix<E, I> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            size: 0,
            edge_count: 0,
            _marker: PhantomData,
        }
    }

    // `capacity` is the expected vertex count: the grid is allocated eagerly at
    // `capacity * capacity` cells so building via a known vertex count (e.g. through
    // `GraphBuilder::with_vertices`) never has to grow the matrix afterwards.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cells: (0..capacity * capacity).map(|_| None).collect(),
            size: capacity,
            edge_count: 0,
            _marker: PhantomData,
        }
    }

    fn ensure_size(&mut self, min_size: usize) {
        if min_size <= self.size {
            return;
        }
        let old_size = self.size;
        let mut old_cells = std::mem::take(&mut self.cells);
        let mut new_cells: Vec<Option<E>> = (0..min_size * min_size).map(|_| None).collect();
        for row in 0..old_size {
            for col in 0..old_size {
                new_cells[row * min_size + col] = old_cells[row * old_size + col].take();
            }
        }
        self.cells = new_cells;
        self.size = min_size;
    }
}

impl<E: 'static, I: UnsignedId> Default for AdjMatrix<E, I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: 'static + MaybeSerde, I: UnsignedId + MaybeSerde> GraphEdgeStorage for AdjMatrix<E, I> {
    type Edge = E;
    type Id = I;

    fn of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        let row = id.id();
        let cols = if row < self.size { 0..self.size } else { 0..0 };
        cols.filter_map(move |col| {
            self.cells[row * self.size + col]
                .as_ref()
                .map(|data| EdgeView::new(id, VertexId::new(col), data))
        })
    }

    fn get_all(&self) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        (0..self.size).flat_map(move |row| self.of(VertexId::new(row)))
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
        let (row, col) = (from.id(), to.id());
        if row >= self.size || col >= self.size {
            return None;
        }
        self.cells[row * self.size + col].as_ref()
    }

    fn has_edge(&self, from: &VertexId<I>, to: &VertexId<I>) -> bool {
        self.get(from, to).is_some()
    }

    fn count(&self) -> usize {
        self.edge_count
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn with_edges(edges: Vec<Edge<E, I>>) -> Self {
        let vertex_count = edges
            .iter()
            .map(|edge| edge.from.id().max(edge.to.id()) + 1)
            .max()
            .unwrap_or(0);
        let mut matrix = Self::with_capacity(vertex_count);
        for edge in edges {
            matrix.add_edge_directed(edge.from, edge.to, edge.data);
        }
        matrix
    }

    fn count_of(&self, id: VertexId<I>) -> usize {
        self.of(id).count()
    }

    fn add_all(&mut self, from: VertexId<I>, edges: impl IntoIterator<Item = (VertexId<I>, E)>) {
        for (to, data) in edges {
            self.add_edge_directed(from, to, data);
        }
    }

    fn remove_all(&mut self, from: VertexId<I>) -> Vec<(VertexId<I>, E)> {
        let row = from.id();
        if row >= self.size {
            return Vec::new();
        }
        (0..self.size)
            .filter_map(|col| {
                let to = VertexId::new(col);
                self.remove_edge_directed(from, to).map(|data| (to, data))
            })
            .collect()
    }
}

impl<E: 'static + MaybeSerde, I: UnsignedId + MaybeSerde> GraphEdgeStorageDirected
    for AdjMatrix<E, I>
{
    fn children_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        self.of(id)
    }

    fn parents_of(&self, id: VertexId<I>) -> impl Iterator<Item = EdgeView<'_, E, I>> {
        let col = id.id();
        let rows = if col < self.size { 0..self.size } else { 0..0 };
        rows.filter_map(move |row| {
            self.cells[row * self.size + col]
                .as_ref()
                .map(|data| EdgeView::new(VertexId::new(row), id, data))
        })
    }

    fn count_incoming(&self, id: VertexId<I>) -> usize {
        self.parents_of(id).count()
    }

    fn count_outgoing(&self, id: VertexId<I>) -> usize {
        self.count_of(id)
    }

    fn add_edge_directed(&mut self, from: VertexId<I>, to: VertexId<I>, data: E) {
        let (row, col) = (from.id(), to.id());
        self.ensure_size(row.max(col) + 1);
        let idx = row * self.size + col;
        if self.cells[idx].is_none() {
            self.edge_count += 1;
        }
        self.cells[idx] = Some(data);
    }

    fn remove_edge_directed(&mut self, from: VertexId<I>, to: VertexId<I>) -> Option<E> {
        let (row, col) = (from.id(), to.id());
        if row >= self.size || col >= self.size {
            return None;
        }
        let removed = self.cells[row * self.size + col].take();
        if removed.is_some() {
            self.edge_count -= 1;
        }
        removed
    }
}
