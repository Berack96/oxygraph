//! Depth-first visit, bound to directed graphs: it follows outgoing edges only, via
//! [`GraphEdgeStorageDirected`].

use crate::{
    edges::GraphEdgeStorageDirected,
    graph::GraphView,
    vertices::VertexId,
    visits::{Mark, VertexMarks, ViewVisit, VisitError, VisitResult},
};

/// Depth-first traversal from a start vertex, returning the vertices in visit order.
#[derive(Debug, Clone, Copy)]
pub struct Dfs;

impl<V: 'static, S: GraphEdgeStorageDirected> ViewVisit<V, S> for Dfs {
    type Output = Vec<VertexId<S::Id>>;

    fn visit<G>(&self, view: &G, start: VertexId<S::Id>) -> VisitResult<Self::Output>
    where
        G: GraphView<V, S>,
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
