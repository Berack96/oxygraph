//! Biconnected components, available for any graph, directed or undirected: reads
//! [`GraphEdgeStorage::of`] as an undirected adjacency between simple (no parallel edges)
//! vertex pairs, same caveat as [`Bridges`](super::Bridges).

use crate::{edges::GraphEdgeStorage, graph::GraphView, vertices::VertexId, visits::VertexMarks};

/// One entry per biconnected component, each holding its member edges.
pub type BiconnectedEdges<I> = Vec<Vec<Edge<I>>>;

type Edge<I> = (VertexId<I>, VertexId<I>);

/// Partitions every edge the view exposes into biconnected components: maximal edge sets
/// where any two edges lie on a common cycle. A bridge forms its own singleton component; a
/// vertex with no edges belongs to none. An articulation point shared by several components
/// appears in each of them, as the endpoint of their edges.
#[derive(Debug, Clone, Copy)]
pub struct BiconnectedComponents;

impl BiconnectedComponents {
    pub fn visit<V: 'static, S: GraphEdgeStorage, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> BiconnectedEdges<S::Id> {
        let mut disc: VertexMarks<Option<u32>> = VertexMarks::new(None);
        let mut low: VertexMarks<u32> = VertexMarks::new(0);
        let mut timer = 0u32;
        let mut edge_stack: Vec<Edge<S::Id>> = Vec::new();
        let mut components = Vec::new();

        for root in view.ids() {
            if disc.get(root).is_some() {
                continue;
            }
            disc.set(root, Some(timer));
            low.set(root, timer);
            timer += 1;

            let root_neighbors: Vec<VertexId<S::Id>> =
                view.edges().of(root).map(|e| e.to).collect();
            let mut stack = vec![(root, None::<VertexId<S::Id>>, root_neighbors, 0usize)];

            while let Some((vertex, parent, neighbors, mut index)) = stack.pop() {
                if index >= neighbors.len() {
                    if let Some(p) = parent {
                        let child_low = *low.get(vertex);
                        low.set(p, (*low.get(p)).min(child_low));
                        let parent_disc = disc
                            .get(p)
                            .expect("a parent is discovered before its child");
                        if child_low >= parent_disc {
                            let mut component = Vec::new();
                            loop {
                                let edge = edge_stack.pop().expect(
                                    "the tree edge (p, vertex) is always still on the stack",
                                );
                                component.push(edge);
                                if edge == (p, vertex) {
                                    break;
                                }
                            }
                            components.push(component);
                        }
                    }
                    continue;
                }

                let next = neighbors[index];
                index += 1;

                if Some(next) == parent {
                    stack.push((vertex, parent, neighbors, index));
                } else if let Some(next_disc) = *disc.get(next) {
                    // Only a genuine back edge to an ancestor lowers `low`; an already-visited
                    // descendant reached through the mirrored arc of an undirected edge (whose
                    // own back edge was already pushed from its side) is skipped so it isn't
                    // counted into a component twice.
                    let vertex_disc = disc.get(vertex).expect("vertex is discovered by now");
                    if next_disc < vertex_disc {
                        low.set(vertex, (*low.get(vertex)).min(next_disc));
                        edge_stack.push((vertex, next));
                    }
                    stack.push((vertex, parent, neighbors, index));
                } else {
                    disc.set(next, Some(timer));
                    low.set(next, timer);
                    timer += 1;
                    edge_stack.push((vertex, next));
                    let next_neighbors: Vec<VertexId<S::Id>> =
                        view.edges().of(next).map(|e| e.to).collect();
                    stack.push((vertex, parent, neighbors, index));
                    stack.push((next, Some(vertex), next_neighbors, 0));
                }
            }
        }

        components
    }
}
