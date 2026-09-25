//! Strongly connected components visit, bound to directed graphs: Kosaraju's algorithm needs
//! both outgoing and incoming edges, via [`GraphEdgeStorageDirected`].

use std::collections::VecDeque;

use crate::{
    edges::GraphEdgeStorageDirected,
    graph::GraphView,
    vertices::VertexId,
    visits::{Mark, VertexMarks},
};

/// One entry per strongly connected component, each holding its member vertices.
pub type SccComponents<I> = Vec<Vec<VertexId<I>>>;

/// Partitions every vertex the view exposes into strongly connected components: maximal sets
/// of vertices mutually reachable from one another following edge direction. Kosaraju's
/// algorithm, run iteratively with explicit stacks so it never recurses, regardless of graph
/// size.
#[derive(Debug, Clone, Copy)]
pub struct StronglyConnectedComponents;

impl StronglyConnectedComponents {
    pub fn visit<V: 'static, S: GraphEdgeStorageDirected, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> SccComponents<S::Id> {
        let mut successors: VertexMarks<Vec<VertexId<S::Id>>> = VertexMarks::new(Vec::new());
        let mut predecessors: VertexMarks<Vec<VertexId<S::Id>>> = VertexMarks::new(Vec::new());
        for vertex in view.ids() {
            successors.set(
                vertex,
                view.edges().children_of(vertex).map(|e| e.to).collect(),
            );
            predecessors.set(
                vertex,
                view.edges().parents_of(vertex).map(|e| e.from).collect(),
            );
        }

        // Pass 1: finish order of a DFS over successors (iterative, explicit stack).
        let mut visited = VertexMarks::new(Mark::Unvisited);
        let mut finish_order = Vec::new();
        for start in view.ids() {
            if *visited.get(start) == Mark::Visited {
                continue;
            }
            visited.set(start, Mark::Visited);
            let mut stack = vec![(start, 0usize)];
            while let Some(&mut (vertex, ref mut next_child)) = stack.last_mut() {
                if let Some(&child) = successors.get(vertex).get(*next_child) {
                    *next_child += 1;
                    if *visited.get(child) == Mark::Unvisited {
                        visited.set(child, Mark::Visited);
                        stack.push((child, 0));
                    }
                } else {
                    finish_order.push(vertex);
                    stack.pop();
                }
            }
        }

        // Pass 2: walk predecessors in reverse finish order; each tree touched is one SCC.
        let mut component_of: VertexMarks<Option<usize>> = VertexMarks::new(None);
        let mut components: SccComponents<S::Id> = Vec::new();
        for &start in finish_order.iter().rev() {
            if component_of.get(start).is_some() {
                continue;
            }
            let index = components.len();
            component_of.set(start, Some(index));
            let mut members = Vec::new();
            let mut queue = VecDeque::from([start]);
            while let Some(vertex) = queue.pop_front() {
                members.push(vertex);
                for &predecessor in predecessors.get(vertex) {
                    if component_of.get(predecessor).is_none() {
                        component_of.set(predecessor, Some(index));
                        queue.push_back(predecessor);
                    }
                }
            }
            components.push(members);
        }

        components
    }
}
