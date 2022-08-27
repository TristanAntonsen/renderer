use std::fs;
mod constants;
mod export;
mod features;
use constants::{Canvas, Ray, Sphere};
use nalgebra::{Matrix1x4};
use features::{position, sphere_intersection, Intersection, Intersections};
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
    // ray.origin[1] = 5.0; //won't intersect
    
    let test_intersection = Intersection::new(&ray, &sphere);
    println!("{:?}",&test_intersection);
    let mut collection = Intersections {all: Vec::new()};
    collection.add(test_intersection);
    println!("{:?}",collection.all[0])
}
