use crate::math::vector3::Vector3;

// Class for representing a plane in 3D space
pub struct Plane {
    n: Vector3, // Normal
    d: f32,     // Distance from center
}

impl Plane {
    pub fn zero() -> Self {
        Self {
            n: Vector3::zero(),
            d: 0.0,
        }
    }

    pub fn new(nx: f32, ny: f32, nz: f32) -> Self {
        Self {
            n: Vector3::new_3f(nx, ny, nz),
            d: 0.0,
        }
    }

    pub fn from_vector3(n: Vector3) -> Self {
        Self { n, d: 0.0 }
    }

    pub fn copy(plane: &Self) -> Self {
        Self {
            n: Vector3::copy(&plane.n),
            d: plane.d,
        }
    }

    pub fn copy_values(&mut self, plane: &Self) {
        self.n.copy_values(&plane.n);
        self.d = plane.d;
    }
}
