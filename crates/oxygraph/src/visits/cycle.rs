//! Directed cycle detection, bound to directed graphs: three-color DFS via
//! [`GraphEdgeStorageDirected::children_of`].

use crate::{
    edges::GraphEdgeStorageDirected, graph::GraphView, vertices::VertexId, visits::VertexMarks,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}

/// Finds a cycle in the view, following edge direction. `None` if the graph is a DAG.
#[derive(Debug, Clone, Copy)]
pub struct CycleDetection;

impl CycleDetection {
    pub fn visit<V: 'static, S: GraphEdgeStorageDirected, G: GraphView<V, S>>(
        &self,
        view: &G,
    ) -> Option<Vec<VertexId<S::Id>>> {
        let mut colors: VertexMarks<Color> = VertexMarks::new(Color::White);
        let mut path: Vec<VertexId<S::Id>> = Vec::new();

        for start in view.ids() {
            if *colors.get(start) != Color::White {
                continue;
            }

            colors.set(start, Color::Gray);
            path.push(start);
            let neighbors: Vec<VertexId<S::Id>> =
                view.edges().children_of(start).map(|e| e.to).collect();
            let mut stack = vec![(neighbors, 0usize)];

            while let Some((neighbors, mut index)) = stack.pop() {
                if index >= neighbors.len() {
                    let finished = path.pop().expect("a pushed vertex is always on the path");
                    colors.set(finished, Color::Black);
                    continue;
                }

                let next = neighbors[index];
                index += 1;
                stack.push((neighbors, index));

                match *colors.get(next) {
                    Color::White => {
                        colors.set(next, Color::Gray);
                        path.push(next);
                        let next_neighbors: Vec<VertexId<S::Id>> =
                            view.edges().children_of(next).map(|e| e.to).collect();
                        stack.push((next_neighbors, 0));
                    }
                    Color::Gray => {
                        let cycle_start = path
                            .iter()
                            .position(|&v| v == next)
                            .expect("a gray vertex is always on the current path");
                        return Some(path[cycle_start..].to_vec());
                    }
                    Color::Black => {}
                }
            }
        }

        None
    }
}
