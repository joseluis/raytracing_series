//!
//!
//!

#![allow(unreachable_code, dead_code)]

use std::{fs::File, io::prelude::*};

use notcurses::*;

mod ray;
mod render;
mod vec3;

use ray::Ray;
use render::color;
use vec3::Vec3;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 50;

fn main() -> NResult<()> {
    let mut nc = Notcurses::new()?;

    let (cols, rows) = nc.term_size();
    // println!("cols:{}, rows:{}", cols, rows); sleep!(1); // DEBUG
    //let geom = nc.term_pixelgeometry();

    // let mut buffer = Vec::with_capacity(geom.max_bitmap_x as usize * geom.max_bitmap_y as usize * 3);
    // plot(&mut buffer, geom.max_bitmap_x, geom.max_bitmap_y);
    let mut buffer = Vec::with_capacity(WIDTH as usize * HEIGHT as usize * 3);
    fill_buffer(&mut buffer, WIDTH, HEIGHT);

    // let mut plane = Plane::with_term_size(&mut nc)?;
    let mut plane = Plane::build().cols_rows(cols, rows).new_pile(&mut nc)?;

    let mut visual = Visual::build()
        // .from_rgb(&buffer, geom.max_bitmap_x, geom.max_bitmap_y, 255)?
        .from_rgb(&buffer, WIDTH, HEIGHT, 255)?
        .blitter(Blitter::Pixel)
        // .interpolate(false)
        .plane(&mut plane)
        .finish()?;

    visual.render_plane(&mut nc)?;
    plane.show()?;
    sleep![1];
    Ok(())
}

fn fill_buffer(buffer: &mut Vec<u8>, w: u32, h: u32) {
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

            let ir = (255.99 * c.r()) as u8;
            let ig = (255.99 * c.g()) as u8;
            let ib = (255.99 * c.b()) as u8;

            buffer.push(ir);
            buffer.push(ig);
            buffer.push(ib);
        }
    }
}
