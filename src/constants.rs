use nalgebra::{Matrix1x4};
pub type color = [f32; 3];

// struct for Sphere object used for calculating intersections
#[derive(Debug)] //automatically implementing traits
pub struct Sphere {
    pub origin: Matrix1x4<f32>,
    pub radius: f32
}

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
//assumes form of [x,y,z,1] (w = 1 means Point)
pub struct _Point {
    pub pos: [f32; 4],
}

//assumes form of [i,j,k,1] (w = 0 means Vector)
pub struct _Vector {
    pub dir: [f32; 4],
}

pub struct Ray {
    pub origin: Matrix1x4<f32>,
    pub direction: Matrix1x4<f32>
}

impl _Point {
    pub fn _new(x: f32, y: f32, z: f32) -> Self {
        Self {
            pos: [x, y, z, 1.0],
        }
    }
}

impl _Vector {
    pub fn _new(x: f32, y: f32, z: f32) -> Self {
        Self {
            dir: [x, y, z, 0.0],
        }
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