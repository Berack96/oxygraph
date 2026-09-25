#![cfg(feature = "serde")]

use oxygraph::{
    Graph, GraphBuilder, VertexId,
    graph_edges::{AdjList, GraphEdgeStorage},
    graph_view::GraphView,
};
use serde::{Deserialize, Serialize};

fn build_simple_graph() -> Graph<String, AdjList<i32, u32>> {
    let mut graph = GraphBuilder::<String, i32>::new()
        .with_vertices(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
            String::from("f"),
        ])
        .build();

    for (from, to, weight) in [
        (0, 1, 10),
        (1, 2, 20),
        (2, 3, 30),
        (3, 4, 40),
        (4, 5, 50),
        (5, 0, 60),
        (0, 3, 70),
    ] {
        graph
            .edges_mut()
            .add(VertexId::new(from), VertexId::new(to), weight);
    }
    graph
}

#[test]
fn round_trips_a_graph_with_simple_vertex_and_edge_types() {
    let graph = build_simple_graph();
    let json = serde_json::to_string(&graph).expect("serialize");

    let deserialized: Graph<String, AdjList<i32, u32>> =
        serde_json::from_str(&json).expect("deserialize");
    let json_again = serde_json::to_string(&deserialized).expect("re-serialize");

    assert_eq!(json, json_again);
    assert_eq!(deserialized.len(), graph.len());
    assert_eq!(deserialized.edges().count(), graph.edges().count());
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct City {
    name: String,
    population: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Road {
    distance_km: f64,
    lanes: u8,
}

fn build_complex_graph() -> Graph<City, AdjList<Road, u32>> {
    let mut graph = GraphBuilder::<City, Road>::new()
        .with_vertices(vec![
            City {
                name: String::from("Turin"),
                population: 870_000,
            },
            City {
                name: String::from("Milan"),
                population: 1_370_000,
            },
            City {
                name: String::from("Genoa"),
                population: 560_000,
            },
            City {
                name: String::from("Bologna"),
                population: 390_000,
            },
            City {
                name: String::from("Florence"),
                population: 380_000,
            },
            City {
                name: String::from("Rome"),
                population: 2_870_000,
            },
        ])
        .build();

    for (from, to, distance_km, lanes) in [
        (0, 1, 140.0, 3u8),
        (1, 2, 160.0, 2),
        (1, 3, 210.0, 3),
        (3, 4, 105.0, 2),
        (4, 5, 275.0, 3),
        (5, 0, 690.0, 2),
        (0, 3, 330.0, 2),
    ] {
        graph.edges_mut().add(
            VertexId::new(from),
            VertexId::new(to),
            Road { distance_km, lanes },
        );
    }
    graph
}

#[test]
fn round_trips_a_graph_with_struct_vertex_and_edge_data() {
    let graph = build_complex_graph();
    let json = serde_json::to_string(&graph).expect("serialize");

    let deserialized: Graph<City, AdjList<Road, u32>> =
        serde_json::from_str(&json).expect("deserialize");
    let json_again = serde_json::to_string(&deserialized).expect("re-serialize");

    assert_eq!(json, json_again);
    for i in 0..graph.len() {
        let id = VertexId::new(i);
        assert_eq!(graph.vertex(id), deserialized.vertex(id));
    }
}
