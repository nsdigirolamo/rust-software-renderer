use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use crate::matrix::{matrix_vector_multiply, x_rotation_matrix, y_rotation_matrix, z_rotation_matrix};
use crate::vector3::{Vector3, add, subtract};
use crate::triangle::Triangle;
use crate::camera::Camera;
use crate::graphics::project;

use sdl2::render::Canvas;

use std::fs;

#[derive(Debug)]
pub struct Mesh {
    pub position: Vector3,
    pub rotation: Vector3,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(position: Vector3, rotation: Vector3, triangles: Vec<Triangle>) -> Self {
        Self { position, rotation, triangles }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position.translate(x, y, z);
        for triangle in &mut self.triangles {
            for vertex in &mut triangle.vertices {
                vertex.translate(x, y, z);
            }
        }
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation.translate(x, y, z);
        let x_rot_matrix = x_rotation_matrix(x);
        let y_rot_matrix = y_rotation_matrix(y);
        let z_rot_matrix = z_rotation_matrix(z);
        for triangle in &mut self.triangles {
            for vertex in &mut triangle.vertices {
                *vertex = subtract(vertex, &self.position);

                let x_mult = matrix_vector_multiply(&x_rot_matrix, vertex);
                *vertex = Vector3::new(x_mult[0][0], x_mult[1][0], x_mult[2][0]);

                let y_mult = matrix_vector_multiply(&y_rot_matrix, vertex);
                *vertex = Vector3::new(y_mult[0][0], y_mult[1][0], y_mult[2][0]);

                let z_mult = matrix_vector_multiply(&z_rot_matrix, vertex);
                *vertex = Vector3::new(z_mult[0][0], z_mult[1][0], z_mult[2][0]);

                *vertex = add(vertex, &self.position);
            }
        }
    }

    pub fn global_rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation.translate(x, y, z);
        let x_rot_matrix = x_rotation_matrix(x);
        let y_rot_matrix = y_rotation_matrix(y);
        let z_rot_matrix = z_rotation_matrix(z);
        for triangle in &mut self.triangles {
            for vertex in &mut triangle.vertices {
                let x_mult = matrix_vector_multiply(&x_rot_matrix, vertex);
                *vertex = Vector3::new(x_mult[0][0], x_mult[1][0], x_mult[2][0]);

                let y_mult = matrix_vector_multiply(&y_rot_matrix, vertex);
                *vertex = Vector3::new(y_mult[0][0], y_mult[1][0], y_mult[2][0]);

                let z_mult = matrix_vector_multiply(&z_rot_matrix, vertex);
                *vertex = Vector3::new(z_mult[0][0], z_mult[1][0], z_mult[2][0]);
            }
        }
    }

    pub fn draw <T: sdl2::render::RenderTarget> (&self, canvas: &mut Canvas<T>, camera: &Camera, color: (u8, u8, u8)) {
        project(canvas, camera, self, color);
    }
}

pub fn cube (size: f32) -> Mesh {
    let half = size / 2.0;
    let v1 = Vector3::new(-half, -half, -half);
    let v2 = Vector3::new( half, -half, -half);
    let v3 = Vector3::new(-half,  half, -half);
    let v4 = Vector3::new( half,  half, -half);
    let v5 = Vector3::new(-half, -half,  half);
    let v6 = Vector3::new( half, -half,  half);
    let v7 = Vector3::new(-half,  half,  half);
    let v8 = Vector3::new( half,  half,  half);

    Mesh {
        position: Vector3::new(0.0, 0.0, 0.0),
        rotation: Vector3::new(0.0, 0.0, 0.0),
        triangles: vec![
                // neg z
                Triangle::new([v1, v3, v4]),
                Triangle::new([v1, v4, v2]),
                // pos z
                Triangle::new([v6, v8, v7]),
                Triangle::new([v6, v7, v5]),
                // neg x
                Triangle::new([v5, v7, v3]),
                Triangle::new([v5, v3, v1]),
                // pos x
                Triangle::new([v2, v4, v8]),
                Triangle::new([v2, v8, v6]),
                // neg y
                Triangle::new([v5, v1, v2]),
                Triangle::new([v5, v2, v6]),
                // pos y
                Triangle::new([v3, v7, v8]),
                Triangle::new([v3, v8, v4]),
    ]}
}

pub fn teapot () -> Mesh {
    let file_path = "/home/nick/Documents/Programming/rust/nicks-software-renderer/src/teapot_bezier.tris";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut triangles: Vec<Triangle> = Vec::new();
    let mut triangle = Triangle { vertices: [
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ] };

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let position: Vec<&str> = line.split(' ').collect();
        let remainder = (index + 1) % 4;
        if remainder != 0 {
            let x = position[0].parse::<f32>().unwrap();
            let y = position[1].parse::<f32>().unwrap();
            let z = position[2].parse::<f32>().unwrap();
            triangle.vertices[remainder - 1] = Vector3 { x, y, z };
        } else {
            triangles.push(triangle);
        }
    }

    Mesh {
        position: Vector3::new(0.0, 0.0, 0.0), 
        rotation: Vector3::new(0.0, 0.0, 0.0),
        triangles,
    }
}