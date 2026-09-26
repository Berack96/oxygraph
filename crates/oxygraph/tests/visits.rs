use oxygraph::GraphBuilder;
use oxygraph::graph_edges::{AdjList, GraphEdgeStorage, GraphEdgeStorageDirected, Weighted};
use oxygraph::graph_view::GraphView;
use oxygraph::graph_visit::{
    ArticulationPoints, Bfs, Bridges, ConnectedComponents, Dfs, Dijkstra, DijkstraReached,
    MultiSourceBfs, MultiSourceDijkstra, Reached, StronglyConnectedComponents, ViewVisit,
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

#[test]
fn dfs_ignores_disconnected_vertices() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let _c = graph.add_vertex("c");
    graph.edges_mut().add_edge_directed(a, b, ());

    let order = Dfs.visit(&graph, a).unwrap();
    assert_eq!(order, vec![a, b]);
}

#[test]
fn dfs_handles_self_loop() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    graph.edges_mut().add_edge_directed(a, a, ());

    let order = Dfs.visit(&graph, a).unwrap();
    assert_eq!(order, vec![a]);
}

#[test]
fn bfs_reports_missing_start_vertex_when_excluded_by_a_filter() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let view = graph.get_filtered_view(Some(|v: &&str| *v != "a"), None);

    assert_eq!(Bfs.visit(&view, a), Err(VisitError::VertexNotFound));
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

#[test]
fn connected_components_groups_vertices_reachable_from_one_another() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    let d = graph.add_vertex("d");
    let e = graph.add_vertex("e");
    let f = graph.add_vertex("f");
    graph.edges_mut().add_edge(a, b, ());
    graph.edges_mut().add_edge(b, c, ());
    graph.edges_mut().add_edge(d, e, ());

    let components = ConnectedComponents.visit(&graph);

    assert_eq!(components, vec![vec![a, b, c], vec![d, e], vec![f]]);
}

#[test]
fn connected_components_of_an_empty_graph_is_empty() {
    let graph = GraphBuilder::<&str, ()>::new().build();

    assert_eq!(ConnectedComponents.visit(&graph), Vec::<Vec<_>>::new());
}

#[test]
fn scc_groups_mutually_reachable_vertices() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    let d = graph.add_vertex("d");
    let e = graph.add_vertex("e");
    // a -> b -> c -> a is a cycle (one SCC); c -> d -> e is a one-way tail (two singletons).
    graph.edges_mut().add_edge_directed(a, b, ());
    graph.edges_mut().add_edge_directed(b, c, ());
    graph.edges_mut().add_edge_directed(c, a, ());
    graph.edges_mut().add_edge_directed(c, d, ());
    graph.edges_mut().add_edge_directed(d, e, ());

    let mut components = StronglyConnectedComponents.visit(&graph);
    for component in &mut components {
        component.sort_by_key(|v| v.id());
    }
    components.sort_by_key(|component| component[0].id());

    assert_eq!(components, vec![vec![a, b, c], vec![d], vec![e]]);
}

#[test]
fn scc_of_an_empty_graph_is_empty() {
    let graph = GraphBuilder::<&str, ()>::new().build();

    assert_eq!(
        StronglyConnectedComponents.visit(&graph),
        Vec::<Vec<_>>::new()
    );
}

#[test]
fn scc_of_a_single_isolated_vertex() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");

    assert_eq!(StronglyConnectedComponents.visit(&graph), vec![vec![a]]);
}

#[test]
fn scc_groups_a_self_loop_into_its_own_component() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    graph.edges_mut().add_edge_directed(a, a, ());
    graph.edges_mut().add_edge_directed(a, b, ());

    let mut components = StronglyConnectedComponents.visit(&graph);
    for component in &mut components {
        component.sort_by_key(|v| v.id());
    }
    components.sort_by_key(|component| component[0].id());

    assert_eq!(components, vec![vec![a], vec![b]]);
}

/// Two triangles {a, b, c} and {d, e, f} joined by a single edge c-d: that edge is the only
/// bridge, and its two endpoints are the only articulation points.
#[allow(clippy::type_complexity)]
fn two_triangles_joined_by_a_bridge() -> (
    oxygraph::Graph<&'static str, AdjList<(), u32>>,
    [oxygraph::VertexId<u32>; 6],
) {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    let d = graph.add_vertex("d");
    let e = graph.add_vertex("e");
    let f = graph.add_vertex("f");
    graph.edges_mut().add_edge(a, b, ());
    graph.edges_mut().add_edge(b, c, ());
    graph.edges_mut().add_edge(c, a, ());
    graph.edges_mut().add_edge(d, e, ());
    graph.edges_mut().add_edge(e, f, ());
    graph.edges_mut().add_edge(f, d, ());
    graph.edges_mut().add_edge(c, d, ());
    (graph, [a, b, c, d, e, f])
}

#[test]
fn bridges_finds_the_single_cut_edge_between_two_cycles() {
    let (graph, [_a, _b, c, d, _e, _f]) = two_triangles_joined_by_a_bridge();

    let bridges = Bridges.visit(&graph);

    assert_eq!(bridges.len(), 1);
    let (from, to) = bridges[0];
    assert!((from == c && to == d) || (from == d && to == c));
}

#[test]
fn bridges_finds_none_inside_a_single_cycle() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    graph.edges_mut().add_edge(a, b, ());
    graph.edges_mut().add_edge(b, c, ());
    graph.edges_mut().add_edge(c, a, ());

    assert_eq!(Bridges.visit(&graph), Vec::new());
}

#[test]
fn articulation_points_finds_the_bridge_endpoints() {
    let (graph, [_a, _b, c, d, _e, _f]) = two_triangles_joined_by_a_bridge();

    let mut points = ArticulationPoints.visit(&graph);
    points.sort_by_key(|v| v.id());
    let mut expected = [c, d];
    expected.sort_by_key(|v| v.id());

    assert_eq!(points, expected);
}

#[test]
fn articulation_points_finds_the_shared_vertex_of_a_bowtie() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let p1 = graph.add_vertex("p1");
    let p2 = graph.add_vertex("p2");
    let m = graph.add_vertex("m");
    let q1 = graph.add_vertex("q1");
    let q2 = graph.add_vertex("q2");
    graph.edges_mut().add_edge(p1, p2, ());
    graph.edges_mut().add_edge(p2, m, ());
    graph.edges_mut().add_edge(m, p1, ());
    graph.edges_mut().add_edge(q1, q2, ());
    graph.edges_mut().add_edge(q2, m, ());
    graph.edges_mut().add_edge(m, q1, ());

    // no single edge disconnects a bowtie, only its shared vertex does.
    assert_eq!(Bridges.visit(&graph), Vec::new());
    assert_eq!(ArticulationPoints.visit(&graph), vec![m]);
}

#[test]
fn bridges_and_articulation_points_ignore_a_self_loop() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    graph.edges_mut().add_edge(a, b, ());
    graph.edges_mut().add_edge(b, c, ());
    graph.edges_mut().add_edge(c, a, ());
    graph.edges_mut().add_edge(a, a, ());

    assert_eq!(Bridges.visit(&graph), Vec::new());
    assert_eq!(ArticulationPoints.visit(&graph), Vec::new());
}

#[test]
fn bridges_and_articulation_points_of_an_empty_graph_are_empty() {
    let graph = GraphBuilder::<&str, ()>::new().build();

    assert_eq!(Bridges.visit(&graph), Vec::new());
    assert_eq!(ArticulationPoints.visit(&graph), Vec::new());
}

#[test]
fn bridges_and_articulation_points_do_not_interfere_across_disconnected_components() {
    let (mut graph, [_a, _b, c, d, _e, _f]) = two_triangles_joined_by_a_bridge();
    let g = graph.add_vertex("g");
    let h = graph.add_vertex("h");
    let i = graph.add_vertex("i");
    let j = graph.add_vertex("j");
    let k = graph.add_vertex("k");
    let l = graph.add_vertex("l");
    graph.edges_mut().add_edge(g, h, ());
    graph.edges_mut().add_edge(h, i, ());
    graph.edges_mut().add_edge(i, g, ());
    graph.edges_mut().add_edge(j, k, ());
    graph.edges_mut().add_edge(k, l, ());
    graph.edges_mut().add_edge(l, j, ());
    graph.edges_mut().add_edge(i, j, ());

    // Two independent bridge structures in one graph: the shared DFS timer used across
    // `for root in view.ids()` roots must not let one component's discovery order corrupt
    // the other's low-link comparisons.
    let bridges = Bridges.visit(&graph);
    let mut points = ArticulationPoints.visit(&graph);
    points.sort_by_key(|v| v.id());
    let mut expected_points = [c, d, i, j];
    expected_points.sort_by_key(|v| v.id());

    let has_bridge = |x, y| {
        bridges
            .iter()
            .any(|&(from, to)| (from == x && to == y) || (from == y && to == x))
    };
    assert_eq!(bridges.len(), 2);
    assert!(has_bridge(c, d));
    assert!(has_bridge(i, j));
    assert_eq!(points, expected_points);
}

#[test]
fn connected_components_only_follows_outgoing_edges_on_a_one_way_graph() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    graph.edges_mut().add_edge_directed(b, a, ()); // b -> a, one-way

    let components = ConnectedComponents.visit(&graph);

    // a has no outgoing edge, so it's claimed as its own component before b's outgoing edge
    // to a can merge them: the documented one-way caveat, not a real pair of islands.
    assert_eq!(components, vec![vec![a], vec![b]]);
}

#[test]
fn multi_source_bfs_ignores_duplicate_sources() {
    let mut graph = GraphBuilder::<&str, ()>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    graph.edges_mut().add_edge(a, b, ());

    let reached = MultiSourceBfs.visit(&graph, [a, a]).unwrap();

    assert_eq!(
        reached,
        vec![
            (
                a,
                Reached {
                    distance: 0,
                    source: a
                }
            ),
            (
                b,
                Reached {
                    distance: 1,
                    source: a
                }
            )
        ]
    );
}

#[test]
fn multi_source_dijkstra_ignores_duplicate_sources() {
    let mut graph = GraphBuilder::<&str, Km>::new().build();
    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    graph.edges_mut().add_edge_directed(a, b, Km(1.0));

    let reached = MultiSourceDijkstra.visit(&graph, [a, a]).unwrap();

    assert_eq!(
        reached,
        vec![
            (
                a,
                DijkstraReached {
                    distance: 0.0,
                    source: a
                }
            ),
            (
                b,
                DijkstraReached {
                    distance: 1.0,
                    source: a
                }
            )
        ]
    );
}
