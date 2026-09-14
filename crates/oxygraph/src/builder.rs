use std::marker::PhantomData;

use crate::{
    graph::Graph,
    storage::{Array, List},
    traits::EdgeStorage,
};

enum StorageStrategy<const N: usize> {
    Fixed,
    Dynamic,
}

pub struct GraphBuilder<V, E, S = List<E>> {
    capacity: usize,
    directed: bool,
    _marker: PhantomData<(V, E, S)>,
}

impl<V, E> GraphBuilder<V, E, List<E>> {
    pub fn new() -> Self {
        Self {
            capacity: 0,
            directed: false,
            _marker: PhantomData,
        }
    }
}

impl<V, E, S> GraphBuilder<V, E, S>
where
    V: Clone + std::fmt::Debug,
    E: Clone + std::fmt::Debug,
    S: EdgeStorage<E>,
{
    // Transizione da Dyn a Fixed usando const generics
    pub fn with_fixed_max_degree<const N: usize>(self) -> GraphBuilder<V, E, Array<E, N>> {
        GraphBuilder {
            capacity: self.capacity,
            directed: self.directed,
            _marker: PhantomData,
        }
    }

    // Transizione esplicita a Dynamic
    pub fn with_unlimited_degree(self) -> GraphBuilder<V, E, List<E>> {
        GraphBuilder {
            capacity: self.capacity,
            directed: self.directed,
            _marker: PhantomData,
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn directed(mut self, directed: bool) -> Self {
        self.directed = directed;
        self
    }
}

// Build per storage dinamico
impl<V, E> GraphBuilder<V, E, List<E>>
where
    V: Clone + std::fmt::Debug,
    E: Clone + std::fmt::Debug,
{
    pub fn build(self) -> Graph<V, E, List<E>> {
        Graph::with_capacity(self.capacity)
    }
}

// Build per storage fisso
impl<V, E, const N: usize> GraphBuilder<V, E, Array<E, N>>
where
    V: Clone + std::fmt::Debug,
    E: Clone + std::fmt::Debug,
{
    pub fn build(self) -> Graph<V, E, Array<E, N>> {
        Graph::with_capacity(self.capacity)
    }
}
