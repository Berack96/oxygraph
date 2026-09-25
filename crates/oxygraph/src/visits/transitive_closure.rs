//! Transitive closure, bound to directed graphs: the full reachability relation, computed by
//! running a breadth-first visit from every vertex via
//! [`GraphEdgeStorageDirected::children_of`].
//!
//! O(V * (V + E)): fine for answering many "can X reach Y" queries once built, expensive to
//! build on a large graph.

use std::collections::VecDeque;

use crate::{
    edges::GraphEdgeStorageDirected,
    graph::GraphView,
    vertices::VertexId,
    visits::{Mark, VertexMarks},
};

/// One entry per vertex, holding every vertex reachable from it. A vertex appears in its own
/// entry only if it lies on a cycle reachable from itself.
pub type Reachability<I> = Vec<(VertexId<I>, Vec<VertexId<I>>)>;

/// Computes the transitive closure of the view: for every vertex, every vertex reachable from
/// it following edge direction.
#[derive(Debug, Clone, Copy)]
pub struct TransitiveClosure;

impl TransitiveClosure {
    pub fn visit<V: 'static, S: GraphEdgeStorageDirected, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> Reachability<S::Id> {
        view.ids()
            .map(|source| {
                let mut marks = VertexMarks::new(Mark::Unvisited);
                let mut queue = VecDeque::new();
                let mut reached = Vec::new();
                let mut source_reached = false;

                marks.set(source, Mark::Visited);
                queue.push_back(source);

                while let Some(current) = queue.pop_front() {
                    for edge in view.edges().children_of(current) {
                        if edge.to == source {
                            // Source is already fully explored; record it once as reachable
                            // via this cycle, but never re-enqueue or re-explore it.
                            if !source_reached {
                                source_reached = true;
                                reached.push(source);
                            }
                        } else if *marks.get(edge.to) == Mark::Unvisited {
                            marks.set(edge.to, Mark::Visited);
                            reached.push(edge.to);
                            queue.push_back(edge.to);
                        }
                    }
                }

                (source, reached)
            })
            .collect()
    }
}
