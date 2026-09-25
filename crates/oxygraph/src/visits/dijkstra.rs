//! Dijkstra shortest-path visit, available for any graph, directed or undirected.

use std::{cmp::Ordering, collections::BinaryHeap};

use crate::{
    edges::{GraphEdgeStorage, Weighted},
    graph::GraphView,
    vertices::{UnsignedId, VertexId},
    visits::{VertexMarks, ViewVisit, VisitError, VisitResult},
};

#[derive(Debug, Clone, Copy)]
enum Mark {
    Unvisited,
    Distance(f64),
}

struct HeapEntry<I: UnsignedId> {
    distance: f64,
    vertex: VertexId<I>,
}

impl<I: UnsignedId> PartialEq for HeapEntry<I> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
impl<I: UnsignedId> Eq for HeapEntry<I> {}
impl<I: UnsignedId> PartialOrd for HeapEntry<I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<I: UnsignedId> Ord for HeapEntry<I> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed: BinaryHeap is a max-heap, Dijkstra needs the smallest distance first.
        other
            .distance
            .partial_cmp(&self.distance)
            .unwrap_or(Ordering::Equal)
    }
}

/// Shortest path from a start vertex to `target`: the vertex sequence and its total weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dijkstra<I: UnsignedId> {
    target: VertexId<I>,
}

impl<I: UnsignedId> Dijkstra<I> {
    pub fn new(target: VertexId<I>) -> Self {
        Self { target }
    }
}

impl<V: 'static, E: 'static, I, S> ViewVisit<V, E, I, S> for Dijkstra<I>
where
    I: UnsignedId,
    E: Weighted,
    S: GraphEdgeStorage<E, I>,
{
    type Output = Option<(Vec<VertexId<I>>, f64)>;

    fn visit<G>(&self, view: &G, start: VertexId<I>) -> VisitResult<Self::Output>
    where
        G: GraphView<V, E, I, S>,
    {
        if view.vertex(start).is_none() || view.vertex(self.target).is_none() {
            return Err(VisitError::VertexNotFound);
        }

        let mut marks = VertexMarks::new(Mark::Unvisited);
        let mut previous: VertexMarks<Option<VertexId<I>>> = VertexMarks::new(None);
        let mut heap = BinaryHeap::new();

        marks.set(start, Mark::Distance(0.0));
        heap.push(HeapEntry {
            distance: 0.0,
            vertex: start,
        });

        while let Some(HeapEntry { distance, vertex }) = heap.pop() {
            if vertex == self.target {
                break;
            }
            if matches!(marks.get(vertex), Mark::Distance(known) if *known < distance) {
                continue;
            }

            for edge in view.edges().of(vertex) {
                let next_distance = distance + edge.data.weight();
                let is_shorter = match marks.get(edge.to) {
                    Mark::Unvisited => true,
                    Mark::Distance(known) => next_distance < *known,
                };
                if is_shorter {
                    marks.set(edge.to, Mark::Distance(next_distance));
                    previous.set(edge.to, Some(vertex));
                    heap.push(HeapEntry {
                        distance: next_distance,
                        vertex: edge.to,
                    });
                }
            }
        }

        let Mark::Distance(total) = *marks.get(self.target) else {
            return Ok(None);
        };

        let mut path = vec![self.target];
        while let Some(prev) = *previous.get(*path.last().unwrap()) {
            path.push(prev);
        }
        path.reverse();

        Ok(Some((path, total)))
    }
}
