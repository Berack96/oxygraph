use oxygraph::{Directed, GraphDirectionality, Undirected};

#[test]
fn reports_directed_storage() {
    assert!(Directed.is_directed());
}

#[test]
fn reports_undirected_storage() {
    assert!(!Undirected.is_directed());
}