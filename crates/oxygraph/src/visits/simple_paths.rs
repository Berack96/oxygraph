//! All simple paths visit, bound to directed graphs: enumerates every simple path (no
//! repeated vertex) from a start vertex to `target`, via
//! [`GraphEdgeStorageDirected::children_of`].
//!
//! Exponential in the worst case: a dense or highly connected graph can have exponentially
//! many simple paths between two vertices. Fine for small or sparse graphs, not meant for
//! large dense ones.

use crate::{
    edges::GraphEdgeStorageDirected,
    graph::GraphView,
    vertices::{UnsignedId, VertexId},
    visits::{VertexMarks, ViewVisit, VisitError, VisitResult},
};

/// Every simple path (no repeated vertex) from a start vertex to `target`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllSimplePaths<I: UnsignedId> {
    target: VertexId<I>,
}

impl<I: UnsignedId> AllSimplePaths<I> {
    pub fn new(target: VertexId<I>) -> Self {
        Self { target }
    }
}

impl<V: 'static, S: GraphEdgeStorageDirected> ViewVisit<V, S> for AllSimplePaths<S::Id> {
    type Output = Vec<Vec<VertexId<S::Id>>>;

    fn visit<G>(&self, view: &G, start: VertexId<S::Id>) -> VisitResult<Self::Output>
    where
        G: GraphView<V, S>,
    {
        if view.vertex(start).is_none() || view.vertex(self.target).is_none() {
            return Err(VisitError::VertexNotFound);
        }

        let mut paths = Vec::new();
        if start == self.target {
            paths.push(vec![start]);
        }

        let mut on_path: VertexMarks<bool> = VertexMarks::new(false);
        let mut path = vec![start];
        on_path.set(start, true);

        let neighbors: Vec<VertexId<S::Id>> =
            view.edges().children_of(start).map(|e| e.to).collect();
        let mut stack = vec![(neighbors, 0usize)];

        while let Some((neighbors, mut index)) = stack.pop() {
            if index >= neighbors.len() {
                let finished = path.pop().expect("a pushed vertex is always on the path");
                on_path.set(finished, false);
                continue;
            }

            let next = neighbors[index];
            index += 1;
            stack.push((neighbors, index));

            if *on_path.get(next) {
                continue;
            }
            if next == self.target {
                let mut found = path.clone();
                found.push(next);
                paths.push(found);
                continue;
            }

            path.push(next);
            on_path.set(next, true);
            let next_neighbors: Vec<VertexId<S::Id>> =
                view.edges().children_of(next).map(|e| e.to).collect();
            stack.push((next_neighbors, 0));
        }

        Ok(paths)
    }
}
