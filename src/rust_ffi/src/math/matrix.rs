/*
Matrix class for 3D transformations.

Linear representation         2D array
                                vx     vy     vz    pos
    0  4  8 12                [0][0] [1][0] [2][0] [3][0]    x
    1  5  9 13                [0][1] [1][1] [2][1] [3][1]    y
    2  6 10 14                [0][2] [1][2] [2][2] [3][2]    z
    3  7 11 15                [0][3] [1][3] [2][3] [3][3]    w
*/
pub struct Matrix {
    m: [[f32; 4]; 4],
}

impl Matrix {
    pub fn zero() -> Self {
        Matrix {
            m: [[0.0; 4]; 4],
        }
    }

    pub fn identity() -> Self {
        let mut m = Matrix::zero();
        m.to_identity();
        m
    }

    pub fn to_identity(&mut self) {
        self.m[0][0] = 1.0;
        self.m[0][1] = 0.0;
        self.m[0][2] = 0.0;
        self.m[0][3] = 0.0;
        self.m[1][0] = 0.0;
        self.m[1][1] = 1.0;
        self.m[1][2] = 0.0;
        self.m[1][3] = 0.0;
        self.m[2][0] = 0.0;
        self.m[2][1] = 0.0;
        self.m[2][2] = 1.0;
        self.m[2][3] = 0.0;
        self.m[3][0] = 0.0;
        self.m[3][1] = 0.0;
        self.m[3][2] = 0.0;
        self.m[3][3] = 1.0;
    }

    pub fn copy(m: &Matrix) -> Self {
        Matrix {
            m: m.m,
        }
    }

    pub fn copy_values(&mut self, m: &Matrix) {
        self.m = m.m;
    }
}
