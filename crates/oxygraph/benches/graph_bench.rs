use oxygraph::{
    VertexId,
    graph_edges::{AdjList, AdjListFixed, GraphEdgeStorage},
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

fn add_edges<S: GraphEdgeStorage<Edge = (), Id = u32>>(storage: &mut S) {
    for from in 0..VERTICES {
        for offset in 0..EDGES_PER_VERTEX {
            let to = (from + offset + 1) % VERTICES;
            storage.add(VertexId::new(from), VertexId::new(to), ());
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

fn main() {
    divan::main();
}
