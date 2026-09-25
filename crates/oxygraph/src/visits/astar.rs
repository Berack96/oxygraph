//! A* shortest-path visit, available for any graph, directed or undirected: Dijkstra guided
//! by a heuristic estimate of the remaining distance to `target`.

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
    priority: f64,
    distance: f64,
    vertex: VertexId<I>,
}

impl<I: UnsignedId> PartialEq for HeapEntry<I> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
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
        // Reversed: BinaryHeap is a max-heap, A* needs the smallest priority first.
        other
            .priority
            .partial_cmp(&self.priority)
            .unwrap_or(Ordering::Equal)
    }
}

/// Shortest path from a start vertex to `target`, guided by `heuristic`: an estimate of the
/// remaining distance from a vertex to `target`. `heuristic` must never overestimate the true
/// remaining distance (be admissible), or the returned path may not be the shortest.
pub struct AStar<I: UnsignedId, H> {
    target: VertexId<I>,
    heuristic: H,
}

impl<I: UnsignedId, H> AStar<I, H>
where
    H: Fn(VertexId<I>) -> f64,
{
    pub fn new(target: VertexId<I>, heuristic: H) -> Self {
        Self { target, heuristic }
    }
}

impl<V: 'static, S: GraphEdgeStorage, H> ViewVisit<V, S> for AStar<S::Id, H>
where
    S::Edge: Weighted,
    H: Fn(VertexId<S::Id>) -> f64,
{
    type Output = Option<(Vec<VertexId<S::Id>>, f64)>;

    fn visit<G>(&self, view: &G, start: VertexId<S::Id>) -> VisitResult<Self::Output>
    where
        G: GraphView<V, S>,
    {
        if view.vertex(start).is_none() || view.vertex(self.target).is_none() {
            return Err(VisitError::VertexNotFound);
        }

        let mut marks = VertexMarks::new(Mark::Unvisited);
        let mut previous: VertexMarks<Option<VertexId<S::Id>>> = VertexMarks::new(None);
        let mut heap = BinaryHeap::new();

        marks.set(start, Mark::Distance(0.0));
        heap.push(HeapEntry {
            priority: (self.heuristic)(start),
            distance: 0.0,
            vertex: start,
        });

        while let Some(HeapEntry {
            distance, vertex, ..
        }) = heap.pop()
        {
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
                        priority: next_distance + (self.heuristic)(edge.to),
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
