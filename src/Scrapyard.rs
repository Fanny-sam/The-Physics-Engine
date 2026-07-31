use spade::{DelaunayTriangulation, Point2, Triangulation, HasPosition};
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    pos: Point2<f32>,
    id: u32,
}

impl HasPosition for Vertex {
    type Scalar = f32;
    fn position(&self) -> Point2<f32> {
        self.pos
    }
}

// ---- 3. Main function: takes slice of Vector2, returns edges as (u32, u32) ----
pub fn triangulate_points(points: &[Vector2]) -> Vec<(u32, u32)> {
    if points.len() < 3 {
        return Vec::new();
    }

    let vertices: Vec<Vertex> = points
        .iter()
        .enumerate()
        .map(|(i, v)| Vertex {
            pos: Point2::new(v.x, v.y),
            id: i as u32,
        })
        .collect();

    let triangulation = DelaunayTriangulation::bulk_load(vertices)
        .expect("Invalid point set (collinear or duplicate points)");

    let mut edge_set = HashSet::new();
    for edge in triangulation.undirected_edges() {
        let v1 = edge.from().data();
        let v2 = edge.to().data();
        let (a, b) = if v1.id < v2.id {
            (v1.id, v2.id)
        } else {
            (v2.id, v1.id)
        };
        edge_set.insert((a, b));
    }

    edge_set.into_iter().collect()
}
pub fn triangulate(point)
