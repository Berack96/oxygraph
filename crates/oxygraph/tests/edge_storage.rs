macro_rules! storage_tests {
    ($( $module:ident => $storage:ty ),+ $(,)?) => {
        $(
            mod $module {
                use oxygraph::{GraphEdgeIter, GraphEdgeStorage, VertexId};

                fn storage() -> $storage {
                    <$storage>::new()
                }

                #[test]
                fn adds_and_iterates_edges() {
                    let mut storage = storage();
                    let from = VertexId::new(1);
                    let to = VertexId::new(2);

                    storage.add_edge(from, to, String::from("edge"));

                    assert!(storage.has_edge(&from, &to));
                    let edges = storage.edges_of(from).collect::<Vec<_>>();
                    assert_eq!(edges.len(), 1);
                    assert_eq!(edges[0].from, from);
                    assert_eq!(edges[0].to, to);
                    assert_eq!(edges[0].data, "edge");
                    assert_eq!(storage.edges().count(), 1);
                }

                #[test]
                fn removes_edges() {
                    let mut storage = storage();
                    let from = VertexId::new(0);
                    let to = VertexId::new(1);
                    storage.add_edge(from, to, String::from("edge"));

                    assert_eq!(storage.remove_edge(from, to), Some(String::from("edge")));
                    assert!(!storage.has_edge(&from, &to));
                    assert_eq!(storage.remove_edge(from, to), None);
                }

                #[test]
                fn handles_unknown_vertices() {
                    let storage = storage();
                    let from = VertexId::new(1);
                    let to = VertexId::new(2);

                    assert!(!storage.has_edge(&from, &to));
                    assert_eq!(storage.edges_of(from).count(), 0);
                }
            }
        )+
    };
}

storage_tests! {
    adj_list => oxygraph::AdjList<String, u32>,
}
