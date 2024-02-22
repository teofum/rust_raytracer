use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::vec4::Vec4;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Mat4(pub [f64; 16]);

impl Mat4 {
    // Constructors

    pub fn identity() -> Self {
        Mat4([
            1.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn from_values(values: [f64; 16]) -> Self {
        Mat4(values)
    }

    pub fn from_rows(r0: Vec4, r1: Vec4, r2: Vec4, r3: Vec4) -> Self {
        Mat4([
            r0[0], r0[1], r0[2], r0[3], //
            r1[0], r1[1], r1[2], r1[3], //
            r2[0], r2[1], r2[2], r2[3], //
            r3[0], r3[1], r3[2], r3[3],
        ])
    }

    pub fn from_columns(c0: Vec4, c1: Vec4, c2: Vec4, c3: Vec4) -> Self {
        Mat4([
            c0[0], c1[0], c2[0], c3[0], //
            c0[1], c1[1], c2[1], c3[1], //
            c0[2], c1[2], c2[2], c3[2], //
            c0[3], c1[3], c2[3], c3[3],
        ])
    }

    // Getters

    pub fn row(&self, index: usize) -> Vec4 {
        if index >= 4 {
            panic!("Index out of bounds");
        }

        Vec4(
            self.0[index * 4 + 0],
            self.0[index * 4 + 1],
            self.0[index * 4 + 2],
            self.0[index * 4 + 3],
        )
    }

    pub fn column(&self, index: usize) -> Vec4 {
        if index >= 4 {
            panic!("Index out of bounds");
        }

        Vec4(self.0[index + 0], self.0[index + 4], self.0[index + 8], self.0[index + 12])
    }

    // Utility fuctions

    pub fn transposed(&self) -> Self {
        Mat4([
            self[0][0], self[1][0], self[2][0], self[3][0], //
            self[0][1], self[1][1], self[2][1], self[3][1], //
            self[0][2], self[1][2], self[2][2], self[3][2], //
            self[0][3], self[1][3], self[2][3], self[3][3],
        ])
    }
}

// Operators (index)

impl Index<usize> for Mat4 {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[(index * 4)..(index * 4 + 4)]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[(index * 4)..(index * 4 + 4)]
    }
}

// Operators (copy)

impl Add for Mat4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; 16];
        for i in 0..16 {
            values[i] = self.0[i] + rhs.0[i];
        }

        Mat4(values)
    }
}

impl Sub for Mat4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; 16];
        for i in 0..16 {
            values[i] = self.0[i] - rhs.0[i];
        }

        Mat4(values)
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; 16];
        let rhs_cols = [rhs.column(0), rhs.column(1), rhs.column(2)];

        for i in 0..4 {
            let row = self.row(i);
            for j in 0..4 {
                let col = rhs_cols[j];
                values[i * 4 + j] = row[0] * col[0] + row[1] * col[1] + row[2] * col[2];
            }
        }

        Mat4(values)
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let mut values = [0.0; 4];

        for i in 0..4 {
            let row = self.row(i);
            values[i] = row[0] * rhs[0] + row[1] * rhs[1] + row[2] * rhs[2] + row[3] * rhs[3];
        }

        let [x, y, z, w] = values;
        Vec4(x, y, z, w)
    }
}

impl Mul<f64> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut values = [0.0; 16];
        for i in 0..16 {
            values[i] = self.0[i] * rhs;
        }

        Mat4(values)
    }
}

impl Div<f64> for Mat4 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let mut values = [0.0; 16];
        for i in 0..16 {
            values[i] = self.0[i] / rhs;
        }

        Mat4(values)
    }
}

// Operators (mutation)

impl AddAssign for Mat4 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..16 {
            self.0[i] += rhs.0[i];
        }
    }
}

impl SubAssign for Mat4 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..16 {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Self) {
        let mut values = [0.0; 16];
        let rhs_cols = [rhs.column(0), rhs.column(1), rhs.column(2)];

        for i in 0..4 {
            let row = self.row(i);
            for j in 0..4 {
                let col = rhs_cols[j];
                values[i * 4 + j] = row[0] * col[0] + row[1] * col[1] + row[2] * col[2];
            }
        }

        for i in 0..16 {
            self.0[i] = values[i];
        }
    }
}

impl MulAssign<f64> for Mat4 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..16 {
            self.0[i] *= rhs;
        }
    }
}

impl DivAssign<f64> for Mat4 {
    fn div_assign(&mut self, rhs: f64) {
        for i in 0..16 {
            self.0[i] /= rhs;
        }
    }
}

impl Neg for Mat4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut values = self.0;
        for i in 0..16 {
            values[i] = -values[i];
        }

        Mat4(values)
    }
}
