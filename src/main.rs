mod constants;
mod export;
mod intersections;
mod geometry;
mod ray;

use intersections::{Intersection,Intersections,intersect_sphere};
use constants::{Canvas};
use ray::Ray;
use geometry::Sphere;
use nalgebra::{Matrix4x1};
extern crate image;


fn main() {

    let ray1 = Ray {
        origin: Matrix4x1::new(1.0, 2.0, 3.0, 1.0), // ray centered on origin
        direction: Matrix4x1::new(0.0, 1.0, 0.0, 0.0) // cast in X direction
    };
    let ray2 = Ray {
        origin: Matrix4x1::new(0.0, 0.0, 0.0, 1.0), // ray centered on y = 0.25
        direction: Matrix4x1::new(1.0, 0.0, 0.0, 0.0) // cast in X direction
    };

    // let ray3 = &ray1.translate(3.0, 4.0, 5.0);
    let ray3 = &ray1.scale(2.0, 3.0, 4.0);

    println!("Ray 1: {:?}",ray1.origin);
    println!("Ray 3: {:?}",ray3.origin);
    println!("Ray 3: {:?}",ray3.direction);
    // ray.origin[1] = 5.0; //won't intersect

    let sphere = Sphere {
        origin: Matrix4x1::new(1.1, 0.0, 0.0, 1.0), // sphere with radius 1 centered on origin
        radius: 1.0
    };
    
    let i1 = Intersection::new(0.4, &sphere);
    let i2 = Intersection::new(0.6, &sphere);

    let mut intersections = Intersections::collect(vec![i1, i2]);


    if let Some(i) = intersect_sphere(&ray1, &sphere) {
        intersections.collection.push(Intersection::new(i.0, &sphere))
    }



    for i in &intersections.collection {
        println!("{:?}",i)
    }

    if let Some(h) = intersections.hit() { // do this if h is Some(...)
        println!("hit: {:?}",h)
    }

}
