//! Topological sort, bound to directed graphs: Kahn's algorithm needs in-degree via
//! [`GraphEdgeStorageDirected::count_incoming`].

use std::collections::VecDeque;

use crate::{
    edges::GraphEdgeStorageDirected, graph::GraphView, vertices::VertexId, visits::VertexMarks,
};

/// Topologically sorts every vertex the view exposes: an order where every edge goes from an
/// earlier vertex to a later one. `None` if the graph has a cycle, since no such order exists.
#[derive(Debug, Clone, Copy)]
pub struct TopologicalSort;

impl TopologicalSort {
    pub fn visit<V: 'static, S: GraphEdgeStorageDirected, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> Option<Vec<VertexId<S::Id>>> {
        let mut in_degree: VertexMarks<usize> = VertexMarks::new(0);
        for vertex in view.ids() {
            in_degree.set(vertex, view.edges().count_incoming(vertex));
        }

        let mut queue: VecDeque<VertexId<S::Id>> =
            view.ids().filter(|&v| *in_degree.get(v) == 0).collect();
        let mut order = Vec::new();

        while let Some(vertex) = queue.pop_front() {
            order.push(vertex);
            for edge in view.edges().children_of(vertex) {
                let remaining = *in_degree.get(edge.to) - 1;
                in_degree.set(edge.to, remaining);
                if remaining == 0 {
                    queue.push_back(edge.to);
                }
            }
        }

        if order.len() == view.len() {
            Some(order)
        } else {
            None
        }
    }
}
