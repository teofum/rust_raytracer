use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::vec3::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Mat3([f64; 9]);

impl Mat3 {
    // Constructors

    pub fn identity() -> Self {
        Mat3([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    }

    pub fn from_values(values: [f64; 9]) -> Self {
        Mat3(values)
    }

    pub fn from_rows(r0: Vec3, r1: Vec3, r2: Vec3) -> Self {
        Mat3([
            r0.0, r0.1, r0.2, //
            r1.0, r1.1, r1.2, //
            r2.0, r2.1, r2.2,
        ])
    }

    pub fn from_columns(c0: Vec3, c1: Vec3, c2: Vec3) -> Self {
        Mat3([
            c0.0, c1.0, c2.0, //
            c0.1, c1.1, c2.1, //
            c0.2, c1.2, c2.2,
        ])
    }

    // Getters

    pub fn row(&self, index: usize) -> Vec3 {
        if index >= 3 {
            panic!("Index out of bounds");
        }

        Vec3(
            self.0[index * 3 + 0],
            self.0[index * 3 + 1],
            self.0[index * 3 + 2],
        )
    }

    pub fn column(&self, index: usize) -> Vec3 {
        if index >= 3 {
            panic!("Index out of bounds");
        }

        Vec3(self.0[index + 0], self.0[index + 3], self.0[index + 6])
    }

    // Utility fuctions

    pub fn transposed(&self) -> Self {
        Mat3([
            self[0][0], self[1][0], self[2][0], //
            self[0][1], self[1][1], self[2][1], //
            self[0][2], self[1][2], self[2][2],
        ])
    }
}

// Operators (index)

impl Index<usize> for Mat3 {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[(index * 3)..(index * 3 + 3)]
    }
}

impl IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[(index * 3)..(index * 3 + 3)]
    }
}

// Operators (copy)

impl Add for Mat3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; 9];
        for i in 0..9 {
            values[i] = self.0[i] + rhs.0[i];
        }

        Mat3(values)
    }
}

impl Sub for Mat3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; 9];
        for i in 0..9 {
            values[i] = self.0[i] - rhs.0[i];
        }

        Mat3(values)
    }
}

impl Mul for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; 9];
        let rhs_cols = [rhs.column(0), rhs.column(1), rhs.column(2)];

        for i in 0..3 {
            let row = self.row(i);
            for j in 0..3 {
                let col = rhs_cols[j];
                values[i * 3 + j] = row.0 * col.0 + row.1 * col.1 + row.2 * col.2;
            }
        }

        Mat3(values)
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut values = [0.0; 3];

        for i in 0..3 {
            let row = self.row(i);
            values[i] = row.0 * rhs.0 + row.1 * rhs.1 + row.2 * rhs.2;
        }

        let [x, y, z] = values;
        Vec3(x, y, z)
    }
}

impl Mul<f64> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut values = [0.0; 9];
        for i in 0..9 {
            values[i] = self.0[i] * rhs;
        }

        Mat3(values)
    }
}

impl Div<f64> for Mat3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let mut values = [0.0; 9];
        for i in 0..9 {
            values[i] = self.0[i] / rhs;
        }

        Mat3(values)
    }
}

// Operators (mutation)

impl AddAssign for Mat3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..9 {
            self.0[i] += rhs.0[i];
        }
    }
}

impl SubAssign for Mat3 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..9 {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl MulAssign for Mat3 {
    fn mul_assign(&mut self, rhs: Self) {
        let mut values = [0.0; 9];
        let rhs_cols = [rhs.column(0), rhs.column(1), rhs.column(2)];

        for i in 0..3 {
            let row = self.row(i);
            for j in 0..3 {
                let col = rhs_cols[j];
                values[i * 3 + j] = row.0 * col.0 + row.1 * col.1 + row.2 * col.2;
            }
        }

        for i in 0..9 {
            self.0[i] = values[i];
        }
    }
}

impl MulAssign<f64> for Mat3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..9 {
            self.0[i] *= rhs;
        }
    }
}

impl DivAssign<f64> for Mat3 {
    fn div_assign(&mut self, rhs: f64) {
        for i in 0..9 {
            self.0[i] /= rhs;
        }
    }
}

impl Neg for Mat3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut values = self.0;
        for i in 0..9 {
            values[i] = -values[i];
        }

        Mat3(values)
    }
}
