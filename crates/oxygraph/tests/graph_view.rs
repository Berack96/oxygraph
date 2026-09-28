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
    let view = graph.get_filtered_view(Some(Box::new(|v: &&str| *v != "b")), None);

    assert_eq!(view.ids().collect::<Vec<_>>(), vec![a, c]);
}

#[test]
fn is_empty_is_false_for_a_view_with_no_filter() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    graph.add_vertex("a");
    let view = graph.get_filtered_view(None, None);

    assert!(!view.is_empty());
}

#[test]
fn is_empty_is_true_when_the_filter_excludes_every_vertex() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    graph.add_vertex("a");
    let view = graph.get_filtered_view(Some(Box::new(|_: &&str| false)), None);

    assert!(view.is_empty());
}

#[test]
fn edges_of_a_vertex_exclude_arcs_reaching_a_vertex_excluded_by_the_filter() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let x = graph.add_vertex("x");
    graph.edges_mut().add_edge(a, x, ());
    let view = graph.get_filtered_view(Some(Box::new(|v: &&str| *v != "x")), None);

    assert_eq!(view.edges().of(a).count(), 0);
}

#[test]
fn is_empty_is_false_when_the_filter_keeps_at_least_one_vertex() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    graph.add_vertex("a");
    graph.add_vertex("b");
    let view = graph.get_filtered_view(Some(Box::new(|v: &&str| *v == "a")), None);

    assert!(!view.is_empty());
}

#[test]
fn filter_accepts_a_closure_capturing_runtime_state() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let _b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    let excluded = String::from("b");
    let view = graph.get_filtered_view(Some(Box::new(move |v: &&str| **v != excluded)), None);

    assert_eq!(view.ids().collect::<Vec<_>>(), vec![a, c]);
}
