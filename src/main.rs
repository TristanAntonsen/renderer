use std::fs;
mod constants;
mod export;
mod features;
use constants::{Canvas, Ray, Env, Projectile};
use nalgebra::{Matrix1x4};
use features::{position};
extern crate image;


fn main() {

    let ray = Ray {
        origin: Matrix1x4::new(0.0, 0.0, 0.0, 1.0),
        direction: Matrix1x4::new(1.0, 0.0, 0.0, 0.0)
    };

    let ray_pos = position(&ray, 5.0);

    println!("Ray origin: {:?}",ray.origin);
    println!("Ray direction: {:?}",ray.direction);
    println!("Ray end position: {:?}",ray_pos);
}
