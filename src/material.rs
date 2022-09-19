use std::vec;

use nalgebra::{Matrix4, Matrix4x1};
use crate::{geometry::Shape, world};

const _BLACK : [f64; 3] = [0.0,0.0,0.0];
const _WHITE : [f64; 3] = [1.0,1.0,1.0];
const _RED : [f64; 3] = [1.0,0.0,0.0];
const _GREEN : [f64; 3] = [0.0,1.0,0.0];
const _BLUE : [f64; 3] = [0.0,0.0,1.0];


pub struct Pattern {
    pub colors: Vec<[f64; 3]>,
    pub transform: Matrix4<f64>
}

impl Pattern {

    pub fn empty() -> Self {
        Self {
            colors: Vec::new(),
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            )
        }
    }

    pub fn stripe(a: [f64;3], b: [f64;3]) -> Self {
        Self {
            colors: vec![a, b],
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
        }
    }
}

pub fn stripe_at_object(pattern: &Pattern, object: &Shape, world_point: Matrix4x1<f64>) -> [f64; 3] {

    let object_point = object.transform.try_inverse().unwrap() * world_point;
    let pattern_point = pattern.transform.try_inverse().unwrap() * object_point;
    stripe_at(pattern, pattern_point)

}

pub fn stripe_at(pattern: &Pattern, point: Matrix4x1<f64>) -> [f64; 3] {
    // Alternate only in X
    let x = point[0];
    if x.floor() % 2.0 == 0.0 {
        pattern.colors[0]
    } else {
        pattern.colors[1]
    }
}



pub struct Material {
    pub color: [f64; 3],
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Pattern
}

impl Material {
    pub fn new(color: [f64; 3], ambient: f64, diffuse: f64, specular: f64, shininess: f64, pattern: Pattern) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            pattern
        }
    }

    pub fn default() -> Self {
        Self {
            color: [1.0, 1.0, 1.0],
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: Pattern::empty()
        }
    }
}

pub fn color_from_rgb(r: u32, g: u32, b: u32) -> [f64; 3] {

    [
        r as f64 / 255.0,
        g as f64 / 255.0,
        b as f64 / 255.0,
    ]

}
