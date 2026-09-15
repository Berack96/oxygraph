use oxygraph::{EdgeView, VertexId};

#[test]
fn creates_vertex_ids_from_usize() {
    let id = VertexId::<u16>::new(7);

    assert_eq!(id.id(), 7);
}

#[test]
fn exposes_the_index_type_maximum() {
    assert_eq!(VertexId::<u8>::max(), u8::MAX);
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
