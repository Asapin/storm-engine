use crate::math::vector3::Vector3;

pub struct Triangle {
    a: Vector3,
    b: Vector3,
    c: Vector3,
}

impl Triangle {
    pub fn zero() -> Self {
        Self {
            a: Vector3::zero(),
            b: Vector3::zero(),
            c: Vector3::zero(),
        }
    }

    pub fn copy(triangle: &Self) -> Self {
        Self {
            a: Vector3::copy(&triangle.a),
            b: Vector3::copy(&triangle.b),
            c: Vector3::copy(&triangle.c),
        }
    }

    pub fn copy_values(&mut self, triangle: &Self) {
        self.a.copy_values(&triangle.a);
        self.b.copy_values(&triangle.b);
        self.c.copy_values(&triangle.c);
    }
}
