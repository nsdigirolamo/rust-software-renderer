use crate::vector3::Vector3;

pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn new(m: [[f32; 4]; 4]) -> Self {
        Self { m }
    }
}

pub fn matrix_scale(m1: &Matrix4x4, k: f32) -> Matrix4x4 {
    let mut m = [[0.0; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            m[row][col] = m1.m[row][col] * k;
        }
    }
    Matrix4x4 { m }
}

pub fn matrix_add(m1: &Matrix4x4, m2: &Matrix4x4) -> Matrix4x4 {
    let mut m = [[0.0; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            m[row][col] = m1.m[row][col] + m2.m[row][col];
        }
    }
    Matrix4x4 { m }
}

pub fn matrix_subtract(m1: &Matrix4x4, m2: &Matrix4x4) -> Matrix4x4 {
    let mut m = [[0.0; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            m[row][col] = m1.m[row][col] - m2.m[row][col];
        }
    }
    Matrix4x4 { m }
}

pub fn matrix_matrix_multiply(m1: &Matrix4x4, m2: &Matrix4x4) -> Matrix4x4 {
    let mut m = [[0.0; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            m[row][col] = m1.m[row][0] * m2.m[0][col] +
                          m1.m[row][1] * m2.m[1][col] +
                          m1.m[row][2] * m2.m[2][col] +
                          m1.m[row][3] * m2.m[3][col];
        }
    }
    Matrix4x4 { m }
}

pub fn matrix_vector_multiply(m1: &Matrix4x4, v: &Vector3) -> [[f32; 1]; 4] {
    let mut m = [[0.0]; 4];
    for row in 0..4 {
        m[row][0] = m1.m[row][0] * v.x +
                    m1.m[row][1] * v.y +
                    m1.m[row][2] * v.z +
                    m1.m[row][3] * 1.0;
    }
    m
}

pub fn identity_matrix() -> Matrix4x4 {
    Matrix4x4 { m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]}
}

pub fn x_rotation_matrix(r: f32) -> Matrix4x4 {
    Matrix4x4 { m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, r.cos(), -r.sin(), 0.0],
        [0.0, r.sin(), r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]}
}

pub fn y_rotation_matrix(r: f32) -> Matrix4x4 {
    Matrix4x4 { m: [
        [r.cos(), 0.0, r.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-r.sin(), 0.0, r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]}
}

pub fn z_rotation_matrix(r: f32) -> Matrix4x4 {
    Matrix4x4 { m: [
        [r.cos(), -r.sin(), 0.0, 0.0],
        [r.sin(), r.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]}
}

pub fn scale_matrix(x: f32, y: f32, z: f32) -> Matrix4x4 {
    Matrix4x4 { m: [
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]}
}

pub fn translation_matrix(x: f32, y: f32, z: f32) -> Matrix4x4 {
    Matrix4x4 { m: [
        [0.0, 0.0, 0.0, x],
        [0.0, 0.0, 0.0, y],
        [0.0, 0.0, 0.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ]}
}