use crate::math::vector3::Vector3;

// Color representation class (float)
pub struct Color {
    r: f32, // Red
    g: f32, // Green
    b: f32, // Blue
    a: f32, // Transparency
}

impl Color {
    pub fn zero() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }

    pub fn from_grayscale(val: f32, a: f32) -> Self {
        Self { r: val, g: val, b: val, a }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_rgb_array(arr: [f32; 3], a: f32) -> Self {
        Self { r: arr[0], g: arr[1], b: arr[2], a }
    }

    pub fn from_vector3(vec: &Vector3, a: f32) -> Self {
        Self { r: vec.x, g: vec.y, b: vec.z, a }
    }

    pub fn unpack(c: u32) -> Self {
        let r = ((c >> 16) & 0xFF) as f32 / 255.0;
        let g = ((c >> 8) & 0xFF) as f32 / 255.0;
        let b = (c & 0xFF) as f32 / 255.0;
        let a = ((c >> 24) & 0xFF) as f32 / 255.0;
        Self { r, g, b, a }
    }

    pub fn copy(&self) -> Self {
        Self { r: self.r, g: self.g, b: self.b, a: self.a }
    }

    pub fn copy_values(&mut self, other: &Self) {
        self.r = other.r;
        self.g = other.g;
        self.b = other.b;
        self.a = other.a;
    }
}
