use crate::{
    ids::{Edge, VertexId},
    traits::EdgeStorage,
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Array<E, const N: usize> {
    edges: [Edge<E>; N],
}

impl<E, const N: usize> EdgeStorage<E> for Array<E, N>
where
    E: Clone + std::fmt::Debug,
{
    fn add_to(&mut self, edge: Edge<E>) {
        if let Some(existing) = self
            .edges
            .iter_mut()
            .find(|e| e.vertex_id.0 == edge.vertex_id.0)
        {
            existing.data = edge.data;
        } else {
            self.edges.push(edge);
        }
    }

    fn remove(&mut self, id: &VertexId) -> Option<E> {
        todo!()
    }

    fn get_all(&self) -> &[E] {
        todo!()
    }

    fn has_edge(&self, id: &VertexId) -> bool {
        self.edges.iter().any(|e| e.vertex_id.0 == id.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct List<E> {
    edges: Vec<E>,
}

impl<E> EdgeStorage<E> for List<E>
where
    E: Clone + std::fmt::Debug,
{
    fn add_to(&mut self, id: &VertexId, edge: E) {
        todo!()
    }

    fn remove(&mut self, id: &VertexId) -> Option<E> {
        todo!()
    }

    fn get_all(&self) -> &[E] {
        todo!()
    }

    fn has_edge(&self, id: &VertexId) -> bool {
        todo!()
    }
}
