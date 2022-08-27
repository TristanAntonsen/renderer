use std::fs;
mod constants;
mod export;
mod features;
use constants::{Canvas, Ray, Sphere};
use nalgebra::{Matrix1x4};
use features::{position, sphere_intersection, intersection};
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
    let mut i1: (f32, f32) = (0.0,0.0); //initializing with zero
    match sphere_intersection(&ray, &sphere) {
        Some(t) => i1 = t,
        None => println!("No intersection.")
    };
    ray.origin[1] = 5.0;
    
    let t1 = i1.0; //first t value of intersection
    let p1 = position(&ray, &t1);

    let i1 = intersection::from_components(t1, &sphere);

    println!("{:?}\n",i1);
    println!("t value: {:?}\n",i1.t);
    println!("object: {:?}",i1.object);
}
