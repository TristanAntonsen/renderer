use crate::geometry::{normal_at, Shape};
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

    // for each object in the scene:
    // 1. Check object for intersection with ray
    // 2. Create intersection for each t value
    // 3. Push intersections to world

    for object in world.objects.iter() {

        if let Some(i) = intersect(object, ray) {
            let mut is = i.collection;
            intersections.collection.append(&mut is);
        }
    }

    intersections
        .collection
        .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap()); //sorting by t1

    intersections
}

// ------------ ABASTRACT SHAPE INTERSECTION FUNCTIONS -------------

pub fn intersect<'a>(shape: &'a Shape, ray: &'a Ray) -> Option<Intersections<'a>> {
    // pass onto concrete intersection implementation
    match shape.shape_id {
        0 => intersect_sphere(&ray, &shape),
        1 => intersect_plane(&ray, &shape),
        2 => intersect_cube(&ray, &shape),
        3 => intersect_cylinder(&ray, &shape),
        _ => None,
    }
}

// ------------ OBJECT SPECIFIC INTERSECTION FUNCTIONS -------------

pub fn intersect_cube<'a>(ray: &'a Ray, cube: &'a Shape) -> Option<Intersections<'a>> {
    
    let (tmin, tmax);
    let (tmin_x, tmax_x, tmin_y, tmax_y, tmin_z, tmax_z);

    let transformation = cube.transform.try_inverse().unwrap();
    let new_ray = ray.transform(transformation);

    if let Some(txs) = check_axis(new_ray.origin.x, new_ray.direction.x) { (tmin_x, tmax_x) = txs; } else { return None }
    if let Some(tys) = check_axis(new_ray.origin.y, new_ray.direction.y) { (tmin_y, tmax_y) = tys; } else { return None }
    if let Some(tzs) = check_axis(new_ray.origin.z, new_ray.direction.z) { (tmin_z, tmax_z) = tzs; } else { return None }

    let mut min_tmp = [tmin_x, tmin_y, tmin_z];
    let mut max_tmp = [tmax_x, tmax_y, tmax_z];

    float_ord::sort(&mut min_tmp);
    float_ord::sort(&mut max_tmp);

    tmin = min_tmp[min_tmp.len() - 1]; //max of mins
    tmax = max_tmp[0]; //min of maxes

    if tmin > tmax {
        return None
    }

    let mut xs = Intersections::init();
    xs.collection.push(Intersection::new(tmin, cube));
    xs.collection.push(Intersection::new(tmax, cube));

    Some(xs)

}

// determine the intersection t values (t1, t2) or None from a ray and a sphere
pub fn intersect_sphere<'a>(ray: &'a Ray, sphere: &'a Shape) -> Option<Intersections<'a>> {
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

    let mut xs = Intersections::init();
    xs.collection.push(Intersection::new(t1, sphere));
    xs.collection.push(Intersection::new(t2, sphere));

    Some(xs)
}

// determine the intersection t values (t1, t2) or None from a ray and a sphere
pub fn intersect_cylinder<'a>(ray: &'a Ray, cylinder: &'a Shape) -> Option<Intersections<'a>> {
    // transform ray prior to calculation
    // multiply by the inverse of cylinder.transform
    let transformation = cylinder.transform.try_inverse().unwrap();
    let new_ray = ray.transform(transformation);

    let a = new_ray.direction.x.powf(2.0) + new_ray.direction.z.powf(2.0);

    if a < EPSILON { return None }

    let b = 2.0 * new_ray.origin.x * new_ray.direction.x +
        2.0 * new_ray.origin.z * new_ray.direction.z;

    let c = new_ray.origin.x.powf(2.0) + new_ray.origin.z.powf(2.0) - 1.0;

    let discriminant = b.powf(2.0) - 4.0 * a * c;

    // if zero intersections
    if discriminant < 0.0 { return None };
    
    // return intersections in ascending order
    let t1_tmp = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2_tmp = (-b + discriminant.sqrt()) / (2.0 * a);

    let (t1, t2);

    if t1_tmp > t2_tmp {
        t1 = t2_tmp;
        t2 = t1_tmp;
    } else {
        t1 = t1_tmp;
        t2 = t2_tmp;
    };

    let mut xs = Intersections::init();

    let y1 = new_ray.origin.y + t1 * new_ray.direction.y;
    if cylinder.bounds[0] < y1 && y1 < cylinder.bounds[1] {
        xs.collection.push(Intersection::new(t1, cylinder));
    }
    
    let y2 = new_ray.origin.y + t2 * new_ray.direction.y;
    if cylinder.bounds[0] < y2 && y2 < cylinder.bounds[1] {
        xs.collection.push(Intersection::new(t2, cylinder));
    }

    Some(xs)
}


pub fn check_cap(ray: &Ray,  t: f64) -> bool {

    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    let b = x.powf(2.0) + z.powf(2.0);

    if b <= 1.0 {
        true
    } else {
        false
    }
}

pub fn check_axis(origin: f64, direction: f64) -> Option<(f64, f64)> { //1 dimensional calculation
    
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let (tmin, tmax);

    if direction.abs() >= EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * f64::INFINITY;
        tmax = tmax_numerator * f64::INFINITY;
        // return None;
    }

    if tmin > tmax {
        return Some((tmax, tmin))
    } else {
        return Some((tmin, tmax))
    }

}

pub fn intersect_plane<'a>(ray: &'a Ray, plane: &'a Shape) -> Option<Intersections<'a>> {
    // if ray is parallel to XY plane
    let transformation = plane.transform.try_inverse().unwrap();
    let new_ray = ray.transform(transformation);

    if ray.direction.y.abs() < EPSILON {
        return None;
    }

    let t = -&new_ray.origin.y / &new_ray.direction.y;
    let mut xs = Intersections::init();

    xs.collection.push(Intersection::new(t, plane));

    Some(xs)}

// ------------ DISPLAY/DEBUG -------------

// impl<'a> fmt::Debug for Intersection<'a> {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //         write!(f, "t: {}, object: {:?}", self.t, self.object)
        //     }
        // }
        
