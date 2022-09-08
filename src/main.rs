mod constants;
mod export;
mod intersections;
mod geometry;
mod ray;

use intersections::{Intersection,Intersections,intersect_sphere};
use constants::{Canvas};
use ray::Ray;
use geometry::Sphere;
use nalgebra::{Matrix1x4};
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
        origin: Matrix1x4::new(1.1, 0.0, 0.0, 1.0), // sphere with radius 1 centered on origin
        radius: 1.0
    };
    
    let i1 = Intersection::new(0.4, &sphere);
    let i2 = Intersection::new(0.6, &sphere);
    let i3 = Intersection::new(0.6, &sphere);
    let i4 = Intersection::new(-0.1, &sphere);
    let i5 = Intersection::new(0.0, &sphere);

    let mut intersections = Intersections::collect(vec![i1, i3, i2, i4, i5]);


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
