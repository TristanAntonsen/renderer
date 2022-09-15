use nalgebra::Matrix4x1;

pub struct PointLight {
    pub intensity :f32,
    pub position: Matrix4x1<f32>
}

impl PointLight {
    pub fn new(intensity: f32, position: Matrix4x1<f32>) -> Self {
        Self {
            intensity,
            position
        }
    }
}