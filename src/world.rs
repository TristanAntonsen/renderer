use crate::geometry::Sphere;
use crate::light::PointLight;
use nalgebra::Matrix4x1;
pub struct World {
    pub lights: Vec<PointLight>,
    pub objects: Vec<Sphere>,
}

impl World {

    pub fn new() -> Self {
        Self { lights: Vec::new(), objects: Vec::new() }
    }

    pub fn default() -> Self {
        let outer_sphere = Sphere::new(0.0, 0.0, 0.0, 1.0);
        let inner_sphere = Sphere::new(0.0, 0.0, 0.0, 0.5);
        let light = PointLight::new(1.0, Matrix4x1::new(-10.0, 10.0, -10.0, 1.0));

        Self {
            lights: vec![light],
            objects: vec![outer_sphere, inner_sphere],
        }
    }
}
