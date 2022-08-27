use std::fs;
mod constants;
mod export;
mod features;
use constants::{Canvas, Ray, Sphere};
use nalgebra::{Matrix1x4};
use features::{position, sphere_intersection};
extern crate image;


fn main() {

    let mut ray = Ray {
        origin: Matrix1x4::new(0.0, 0.0, 0.0, 1.0),
        direction: Matrix1x4::new(1.0, 0.0, 0.0, 0.0)
    };

    let sphere = Sphere {
        origin: Matrix1x4::new(0.0, 0.0, 0.0, 1.0),
        radius: 1.0
    };
    let mut intersections: Vec<(f32, f32)> = Vec::new();
    
    match sphere_intersection(&ray, &sphere) {
        Some(t) => intersections.push(t),
        None => println!("No intersection.")
    };
    ray.origin[1] = 5.0;

    match sphere_intersection(&ray, &sphere) {
        Some(t) => intersections.push(t),
        None => println!("No intersection.") //can also be None => ()
    };
    
}
