use nalgebra::{Matrix1x4};

pub struct Ray {
    pub origin: Matrix1x4<f32>,
    pub direction: Matrix1x4<f32>
}