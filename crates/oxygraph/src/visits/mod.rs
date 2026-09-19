use crate::{edges::GraphEdgeStorage, vertices::UnsignedId, views::GraphFilteredView};

pub trait ViewVisit<'a, V: 'static, E: 'static, I, S>
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    fn visit(&self, graph: &GraphFilteredView<'a, V, E, I, S>) -> VisitResult;
}

pub type VisitResult = Result<(), String>;
