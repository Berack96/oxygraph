//! Eccentricity and diameter, available for any graph, directed or undirected: a breadth-first
//! visit from every vertex via [`GraphEdgeStorage::of`].
//!
//! An unreachable vertex contributes no distance: on a disconnected graph, each vertex's
//! eccentricity is the farthest distance within its own reach, not infinity.

use std::collections::VecDeque;

use crate::{edges::GraphEdgeStorage, graph::GraphView, vertices::VertexId, visits::VertexMarks};

/// One entry per vertex, its eccentricity: the greatest shortest-path distance from it to any
/// vertex it can reach.
pub type Eccentricities<I> = Vec<(VertexId<I>, u32)>;

/// Computes the eccentricity of every vertex the view exposes, and the graph's diameter: the
/// greatest eccentricity among them. `None` diameter only for an empty graph.
#[derive(Debug, Clone, Copy)]
pub struct Eccentricity;

impl Eccentricity {
    pub fn visit<V: 'static, S: GraphEdgeStorage, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> (Eccentricities<S::Id>, Option<u32>) {
        let mut eccentricities = Vec::new();
        let mut diameter = None;

        for source in view.ids() {
            let mut distance: VertexMarks<Option<u32>> = VertexMarks::new(None);
            let mut queue = VecDeque::new();
            let mut farthest = 0u32;

            distance.set(source, Some(0));
            queue.push_back(source);

            while let Some(v) = queue.pop_front() {
                let dv: u32 = distance.get(v).expect("queued vertices have a distance");
                farthest = farthest.max(dv);
                for edge in view.edges().of(v) {
                    if distance.get(edge.to).is_none() {
                        distance.set(edge.to, Some(dv + 1));
                        queue.push_back(edge.to);
                    }
                }
            }

            eccentricities.push((source, farthest));
            diameter = Some(diameter.unwrap_or(0).max(farthest));
        }

        (eccentricities, diameter)
    }
}
