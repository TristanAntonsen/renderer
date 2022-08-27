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

    let ray_pos = position(&ray, 5.0);
    let sphere = Sphere {
        origin: Matrix1x4::new(0.0, 0.0, 0.0, 1.0),
        radius: 1.0
    };

    ray.origin[1] = 5.0;
    
    let mut intersections: Vec<(f32, f32)> = Vec::new();
    
    let mut result = sphere_intersection(&ray, &sphere);
    
    match result {
        Some(x) => intersections.push(x),
        None => println!("No intersection found.")
    }
    println!("Intersections: {:?}",intersections);
    // ray.origin[1] = 5.0;
}
