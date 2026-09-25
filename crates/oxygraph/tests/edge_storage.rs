macro_rules! storage_tests {
    ($( $module:ident => $storage:ty ),+ $(,)?) => {
        $(
            mod $module {
                use oxygraph::VertexId;
                use oxygraph::graph_edges::{GraphEdgeStorage, GraphEdgeStorageDirected};

                fn storage() -> $storage {
                    <$storage>::new()
                }

                #[test]
                fn adds_and_iterates_edges() {
                    let mut storage = storage();
                    let from = VertexId::new(1);
                    let to = VertexId::new(2);

                    storage.add_edge_directed(from, to, String::from("edge"));

                    assert!(storage.has_edge(&from, &to));
                    let edges = storage.of(from).collect::<Vec<_>>();
                    assert_eq!(edges.len(), 1);
                    assert_eq!(edges[0].from, from);
                    assert_eq!(edges[0].to, to);
                    assert_eq!(edges[0].data, "edge");
                    assert_eq!(storage.get_all().count(), 1);
                }

                #[test]
                fn removes_edges() {
                    let mut storage = storage();
                    let from = VertexId::new(0);
                    let to = VertexId::new(1);
                    storage.add_edge_directed(from, to, String::from("edge"));

                    assert_eq!(
                        storage.remove_edge_directed(from, to),
                        Some(String::from("edge"))
                    );
                    assert!(!storage.has_edge(&from, &to));
                    assert_eq!(storage.remove_edge_directed(from, to), None);
                }

                #[test]
                fn handles_unknown_vertices() {
                    let storage = storage();
                    let from = VertexId::new(1);
                    let to = VertexId::new(2);

                    assert!(!storage.has_edge(&from, &to));
                    assert_eq!(storage.of(from).count(), 0);
                }

                #[test]
                fn add_edge_is_undirected() {
                    let mut storage = storage();
                    let a = VertexId::new(0);
                    let b = VertexId::new(1);

                    storage.add_edge(a, b, String::from("edge"));

                    assert!(storage.has_edge(&a, &b));
                    assert!(storage.has_edge(&b, &a));
                    assert_eq!(storage.get_all().count(), 2);
                }

                #[test]
                fn add_edge_self_loop_is_not_duplicated() {
                    let mut storage = storage();
                    let a = VertexId::new(0);

                    storage.add_edge(a, a, String::from("loop"));

                    assert_eq!(storage.get_all().count(), 1);
                }

                #[test]
                fn add_edge_keeps_lower_indexed_rows_when_growing_from_a_higher_vertex() {
                    let mut storage = storage();
                    let a = VertexId::new(2);
                    let b = VertexId::new(0);

                    storage.add_edge(a, b, String::from("edge"));

                    assert!(storage.has_edge(&a, &b));
                    assert!(storage.has_edge(&b, &a));
                    assert_eq!(storage.get_all().count(), 2);
                }

                #[test]
                fn remove_edge_clears_both_directions() {
                    let mut storage = storage();
                    let a = VertexId::new(0);
                    let b = VertexId::new(1);
                    storage.add_edge(a, b, String::from("edge"));

                    assert_eq!(storage.remove_edge(a, b), Some(String::from("edge")));
                    assert!(!storage.has_edge(&a, &b));
                    assert!(!storage.has_edge(&b, &a));
                }
            }
        )+
    };
}

storage_tests! {
    adj_list => oxygraph::graph_edges::AdjList<String, u32>,
    adj_list_fixed => oxygraph::graph_edges::AdjListFixed<String, u32, 2>,
    adj_csr => oxygraph::graph_edges::AdjCsr<String, u32>,
    adj_matrix => oxygraph::graph_edges::AdjMatrix<String, u32>,
}

#[cfg(test)]
mod fixed_storage_tests {
    use oxygraph::VertexId;
    use oxygraph::graph_edges::{AdjListFixed, GraphEdgeStorageDirected};

    #[test]
    fn reuses_a_slot_after_removing_an_edge() {
        let mut storage = AdjListFixed::<String, u32, 1>::new();
        let from = VertexId::new(0);

        storage.add_edge_directed(from, VertexId::new(1), String::from("first"));
        assert_eq!(
            storage.remove_edge_directed(from, VertexId::new(1)),
            Some(String::from("first"))
        );
        storage.add_edge_directed(from, VertexId::new(2), String::from("second"));

        let edges = storage.children_of(from).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].to, VertexId::new(2));
        assert_eq!(edges[0].data, "second");
    }

    #[test]
    #[should_panic(expected = "maximum degree exceeded")]
    fn rejects_edges_above_maximum_degree() {
        let mut storage = AdjListFixed::<String, u32, 1>::new();
        let from = VertexId::new(0);

        storage.add_edge_directed(from, VertexId::new(1), String::from("first"));
        storage.add_edge_directed(from, VertexId::new(2), String::from("second"));
    }
}

#[cfg(test)]
mod memory_layout_tests {
    use std::{
        mem::size_of,
        num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroUsize},
    };

    use oxygraph::VertexId;

    #[allow(dead_code)]
    struct EdgeU8<E> {
        to: VertexId<u8>,
        data: E,
    }

    #[allow(dead_code)]
    struct EdgeU16<E> {
        to: VertexId<u16>,
        data: E,
    }

    #[allow(dead_code)]
    struct EdgeU32<E> {
        to: VertexId<u32>,
        data: E,
    }

    #[allow(dead_code)]
    struct EdgeU64<E> {
        to: VertexId<u64>,
        data: E,
    }

    #[allow(dead_code)]
    struct EdgeUsize<E> {
        to: VertexId<usize>,
        data: E,
    }

    #[allow(dead_code)]
    struct NestedPayload {
        prefix: u8,
        values: [u64; 3],
        suffix: Option<NonZeroU16>,
    }

    #[allow(dead_code)]
    struct WidePayload {
        bytes: [u8; 17],
        number: u128,
    }

    macro_rules! assert_edge_layout {
        ($edge:ident, $vertex:ty, $nonzero:ty) => {
            assert_eq!(size_of::<$vertex>(), size_of::<$nonzero>());
            assert_eq!(size_of::<$edge<u8>>(), size_of::<Option<$edge<u8>>>());
            assert_eq!(size_of::<$edge<u64>>(), size_of::<Option<$edge<u64>>>());
            assert_eq!(
                size_of::<$edge<[u8; 32]>>(),
                size_of::<Option<$edge<[u8; 32]>>>(),
            );
            assert_eq!(
                size_of::<$edge<(u16, u32, [u8; 5])>>(),
                size_of::<Option<$edge<(u16, u32, [u8; 5])>>>(),
            );
            assert_eq!(
                size_of::<$edge<NestedPayload>>(),
                size_of::<Option<$edge<NestedPayload>>>(),
            );
            assert_eq!(
                size_of::<$edge<WidePayload>>(),
                size_of::<Option<$edge<WidePayload>>>(),
            );
            assert_eq!(
                size_of::<$edge<String>>(),
                size_of::<Option<$edge<String>>>(),
            );
        };
    }

    #[test]
    fn option_edge_uses_vertex_id_niche_for_varied_layouts() {
        assert_edge_layout!(EdgeU8, VertexId<u8>, NonZeroU8);
        assert_edge_layout!(EdgeU16, VertexId<u16>, NonZeroU16);
        assert_edge_layout!(EdgeU32, VertexId<u32>, NonZeroU32);
        assert_edge_layout!(EdgeU64, VertexId<u64>, NonZeroU64);
        assert_edge_layout!(EdgeUsize, VertexId<usize>, NonZeroUsize);
    }
}
