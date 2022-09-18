use crate::geometry::{Shape, normal_at};
use crate::ray::position;
use crate::ray::Ray;
use crate::world::World;
use nalgebra::{Matrix4x1};

// ------------ INTERSECTION STRUCTS -------------

// intersections structure for aggregating intersections & performing methods
pub struct Intersections<'a> {
    pub collection: Vec<Intersection<'a>>
}
// intersection structure for t and object for given intersection
pub struct Intersection<'a> {
    pub t : f64,
    pub object: &'a Shape // object must outlive Intersection
}

// ------------ INTERSECTION TRAITS -------------

impl<'a> Intersections<'a> {

    pub fn init() -> Self {
        Self {
            collection: Vec::new()
        }
    }

    pub fn collect(ints: Vec<Intersection<'a>>) -> Self {
        Self {
            collection: ints
        }
    }

    pub fn hit(mut self) -> Option<Intersection<'a>> {

        //collect t values of self into a vector
        // let mut t_vals = self.collection.iter().map(|I| I.t).collect::<Vec<f64>>();
        
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
    pub fn new(t: f64, s: &'a Shape) -> Self {
        Self {
            t: t,
            object: s
        }
    }
}

// ------------ Precomputing & storing computations ------------

pub struct Comps<'a> {
    pub object: &'a Shape,
    pub point: Matrix4x1<f64>,
    pub over_point: Matrix4x1<f64>,
    pub eyev: Matrix4x1<f64>,
    pub normalv: Matrix4x1<f64>,
    pub inside: bool
}


pub fn prepare_computations<'a>(int: &'a Intersection, ray: &Ray) -> Comps<'a> {
    let object = int.object;
    let point = position(ray, int.t);
    let inside;
    let mut normal = normal_at(object, point);
    let EPSILON = 0.00001;
    let over_point = point + normal * EPSILON;
    let eyev = -ray.direction;
    if normal.dot(&eyev) < 0.0 {
        inside = true;
        normal = -normal;
    } else {
        inside = false;
    }


    Comps {
        object,
        point,
        over_point,
        eyev,
        normalv : normal,
        inside
    }
}

// ------------ WORLD INTERSECTIONS ------------

pub fn intersect_world<'a>(ray: &'a Ray, world: &'a World) -> Intersections<'a> {

    let mut intersections = Intersections::init();

    for object in world.objects.iter() {
        if let Some(i) = intersect_sphere(&ray, &object) {
            intersections.collection.push(Intersection::new(i.0, &object));
            intersections.collection.push(Intersection::new(i.1, &object));
        }
    }

    intersections.collection.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap()); //sorting by t1

    intersections

}

// ------------ ABASTRACT SHAPE INTERSECTION FUNCTIONS -------------

pub fn intersect(shape: &Shape, ray: &Ray) {
    // make sure ray is transformed
    let local_ray = ray.transform(shape.transform.try_inverse().unwrap());
    // pass onto concrete intersection implementation
    // return local_intersect(shape, local_ray);
}


// ------------ OBJECT SPECIFIC INTERSECTION FUNCTIONS -------------

// determine the intersection t values (t1, t2) or None from a ray and a sphere
pub fn intersect_sphere(ray: &Ray, sphere: &Shape) -> Option<(f64, f64)> {

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

// impl<'a> fmt::Debug for Intersection<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "t: {}, object: {:?}", self.t, self.object)
//     }
// }