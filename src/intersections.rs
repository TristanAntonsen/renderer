use crate::constants::Ray;
use crate::geometry::Sphere;
use nalgebra::{min, Matrix1x4};


// determine the intersection t values (t1, t2) or None from a ray and a sphere
pub fn intersect_sphere(ray: &Ray, sphere: &Sphere) -> Option<(f32, f32)> {
    // vector from sphere origin to ray origin
    let sphere_to_ray = ray.origin - sphere.origin;

    let a = &ray.direction.dot(&ray.direction);
    let b = 2.0 * &ray.direction.dot(&sphere_to_ray);
    let c = &sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b.powf(2.0) - 4.0 * a * c;
    // if zero intersections
    if discriminant < 0.0 {
        return None;
    }
    // return intersections in ascending order
    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    Some((t1, t2))
}