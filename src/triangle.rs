use crate::vector3::{Vector3, subtract, cross_product};

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub vertices: [Vector3; 3],
}

impl Triangle {
    pub fn new(vertices: [Vector3; 3]) -> Self {
        Self { vertices }
    }
}

pub fn calculate_normal(triangle: &Triangle) -> Vector3 {
    let line1 = subtract(&triangle.vertices[1], &triangle.vertices[0]);
    let line2 = subtract(&triangle.vertices[2], &triangle.vertices[0]);
    let mut normal = cross_product(&line1, &line2);
    normal.normalize();
    normal
}