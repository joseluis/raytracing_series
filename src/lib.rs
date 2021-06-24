//! Implementation of the RayTracing series
//!
//! - <https://raytracing.github.io/books/RayTracingInOneWeekend.html>
//! - <https://raytracing.github.io/books/RayTracingTheNextWeek.html>
//! - <https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html>
//!

mod ray;
mod render;
mod vec3;

pub use ray::Ray;
pub use render::{color, hit_sphere};
pub use vec3::Vec3;
