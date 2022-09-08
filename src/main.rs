mod constants;
mod export;
mod intersections;
mod geometry;

use intersections::{Intersection,Intersections,intersect_sphere};
use constants::{Canvas, Ray};
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
        origin: Matrix1x4::new(0.0, 0.0, 0.0, 1.0), // sphere with radius 1 centered on origin
        radius: 1.0
    };
    
    let i1 = Intersection::new(0.4, &sphere);
    let i2 = Intersection::new(0.6, &sphere);
    let i3 = Intersection::new(-0.6, &sphere);
    let i4 = Intersection::new(0.1, &sphere);
    let i5 = Intersection::new(0.0, &sphere);

    println!("{},{:?}",i1.t, i1.object);
    println!("{},{:?}",i2.t, i2.object);

    println!("{:?}",i1);

    let intersections = Intersections::collect(vec![i1, i2, i3, i4, i5]);

    for i in &intersections.collection {
        println!("{:?}",i)
    }

    if let h = intersections.hit() { // do this if h is Some(...)
        println!("hit: {:?}",h.unwrap())
    }

}
