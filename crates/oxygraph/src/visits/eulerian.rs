//! Eulerian trail, bound to directed graphs: Hierholzer's algorithm over the directed arcs as
//! stored, via [`GraphEdgeStorageDirected::children_of`]/[`parents_of`](GraphEdgeStorageDirected::parents_of).
//!
//! Treats every stored arc as one distinct edge to traverse: on a graph built with
//! [`add_edge`](crate::edges::GraphEdgeStorage::add_edge), an undirected edge is stored as a
//! mirrored pair of arcs and walked once in each direction, as two separate edges. For a walk
//! that visits every undirected edge exactly once, build the graph one direction at a time via
//! [`add_edge_directed`](GraphEdgeStorageDirected::add_edge_directed) instead.

use std::collections::VecDeque;

use crate::{
    edges::GraphEdgeStorageDirected, graph::GraphView, vertices::VertexId, visits::VertexMarks,
};

/// Finds an Eulerian trail (or circuit, if it starts and ends at the same vertex) that
/// traverses every arc the view exposes exactly once. `None` if no such trail exists, or the
/// view has no edges at all.
#[derive(Debug, Clone, Copy)]
pub struct EulerianTrail;

impl EulerianTrail {
    pub fn visit<V: 'static, S: GraphEdgeStorageDirected, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> Option<Vec<VertexId<S::Id>>> {
        let mut out_degree: VertexMarks<u32> = VertexMarks::new(0);
        let mut in_degree: VertexMarks<u32> = VertexMarks::new(0);
        for v in view.ids() {
            out_degree.set(v, view.edges().count_outgoing(v) as u32);
            in_degree.set(v, view.edges().count_incoming(v) as u32);
        }

        let with_edges: Vec<VertexId<S::Id>> = view
            .ids()
            .filter(|&v| *out_degree.get(v) + *in_degree.get(v) > 0)
            .collect();
        if with_edges.is_empty() {
            return None;
        }
        if !is_weakly_connected(view, &with_edges) {
            return None;
        }

        let mut start = with_edges[0];
        let mut plus_ones = 0u32;
        let mut minus_ones = 0u32;
        for &v in &with_edges {
            let diff = *out_degree.get(v) as i64 - *in_degree.get(v) as i64;
            match diff {
                0 => {}
                1 => {
                    plus_ones += 1;
                    start = v;
                }
                -1 => minus_ones += 1,
                _ => return None,
            }
        }
        if !matches!((plus_ones, minus_ones), (0, 0) | (1, 1)) {
            return None;
        }

        Some(hierholzer(view, start))
    }
}

/// Whether every vertex in `with_edges` is mutually reachable from the others, ignoring edge
/// direction: a necessary condition for an Eulerian trail to exist.
fn is_weakly_connected<V: 'static, S: GraphEdgeStorageDirected, G: GraphView<V, S>>(
    view: &G,
    with_edges: &[VertexId<S::Id>],
) -> bool {
    let mut visited: VertexMarks<bool> = VertexMarks::new(false);
    let mut queue = VecDeque::from([with_edges[0]]);
    visited.set(with_edges[0], true);
    let mut visited_count = 1usize;

    while let Some(v) = queue.pop_front() {
        let mut neighbors: Vec<VertexId<S::Id>> =
            view.edges().children_of(v).map(|e| e.to).collect();
        neighbors.extend(view.edges().parents_of(v).map(|e| e.from));
        for w in neighbors {
            if !*visited.get(w) {
                visited.set(w, true);
                visited_count += 1;
                queue.push_back(w);
            }
        }
    }

    visited_count == with_edges.len()
}

/// Hierholzer's algorithm, iterative: walks arbitrarily until stuck, then backtracks,
/// splicing in the leftover branches it finds along the way, until every arc is consumed.
fn hierholzer<V: 'static, S: GraphEdgeStorageDirected, G: GraphView<V, S>>(
    view: &G,
    start: VertexId<S::Id>,
) -> Vec<VertexId<S::Id>> {
    let mut adjacency: VertexMarks<Vec<VertexId<S::Id>>> = VertexMarks::new(Vec::new());
    for v in view.ids() {
        adjacency.set(v, view.edges().children_of(v).map(|e| e.to).collect());
    }

    let mut stack = vec![start];
    let mut trail = Vec::new();

    while let Some(&v) = stack.last() {
        let mut neighbors = adjacency.get(v).clone();
        if let Some(w) = neighbors.pop() {
            adjacency.set(v, neighbors);
            stack.push(w);
        } else {
            trail.push(stack.pop().expect("the stack is non-empty in this branch"));
        }
    }

    trail.reverse();
    trail
}
