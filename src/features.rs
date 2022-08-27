use crate::constants::{Ray, Sphere};
use nalgebra::{Matrix1x4, Matrix4, Matrix4x1};

pub fn position(mut ray: &Ray, t: &f32) -> Matrix1x4<f32> {
    let mut result = ray.origin + ray.direction * *t; //dereference t?
    result[3] = 0.0; //point type

    return result;
}
#[derive(Debug)] //automatically implementing traits
pub struct intersection<'a> {
    t: f32,
    object: &'a Sphere,
}

impl<'a> intersection<'a> {
    pub fn from_components(t: f32, object: &'a Sphere) -> Self {
        Self {
            t: t,
            object: object
        }
    }
}

pub fn sphere_intersection(ray: &Ray, sphere: &Sphere) -> Option<(f32, f32)> {
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
