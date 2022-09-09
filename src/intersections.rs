use crate::geometry::Sphere;
use crate::ray::Ray;
use nalgebra::{min, Matrix1x4};
use std::fmt;

// ------------ INTERSECTION STRUCTS -------------

// intersections structure for aggregating intersections & performing methods
pub struct Intersections<'a> {
    pub collection: Vec<Intersection<'a>>
}
// intersection structure for t and object for given intersection
pub struct Intersection<'a> {
    pub t : f32,
    pub object: &'a Sphere // object must outlive Intersection
}

// ------------ INTERSECTION TRAITS -------------

impl<'a> Intersections<'a> {
    pub fn collect(ints: Vec<Intersection<'a>>) -> Self {
        Self {
            collection: ints
        }
    }

    pub fn hit(mut self) -> Option<Intersection<'a>> {

        //collect t values of self into a vector
        // let mut t_vals = self.collection.iter().map(|I| I.t).collect::<Vec<f32>>();
        
        self.collection.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap()); //sorting by t1

        for i in self.collection {
            if i.t > 0.0 {
                return Some(i)
            }
        }
        return None


    }

}

impl<'a> Intersection<'a> { //trait must also outlive Intersection
    pub fn new(t: f32, s: &'a Sphere) -> Self {
        Self {
            t: t,
            object: s
        }
    }
}


// ------------ OBJECT INTERSECTION FUNCTIONS -------------

// determine the intersection t values (t1, t2) or None from a ray and a sphere
pub fn intersect_sphere(ray: &Ray, sphere: &Sphere) -> Option<(f32, f32)> {

    // transform ray prior to calculation
    // multiply by the inverse of sphere.transform
    let transformation = sphere.transform.try_inverse().unwrap();

    let new_ray = ray.transform(transformation);

    // vector from sphere origin to ray origin
    let sphere_to_ray = new_ray.origin - sphere.origin;

    let a = &new_ray.direction.dot(&new_ray.direction);
    let b = 2.0 * &new_ray.direction.dot(&sphere_to_ray);
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


// ------------ DISPLAY/DEBUG -------------

impl<'a> fmt::Debug for Intersection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "t: {}, object: {:?}", self.t, self.object)
    }
}