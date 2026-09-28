//! Maximum flow visit, bound to directed graphs with weighted capacities: Edmonds-Karp,
//! repeatedly augmenting along the shortest (fewest-arc) available path in the residual
//! graph. Capacities (edge weights) must be non-negative.

use std::collections::{HashMap, VecDeque};

use crate::{
    edges::{GraphEdgeStorageDirected, Weighted},
    graph::GraphView,
    vertices::{UnsignedId, VertexId},
    visits::{VertexMarks, ViewVisit, VisitError, VisitResult, reconstruct_path},
};

/// One entry per min-cut edge: an original arc from the source side of the final residual
/// graph to the sink side.
pub type MinCutEdges<I> = Vec<(VertexId<I>, VertexId<I>)>;

type ResidualCapacities<I> = HashMap<(VertexId<I>, VertexId<I>), f64>;

/// Maximum flow from a start vertex to `sink`: the total flow value, and the min-cut edges
/// whose combined capacity equals it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxFlow<I: UnsignedId> {
    sink: VertexId<I>,
}

impl<I: UnsignedId> MaxFlow<I> {
    pub fn new(sink: VertexId<I>) -> Self {
        Self { sink }
    }
}

impl<V: 'static, S: GraphEdgeStorageDirected> ViewVisit<V, S> for MaxFlow<S::Id>
where
    S::Edge: Weighted,
{
    type Output = (f64, MinCutEdges<S::Id>);

    fn visit<G>(&self, view: &G, start: VertexId<S::Id>) -> VisitResult<Self::Output>
    where
        G: GraphView<V, S>,
    {
        if view.vertex(start).is_none() || view.vertex(self.sink).is_none() {
            return Err(VisitError::VertexNotFound);
        }
        if start == self.sink {
            return Ok((0.0, Vec::new()));
        }

        // Candidate residual neighbors of each vertex: either an original arc, or the reverse
        // of one, whichever way flow could still move.
        let mut neighbors: VertexMarks<Vec<VertexId<S::Id>>> = VertexMarks::new(Vec::new());
        let mut residual: ResidualCapacities<S::Id> = HashMap::new();
        for v in view.ids() {
            let mut list: Vec<VertexId<S::Id>> =
                view.edges().children_of(v).map(|e| e.to).collect();
            list.extend(view.edges().parents_of(v).map(|e| e.from));
            neighbors.set(v, list);
        }
        for edge in view.edges().get_all() {
            *residual.entry((edge.from, edge.to)).or_insert(0.0) += edge.data.weight();
            residual.entry((edge.to, edge.from)).or_insert(0.0);
        }

        let mut max_flow = 0.0;

        let final_reachable = loop {
            let mut visited: VertexMarks<bool> = VertexMarks::new(false);
            let mut previous: VertexMarks<Option<VertexId<S::Id>>> = VertexMarks::new(None);
            let mut queue = VecDeque::from([start]);
            visited.set(start, true);

            while let Some(v) = queue.pop_front() {
                for &w in neighbors.get(v) {
                    if !*visited.get(w) && *residual.get(&(v, w)).unwrap_or(&0.0) > 0.0 {
                        visited.set(w, true);
                        previous.set(w, Some(v));
                        queue.push_back(w);
                    }
                }
            }

            if !*visited.get(self.sink) {
                break visited;
            }

            let path = reconstruct_path(self.sink, &previous);

            let bottleneck = path
                .windows(2)
                .map(|pair| *residual.get(&(pair[0], pair[1])).unwrap_or(&0.0))
                .fold(f64::INFINITY, f64::min);

            for pair in path.windows(2) {
                *residual.entry((pair[0], pair[1])).or_insert(0.0) -= bottleneck;
                *residual.entry((pair[1], pair[0])).or_insert(0.0) += bottleneck;
            }
            max_flow += bottleneck;
        };

        // `get_all` isn't vertex-filtered, so it's checked against `view.vertex` here too: an
        // arc from a reachable in-view vertex to one a `GraphFilteredView` excludes would
        // otherwise be reported as a cut edge, even though its far endpoint isn't part of the
        // view at all.
        let min_cut: MinCutEdges<S::Id> = view
            .edges()
            .get_all()
            .filter(|edge| view.vertex(edge.from).is_some() && view.vertex(edge.to).is_some())
            .filter(|edge| *final_reachable.get(edge.from) && !*final_reachable.get(edge.to))
            .map(|edge| (edge.from, edge.to))
            .collect();

        Ok((max_flow, min_cut))
    }
}
