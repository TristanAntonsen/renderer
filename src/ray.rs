use nalgebra::{Matrix4x1, Matrix4};

#[derive(Clone)]
pub struct Ray {
    pub origin: Matrix4x1<f32>,
    pub direction: Matrix4x1<f32>
}

impl Ray {
    pub fn translate(&self, x: f32, y: f32, z: f32) -> Ray {

        let mut new_ray = self.clone();
        let translation_matrix = Matrix4::new(
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        );

        new_ray.origin = translation_matrix * self.origin;

        new_ray
    }
    pub fn scale(&self, x: f32, y: f32, z: f32) -> Ray {

        let mut new_ray = self.clone();
        let scaling_matrix = Matrix4::new(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        new_ray.origin = scaling_matrix * self.origin;

        new_ray
    }
}