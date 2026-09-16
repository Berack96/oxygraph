use oxygraph::{AdjList, Directed, GraphBuilder, GraphView, ViewVisit, VisitResult};

struct NoopVisitor;

impl<'a> ViewVisit<'a, String, (), u32, AdjList<(), u32>> for NoopVisitor {
    type Directionality = Directed;

    fn visit(&self, _graph: &GraphView<'a, String, (), u32, AdjList<(), u32>>) -> VisitResult {
        Ok(())
    }
}

#[test]
fn visits_a_graph_view() {
    let graph = GraphBuilder::<String, ()>::new().build();
    let view = GraphView::new(&graph);

    assert_eq!(NoopVisitor.visit(&view), Ok(()));
}
