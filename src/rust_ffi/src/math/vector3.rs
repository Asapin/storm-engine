pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new_1f(a: f32) -> Self {
        Self { x: a, y: a, z: a }
    }

    pub fn new_1d(a: f64) -> Self {
        let a: f32 = a as f32;
        Self { x: a, y: a, z: a }
    }

    pub fn new_3f(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn new_3d(x: f64, y: f64, z: f64) -> Self {
        Self { x: x as f32, y: y as f32, z: z as f32 }
    }

    pub fn copy(v: &Self) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }

    pub fn copy_values(&mut self, v: &Self) {
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
    }
}
