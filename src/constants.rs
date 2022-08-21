use nalgebra::{Matrix1x3,Matrix1x4};
// pub type point = [f32; 3];
pub type point = Matrix1x4<f32>;
// pub type vector = [f32; 4];
pub type vector = Matrix1x4<f32>;

pub struct Point {
    pub pos: [f32; 4]
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            pos: [
                x, y, z, 1.0
            ]
        }
    }
}

pub struct Vector {
    pub dir: [f32; 4]
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            dir: [
                x, y, z, 0.0
            ]
        }
    }
}