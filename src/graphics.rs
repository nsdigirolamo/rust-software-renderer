use crate::matrix::{Matrix4x4, matrix_vector_multiply};
use crate::mesh::Mesh;
use crate::camera::Camera;
use crate::triangle::calculate_normal;
use crate::vector3::{subtract, dot_product};

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

pub fn project <T: sdl2::render::RenderTarget> (canvas: &mut Canvas<T>, camera: &Camera, mesh: &Mesh, color: (u8, u8, u8)) {
    let size = canvas.output_size().unwrap();
    let half_width = size.0 as f32 / 2.0;
    let half_height = size.1 as f32 / 2.0;

    let hf = 1.0 / (camera.h_fov.to_radians() / 2.0).tan();
    let vf = 1.0 / (camera.v_fov.to_radians() / 2.0).tan();
    let lambda = camera.z_far / (camera.z_far - camera.z_near);

    //  1/tan(hfov/2)              0                  0                           0
    //              0  1/tan(vfov/2)                  0                           0
    //              0              0  zfar/(zfar-znear)  -znear * zfar/(zfar-znear)
    //              0              0                  1                           0
    let mut projection_matrix = Matrix4x4 { m: [[0.0; 4]; 4] };
    projection_matrix.m[0][0] = hf;
    projection_matrix.m[1][1] = vf;
    projection_matrix.m[2][2] = lambda;
    projection_matrix.m[2][3] = -camera.z_near * lambda;
    projection_matrix.m[3][2] = 1.0;

    'triangle: for triangle in &mesh.triangles {
        let normal = calculate_normal(triangle);
        let camera_ray = subtract(&triangle.vertices[0], &camera.position);
        if dot_product(&normal, &camera_ray) < 0.0 {
            let mut points = [Point::new(0, 0); 4];
            for (i, vertex) in triangle.vertices.iter().enumerate() {
                let point = matrix_vector_multiply(&projection_matrix, vertex);
                let mut x = point[0][0];
                let mut y = -point[1][0];
                let mut z = point[2][0];
                let w = point[3][0];

                // if w is less than 1 it rapidly scales the x/y/z values
                if w > 1.0 {
                    x /= w;
                    y /= w;
                    z /= w;
                } else {
                    break 'triangle;
                }

                if x.abs() > 1.0 || y.abs() > 1.0 || z.abs() > 1.0 {
                    break 'triangle;
                }
                x = x * half_width + half_width;
                y = y * half_height + half_height;
                points[i] = Point::new(x as i32, y as i32);
            }
            points[3] = points[0];
            canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
            fill_triangle(canvas, points[0], points[1], points[2]);
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.draw_lines(&points[..]).unwrap();
        }
    }
}

// http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
pub fn fill_triangle <T: sdl2::render::RenderTarget> (canvas: &mut Canvas<T>, v1: Point, v2: Point, v3: Point) {
    let mut vrts = [v1, v2, v3];
    loop {
        if vrts[0].y <= vrts[1].y && vrts[1].y <= vrts[2].y {
            break;
        } else if vrts[1].y < vrts[0].y {
            vrts.swap(0, 1);
        } else if vrts[2].y < vrts[1].y {
            vrts.swap(1, 2);
        }
    }
    let v1 = vrts[0];
    let v2 = vrts[1];
    let v3 = vrts[2];

    if v2.y == v3.y {
        fill_bottom_flat_triangle(canvas, v1, v2, v3);
    } else if v1.y == v2.y {
        fill_top_flat_triangle(canvas, v1, v2, v3);
    } else {
        let v4 = Point::new(
            (v1.x as f32 + ((v2.y - v1.y) as f32 / (v3.y - v1.y) as f32) * (v3.x - v1.x) as f32) as i32,
            v2.y,
        );
        fill_bottom_flat_triangle(canvas, v1, v2, v4);
        fill_top_flat_triangle(canvas, v2, v4, v3);
    }
}

// http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
fn fill_bottom_flat_triangle<T: sdl2::render::RenderTarget> (canvas: &mut Canvas<T>, v1: Point, v2: Point, v3: Point) {
    let inv_slope1 = (v2.x - v1.x) as f32 / (v2.y - v1.y) as f32;
    let inv_slope2 = (v3.x - v1.x) as f32 / (v3.y - v1.y) as f32;
    let mut cursor1 = v1.x as f32;
    let mut cursor2 = v1.x as f32;
    let mut scanline = v1.y as f32;
    while scanline <= v2.y as f32 {
        let p1 = Point::new(cursor1 as i32, scanline as i32);
        let p2 = Point::new(cursor2 as i32, scanline as i32);
        canvas.draw_line(p1, p2).unwrap();
        cursor1 += inv_slope1;
        cursor2 += inv_slope2;
        scanline += 1.0;
    }
}

// http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
fn fill_top_flat_triangle<T: sdl2::render::RenderTarget> (canvas: &mut Canvas<T>, v1: Point, v2: Point, v3: Point) {
    let inv_slope1 = (v3.x - v1.x) as f32 / (v3.y - v1.y) as f32;
    let inv_slope2 = (v3.x - v2.x) as f32 / (v3.y - v2.y) as f32;
    let mut cursor1 = v3.x as f32;
    let mut cursor2 = v3.x as f32;
    let mut scanline = v3.y as f32;
    while scanline >= v1.y as f32 {
        let p1 = Point::new(cursor1 as i32, scanline as i32);
        let p2 = Point::new(cursor2 as i32, scanline as i32);
        canvas.draw_line(p1, p2).unwrap();
        cursor1 -= inv_slope1;
        cursor2 -= inv_slope2;
        scanline -= 1.0;
    }
}