use oxygraph::{
    VertexId,
    graph_edges::{
        AdjCsr, AdjList, AdjListFixed, AdjMatrix, GraphEdgeStorage, GraphEdgeStorageDirected,
    },
};

#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

const VERTICES: usize = 1_024;
const EDGES_PER_VERTEX: usize = 4;

fn build_adj_list() -> AdjList<(), u32> {
    let mut storage = AdjList::with_capacity(VERTICES);
    add_edges(&mut storage);
    storage
}

fn build_adj_list_fixed() -> AdjListFixed<(), u32, EDGES_PER_VERTEX> {
    let mut storage = AdjListFixed::with_capacity(VERTICES);
    add_edges(&mut storage);
    storage
}

fn build_csr() -> AdjCsr<(), u32> {
    let mut storage = AdjCsr::with_capacity(VERTICES);
    add_edges(&mut storage);
    storage
}

fn build_matrix() -> AdjMatrix<(), u32> {
    let mut storage = AdjMatrix::with_capacity(VERTICES);
    add_edges(&mut storage);
    storage
}

fn add_edges<S: GraphEdgeStorageDirected<Edge = (), Id = u32>>(storage: &mut S) {
    for from in 0..VERTICES {
        for offset in 0..EDGES_PER_VERTEX {
            let to = (from + offset + 1) % VERTICES;
            storage.add_edge_directed(VertexId::new(from), VertexId::new(to), ());
        }
    }
}

#[divan::bench(counter = divan::counter::ItemsCount::new(
    VERTICES * EDGES_PER_VERTEX,
))]
fn adj_list_build() -> AdjList<(), u32> {
    build_adj_list()
}

#[divan::bench(counter = divan::counter::ItemsCount::new(
    VERTICES * EDGES_PER_VERTEX,
))]
fn adj_list_fixed_build() -> AdjListFixed<(), u32, EDGES_PER_VERTEX> {
    build_adj_list_fixed()
}

#[divan::bench(counter = divan::counter::ItemsCount::new(
    VERTICES * EDGES_PER_VERTEX,
))]
fn adj_list_iterate(bencher: divan::Bencher) {
    let storage = build_adj_list();
    bencher.bench(|| divan::black_box(storage.get_all().count()));
}

#[divan::bench(counter = divan::counter::ItemsCount::new(
    VERTICES * EDGES_PER_VERTEX,
))]
fn adj_list_fixed_iterate(bencher: divan::Bencher) {
    let storage = build_adj_list_fixed();
    bencher.bench(|| divan::black_box(storage.get_all().count()));
}

#[divan::bench(counter = divan::counter::ItemsCount::new(
    VERTICES * EDGES_PER_VERTEX,
))]
fn csr_build() -> AdjCsr<(), u32> {
    build_csr()
}

#[divan::bench(counter = divan::counter::ItemsCount::new(
    VERTICES * EDGES_PER_VERTEX,
))]
fn csr_iterate(bencher: divan::Bencher) {
    let storage = build_csr();
    bencher.bench(|| divan::black_box(storage.get_all().count()));
}

#[divan::bench(counter = divan::counter::ItemsCount::new(
    VERTICES * EDGES_PER_VERTEX,
))]
fn matrix_build() -> AdjMatrix<(), u32> {
    build_matrix()
}

#[divan::bench(counter = divan::counter::ItemsCount::new(
    VERTICES * EDGES_PER_VERTEX,
))]
fn matrix_iterate(bencher: divan::Bencher) {
    let storage = build_matrix();
    bencher.bench(|| divan::black_box(storage.get_all().count()));
}

fn main() {
    divan::main();
}
