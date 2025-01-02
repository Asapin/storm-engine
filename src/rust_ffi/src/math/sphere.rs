pub struct Sphere {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
}

impl Sphere {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            radius: 0.0,
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            radius: self.radius,
        }
    }

    pub fn copy_values(&mut self, other: &Self) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
        self.radius = other.radius;
    }
}
