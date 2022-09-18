use nalgebra::Matrix4x1;

pub struct PointLight {
    pub intensity :f64,
    pub position: Matrix4x1<f64>
}

impl PointLight {
    pub fn new(intensity: f64, position: Matrix4x1<f64>) -> Self {
        Self {
            intensity,
            position
        }
    }
}