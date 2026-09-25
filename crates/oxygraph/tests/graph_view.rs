use oxygraph::graph_edges::GraphEdgeStorage;
use oxygraph::graph_view::GraphView;
use oxygraph::{GraphBuilder, VertexId};

#[test]
fn reads_vertices_from_a_graph_view() {
    let mut graph = GraphBuilder::<String, ()>::new().build();
    let vertex = graph.add_vertex(String::from("vertex"));
    let view = graph.get_filtered_view(None, None);

    assert_eq!(view.vertex(vertex), Some(&String::from("vertex")));
    assert_eq!(view.vertex(VertexId::new(1)), None);
}

#[test]
fn iterates_no_edges_from_an_empty_graph() {
    let graph = GraphBuilder::<String, ()>::new().build();
    let view = graph.get_filtered_view(None, None);

    assert_eq!(view.edges().count(), 0);
}

#[test]
fn ids_enumerates_every_vertex_in_order() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let c = graph.add_vertex("c");

    assert_eq!(graph.get_view().ids().collect::<Vec<_>>(), vec![a, b, c]);
}

#[test]
fn ids_skips_vertices_filtered_out_of_the_view() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let _b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    let view = graph.get_filtered_view(Some(|v: &&str| *v != "b"), None);

    assert_eq!(view.ids().collect::<Vec<_>>(), vec![a, c]);
}
