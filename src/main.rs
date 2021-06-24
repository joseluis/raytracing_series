//!
//!
//!

#![allow(unreachable_code, dead_code)]

use std::{fs::File, io::prelude::*};

mod ray;
mod render;
mod vec3;

use ray::Ray;
use render::color;
use vec3::Vec3;

fn main() {
    let height = 500;
    write_ppm(height * 2, height, 255)
}

fn write_ppm(w: u32, h: u32, max_value: u32) {
    let mut file = File::create("render-output.ppm").expect("File create failed");
    let buffer = format!("P3\n{} {}\n{}\n", w, h, max_value);
    file.write_all(buffer.as_bytes())
        .expect("File write failed");

    let lower_left_corner = Vec3(-2., -1., -1.);
    let horizontal = Vec3(4., 0., 0.);
    let vertical = Vec3(0., 2., 0.);
    let origin = Vec3(0., 0., 0.);

    for j in (0..h).rev() {
        for i in 0..w {
            let u = i as f32 / w as f32;
            let v = j as f32 / h as f32;
            let direction = lower_left_corner + horizontal * u + vertical * v;

            let r = Ray::new(&origin, &direction);
            let c = color(&r);

            let ir = (255.99 * c.r()) as u32;
            let ig = (255.99 * c.g()) as u32;
            let ib = (255.99 * c.b()) as u32;

            file.write_all(format!("{0} {1} {2}\n", ir, ig, ib).as_bytes())
                .expect("File write failed");
        }
    }
}
