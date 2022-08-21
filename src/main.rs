use std::fs;
mod constants;
mod export;
mod tests;
use constants::{point, vector, Canvas, Env, Projectile, Point, Vector};
use nalgebra::Matrix1x4;
use tests::{is_point, is_vector};
extern crate image;

fn main() {
    println!("Hello, world!");

    let test_point = Matrix1x4::new(0.0, 0.0, 3.0, 1.0);
    let test_vector_2 = Vector::new(1.0, 1.0, 1.0);

    let mut projectile_canvas = Canvas::new(500, 500);

    let env = Env::new(0.050, 0.0);
    let starting_position = Matrix1x4::new(0.0, 0.0, 0.0, 1.0);
    let starting_velocity = Matrix1x4::new(3.5, 3.5, 0.0, 1.0);
    let trajectory = tests::launch(projectile_canvas, env, starting_position, starting_velocity);


    // projectile_canvas.write_pixel(2, 8, [1.0, 1.0, 1.0]);


    export::save_png("projectile.png", trajectory);
}
