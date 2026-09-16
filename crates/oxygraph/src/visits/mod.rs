use crate::{
    directionality::GraphDirectionality, edges::GraphEdgeStorage, graph::GraphView, ids::UnsignedId,
};

pub trait ViewVisit<'a, V: 'static, E: 'static, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    type Directionality: GraphDirectionality;

    fn visit(&self, graph: &GraphView<'a, V, E, I, S>) -> VisitResult
    where
        S: GraphEdgeStorage<E, I, Directionality = Self::Directionality>;
}

pub type VisitResult = Result<(), ()>;
