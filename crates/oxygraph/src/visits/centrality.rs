//! Betweenness centrality, available for any graph, directed or undirected: Brandes'
//! algorithm run over unweighted shortest paths, reading [`GraphEdgeStorage::of`] as the
//! adjacency to expand.

use std::collections::VecDeque;

use crate::{edges::GraphEdgeStorage, graph::GraphView, vertices::VertexId, visits::VertexMarks};

/// One entry per vertex, its betweenness centrality score: how often it sits on a shortest
/// path between two other vertices, summed over every ordered pair that has one.
pub type Centrality<I> = Vec<(VertexId<I>, f64)>;

/// Computes betweenness centrality for every vertex the view exposes, via Brandes'
/// algorithm on unweighted shortest paths.
#[derive(Debug, Clone, Copy)]
pub struct BetweennessCentrality;

impl BetweennessCentrality {
    pub fn visit<V: 'static, S: GraphEdgeStorage, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> Centrality<S::Id> {
        let mut score: VertexMarks<f64> = VertexMarks::new(0.0);

        for source in view.ids() {
            let mut distance: VertexMarks<Option<u32>> = VertexMarks::new(None);
            let mut sigma: VertexMarks<f64> = VertexMarks::new(0.0);
            let mut predecessors: VertexMarks<Vec<VertexId<S::Id>>> = VertexMarks::new(Vec::new());
            let mut order = Vec::new();
            let mut queue = VecDeque::new();

            distance.set(source, Some(0));
            sigma.set(source, 1.0);
            queue.push_back(source);

            while let Some(v) = queue.pop_front() {
                order.push(v);
                let dv: u32 = distance.get(v).expect("queued vertices have a distance");
                let sigma_v = *sigma.get(v);
                let neighbors: Vec<VertexId<S::Id>> = view.edges().of(v).map(|e| e.to).collect();

                for w in neighbors {
                    if distance.get(w).is_none() {
                        distance.set(w, Some(dv + 1));
                        queue.push_back(w);
                    }
                    if *distance.get(w) == Some(dv + 1) {
                        sigma.set(w, *sigma.get(w) + sigma_v);
                        let mut preds = predecessors.get(w).clone();
                        preds.push(v);
                        predecessors.set(w, preds);
                    }
                }
            }

            let mut delta: VertexMarks<f64> = VertexMarks::new(0.0);
            for &w in order.iter().rev() {
                let coefficient = (1.0 + *delta.get(w)) / *sigma.get(w);
                for &v in predecessors.get(w) {
                    let updated = *delta.get(v) + *sigma.get(v) * coefficient;
                    delta.set(v, updated);
                }
                if w != source {
                    let updated = *score.get(w) + *delta.get(w);
                    score.set(w, updated);
                }
            }
        }

        view.ids().map(|id| (id, *score.get(id))).collect()
    }
}
