mod constants;
mod export;
mod intersections;
mod geometry;
mod ray;

use intersections::{Intersection,Intersections,intersect_sphere};
use constants::{Canvas};
use ray::Ray;
use geometry::{Sphere, scaling, translation};
use nalgebra::{Matrix4x1, Matrix4};
extern crate image;


fn main() {

    let ray1 = Ray {
        origin: Matrix4x1::new(0.0, 0.0, -5.0, 1.0), // ray centered on origin
        direction: Matrix4x1::new(0.0, 0.0, 1.0, 0.0) // cast in X direction
    };

    let mut sphere = Sphere::new(0.0, 0.0, 0.0, 1.0);
    sphere.set_transform(translation(5.0, 0.0, 0.0));
    

    let mut intersections = Intersections{collection: Vec::new()};


    if let Some(i) = intersect_sphere(&ray1, &sphere) {
         // may need to be refactored so both intersections are added automatically (pg. 68)
        intersections.collection.push(Intersection::new(i.0, &sphere));
        intersections.collection.push(Intersection::new(i.1, &sphere));
    }

    println!("Intersection count: {:?}",intersections.collection.len());
    println!("I2 t: {:?}",intersections.collection[0].t);
    println!("I1 t: {:?}",intersections.collection[1].t);

    if let Some(h) = intersections.hit() { // do this if h is Some(...)
        println!("hit: {:?}",h)
    }


}
