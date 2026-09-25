//! Bridges and articulation points, available for any graph, directed or undirected: both
//! read [`GraphEdgeStorage::of`] as an undirected adjacency between simple (no parallel
//! edges) vertex pairs, same caveat as [`ConnectedComponents`](super::ConnectedComponents).

use crate::{edges::GraphEdgeStorage, graph::GraphView, vertices::VertexId, visits::VertexMarks};

/// One entry per bridge: an edge whose removal would split its component in two. Each is
/// reported once, in an arbitrary direction (undirected edges have no canonical one).
pub type BridgeEdges<I> = Vec<(VertexId<I>, VertexId<I>)>;

/// Finds every bridge in the view: an edge that is the only path between the two components
/// it would split into if removed.
#[derive(Debug, Clone, Copy)]
pub struct Bridges;

impl Bridges {
    pub fn visit<V: 'static, S: GraphEdgeStorage, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> BridgeEdges<S::Id> {
        tarjan_low_link(view).0
    }
}

/// Finds every articulation point (cut vertex) in the view: a vertex whose removal would
/// split its component in two.
#[derive(Debug, Clone, Copy)]
pub struct ArticulationPoints;

impl ArticulationPoints {
    pub fn visit<V: 'static, S: GraphEdgeStorage, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> Vec<VertexId<S::Id>> {
        tarjan_low_link(view).1
    }
}

/// Tarjan's low-link DFS, run once for both [`Bridges`] and [`ArticulationPoints`]: they
/// examine the exact same DFS tree, just under different conditions on the same `disc`/`low`
/// values, so there is no reason to walk the graph twice.
fn tarjan_low_link<V: 'static, S: GraphEdgeStorage, G: GraphView<V, S>>(
    view: &G,
) -> (BridgeEdges<S::Id>, Vec<VertexId<S::Id>>) {
    let mut disc: VertexMarks<Option<u32>> = VertexMarks::new(None);
    let mut low: VertexMarks<u32> = VertexMarks::new(0);
    let mut is_cut: VertexMarks<bool> = VertexMarks::new(false);
    let mut timer = 0u32;
    let mut bridges = Vec::new();

    for root in view.ids() {
        if disc.get(root).is_some() {
            continue;
        }
        disc.set(root, Some(timer));
        low.set(root, timer);
        timer += 1;
        let mut root_children = 0u32;

        let root_neighbors: Vec<VertexId<S::Id>> = view.edges().of(root).map(|e| e.to).collect();
        let mut stack = vec![(root, None::<VertexId<S::Id>>, root_neighbors, 0usize)];

        while let Some((vertex, parent, neighbors, mut index)) = stack.pop() {
            if index >= neighbors.len() {
                if let Some(p) = parent {
                    let child_low = *low.get(vertex);
                    low.set(p, (*low.get(p)).min(child_low));
                    let parent_disc = disc
                        .get(p)
                        .expect("a parent is discovered before its child");
                    if child_low > parent_disc {
                        bridges.push((p, vertex));
                    }
                    if p != root && child_low >= parent_disc {
                        is_cut.set(p, true);
                    }
                }
                continue;
            }

            let next = neighbors[index];
            index += 1;
            if Some(next) == parent {
                stack.push((vertex, parent, neighbors, index));
            } else if let Some(next_disc) = *disc.get(next) {
                low.set(vertex, (*low.get(vertex)).min(next_disc));
                stack.push((vertex, parent, neighbors, index));
            } else {
                if vertex == root {
                    root_children += 1;
                }
                disc.set(next, Some(timer));
                low.set(next, timer);
                timer += 1;
                let next_neighbors: Vec<VertexId<S::Id>> =
                    view.edges().of(next).map(|e| e.to).collect();
                stack.push((vertex, parent, neighbors, index));
                stack.push((next, Some(vertex), next_neighbors, 0));
            }
        }

        if root_children > 1 {
            is_cut.set(root, true);
        }
    }

    let articulation_points = view.ids().filter(|&id| *is_cut.get(id)).collect();
    (bridges, articulation_points)
}
