use world::World;

mod constants;
mod geometry;
mod intersections;
mod light;
mod material;
mod ray;
mod world;

#[cfg(test)]
mod tests {
    use crate::geometry::{_Point};
    use crate::intersections::{intersect_sphere, Intersection};
    use crate::ray::{Ray,position};
    use nalgebra::{Matrix4, Matrix4x1};
    use crate::world::World;

    #[test]
    fn test_sphere_intersection() {
        //not sure if this is helpful
        let ray = Ray {
            origin: Matrix4x1::new(0.0, 0.0, -5.0, 1.0),
            direction: Matrix4x1::new(0.0, 0.0, 1.0, 0.0),
        };

        let world = World::default();
        let sphere = &world.objects[0];


        if let Some(i) = intersect_sphere(&ray, &sphere) {
            let p1 = position(&ray, i.0);
            let p2 = position(&ray, i.1);
            println!("Point 1: {:?}", p1);
            println!("Point 2: {:?}", p2);
        }
        // assert_eq!(i1.object.origin, sphere.origin);
    }

    // #[test]
    // fn test_cube_intersection() {
    //     let ray = Ray {
    //         origin: Matrix4x1::new(5.0, 0.5, 5.0, 1.0),
    //         direction: Matrix4x1::new(-1.0, 0.0, 0.0, 0.0),
    //     };

    //     let world = World::default();
    //     let cube = &world.objects[0];

    //     let (t1, t2);

    //     if let Some(t) = intersect_sphere(&ray, &cube) {
    //         t1 = t.0;
    //         t2 = t.1;
    //         assert_eq!(t1, 4.0);
    //         assert_eq!(t2, 6.0);
    //     } else {
    //         panic!("No intersection.")
    //     }
    // }


}
