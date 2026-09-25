//! Multi-source breadth-first visit, available for any graph, directed or undirected.

use std::collections::VecDeque;

use crate::{
    edges::GraphEdgeStorage,
    graph::GraphView,
    vertices::{UnsignedId, VertexId},
    visits::{VertexMarks, VisitError, VisitResult},
};

/// Distance from, and identity of, the source that reached a vertex first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reached<I: UnsignedId> {
    pub distance: u32,
    pub source: VertexId<I>,
}

/// One `(vertex, reached)` entry per vertex a multi-source visit reached.
pub type Reaching<I> = Vec<(VertexId<I>, Reached<I>)>;

/// Breadth-first traversal from several start vertices at once: every reachable vertex is
/// attributed to whichever source's wavefront reaches it first, ties broken by `sources`
/// input order, rather than by running one BFS per source and merging the results.
#[derive(Debug, Clone, Copy)]
pub struct MultiSourceBfs;

impl MultiSourceBfs {
    /// Runs the visit; the result lists every reached vertex, sources first, in discovery
    /// order. Fails if any `sources` vertex is missing from `view`.
    pub fn visit<V: 'static, S: GraphEdgeStorage, G: GraphView<V, S>>(
        &self,
        view: &G,
        sources: impl IntoIterator<Item = VertexId<S::Id>>,
    ) -> VisitResult<Reaching<S::Id>> {
        let mut marks: VertexMarks<Option<Reached<S::Id>>> = VertexMarks::new(None);
        let mut queue = VecDeque::new();
        let mut order = Vec::new();

        for source in sources {
            if view.vertex(source).is_none() {
                return Err(VisitError::VertexNotFound);
            }
            if marks.get(source).is_some() {
                continue;
            }
            let reached = Reached {
                distance: 0,
                source,
            };
            marks.set(source, Some(reached));
            order.push((source, reached));
            queue.push_back(source);
        }

        while let Some(current) = queue.pop_front() {
            let Reached { distance, source } = marks
                .get(current)
                .expect("queued vertices are always marked");
            let next_distance = distance + 1;

            for edge in view.edges().of(current) {
                if marks.get(edge.to).is_none() {
                    let reached = Reached {
                        distance: next_distance,
                        source,
                    };
                    marks.set(edge.to, Some(reached));
                    order.push((edge.to, reached));
                    queue.push_back(edge.to);
                }
            }
        }

        Ok(order)
    }
}
