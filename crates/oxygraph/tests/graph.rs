use oxygraph::graph_view::GraphView;
use oxygraph::{GraphBuilder, VertexId};

#[test]
fn adds_and_reads_vertices() {
    let mut graph = GraphBuilder::<String, ()>::new().build();

    let first = graph.add_vertex(String::from("first"));
    let second = graph.add_vertex(String::from("second"));

    assert_eq!(first, VertexId::new(0));
    assert_eq!(second, VertexId::new(1));
    assert_eq!(graph.vertex(first), Some(&String::from("first")));
    assert_eq!(graph.vertex(second), Some(&String::from("second")));
}

#[test]
fn mutates_a_vertex_in_place() {
    let mut graph = GraphBuilder::<String, ()>::new().build();
    let vertex = graph.add_vertex(String::from("before"));

    *graph.vertex_mut(vertex).unwrap() = String::from("after");

    assert_eq!(graph.vertex(vertex), Some(&String::from("after")));
}

#[test]
fn returns_none_for_unknown_vertex() {
    let graph = GraphBuilder::<String, ()>::new().build();

    assert_eq!(graph.vertex(VertexId::new(0)), None);
}

#[test]
fn vertex_mut_returns_none_for_unknown_vertex() {
    let mut graph = GraphBuilder::<String, ()>::new().build();

    assert_eq!(graph.vertex_mut(VertexId::new(0)), None);
}

#[test]
fn supports_fixed_storage() {
    let mut graph = GraphBuilder::<String, ()>::new()
        .as_adjlist_fixed_max_degree::<4>()
        .build();

    let vertex = graph.add_vertex(String::from("fixed"));

    assert_eq!(graph.vertex(vertex), Some(&String::from("fixed")));
}
