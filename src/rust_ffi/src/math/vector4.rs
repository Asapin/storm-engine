pub struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector4 {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn new_1f(a: f32) -> Self {
        Self { x: a, y: a, z: a, w: a }
    }

    pub fn new_1d(a: f64) -> Self {
        let a: f32 = a as f32;
        Self { x: a, y: a, z: a, w: a }
    }

    pub fn new_3f(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn new_3d(x: f64, y: f64, z: f64) -> Self {
        Self { x: x as f32, y: y as f32, z: z as f32, w: 1.0 }
    }

    pub fn new_4f(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn new_4d(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x: x as f32, y: y as f32, z: z as f32, w: w as f32 }
    }

    pub fn copy(v: &Self) -> Self {
        Self { x: v.x, y: v.y, z: v.z, w: v.w }
    }

    pub fn copy_values(&mut self, v: &Self) {
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
        self.w = v.w;
    }
}
