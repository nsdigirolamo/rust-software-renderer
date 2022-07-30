use crate::camera::Camera;
use crate::mesh::{Mesh, teapot};
use crate::vector3::Vector3;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;

use std::time::Duration;

mod matrix;
mod vector3;
mod triangle;
mod mesh;
mod camera;
mod graphics;

mod test;

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;
const FOV: f32 = 100.0;

fn handle_input (event_pump: &EventPump, mesh: &mut Mesh) {
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::W) {
        mesh.translate(0.0, 0.0, -0.1);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::S) {
        mesh.translate(0.0, 0.0, 0.1);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::D) {
        mesh.translate(-0.1, 0.0, 0.0);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::A) {
        mesh.translate(0.1, 0.0, 0.0);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Space) {
        mesh.translate(0.0, -0.1, 0.0);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::LCtrl) {
        mesh.translate(0.0, 0.1, 0.0);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Left) {
        mesh.global_rotate(0.0, 0.025, 0.0);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Right) {
        mesh.global_rotate(0.0, -0.025, 0.0);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Up) {
        mesh.global_rotate(-0.025, 0.0, 0.0);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Down) {
        mesh.global_rotate(0.025, 0.0, 0.0);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Q) {
        mesh.global_rotate(0.0, 0.0, -0.025);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::E) {
        mesh.global_rotate(0.0, 0.0, 0.025);
    }
}

fn control_fov (event_pump: &EventPump, camera: &mut Camera) {
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::O) { 
        camera.h_fov -= 0.25;
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::P) { 
        camera.h_fov += 0.25;
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::K) { 
        camera.v_fov -= 0.25;
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::L) { 
        camera.v_fov += 0.25;
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 cube playground", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut teapot1 = teapot();
    teapot1.translate(-10.0, 0.0, 10.0);
    let mut teapot2 = teapot();
    teapot2.translate(0.0, 0.0, 10.0);
    let mut teapot3 = teapot();
    teapot3.translate(10.0, 0.0, 10.0);

    let mut camera = Camera {
        position: Vector3::new(0.0, 0.0, 0.0),
        rotation: Vector3::new(0.0, 0.0, 0.0),
        h_fov: FOV,
        v_fov: (FOV * (WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32)) - FOV,
        z_near: 1.0,
        z_far: 200.0,
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        handle_input(&event_pump, &mut teapot1);
        handle_input(&event_pump, &mut teapot2);
        handle_input(&event_pump, &mut teapot3);
        control_fov(&event_pump, &mut camera);

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        teapot1.draw(&mut canvas, &camera, (255, 127, 127));
        teapot2.draw(&mut canvas, &camera, (127, 255, 127));
        teapot3.draw(&mut canvas, &camera, (127, 127, 255));

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
