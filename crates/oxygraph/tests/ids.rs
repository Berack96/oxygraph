use oxygraph::VertexId;
use oxygraph::graph_view::EdgeView;

#[test]
fn creates_vertex_ids_from_usize() {
    let id = VertexId::<u16>::new(7);

    assert_eq!(id.id(), 7);
}

#[test]
fn creates_the_highest_representable_vertex_id() {
    let id = VertexId::<u8>::new((u8::MAX - 1) as usize);

    assert_eq!(id.id(), (u8::MAX - 1) as usize);
}

#[test]
fn creates_edge_views() {
    let data = String::from("edge");
    let from = VertexId::<u32>::new(1);
    let to = VertexId::<u32>::new(2);
    let edge = EdgeView::new(from, to, &data);

    assert_eq!(edge.from, from);
    assert_eq!(edge.to, to);
    assert_eq!(edge.data, &data);
}
