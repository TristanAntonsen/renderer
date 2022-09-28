use crate::geometry::{normal_at, sphere_normal_at, Shape};
use crate::ray::Ray;
use crate::ray::{position, reflect};
use crate::world::World;
use nalgebra::Matrix4x1;
const EPSILON: f64 = 0.00001;

// ------------ INTERSECTION STRUCTS -------------

// intersections structure for aggregating intersections & performing methods

pub struct Intersections<'a> {
    pub collection: Vec<Intersection<'a>>,
}
// intersection structure for t and object for given intersection
#[derive(Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Shape, // object must outlive Intersection
}

// ------------ INTERSECTION TRAITS -------------
impl<'a> Intersections<'a> {
    pub fn init() -> Self {
        Self {
            collection: Vec::new(),
        }
    }

    pub fn collect(ints: Vec<Intersection<'a>>) -> Self {
        Self { collection: ints }
    }

    pub fn hit(mut self) -> Option<Intersection<'a>> {
        //collect t values of self into a vector
        // let mut t_vals = self.collection.iter().map(|I| I.t).collect::<Vec<f64>>();
        self.collection
            .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap()); //sorting by t1

        for i in self.collection {
            if i.t > 0.0 {
                return Some(i);
            }
        }
        return None;
    }
}

impl<'a> Intersection<'a> {
    //trait must also outlive Intersection
    pub fn new(t: f64, s: &'a Shape) -> Self {
        Self { t: t, object: s }
    }
}

// ------------ Precomputing & storing computations ------------

pub struct Comps<'a> {
    pub object: &'a Shape,
    pub point: Matrix4x1<f64>,
    pub over_point: Matrix4x1<f64>,
    pub eyev: Matrix4x1<f64>,
    pub normalv: Matrix4x1<f64>,
    pub reflectv: Matrix4x1<f64>,
    pub n1: f64,
    pub n2: f64,
    pub inside: bool,
}

pub fn prepare_computations<'a>(int: &'a Intersection, ray: &Ray, xs: &Vec<Intersection<'a>>) -> Comps<'a> {
    //int = the hit
    //currently passing in xs.collection instead of xs. May need to refactor later to do more intelligenly
    let object = int.object;
    let point = position(ray, int.t);
    let inside;
    let mut normal = normal_at(object, point);
    let over_point = point + normal * EPSILON;
    let eyev = -ray.direction;
    let reflectv = reflect(ray.direction, normal);

    if normal.dot(&eyev) < 0.0 {
        inside = true;
        normal = -normal;
    } else {
        inside = false;
    }

    // refraction portion
    let mut containers: Vec<&Shape> = Vec::new();
    let mut n1 = 1.0;
    let mut n2 = 1.0;
    let mut is_hit : bool;
    for i in xs.iter() {

        is_hit = i.t == int.t && i.object == int.object; // if intersection is the hit. Enabled by PartialEq impl for Shape
        
        if is_hit { // if i = the hit
            if containers.len() == 0 {
                n1 = 1.0;
            } else {
                n1 = containers[containers.len() - 1].material.refractive_index;
            }
        };
        if containers.contains(&i.object) { // if containers includes i.object, remove from containers
            containers.retain(|o| o != &i.object) // may be slow and need to update later
        } else {
            containers.push(&i.object)
        };
        if is_hit { 
        
            if containers.len() == 0 {
                n2 = 1.0;
            } else {
                n2 = containers[containers.len() - 1].material.refractive_index;
            } 
        }
        // println!("{}, {}", n1, n2);
    }

    Comps {
        object,
        point,
        over_point,
        eyev,
        normalv: normal,
        reflectv,
        n1,
        n2,
        inside,
    }
}

// ------------ WORLD INTERSECTIONS ------------

pub fn intersect_world<'a>(ray: &'a Ray, world: &'a World) -> Intersections<'a> {
    let mut intersections = Intersections::init();

    for object in world.objects.iter() {
        if let Some(i) = intersect(object, ray) {
            // if let Some(i) = intersect_sphere(ray, object) {
            intersections
                .collection
                .push(Intersection::new(i.0, &object));
            intersections
                .collection
                .push(Intersection::new(i.1, &object));
        }
    }

    intersections
        .collection
        .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap()); //sorting by t1

    intersections
}

// ------------ ABASTRACT SHAPE INTERSECTION FUNCTIONS -------------

pub fn intersect(shape: &Shape, ray: &Ray) -> Option<(f64, f64)> {
    // make sure ray is transformed
    // this is already done in local functions, so could refactor and include here later
    // let local_ray = ray.transform(shape.transform.try_inverse().unwrap());
    // pass onto concrete intersection implementation
    // "local intersect" here until it needs to be moved elsewhere
    match shape.shape_id {
        0 => intersect_sphere(&ray, &shape),
        1 => intersect_plane(&ray, &shape),
        _ => None,
    }
}

// ------------ OBJECT SPECIFIC INTERSECTION FUNCTIONS -------------

pub fn intersect_cube(ray: &Ray, cube: &Shape) -> Option<(f64, f64)> {
    
    let (tmin, tmax);
    if let Some(txs) = check_axis(ray.origin.x, ray.direction.x) {
        
        let (tmin_x, tmax_x) = txs;
        let (tmin_y, tmax_y) = check_axis(ray.origin.y, ray.direction.y).unwrap();
        let (tmin_z, tmax_z) = check_axis(ray.origin.z, ray.direction.z).unwrap();
        
        tmin = f64::min(f64::min(tmin_x, tmin_y), tmin_z); //lazy, fix this later
        tmax = f64::max(f64::max(tmax_x, tmax_y), tmax_z);
    } else {
        return None
    }

    Some((tmin, tmax))

}

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

pub fn check_axis(origin: f64, direction: f64) -> Option<(f64, f64)> { //1 dimensional calculation
    
    let tmin_numerator = -1.0 - &origin;
    let tmax_numerator = 1.0 - &origin;

    let (tmin, tmax);

    if direction.abs() >= EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = f64::INFINITY;
        tmax = f64::INFINITY;
    }

    if tmin > tmax {
        return Some((tmax, tmin))
    } else {
        return Some((tmin, tmax))
    }

}

pub fn intersect_plane(ray: &Ray, plane: &Shape) -> Option<(f64, f64)> {
    // if ray is parallel to XY plane
    let transformation = plane.transform.try_inverse().unwrap();
    let new_ray = ray.transform(transformation);

    if ray.direction.y.abs() < EPSILON {
        return None;
    }

    let t = -&new_ray.origin.y / &new_ray.direction.y;
    return Some((t, t)); // might need to rework the double t
}

// ------------ DISPLAY/DEBUG -------------

// impl<'a> fmt::Debug for Intersection<'a> {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //         write!(f, "t: {}, object: {:?}", self.t, self.object)
        //     }
        // }
        
        
// ------------ TESTS -------------

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn cube_intersection() {
//         let cube = Shape::cube()
//         assert_eq!(result, 4);
//     }
// }