//! Breadth-first visit, available for any graph, directed or undirected.

use std::collections::VecDeque;

use crate::{
    edges::GraphEdgeStorage,
    graph::GraphView,
    vertices::{UnsignedId, VertexId},
    visits::{Mark, VertexMarks, ViewVisit, VisitError, VisitResult},
};

/// Breadth-first traversal from a start vertex, returning the vertices in visit order.
#[derive(Debug, Clone, Copy)]
pub struct Bfs;

impl<V: 'static, E: 'static, I, S> ViewVisit<V, E, I, S> for Bfs
where
    I: UnsignedId,
    S: GraphEdgeStorage<E, I>,
{
    type Output = Vec<VertexId<I>>;

    fn visit<G>(&self, view: &G, start: VertexId<I>) -> VisitResult<Self::Output>
    where
        G: GraphView<V, E, I, S>,
    {
        if view.vertex(start).is_none() {
            return Err(VisitError::VertexNotFound);
        }

        let mut marks = VertexMarks::new(Mark::Unvisited);
        let mut order = Vec::new();
        let mut queue = VecDeque::new();

        marks.set(start, Mark::Visited);
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            order.push(current);
            for edge in view.edges().of(current) {
                if *marks.get(edge.to) == Mark::Unvisited {
                    marks.set(edge.to, Mark::Visited);
                    queue.push_back(edge.to);
                }
            }
        }

        Ok(order)
    }
}
