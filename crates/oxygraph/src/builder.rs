use std::marker::PhantomData;

use crate::{
    edges::{AdjCsr, AdjList, AdjListFixed, AdjMatrix, GraphEdgeStorage, MaybeSerde},
    graph::Graph,
    vertices::UnsignedId,
};

pub struct GraphBuilder<V: 'static, E: 'static, I = u32, S = AdjList<E, I>>
where
    I: UnsignedId,
    S: GraphEdgeStorage<Edge = E, Id = I>,
{
    vertices: Option<Vec<V>>,
    _marker: PhantomData<(V, E, I, S)>,
}

// MaybeSerde is intentionally crate-internal (see its doc comment); it only shapes what
// E/I must satisfy, it isn't part of the public API surface.
#[allow(private_bounds)]
impl<V, E: MaybeSerde> GraphBuilder<V, E> {
    pub fn new() -> Self {
        Self {
            vertices: None,
            _marker: PhantomData,
        }
    }
}

#[allow(private_bounds)]
impl<V, E: MaybeSerde> Default for GraphBuilder<V, E> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(private_bounds)]
impl<V, E: MaybeSerde, I, S> GraphBuilder<V, E, I, S>
where
    I: UnsignedId + MaybeSerde,
    S: GraphEdgeStorage<Edge = E, Id = I>,
{
    // Switch from the dynamic adjacency list to the fixed-capacity one, sized by a const generic.
    pub fn as_adjlist_fixed_max_degree<const N: usize>(
        self,
    ) -> GraphBuilder<V, E, I, AdjListFixed<E, I, N>> {
        GraphBuilder {
            vertices: self.vertices,
            _marker: PhantomData,
        }
    }

    // Switch explicitly back to the dynamic adjacency list.
    pub fn as_adjlist(self) -> GraphBuilder<V, E, I, AdjList<E, I>> {
        GraphBuilder {
            vertices: self.vertices,
            _marker: PhantomData,
        }
    }

    // Switch to the compressed sparse row storage.
    pub fn as_csr(self) -> GraphBuilder<V, E, I, AdjCsr<E, I>> {
        GraphBuilder {
            vertices: self.vertices,
            _marker: PhantomData,
        }
    }

    // Switch to the dense adjacency matrix storage.
    pub fn as_matrix(self) -> GraphBuilder<V, E, I, AdjMatrix<E, I>> {
        GraphBuilder {
            vertices: self.vertices,
            _marker: PhantomData,
        }
    }

    pub fn with_vertices(mut self, vertices: Vec<V>) -> Self {
        self.vertices = Some(vertices);
        self
    }

    pub fn build(self) -> Graph<V, S> {
        let vertices = self.vertices.unwrap_or_default();
        let size = vertices.len();
        Graph::new_with(vertices, S::with_capacity(size))
    }
}
