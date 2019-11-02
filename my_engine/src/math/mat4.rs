use crate::math::Vec4;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Row major xyzw
#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
    pub w: Vec4,
}

impl Mat4 {
    #[inline]
    pub fn new(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub fn identity() -> Self {
        Self {
            x: Vec4::new(1.0, 0.0, 0.0, 0.0),
            y: Vec4::new(0.0, 1.0, 0.0, 0.0),
            z: Vec4::new(0.0, 0.0, 1.0, 0.0),
            w: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        Self {
            x: Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x),
            y: Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y),
            z: Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z),
            w: Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w),
        }
    }

    #[inline]
    pub fn scalar(
        x_scalar: impl Into<f64>,
        y_scalar: impl Into<f64>,
        z_scalar: impl Into<f64>,
    ) -> Self {
        Self {
            x: Vec4::new(x_scalar, 0.0, 0.0, 0.0),
            y: Vec4::new(0.0, y_scalar, 0.0, 0.0),
            z: Vec4::new(0.0, 0.0, z_scalar, 0.0),
            w: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    #[inline]
    pub fn scalar_from_one(scalar: impl Into<f64> + Copy) -> Self {
        Self::scalar(scalar, scalar, scalar)
    }

    #[inline]
    pub fn translation<T>(x_tr: T, y_tr: T, z_tr: T) -> Self
    where
        T: Into<f64>,
    {
        Self {
            x: Vec4::new(1.0, 0.0, 0.0, x_tr),
            y: Vec4::new(0.0, 1.0, 0.0, y_tr),
            z: Vec4::new(0.0, 0.0, 1.0, z_tr),
            w: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl Add for Mat4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl AddAssign for Mat4 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Mat4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl SubAssign for Mat4 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<Mat4> for f64 {
    type Output = Mat4;

    fn mul(self, mat: Mat4) -> Mat4 {
        Mat4 {
            x: mat.x * self,
            y: mat.y * self,
            z: mat.z * self,
            w: mat.w * self,
        }
    }
}

impl<T: Into<f64> + Copy> Mul<T> for Mat4 {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar.into(),
            y: self.y * scalar.into(),
            z: self.z * scalar.into(),
            w: self.w * scalar.into(),
        }
    }
}

impl<T: Into<f64> + Copy> MulAssign<T> for Mat4 {
    fn mul_assign(&mut self, scalar: T) {
        *self = Self {
            x: self.x * scalar.into(),
            y: self.y * scalar.into(),
            z: self.z * scalar.into(),
            w: self.w * scalar.into(),
        }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // transpose rhs matrix to make multiplication simpler
        let tr_rhs = rhs.transpose();
        // this is multiplying the rows of self by columns of rhs
        let tr_product = Self {
            x: &self * tr_rhs.x,
            y: &self * tr_rhs.y,
            z: &self * tr_rhs.z,
            w: &self * tr_rhs.w,
        };

        // transpose back to get proper output
        tr_product.transpose()
    }
}

// follows real math semantics, vector has to be rhs
impl Mul<Vec4> for &Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x.dot(&rhs),
            y: self.y.dot(&rhs),
            z: self.z.dot(&rhs),
            w: self.w.dot(&rhs),
        }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x.dot(&rhs),
            y: self.y.dot(&rhs),
            z: self.z.dot(&rhs),
            w: self.w.dot(&rhs),
        }
    }
}

impl<T: Into<f64>> From<(((T, T, T, T), (T, T, T, T), (T, T, T, T), (T, T, T, T)))> for Mat4 {
    fn from(tuple: ((T, T, T, T), (T, T, T, T), (T, T, T, T), (T, T, T, T))) -> Self {
        Self::new(
            tuple.0.into(),
            tuple.1.into(),
            tuple.2.into(),
            tuple.3.into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::math::*;
    #[test]
    fn test_mult() {
        let vec = Vec4::new(1.0, 2, 3.0, 1);

        let mat = Mat4::scalar_from_one(5.0);

        println!("{:#?}", mat * vec);

        /*
         *  should output:
         *  [5.0, 10.0, 15.0, 1.0]
         */

        let mat = Mat4::scalar_from_one(5.0);
        let rhs = Mat4::identity();

        /*
         *  should output:
         *  [5.0, 0.0, 0.0, 0.0]
         *  [0.0, 5.0, 0.0, 0.0]
         *  [0.0, 0.0, 5.0, 0.0]
         *  [0.0, 0.0, 0.0, 1.0]
         */
        println!("{:#?}", mat * rhs);

        let lhs: Mat4 = (
            (5.0, 7.0, 9.0, 10.0),
            (2.0, 3.0, 3.0, 8.0),
            (8.0, 10.0, 2.0, 3.0),
            (3.0, 3.0, 4.0, 8.0),
        )
            .into();

        let rhs = Mat4::new(
            (3, 10, 12, 18).into(),
            (12, 1, 4, 9).into(),
            (9, 10, 12, 2).into(),
            (3, 12, 4, 10).into(),
        );

        /*
         *  should output:
         *  [210, 267, 236, 271]
         *  [ 93, 149, 104, 149]
         *  [171, 146, 172, 268]
         *  [105, 169, 128, 169]
         */

        println!("{:#?}", lhs * rhs);
    }
}
