use std::marker::PhantomData;

use crate::{
    edges::{AdjList, AdjListFixed, GraphEdgeStorage},
    graph::Graph,
    ids::UnsignedId,
};

pub struct GraphBuilder<V: 'static, E: 'static, I = u32, S = AdjList<E, I>>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    capacity: usize,
    directed: bool,
    _marker: PhantomData<(V, E, I, S)>,
}

impl<V, E> GraphBuilder<V, E> {
    pub fn new() -> Self {
        Self {
            capacity: 1, // If zero then no allocation for vec
            directed: false,
            _marker: PhantomData,
        }
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
            capacity: self.capacity,
            directed: self.directed,
            _marker: PhantomData,
        }
    }

    // Transizione esplicita a Dynamic
    pub fn as_adjlist(self) -> GraphBuilder<V, E, I, AdjList<E, I>> {
        GraphBuilder {
            capacity: self.capacity,
            directed: self.directed,
            _marker: PhantomData,
        }
    }

    pub fn change_vec_indexing<J>(self) -> GraphBuilder<V, E, J, S::Index<J>>
    where
        J: UnsignedId,
    {
        GraphBuilder {
            capacity: self.capacity,
            directed: self.directed,
            _marker: PhantomData,
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = if capacity == 0 { 1 } else { capacity };
        self
    }

    pub fn directed(mut self, directed: bool) -> Self {
        self.directed = directed;
        self
    }
}

// Build per storage dinamico
impl<V, E, I> GraphBuilder<V, E, I, AdjList<E, I>>
where
    I: UnsignedId,
{
    pub fn build(self) -> Graph<V, E, I, AdjList<E, I>> {
        Graph::new_with(
            Vec::with_capacity(self.capacity),
            AdjList::with_capacity(self.capacity),
        )
    }
}

// Build per storage fisso
impl<V, E, I, const N: usize> GraphBuilder<V, E, I, AdjListFixed<E, I, N>>
where
    I: UnsignedId,
{
    pub fn build(self) -> Graph<V, E, I, AdjListFixed<E, I, N>> {
        Graph::new_with(
            Vec::with_capacity(self.capacity),
            AdjListFixed::with_capacity(self.capacity),
        )
    }
}
