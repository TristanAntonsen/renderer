use nalgebra::{Matrix1x4};
pub type color = [f32; 3];

pub struct Canvas {
    pub pixels: Vec<Vec<color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![[0.0, 0.0, 0.0]; height]; width],
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: color) {
        self.pixels[x][y] = color
    }
}
pub struct _Env {
    pub gravity: Matrix1x4<f32>,
    pub wind: Matrix1x4<f32>
}
impl _Env {
    pub fn new(g: f32, w: f32) -> Self {
        Self {
            gravity: Matrix1x4::new(0.0, -g, 0.0, 0.0),
            wind: Matrix1x4::new(-w, 0.0, 0.0, 0.0)
        }
    }
}