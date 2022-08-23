use nalgebra::{Matrix1x4,Matrix4x1, Matrix4};
use crate::constants::Ray;


pub fn position(mut ray: &Ray, t: f32) -> Matrix1x4<f32> {
    let mut result =  ray.origin + ray.direction * t;
    result[3] = 0.0;

    return result
}