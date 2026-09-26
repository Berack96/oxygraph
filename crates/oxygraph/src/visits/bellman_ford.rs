//! Bellman-Ford shortest-path visit, available for any graph, directed or undirected: unlike
//! [`Dijkstra`](super::Dijkstra), it tolerates negative edge weights and reports a negative
//! cycle reachable from the start vertex instead of a path.

use crate::{
    edges::{GraphEdgeStorage, Weighted},
    graph::GraphView,
    vertices::{UnsignedId, VertexId},
    visits::{VertexMarks, ViewVisit, VisitError, VisitResult, reconstruct_path},
};

/// Shortest path from a start vertex to `target`: the vertex sequence and its total weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BellmanFord<I: UnsignedId> {
    target: VertexId<I>,
}

type WeightedEdges<I> = Vec<(VertexId<I>, VertexId<I>, f64)>;

impl<I: UnsignedId> BellmanFord<I> {
    pub fn new(target: VertexId<I>) -> Self {
        Self { target }
    }
}

impl<V: 'static, S: GraphEdgeStorage> ViewVisit<V, S> for BellmanFord<S::Id>
where
    S::Edge: Weighted,
{
    type Output = Option<(Vec<VertexId<S::Id>>, f64)>;

    fn visit<G>(&self, view: &G, start: VertexId<S::Id>) -> VisitResult<Self::Output>
    where
        G: GraphView<V, S>,
    {
        if view.vertex(start).is_none() || view.vertex(self.target).is_none() {
            return Err(VisitError::VertexNotFound);
        }

        let edges: WeightedEdges<S::Id> = view
            .edges()
            .get_all()
            .map(|edge| (edge.from, edge.to, edge.data.weight()))
            .collect();

        let mut distance: VertexMarks<Option<f64>> = VertexMarks::new(None);
        let mut previous: VertexMarks<Option<VertexId<S::Id>>> = VertexMarks::new(None);
        distance.set(start, Some(0.0));

        // `edges.len()` rounds always suffice: a shortest (simple) path uses each edge at most
        // once, so it never needs more relaxation rounds than there are edges to settle. Bounded
        // by `edges.len()` rather than `view.len()`, since `edges` isn't vertex-filtered and can
        // reach further than the view's own vertex count on a `GraphFilteredView`.
        for _ in 0..edges.len() {
            let mut changed = false;
            for &(from, to, weight) in &edges {
                if relax(&mut distance, &mut previous, from, to, weight) {
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        // One extra round: if any edge can still be relaxed, a negative cycle reachable from
        // `start` keeps driving some distance down forever.
        for &(from, to, weight) in &edges {
            if relax(&mut distance, &mut previous, from, to, weight) {
                return Err(VisitError::NegativeCycle);
            }
        }

        let Some(total) = *distance.get(self.target) else {
            return Ok(None);
        };

        Ok(Some((reconstruct_path(self.target, &previous), total)))
    }
}

/// Relaxes a single edge, updating `to`'s distance and predecessor if going through `from`
/// is cheaper. Returns whether it improved.
fn relax<I: UnsignedId>(
    distance: &mut VertexMarks<Option<f64>>,
    previous: &mut VertexMarks<Option<VertexId<I>>>,
    from: VertexId<I>,
    to: VertexId<I>,
    weight: f64,
) -> bool {
    let Some(from_distance) = *distance.get(from) else {
        return false;
    };
    let candidate = from_distance + weight;
    let improves = match *distance.get(to) {
        None => true,
        Some(known) => candidate < known,
    };
    if improves {
        distance.set(to, Some(candidate));
        previous.set(to, Some(from));
    }
    improves
}
