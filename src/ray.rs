// Ray type

use crate::vec3::Vec3;

///
pub struct Ray<'a> {
    origin: &'a Vec3,
    direction: &'a Vec3,
}

impl<'a> Ray<'a> {
    ///
    pub const fn new(origin: &'a Vec3, direction: &'a Vec3) -> Ray<'a> {
        Ray { origin, direction }
    }

    ///
    pub const fn origin(&self) -> &Vec3 {
        self.origin
    }

    ///
    pub const fn direction(&self) -> &Vec3 {
        self.direction
    }

    ///
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
