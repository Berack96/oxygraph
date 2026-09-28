//! Unweighted breadth-first visit from a start vertex to the first of several target
//! vertices it reaches, available for any graph, directed or undirected.

use std::collections::VecDeque;

use crate::{
    edges::GraphEdgeStorage,
    graph::GraphView,
    vertices::{UnsignedId, VertexId},
    visits::{Mark, VertexMarks, ViewVisit, VisitError, VisitResult, reconstruct_path},
};

/// Unweighted shortest path from a start vertex to the first of `targets` a breadth-first
/// search reaches, along with which target that was. Unlike [`Dijkstra`](crate::visits::Dijkstra)
/// or [`AStar`](crate::visits::AStar), it needs no `Weighted` edge data: every edge costs 1.
pub struct BfsToAny<I: UnsignedId> {
    targets: Vec<VertexId<I>>,
}

impl<I: UnsignedId> BfsToAny<I> {
    pub fn new(targets: impl IntoIterator<Item = VertexId<I>>) -> Self {
        Self {
            targets: targets.into_iter().collect(),
        }
    }
}

impl<V: 'static, S: GraphEdgeStorage> ViewVisit<V, S> for BfsToAny<S::Id> {
    type Output = Option<(Vec<VertexId<S::Id>>, VertexId<S::Id>)>;

    fn visit<G>(&self, view: &G, start: VertexId<S::Id>) -> VisitResult<Self::Output>
    where
        G: GraphView<V, S>,
    {
        if view.vertex(start).is_none() {
            return Err(VisitError::VertexNotFound);
        }

        let mut is_target = VertexMarks::new(false);
        for &target in &self.targets {
            if view.vertex(target).is_none() {
                return Err(VisitError::VertexNotFound);
            }
            is_target.set(target, true);
        }

        let mut marks = VertexMarks::new(Mark::Unvisited);
        let mut previous: VertexMarks<Option<VertexId<S::Id>>> = VertexMarks::new(None);
        let mut queue = VecDeque::new();

        marks.set(start, Mark::Visited);
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            if *is_target.get(current) {
                return Ok(Some((reconstruct_path(current, &previous), current)));
            }
            for edge in view.edges().of(current) {
                if *marks.get(edge.to) == Mark::Unvisited {
                    marks.set(edge.to, Mark::Visited);
                    previous.set(edge.to, Some(current));
                    queue.push_back(edge.to);
                }
            }
        }

        Ok(None)
    }
}
