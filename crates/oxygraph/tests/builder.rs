#[cfg(test)]
mod test_builder {
    use oxygraph::{GraphBuilder, VertexId, graph_view::GraphView};

    #[test]
    fn builds_dynamic_adjacency_list_with_default_capacity() {
        let graph = GraphBuilder::<String, f32>::new().build();
        assert!(graph.len() == 0);
        assert!(graph.is_empty());
        assert!(graph.vertex(VertexId::new(0)).is_none());
    }

    #[test]
    fn keeps_requested_capacity_for_dynamic_storage() {
        let graph = GraphBuilder::<&str, f32>::new()
            .with_vertices(vec!["a", "b", "c", "d", "e", "f", "g", "h"])
            .build();

        assert!(graph.len() == 8);
        assert!(!graph.is_empty());
        assert!(graph.vertex(VertexId::new(0)).is_some());
        assert!(graph.vertex(VertexId::new(7)).is_some());
        assert!(graph.vertex(VertexId::new(8)).is_none());
    }

    #[test]
    fn keeps_zero_capacity_for_empty_vertices() {
        let graph = GraphBuilder::<&str, f32>::new()
            .with_vertices(vec![])
            .build();
        assert!(graph.len() == 0);
        assert!(graph.is_empty());
        assert!(graph.vertex(VertexId::new(0)).is_none());
    }

    #[test]
    fn builds_fixed_adjacency_list() {
        let graph = GraphBuilder::<String, f32>::new()
            .directed(true)
            .with_vertices(vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
                String::from("d"),
            ])
            .as_adjlist_fixed_max_degree::<6>()
            .build();

        assert!(graph.len() == 4);
        assert!(!graph.is_empty());
        assert!(graph.vertex(VertexId::new(0)).is_some());
        assert!(graph.vertex(VertexId::new(3)).is_some());
        assert!(graph.vertex(VertexId::new(4)).is_none());
    }
}
