mod constants;
mod geometry;
mod features;
#[cfg(test)]
mod tests {
    use crate::geometry::{_Point, Sphere};
    use crate::constants::Ray;
    use crate::features::{Intersection};
    use nalgebra::Matrix1x4;

    #[test]
    fn test_point() {
        let point = _Point{pos: [0.0, 0.0, 0.0, 1.0]};
        assert_eq!(point.pos[3], 1.0);
    }

    #[test]
    fn test_intersection() { //not sure if this is helpful
        let ray = Ray {
            origin: Matrix1x4::new(0.0, 0.0, 0.0, 1.0),
            direction: Matrix1x4::new(1.0, 0.0, 0.0, 0.0)
        };
        let sphere = Sphere {
            origin: Matrix1x4::new(5.0, 0.0, 0.0, 1.0),
            radius: 1.0
        };
        let i1 = Intersection::new(&ray, &sphere);

        assert_eq!(i1.object.origin, sphere.origin);
    }

}