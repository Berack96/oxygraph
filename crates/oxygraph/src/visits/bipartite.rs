//! Bipartiteness check, available for any graph, directed or undirected: reads
//! [`GraphEdgeStorage::of`] as an undirected adjacency, same caveat as
//! [`ConnectedComponents`](super::ConnectedComponents).

use std::collections::VecDeque;

use crate::{edges::GraphEdgeStorage, graph::GraphView, vertices::VertexId, visits::VertexMarks};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    A,
    B,
}

/// The two color classes of a bipartite graph.
pub type Bipartition<I> = (Vec<VertexId<I>>, Vec<VertexId<I>>);

/// Checks whether the view is bipartite: every edge connects a vertex in one class to a
/// vertex in the other. `None` if it isn't (some edge connects two vertices in the same
/// class, as an odd cycle or a self-loop would).
#[derive(Debug, Clone, Copy)]
pub struct Bipartite;

impl Bipartite {
    pub fn visit<V: 'static, S: GraphEdgeStorage, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> Option<Bipartition<S::Id>> {
        let mut colors: VertexMarks<Option<Color>> = VertexMarks::new(None);

        for start in view.ids() {
            if colors.get(start).is_some() {
                continue;
            }

            colors.set(start, Some(Color::A));
            let mut queue = VecDeque::from([start]);

            while let Some(current) = queue.pop_front() {
                let current_color: Color = colors
                    .get(current)
                    .expect("queued vertices are always colored");
                let next_color = match current_color {
                    Color::A => Color::B,
                    Color::B => Color::A,
                };

                for edge in view.edges().of(current) {
                    match *colors.get(edge.to) {
                        None => {
                            colors.set(edge.to, Some(next_color));
                            queue.push_back(edge.to);
                        }
                        Some(existing) if existing == current_color => return None,
                        Some(_) => {}
                    }
                }
            }
        }

        let mut class_a = Vec::new();
        let mut class_b = Vec::new();
        for id in view.ids() {
            match colors.get(id).expect("every vertex is colored by now") {
                Color::A => class_a.push(id),
                Color::B => class_b.push(id),
            }
        }

        Some((class_a, class_b))
    }
}
