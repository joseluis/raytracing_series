// Vec3 type
//
// - Operation overloading
//   - Works:
//      - Vec3+Vec3, Vec3-Vec3, Vec3*Vec3, Vec3/Vec3
//      - Vec3*f32, Vec3/f32
//   - Doesn't work:
//      - f32*Vec3, f32*Vec3
//

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl fmt::Display for Vec3 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(fmt, "[{}, {}, {}]", self.0, self.1, self.2)
        write!(fmt, "Vec3({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3(0., 0., 0.)
    }

    // NOTE: can't be const with sqrt
    // https://github.com/rust-lang/rust/issues/57563
    #[inline]
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }
    #[inline]
    pub fn squared_length(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    #[inline]
    pub const fn x(&self) -> f32 {
        self.0
    }
    #[inline]
    pub const fn y(&self) -> f32 {
        self.1
    }
    #[inline]
    pub const fn z(&self) -> f32 {
        self.2
    }

    #[inline]
    pub const fn r(&self) -> f32 {
        self.0
    }
    #[inline]
    pub const fn g(&self) -> f32 {
        self.1
    }
    #[inline]
    pub const fn b(&self) -> f32 {
        self.2
    }

    #[inline]
    pub fn scale(&self, float: f32) -> Vec3 {
        Vec3(self.0 * float, self.1 * float, self.2 * float)
    }

    // Inner product
    #[inline]
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    // Cross product
    #[inline]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self.0 * rhs.2 - rhs.1 * self.2,
            self.1 * rhs.0 - rhs.2 * self.0,
            self.2 * rhs.1 - rhs.0 * self.1,
        )
    }
}

// Neg (-)
impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}
impl Neg for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

// Add (+)
impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Add<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

// Sub (-)
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// Mul (*)
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl Mul<Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, float: f32) -> Self::Output {
        Vec3(self.0 * float, self.1 * float, self.2 * float)
    }
}
impl Mul<f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, float: f32) -> Self::Output {
        Vec3(self.0 * float, self.1 * float, self.2 * float)
    }
}

// Div (/)
impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl Div<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl Div<Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl Div<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, float: f32) -> Self::Output {
        Vec3(self.0 / float, self.1 / float, self.2 / float)
    }
}
impl Div<f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, float: f32) -> Self::Output {
        Vec3(self.0 / float, self.1 / float, self.2 / float)
    }
}

// TODO: +assign
// -----
//
// AddAssign (+=)
impl AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

// SubAssign (-=)
impl SubAssign<Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// MulAssign (*=)
impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

// DivAssign (/=)
impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec3) {
        *self = Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
