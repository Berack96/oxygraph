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
    pub use super::views::{EdgeFilter, EdgeFilteredView, GraphFilteredView, VertexFilter};
}

pub mod graph_visit {
    pub use super::visits::{
        AStar, AllSimplePaths, ArticulationPoints, BellmanFord, BetweennessCentrality, Bfs,
        BfsToAny, BiconnectedComponents, BiconnectedEdges, Bipartite, Bipartition, BridgeEdges,
        Bridges, Centrality, Components, ConnectedComponents, CycleDetection, Dfs, Dijkstra,
        DijkstraReached, DijkstraReaching, Eccentricities, Eccentricity, EulerianTrail, MaxFlow,
        MinCutEdges, MinimumSpanningTree, MstEdges, MultiSourceBfs, MultiSourceDijkstra,
        Reachability, Reached, Reaching, SccComponents, StronglyConnectedComponents,
        TopologicalSort, TransitiveClosure, ViewVisit, VisitError, VisitResult,
    };
}
