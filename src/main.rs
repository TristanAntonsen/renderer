mod constants;
mod export;
mod intersections;
mod geometry;

use intersections::{Intersection,intersect_sphere};
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
    
    let intersection1 = Intersection::new(0.4, &sphere);
    let intersection2 = Intersection::new(0.6, &sphere);


    println!("{},{:?}",intersection1.t, intersection1.object);
    println!("{},{:?}",intersection2.t, intersection2.object);

}
