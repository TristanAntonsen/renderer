use nalgebra::{Matrix4x1, Matrix4};


pub fn reflect(in_vector:Matrix4x1<f32>, normal: Matrix4x1<f32>) -> Matrix4x1<f32> {
    return in_vector - normal * 2.0 * in_vector.dot(&normal)
}



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
        new_ray.direction = scaling_matrix * self.direction;
        new_ray
    }
    pub fn transform(&self, mat: Matrix4<f32>) -> Ray {
        // to do:
        // try making the matrix a struct/enum with a translation type and translation type

        let mut new_ray = self.clone();
        new_ray.origin = mat * self.origin;
        new_ray.direction = mat * self.direction; // not sure if translation will break this, need to test
        new_ray

    }

    // may need to combine into one single transform() trait

}

