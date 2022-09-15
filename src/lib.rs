mod constants;
mod geometry;
mod intersections;
mod light;
mod material;
mod ray;
mod world;

#[cfg(test)]
mod tests {
    use crate::geometry::{Sphere, _Point};
    use crate::intersections::{intersect_sphere, Intersection};
    use crate::ray::{Ray,position};
    // use crate::intersections::{Intersection};
    use nalgebra::{Matrix4, Matrix4x1};

    #[test]
    fn test_sphere_intersection() {
        //not sure if this is helpful
        let ray = Ray {
            origin: Matrix4x1::new(0.0, 0.0, -5.0, 1.0),
            direction: Matrix4x1::new(0.0, 0.0, 1.0, 0.0),
        };
        let sphere = Sphere::new(0.0, 0.0, 0.0, 1.0);
        println!("Hey");
        if let Some(i) = intersect_sphere(&ray, &sphere) {
            let p1 = position(&ray, i.0);
            let p2 = position(&ray, i.1);
            println!("Point 1: {:?}", p1);
            println!("Point 2: {:?}", p2);
        }
        // assert_eq!(i1.object.origin, sphere.origin);
    }
}
