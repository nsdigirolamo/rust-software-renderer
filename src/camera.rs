use crate::vector3::Vector3;

pub struct Camera {
    pub position: Vector3,
    pub rotation: Vector3,
    pub h_fov: f32,
    pub v_fov: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn new (position: Vector3, rotation: Vector3, h_fov: f32, v_fov: f32, z_near: f32, z_far: f32) -> Self {
        Self { position, rotation, h_fov, v_fov, z_near, z_far }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position.translate(x, y, z);
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation.translate(x, y, z);
    }
}