mod constants;
mod export;
mod intersections;
mod geometry;
use constants::{Canvas, Ray};
use geometry::Sphere;
use nalgebra::{Matrix1x4};
use intersections::{Intersection, Intersections};
extern crate image;


fn main() {

    let ray1 = Ray {
        origin: Matrix1x4::new(0.0, 0.0, 0.0, 1.0), // ray centered on origin
        direction: Matrix1x4::new(1.0, 0.0, 0.0, 0.0) // cast in X direction
    };
    let ray2 = Ray {
        origin: Matrix1x4::new(-5.0, 0.25, 0.0, 1.0), // ray centered on y = 0.25
        direction: Matrix1x4::new(1.0, 0.0, 0.0, 0.0) // cast in X direction
    };
    // ray.origin[1] = 5.0; //won't intersect

    let sphere = Sphere {
        origin: Matrix1x4::new(0.0, 0.0, 0.0, 1.0), // sphere with radius 1 centered on origin
        radius: 1.0
    };
    
    let test_intersection_1 = Intersection::new(&ray1, &sphere); 
    let test_intersection_2 = Intersection::new(&ray2, &sphere);

    let mut collection = Intersections {all: Vec::new()};
    collection.add(test_intersection_1);
    collection.add(test_intersection_2);

    for int in &collection.all {
        println!("Object origin: {:}",&int.object.origin);
        println!("t values: {}, {}\n",&int.t1, &int.t2);
    }

    let h = collection.hit();
    // println!("Hit: {:?}",&h.object);
    match h {
        Some(x) => println!("Hit: {}",&x._closest_to_zero()),
        None => println!("No non-zero t values.")
    }
}
