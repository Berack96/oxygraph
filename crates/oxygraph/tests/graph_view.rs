use oxygraph::{GraphBuilder, GraphEdgeIter, GraphView, VertexId};

#[test]
fn reads_vertices_from_a_graph_view() {
    let mut graph = GraphBuilder::<String, ()>::new().build();
    let vertex = graph.add_vertex(String::from("vertex"));
    let view = GraphView::new(&graph);

    assert_eq!(view.get_vertex(vertex), Some(&String::from("vertex")));
    assert_eq!(view.get_vertex(VertexId::new(1)), None);
}

#[test]
fn iterates_no_edges_from_an_empty_graph() {
    let graph = GraphBuilder::<String, ()>::new().build();
    let view = GraphView::new(&graph);

    assert_eq!(view.edges().count(), 0);
}
