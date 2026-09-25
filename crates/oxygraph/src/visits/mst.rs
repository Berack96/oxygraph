//! Minimum spanning tree, available for any graph, directed or undirected: reads
//! [`GraphEdgeStorage::get_all`] as an undirected multiset of weighted arcs, same one-way
//! caveat as [`ConnectedComponents`](super::ConnectedComponents) on a graph built one
//! direction at a time via
//! [`add_edge_directed`](crate::edges::GraphEdgeStorageDirected::add_edge_directed).

use crate::{
    edges::{GraphEdgeStorage, Weighted},
    graph::GraphView,
    vertices::VertexId,
};

/// One entry per selected edge, plus its weight.
pub type MstEdges<I> = Vec<(VertexId<I>, VertexId<I>, f64)>;

/// Builds a minimum spanning forest (one tree per connected component) via Kruskal's
/// algorithm: repeatedly takes the lightest remaining edge that doesn't close a cycle.
/// Returns the selected edges and their total weight.
#[derive(Debug, Clone, Copy)]
pub struct MinimumSpanningTree;

impl MinimumSpanningTree {
    pub fn visit<V: 'static, S: GraphEdgeStorage, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> (MstEdges<S::Id>, f64)
    where
        S::Edge: Weighted,
    {
        let mut edges: MstEdges<S::Id> = view
            .edges()
            .get_all()
            .map(|edge| (edge.from, edge.to, edge.data.weight()))
            .collect();
        edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

        let mut parent: Vec<usize> = (0..view.len()).collect();
        let mut mst = Vec::new();
        let mut total = 0.0;

        for (from, to, weight) in edges {
            if from == to {
                continue;
            }
            let root_from = find(&mut parent, from.id());
            let root_to = find(&mut parent, to.id());
            if root_from != root_to {
                parent[root_from] = root_to;
                mst.push((from, to, weight));
                total += weight;
            }
        }

        (mst, total)
    }
}

/// Union-find lookup with path compression, iterative to keep the depth bounded regardless
/// of graph size.
fn find(parent: &mut [usize], x: usize) -> usize {
    let mut root = x;
    while parent[root] != root {
        root = parent[root];
    }
    let mut current = x;
    while parent[current] != root {
        let next = parent[current];
        parent[current] = root;
        current = next;
    }
    root
}
