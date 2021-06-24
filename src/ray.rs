// Ray type

use crate::vec3::Vec3;

pub struct Ray<'a> {
    _a: &'a Vec3,
    _b: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub const fn new(_a: &'a Vec3, _b: &'a Vec3) -> Ray<'a> {
        Ray { _a, _b }
    }

    pub const fn origin(&self) -> &Vec3 {
        self._a
    }
    pub const fn direction(&self) -> &Vec3 {
        self._b
    }
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self._a + self._b * t
    }
}
