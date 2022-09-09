mod constants;
mod export;
mod intersections;
mod geometry;
mod ray;
mod renderer;

use intersections::{Intersection,Intersections,intersect_sphere};
use constants::{Canvas};
use ray::Ray;
use renderer::camera_ray;
use geometry::{Sphere, scaling, translation};
use nalgebra::{Matrix4x1, Matrix4};

use crate::export::_save_png;
extern crate image;


fn main() {

    // --------- Image parameters ----------
    const WIDTH : usize = 500;
    const HEIGHT : usize = 500;
    const CANVAS_DISTANCE : f32 = 25.0;
    let mut canvas = Canvas::new(WIDTH, HEIGHT);


    let camera_origin = Matrix4x1::new(0.0, 0.0, 0.0, 1.0);
    let sphere_origin = Matrix4x1::new(0.0, 0.0, 5.0, 1.0);
    
    let mut ray_direction = Matrix4x1::new(0.0, 0.0, 1.0, 0.0);

    let ray = camera_ray(10, 10, camera_origin, CANVAS_DISTANCE);
    println!("ray: {:?}",ray.origin);
    println!("ray: {:?}",ray.direction);
    let mut sphere = Sphere::new(0.0, 0.0, 0.0, 1.0);
    // sphere.set_transform(translation(5.0, 0.0, 0.0));
    

    let mut intersections = Intersections{collection: Vec::new()};


    // if let Some(i) = intersect_sphere(&ray1, &sphere) {
    //      // may need to be refactored so both intersections are added automatically (pg. 68)
    //     intersections.collection.push(Intersection::new(i.0, &sphere));
    //     intersections.collection.push(Intersection::new(i.1, &sphere));
    // }

    // if let Some(h) = intersections.hit() { // do this if h is Some(...)
    //     println!("hit: {:?}",h)
    // }

    _save_png("sphere.png", canvas);


}
