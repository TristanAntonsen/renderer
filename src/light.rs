use nalgebra::Matrix4x1;

struct PointLight {
    pub intensity :[f32; 3],
    pub position: Matrix4x1<f32>
}

impl PointLight {
    pub fn new(intensity: [f32;3],position: Matrix4x1<f32>) -> Self {
        Self {
            intensity,
            position
        }
    }
}