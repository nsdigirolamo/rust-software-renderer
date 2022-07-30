#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    pub fn scale(&mut self, k: f32) {
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    pub fn non_uniform_scale(&mut self, x: f32, y: f32, z: f32) {
        self.x *= x;
        self.y *= y;
        self.z *= z;
    }

    pub fn normalize(&mut self) {
        let mag = calculate_magnitude(self);
        let mut k = 0.0;

        if mag != 0.0 {
            k = 1.0 / mag;
        }

        self.scale(k)
    }

    pub fn negate(&mut self) {
        self.scale(-1.0)
    }
}

pub fn calculate_magnitude(v: &Vector3) -> f32 {
    (v.x.powf(2.0) + v.y.powf(2.0) + v.z.powf(2.0)).sqrt()
}

pub fn add(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
        z: v1.z + v2.z,
    }
}

pub fn subtract(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}

pub fn dot_product(v1: &Vector3, v2: &Vector3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross_product(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}