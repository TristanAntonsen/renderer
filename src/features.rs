use crate::constants::{Ray, Sphere};
use nalgebra::{Matrix1x4, Matrix4, Matrix4x1};

// Calculating the position of an intersection (from distance t along a ray)
pub fn position(mut ray: &Ray, t: &f32) -> Matrix1x4<f32> {
    let mut result = ray.origin + ray.direction * *t; //dereference t?
    result[3] = 0.0; //point type

    return result;
}
// Struct containing the t value (distance along ray) and the object the ray is intersecting
#[derive(Debug)] //automatically implementing traits
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

// Struct for collecting many intersections including information about the objects
pub struct Intersections<'a> {
    pub all: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    // adding a new Intersection to Intersections
    pub fn add(&mut self, i: Intersection<'a>) {
        self.all.push(i);
    }
}

impl<'a> Intersection<'a> {
    // creating a new intersection from t and the object (sphere for now)
    pub fn from_components(t: f32, object: &'a Sphere) -> Self {
        Self {
            t: t,
            object: object,
        }
    }
    // need a function to create an intersection from a ray and object
    pub fn new(ray: &Ray, object: &'a Sphere) -> Self{
        let mut i: (f32, f32) = (0.0,0.0); //initializing with zero
        match sphere_intersection(&ray, &object) { //intersect with sphere (generalize later)
            Some(t) => i = t,
            None => ()
        };
        Self {
            t: i.0, //first t value
            object: object, //original object
        }
    }
}

// determine the intersection t values (t1, t2) or None from a ray and a sphere
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
