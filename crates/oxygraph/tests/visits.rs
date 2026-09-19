use oxygraph::GraphBuilder;
use oxygraph::graph_edges::AdjList;
use oxygraph::graph_view::GraphFilteredView;
use oxygraph::graph_visit::{ViewVisit, VisitResult};

struct NoopVisitor;

impl<'a> ViewVisit<'a, String, (), u32, AdjList<(), u32>> for NoopVisitor {
    fn visit(
        &self,
        _graph: &GraphFilteredView<'a, String, (), u32, AdjList<(), u32>>,
    ) -> VisitResult {
        Ok(())
    }
}

#[test]
fn visits_a_graph_view() {
    let graph = GraphBuilder::<String, ()>::new().build();
    let view = graph.get_filtered_view(None, None);

    assert_eq!(NoopVisitor.visit(&view), Ok(()));
}
