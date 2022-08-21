use nalgebra::{Matrix1x3, Matrix1x4};
pub type point = Matrix1x4<f32>;
pub type vector = Matrix1x4<f32>;
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
}

impl Canvas {
    pub fn write_pixel(&mut self, x: usize, y: usize, color: color) {
        self.pixels[x][y] = color
    }
}
pub struct Point {
    pub pos: [f32; 4],
}

pub struct Vector {
    pub dir: [f32; 4],
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            pos: [x, y, z, 1.0],
        }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            dir: [x, y, z, 0.0],
        }
    }
}

pub struct Env {
    pub gravity: Matrix1x4<f32>,
    pub wind: Matrix1x4<f32>
}
impl Env {
    pub fn new(g: f32, w: f32) -> Self {
        Self {
            gravity: Matrix1x4::new(0.0, -g, 0.0, 0.0),
            wind: Matrix1x4::new(-w, 0.0, 0.0, 0.0)
        }
    }
}

pub struct Projectile {
    pub position: Matrix1x4<f32>,
    pub velocity: Matrix1x4<f32>
}
