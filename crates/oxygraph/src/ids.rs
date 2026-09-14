#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VertexId(pub usize);

impl VertexId {
    const INFINITE: usize = usize::MAX;
    const MAX: usize = usize::MAX;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Edge<E> {
    pub vertex_id: VertexId,
    pub data: E,
}
