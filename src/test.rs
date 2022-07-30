use crate::matrix::*;
use crate::vector3::Vector3;
use crate::mesh::cube;

#[test]
fn matrix_scale_test () {
    let m1 = Matrix4x4 { m: [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ]};
    let m2 = Matrix4x4 { m:  [
        [2.0, 4.0, 6.0, 8.0],
        [10.0, 12.0, 14.0, 16.0],
        [18.0, 20.0, 22.0, 24.0],
        [26.0, 28.0, 30.0, 32.0],
    ]};
    let m3 = matrix_scale(&m1, 2.0);
    for row in 0..4 {
        for col in 0..4 {
            assert_eq!(m2.m[row][col], m3.m[row][col]);
        }
    }
}

#[test]
fn matrix_add_test () {
    let m1 = Matrix4x4 { m: [
        [1.0, 1.0, 1.0, 1.0],
        [2.0, 2.0, 2.0, 2.0],
        [3.0, 3.0, 3.0, 3.0],
        [4.0, 4.0, 4.0, 4.0],
    ]};
    let m2 = Matrix4x4 { m: [
        [1.1, 1.2, 1.3, 1.4],
        [2.5, 2.6, 2.7, 2.8],
        [3.9, 3.1, 3.2, 3.3],
        [4.4, 4.5, 4.6, 4.7],
    ]};
    let m3 = Matrix4x4 { m: [
        [2.1, 2.2, 2.3, 2.4],
        [4.5, 4.6, 4.7, 4.8],
        [6.9, 6.1, 6.2, 6.3],
        [8.4, 8.5, 8.6, 8.7],
    ]};
    let m4 = matrix_add(&m1, &m2);
    for row in 0..4 {
        for col in 0..4 {
            assert_eq!(m3.m[row][col], m4.m[row][col]);
        }
    }
}

#[test]
fn matrix_subtract_test () {
    let m1 = Matrix4x4 { m: [
        [1.0, 1.0, 1.0, 1.0],
        [2.0, 2.0, 2.0, 2.0],
        [3.0, 3.0, 3.0, 3.0],
        [4.0, 4.0, 4.0, 4.0],
    ]};
    let m2 = Matrix4x4 { m: [
        [1.1, 1.2, 1.3, 1.4],
        [2.5, 2.6, 2.7, 2.8],
        [3.9, 3.1, 3.2, 3.3],
        [4.4, 4.5, 4.6, 4.7],
    ]};
    let m3 = Matrix4x4 { m: [
        [-0.1, -0.2, -0.3, -0.4],
        [-0.5, -0.6, -0.7, -0.8],
        [-0.9, -0.1, -0.2, -0.3],
        [-0.4, -0.5, -0.6, -0.7],
    ]};
    let m4 = matrix_subtract(&m1, &m2);
    for row in 0..4 {
        for col in 0..4 {
            assert_eq!((m3.m[row][col] * 1_000_000.0) as u32, (m4.m[row][col] * 1_000_000.0) as u32);
        }
    }
}

#[test]
fn matrix_matrix_multiply_test () {
    let m1 = Matrix4x4 { m: [
        [1.0, 1.0, 1.0, 1.0],
        [2.0, 2.0, 2.0, 2.0],
        [3.0, 3.0, 3.0, 3.0],
        [4.0, 4.0, 4.0, 4.0],
    ]};
    let m2 = Matrix4x4 { m: [
        [5.0, 5.0, 5.0, 5.0],
        [6.0, 6.0, 6.0, 6.0],
        [7.0, 7.0, 7.0, 7.0],
        [8.0, 8.0, 8.0, 8.0],
    ]};
    let m3 = Matrix4x4 { m: [
        [26.0, 26.0, 26.0, 26.0],
        [52.0, 52.0, 52.0, 52.0],
        [78.0, 78.0, 78.0, 78.0],
        [104.0, 104.0, 104.0, 104.0],
    ]};
    let m4 = matrix_matrix_multiply(&m1, &m2);
    for row in 0..4 {
        for col in 0..4 {
            assert_eq!(m3.m[row][col], m4.m[row][col]);
        }
    }
}

#[test]
fn matrix_vector_multiply_test () {
    let m1 = Matrix4x4 { m: [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ]};
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let m3 = [
        [18.0],
        [46.0],
        [74.0],
        [102.0],
    ];
    let m4 = matrix_vector_multiply(&m1, &v1);
    for row in 0..4 {
        for col in 0..1 {
            assert_eq!(m3[row][col], m4[row][col]);
        }
    }
}

#[test]
fn mesh_translate_test () {
    let mut cube1 = cube(1.0);
    let cube2 = cube(1.0);
    cube1.translate(10.0, -10.0, 30.3);
    for (i, triangle) in cube1.triangles.iter().enumerate() {
        for (j, vertex) in triangle.vertices.iter().enumerate() {
            let x = cube2.triangles[i].vertices[j].x;
            let y = cube2.triangles[i].vertices[j].y;
            let z = cube2.triangles[i].vertices[j].z;
            assert_eq!(vertex.x, x + 10.0);
            assert_eq!(vertex.y, y - 10.0);
            assert_eq!(vertex.z, z + 30.3);
        }
    }
    assert_eq!(cube1.position.x, cube2.position.x + 10.0);
    assert_eq!(cube1.position.y, cube2.position.y - 10.0);
    assert_eq!(cube1.position.z, cube2.position.z + 30.3);
}
