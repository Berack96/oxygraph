use std::marker::PhantomData;

use crate::{
    edges::{AdjList, AdjListFixed, GraphEdgeStorage},
    graph::Graph,
    vertices::UnsignedId,
};

pub struct GraphBuilder<V: 'static, E: 'static, I = u32, S = AdjList<E, I>>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    vertices: Option<Vec<V>>,
    directed: bool,
    _marker: PhantomData<(V, E, I, S)>,
}

impl<V, E> GraphBuilder<V, E> {
    pub fn new() -> Self {
        Self {
            vertices: None,
            directed: false,
            _marker: PhantomData,
        }
    }
}

impl<V, E> Default for GraphBuilder<V, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V, E, I, S> GraphBuilder<V, E, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    // Transizione da Dyn a Fixed usando const generics
    pub fn as_adjlist_fixed_max_degree<const N: usize>(
        self,
    ) -> GraphBuilder<V, E, I, AdjListFixed<E, I, N>> {
        GraphBuilder {
            vertices: self.vertices,
            directed: self.directed,
            _marker: PhantomData,
        }
    }

    // Transizione esplicita a Dynamic
    pub fn as_adjlist(self) -> GraphBuilder<V, E, I, AdjList<E, I>> {
        GraphBuilder {
            vertices: self.vertices,
            directed: self.directed,
            _marker: PhantomData,
        }
    }

    pub fn with_vertices(mut self, vertices: Vec<V>) -> Self {
        self.vertices = Some(vertices);
        self
    }

    pub fn directed(mut self, directed: bool) -> Self {
        self.directed = directed;
        self
    }

    pub fn build(self) -> Graph<V, E, I, S> {
        let vertices = self.vertices.unwrap_or_default();
        let size = vertices.len();
        Graph::new_with(vertices, S::with_capacity(size))
    }
}
