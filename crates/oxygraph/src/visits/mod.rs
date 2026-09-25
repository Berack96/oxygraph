//! Generic traversal (visit) framework, running over any [`GraphView`] rather than a
//! concrete `Graph`.
//!
//! A visit is bound to the kind of graph it supports through the storage bound `S`:
//! a bound of `S: GraphEdgeStorage` accepts any graph, directed or undirected,
//! while `S: GraphEdgeStorageDirected` restricts the visit to graphs whose storage
//! exposes direction-aware navigation ([`children_of`](crate::edges::GraphEdgeStorageDirected::children_of) /
//! [`parents_of`](crate::edges::GraphEdgeStorageDirected::parents_of)).

mod astar;
mod bellman_ford;
mod bfs;
mod biconnected;
mod bridges;
mod connected_components;
mod cycle;
mod dfs;
mod dijkstra;
mod mst;
mod multi_source_bfs;
mod multi_source_dijkstra;
mod scc;
mod simple_paths;
mod topological_sort;

pub use astar::AStar;
pub use bellman_ford::BellmanFord;
pub use bfs::Bfs;
pub use biconnected::{BiconnectedComponents, BiconnectedEdges};
pub use bridges::{ArticulationPoints, BridgeEdges, Bridges};
pub use connected_components::{Components, ConnectedComponents};
pub use cycle::CycleDetection;
pub use dfs::Dfs;
pub use dijkstra::Dijkstra;
pub use mst::{MinimumSpanningTree, MstEdges};
pub use multi_source_bfs::{MultiSourceBfs, Reached, Reaching};
pub use multi_source_dijkstra::{DijkstraReached, DijkstraReaching, MultiSourceDijkstra};
pub use scc::{SccComponents, StronglyConnectedComponents};
pub use simple_paths::AllSimplePaths;
pub use topological_sort::TopologicalSort;

use crate::{
    edges::GraphEdgeStorage,
    graph::GraphView,
    vertices::{UnsignedId, VertexId},
};

/// A traversal over a generic graph view, bound to the graph kinds accepted by `S`.
pub trait ViewVisit<V: 'static, S: GraphEdgeStorage> {
    /// Result produced by the visit, custom to each implementation.
    type Output;

    fn visit<G>(&self, view: &G, start: VertexId<S::Id>) -> VisitResult<Self::Output>
    where
        G: GraphView<V, S>;
}

pub type VisitResult<T> = Result<T, VisitError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitError {
    VertexNotFound,
    /// A negative-weight cycle is reachable from the start vertex, so no shortest path is
    /// well-defined: its cost can be driven arbitrarily low by looping through it.
    NegativeCycle,
}

/// Visited/unvisited marker shared by the visits that don't need extra per-vertex data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mark {
    Unvisited,
    Visited,
}

/// Per-vertex marking storage shared by visits, backed by a densely-indexed growable array.
/// Each visit picks its own mark type `M` (a visited/unvisited enum, a distance, ...).
pub struct VertexMarks<M> {
    marks: Vec<M>,
    default: M,
}

impl<M: Clone> VertexMarks<M> {
    pub fn new(default: M) -> Self {
        Self {
            marks: Vec::new(),
            default,
        }
    }

    pub fn get<I: UnsignedId>(&self, id: VertexId<I>) -> &M {
        self.marks.get(id.id()).unwrap_or(&self.default)
    }

    pub fn set<I: UnsignedId>(&mut self, id: VertexId<I>, mark: M) {
        let idx = id.id();
        if idx >= self.marks.len() {
            self.marks.resize(idx + 1, self.default.clone());
        }
        self.marks[idx] = mark;
    }
}
