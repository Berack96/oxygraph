//! A graph tool library for building memory efficient and fast graphs.

mod builder;
mod edges;
mod graph;
mod vertices;
mod views;
mod visits;

pub use builder::GraphBuilder;
pub use graph::Graph;
pub use vertices::VertexId;

pub mod graph_edges {
    pub use super::edges::{
        AdjCsr, AdjList, AdjListFixed, AdjMatrix, Edge, EdgeSimple, GraphEdgeStorage,
        GraphEdgeStorageDirected, Weighted,
    };
}

pub mod graph_view {
    pub use super::edges::EdgeView;
    pub use super::graph::GraphView;
    pub use super::views::{EdgeFilteredView, GraphFilteredView};
}

pub mod graph_visit {
    pub use super::visits::{
        ArticulationPoints, Bfs, BridgeEdges, Bridges, Components, ConnectedComponents,
        CycleDetection, Dfs, Dijkstra, DijkstraReached, DijkstraReaching, MultiSourceBfs,
        MultiSourceDijkstra, Reached, Reaching, SccComponents, StronglyConnectedComponents,
        TopologicalSort, ViewVisit, VisitError, VisitResult,
    };
}
