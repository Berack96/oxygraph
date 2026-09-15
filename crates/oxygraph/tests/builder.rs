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
}
