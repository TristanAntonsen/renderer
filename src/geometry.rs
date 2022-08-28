use nalgebra::{Matrix1x4};

// struct for Sphere object used for calculating intersections
#[derive(Debug)] //automatically implementing traits
pub struct Sphere {
    pub origin: Matrix1x4<f32>,
    pub radius: f32
}


//assumes form of [x,y,z,1] (w = 1 means Point)
pub struct _Point {
    pub pos: [f32; 4],
}

//assumes form of [i,j,k,1] (w = 0 means Vector)
pub struct _Vector {
    pub dir: [f32; 4],
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