use std::fs;
mod constants;
mod export;
mod tests;
use constants::{point, vector, Canvas, Ray, Env, Projectile, Point, Vector};
use nalgebra::{Matrix1x4, Matrix4, Matrix4x1};
use tests::{is_point, is_vector, clock, position};
extern crate image;

fn main() {
    println!("Hello, world!");

    let test_point = Matrix1x4::new(0.0, 0.0, 3.0, 1.0);
    let test_vector_2 = Vector::new(1.0, 1.0, 1.0);

    let mut clock_canvas = Canvas::new(500, 500);

    // projectile_canvas.write_pixel(2, 8, [1.0, 1.0, 1.0]);

    let ray = Ray {
        origin: Matrix1x4::new(0.0, 0.0, 0.0, 1.0),
        direction: Matrix1x4::new(1.0, 0.0, 0.0, 0.0)
    };

    let ray_pos = position(&ray, 5.0);

    println!("Ray origin: {:?}",ray.origin);
    println!("Ray direction: {:?}",ray.direction);
    println!("Ray end position: {:?}",ray_pos);
}
