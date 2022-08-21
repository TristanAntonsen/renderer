use std::fs;
mod constants;
mod export;
mod tests;
use constants::{point, vector, Canvas, Env, Projectile, Point, Vector};
use nalgebra::{Matrix1x4, Matrix4, Matrix4x1};
use tests::{is_point, is_vector, clock};
extern crate image;

fn main() {
    println!("Hello, world!");

    let test_point = Matrix1x4::new(0.0, 0.0, 3.0, 1.0);
    let test_vector_2 = Vector::new(1.0, 1.0, 1.0);

    let mut clock_canvas = Canvas::new(500, 500);


    // projectile_canvas.write_pixel(2, 8, [1.0, 1.0, 1.0]);
    let clock = clock(clock_canvas);

    //translating starting poi
    export::save_png("clock.png", clock);
}
