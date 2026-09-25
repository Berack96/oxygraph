//! Depth-first visit, bound to directed graphs: it follows outgoing edges only, via
//! [`GraphEdgeStorageDirected`].

use crate::{
    edges::GraphEdgeStorageDirected,
    graph::GraphView,
    vertices::{UnsignedId, VertexId},
    visits::{VertexMarks, ViewVisit, VisitError, VisitResult},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mark {
    Unvisited,
    Visited,
}

/// Depth-first traversal from a start vertex, returning the vertices in visit order.
pub struct Dfs;

impl<V: 'static, E: 'static, I, S> ViewVisit<V, E, I, S> for Dfs
where
    I: UnsignedId,
    S: GraphEdgeStorageDirected<E, I>,
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
        let mut stack = vec![start];

        while let Some(current) = stack.pop() {
            if *marks.get(current) == Mark::Visited {
                continue;
            }
            marks.set(current, Mark::Visited);
            order.push(current);

            for edge in view.edges().children_of(current) {
                if *marks.get(edge.to) == Mark::Unvisited {
                    stack.push(edge.to);
                }
            }
        }

        Ok(order)
    }
}
