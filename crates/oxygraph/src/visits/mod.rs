use num_traits::{PrimInt, Unsigned};

use crate::{directionality::GraphDirectionality, edges::GraphEdgeStorage, graph::GraphView};

pub trait ViewVisit<'a, V: 'static, E: 'static, I, S>
where
    I: Unsigned + PrimInt,
    S: GraphEdgeStorage<E, I>,
{
    type Directionality: GraphDirectionality;

    fn visit(&self, graph: &GraphView<'a, V, E, I, S>) -> VisitResult
    where
        S: GraphEdgeStorage<E, I, Directionality = Self::Directionality>;
}

pub type VisitResult = Result<(), ()>;
