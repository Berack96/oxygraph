use oxygraph::GraphBuilder;
use oxygraph::graph_edges::{AdjList, GraphEdgeStorage, Weighted};
use oxygraph::graph_view::GraphView;
use oxygraph::graph_visit::{Bfs, Dfs, Dijkstra, ViewVisit, VisitError};

struct NoopVisitor;

impl ViewVisit<String, (), u32, AdjList<(), u32>> for NoopVisitor {
    type Output = ();

    fn visit<G>(
        &self,
        _view: &G,
        _start: oxygraph::VertexId<u32>,
    ) -> oxygraph::graph_visit::VisitResult<()>
    where
        G: GraphView<String, (), u32, AdjList<(), u32>>,
    {
        Ok(())
    }
}

#[test]
fn visits_a_generic_graph_view() {
    let mut graph = GraphBuilder::<String, ()>::new().build();
    let a = graph.add_vertex("a".into());

    assert_eq!(NoopVisitor.visit(&graph, a), Ok(()));
}

#[test]
fn bfs_visits_reachable_vertices_in_order() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    graph.edges_mut().add(a, b, ());
    graph.edges_mut().add(b, c, ());

    let order = Bfs.visit(&graph, a).unwrap();
    assert_eq!(order, vec![a, b, c]);
}

#[test]
fn bfs_reports_missing_start_vertex() {
    let graph = GraphBuilder::<&str, ()>::new().build();
    let missing = oxygraph::VertexId::new(0);

    assert_eq!(Bfs.visit(&graph, missing), Err(VisitError::VertexNotFound));
}

#[test]
fn dfs_visits_reachable_vertices() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    graph.edges_mut().add(a, b, ());
    graph.edges_mut().add(a, c, ());

    let order = Dfs.visit(&graph, a).unwrap();
    assert_eq!(order[0], a);
    assert_eq!(order.len(), 3);
}

struct Km(f64);
impl Weighted for Km {
    fn weight(&self) -> f64 {
        self.0
    }
}

#[test]
fn dijkstra_finds_shortest_path() {
    let mut graph = GraphBuilder::<&str, Km>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    graph.edges_mut().add(a, b, Km(5.0));
    graph.edges_mut().add(a, c, Km(1.0));
    graph.edges_mut().add(c, b, Km(1.0));

    let (path, total) = Dijkstra::new(b).visit(&graph, a).unwrap().unwrap();
    assert_eq!(path, vec![a, c, b]);
    assert_eq!(total, 2.0);
}
