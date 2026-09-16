#[cfg(test)]
mod test_builder {
    use oxygraph::GraphBuilder;

    #[test]
    fn builds_dynamic_adjacency_list_with_default_capacity() {
        let mut graph = GraphBuilder::<String, f32>::new().build();

        graph.run_with(|vertices, _| assert_eq!(vertices.capacity(), 1));
    }

    #[test]
    fn keeps_requested_capacity_for_dynamic_storage() {
        let mut graph = GraphBuilder::<String, f32>::new().with_capacity(8).build();

        graph.run_with(|vertices, _| assert_eq!(vertices.capacity(), 8));
    }

    #[test]
    fn replaces_zero_capacity_with_one() {
        let mut graph = GraphBuilder::<String, f32>::new().with_capacity(0).build();

        graph.run_with(|vertices, _| assert_eq!(vertices.capacity(), 1));
    }

    #[test]
    fn builds_fixed_adjacency_list() {
        let mut graph = GraphBuilder::<String, f32>::new()
            .directed(true)
            .with_capacity(4)
            .as_adjlist_fixed_max_degree::<6>()
            .build();

        graph.run_with(|vertices, _| assert_eq!(vertices.capacity(), 4));
    }

    #[test]
    fn changes_vertex_index_type_with_dynamic_storage() {
        let mut graph = GraphBuilder::<String, f32>::new()
            .change_vec_indexing::<u16>()
            .build();

        let vertex_id = graph.add_vertex(String::from("vertex"));

        assert_eq!(vertex_id, oxygraph::VertexId::<u16>::new(0));
        assert_eq!(graph.get_vertex(vertex_id), Some(&String::from("vertex")));
    }

    #[test]
    fn changes_vertex_index_type_with_fixed_storage() {
        let mut graph = GraphBuilder::<String, f32>::new()
            .as_adjlist_fixed_max_degree::<6>()
            .change_vec_indexing::<u16>()
            .build();

        let vertex_id = graph.add_vertex(String::from("vertex"));

        assert_eq!(vertex_id, oxygraph::VertexId::<u16>::new(0));
        assert_eq!(graph.get_vertex(vertex_id), Some(&String::from("vertex")));
    }
}
