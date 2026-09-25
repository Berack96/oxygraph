use oxygraph::GraphBuilder;
use oxygraph::graph_edges::{AdjList, GraphEdgeStorage, GraphEdgeStorageDirected, Weighted};
use oxygraph::graph_view::GraphView;
use oxygraph::graph_visit::{
    Bfs, Dfs, Dijkstra, DijkstraReached, MultiSourceBfs, MultiSourceDijkstra, Reached, ViewVisit,
    VisitError,
};

struct NoopVisitor;

impl ViewVisit<String, AdjList<(), u32>> for NoopVisitor {
    type Output = ();

    fn visit<G>(
        &self,
        _view: &G,
        _start: oxygraph::VertexId<u32>,
    ) -> oxygraph::graph_visit::VisitResult<()>
    where
        G: GraphView<String, AdjList<(), u32>>,
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
    graph.edges_mut().add_edge_directed(a, b, ());
    graph.edges_mut().add_edge_directed(b, c, ());

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
fn bfs_ignores_disconnected_vertices() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let _c = graph.add_vertex("c");
    graph.edges_mut().add_edge_directed(a, b, ());

    let order = Bfs.visit(&graph, a).unwrap();
    assert_eq!(order, vec![a, b]);
}

#[test]
fn bfs_handles_self_loop() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    graph.edges_mut().add_edge_directed(a, a, ());

    let order = Bfs.visit(&graph, a).unwrap();
    assert_eq!(order, vec![a]);
}

#[test]
fn dfs_visits_reachable_vertices() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    graph.edges_mut().add_edge_directed(a, b, ());
    graph.edges_mut().add_edge_directed(a, c, ());

    // Iterative DFS pops the stack LIFO, so children are visited in reverse
    // of the order children_of() yields them: c (pushed last) before b.
    let order = Dfs.visit(&graph, a).unwrap();
    assert_eq!(order, vec![a, c, b]);
}

#[test]
fn dfs_reports_missing_start_vertex() {
    let graph = GraphBuilder::<&str, ()>::new().build();
    let missing = oxygraph::VertexId::new(0);

    assert_eq!(Dfs.visit(&graph, missing), Err(VisitError::VertexNotFound));
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    graph.edges_mut().add_edge_directed(a, b, Km(5.0));
    graph.edges_mut().add_edge_directed(a, c, Km(1.0));
    graph.edges_mut().add_edge_directed(c, b, Km(1.0));

    let (path, total) = Dijkstra::new(b).visit(&graph, a).unwrap().unwrap();
    assert_eq!(path, vec![a, c, b]);
    assert_eq!(total, 2.0);
}

#[test]
fn dijkstra_returns_none_for_unreachable_target() {
    let mut graph = GraphBuilder::<&str, Km>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");

    assert_eq!(Dijkstra::new(b).visit(&graph, a).unwrap(), None);
}

#[test]
fn dijkstra_reports_missing_start_vertex() {
    let mut graph = GraphBuilder::<&str, Km>::new().build();
    let b = graph.add_vertex("b");
    let missing = oxygraph::VertexId::new(1);

    assert_eq!(
        Dijkstra::new(b).visit(&graph, missing),
        Err(VisitError::VertexNotFound)
    );
}

#[test]
fn dijkstra_reports_missing_target_vertex() {
    let mut graph = GraphBuilder::<&str, Km>::new().build();
    let a = graph.add_vertex("a");
    let missing = oxygraph::VertexId::new(1);

    assert_eq!(
        Dijkstra::new(missing).visit(&graph, a),
        Err(VisitError::VertexNotFound)
    );
}

#[test]
fn multi_source_bfs_attributes_each_vertex_to_the_nearest_source() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let s1 = graph.add_vertex("s1");
    let x = graph.add_vertex("x");
    let s2 = graph.add_vertex("s2");
    let far = graph.add_vertex("far");
    graph.edges_mut().add_edge(s1, x, ());
    graph.edges_mut().add_edge(s2, x, ());
    graph.edges_mut().add_edge(s2, far, ());

    let reached = MultiSourceBfs.visit(&graph, [s1, s2]).unwrap();
    let of = |v| reached.iter().find(|(id, _)| *id == v).map(|(_, r)| *r);

    assert_eq!(
        of(s1),
        Some(Reached {
            distance: 0,
            source: s1
        })
    );
    assert_eq!(
        of(s2),
        Some(Reached {
            distance: 0,
            source: s2
        })
    );
    // x is equidistant from s1 and s2: ties are broken by input order, s1 first.
    assert_eq!(
        of(x),
        Some(Reached {
            distance: 1,
            source: s1
        })
    );
    assert_eq!(
        of(far),
        Some(Reached {
            distance: 1,
            source: s2
        })
    );
}

#[test]
fn multi_source_bfs_omits_unreached_vertices() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let _isolated = graph.add_vertex("isolated");

    let reached = MultiSourceBfs.visit(&graph, [a]).unwrap();
    assert_eq!(
        reached,
        vec![(
            a,
            Reached {
                distance: 0,
                source: a
            }
        )]
    );
}

#[test]
fn multi_source_bfs_reports_missing_source_vertex() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let missing = oxygraph::VertexId::new(5);

    assert_eq!(
        MultiSourceBfs.visit(&graph, [a, missing]),
        Err(VisitError::VertexNotFound)
    );
}

#[test]
fn multi_source_dijkstra_attributes_each_vertex_to_the_cheapest_source() {
    let mut graph = GraphBuilder::<&str, Km>::new().build();
    let s1 = graph.add_vertex("s1");
    let s2 = graph.add_vertex("s2");
    let x = graph.add_vertex("x");
    let y = graph.add_vertex("y");
    graph.edges_mut().add_edge_directed(s1, x, Km(5.0));
    graph.edges_mut().add_edge_directed(s2, x, Km(1.0));
    graph.edges_mut().add_edge_directed(s1, y, Km(2.0));
    graph.edges_mut().add_edge_directed(s2, y, Km(2.0));

    let reached = MultiSourceDijkstra.visit(&graph, [s1, s2]).unwrap();
    let of = |v| reached.iter().find(|(id, _)| *id == v).map(|(_, r)| *r);

    assert_eq!(
        of(s1),
        Some(DijkstraReached {
            distance: 0.0,
            source: s1
        })
    );
    assert_eq!(
        of(s2),
        Some(DijkstraReached {
            distance: 0.0,
            source: s2
        })
    );
    // s2 reaches x more cheaply (1.0 < 5.0), despite s1 being first in input order.
    assert_eq!(
        of(x),
        Some(DijkstraReached {
            distance: 1.0,
            source: s2
        })
    );
    // y is equidistant from both sources: ties are broken by input order, s1 first.
    assert_eq!(
        of(y),
        Some(DijkstraReached {
            distance: 2.0,
            source: s1
        })
    );
}

#[test]
fn multi_source_dijkstra_omits_unreached_vertices() {
    let mut graph = GraphBuilder::<&str, Km>::new().build();
    let a = graph.add_vertex("a");
    let _isolated = graph.add_vertex("isolated");

    let reached = MultiSourceDijkstra.visit(&graph, [a]).unwrap();
    assert_eq!(
        reached,
        vec![(
            a,
            DijkstraReached {
                distance: 0.0,
                source: a
            }
        )]
    );
}

#[test]
fn multi_source_dijkstra_reports_missing_source_vertex() {
    let mut graph = GraphBuilder::<&str, Km>::new().build();
    let a = graph.add_vertex("a");
    let missing = oxygraph::VertexId::new(5);

    assert_eq!(
        MultiSourceDijkstra.visit(&graph, [a, missing]),
        Err(VisitError::VertexNotFound)
    );
}
