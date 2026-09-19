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
