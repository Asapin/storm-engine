pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn copy(q: &Self) -> Self {
        Self {
            x: q.x,
            y: q.y,
            z: q.z,
            w: q.w,
        }
    }

    pub fn copy_values(&mut self, q: &Self) {
        self.x = q.x;
        self.y = q.y;
        self.z = q.z;
        self.w = q.w;
    }
}
