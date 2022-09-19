use nalgebra::{Matrix4, Matrix4x1, ComplexField};
use noise::{NoiseFn, Perlin};
use crate::{geometry::Shape, world};
const _BLACK : [f64; 3] = [0.0,0.0,0.0];
const _WHITE : [f64; 3] = [1.0,1.0,1.0];
const _RED : [f64; 3] = [1.0,0.0,0.0];
const _GREEN : [f64; 3] = [0.0,1.0,0.0];
const _BLUE : [f64; 3] = [0.0,0.0,1.0];

pub struct Pattern {
    pub colors: Vec<[f64; 3]>,
    pub transform: Matrix4<f64>,
    pub type_id: u8,
}

impl Pattern {

    pub fn empty() -> Self {
        Self {
            colors: Vec::new(),
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            type_id : 0
        }
    }

    pub fn gradient(a: [f64;3], b: [f64;3]) -> Self {
        Self {
            colors: vec![a, b],
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            type_id : 1
        }
    }

    pub fn stripe(a: [f64;3], b: [f64;3]) -> Self {
        Self {
            colors: vec![a, b],
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            type_id : 2
        }
    }

    pub fn rings(a: [f64;3], b: [f64;3]) -> Self {
        Self {
            colors: vec![a, b],
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            type_id : 3
        }
    }

    pub fn checker(a: [f64;3], b: [f64;3]) -> Self {
        Self {
            colors: vec![a, b],
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            type_id : 4
        }
    }

    pub fn perlin(a: [f64;3]) -> Self {
        Self {
            colors: vec![a],
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            type_id : 5
        }
    }

}

pub fn pattern_at_shape(pattern: &Pattern, object: &Shape, world_point: Matrix4x1<f64>) -> [f64; 3] {

    let object_point = object.transform.try_inverse().unwrap() * world_point;
    let pattern_point = pattern.transform.try_inverse().unwrap() * object_point;
    pattern_at(pattern, pattern_point)

}

pub fn pattern_at(pattern: &Pattern, point: Matrix4x1<f64>) -> [f64; 3] {

    let perlin = Perlin::new(); // probably quite slow due to calling many times, update for performance later

    match pattern.type_id {
        1 => gradient_at(pattern, point),
        2 => stripe_at(pattern, point),
        3 => rings_at(pattern, point),
        4 => checker_at(pattern, point),
        5 => perlin_at(pattern, &perlin, point),
        _ => checker_at(pattern, point)
    }

}


pub fn gradient_at(pattern: &Pattern, point: Matrix4x1<f64>) -> [f64; 3] {
    // Gradient in X direction
    let fraction = point.x - point.x.floor();
    let gradient = [ // do this in a more intelligent way
        // color a + distance * fraction
        pattern.colors[0][0] + (pattern.colors[1][0] - pattern.colors[0][0]) * fraction,
        pattern.colors[0][1] + (pattern.colors[1][1] - pattern.colors[0][1]) * fraction,
        pattern.colors[0][2] + (pattern.colors[1][2] - pattern.colors[0][2]) * fraction,
        ];


    gradient

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

pub fn rings_at(pattern: &Pattern, point: Matrix4x1<f64>) -> [f64; 3] {
    // Rings based on X & Z
    let d = (point[0].powf(2.0) + point[2].powf(2.0)).sqrt();
    if d.floor() % 2.0 == 0.0 {
        pattern.colors[0]
    } else {
        pattern.colors[1]
    }
}

pub fn checker_at(pattern: &Pattern, point: Matrix4x1<f64>) -> [f64; 3] {
    // Rings based on X & Z
    let c = point[0].floor() + point[1].floor() + point[2].floor();
    if c % 2.0 == 0.0 {
        pattern.colors[0]
    } else {
        pattern.colors[1]
    }
}

pub fn perlin_at(pattern: &Pattern, noise: &Perlin, point: Matrix4x1<f64>) -> [f64; 3] {
    // Perlin noise
    let scale = 10.0;
    let p = noise.get([point[0] * scale, point[1] * scale, point[2] * scale]);
    let c = pattern.colors[0];
    [
        c[0] * p,
        c[1] * p,
        c[2] * p
    ]
}



pub struct Material {
    pub color: [f64; 3],
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub pattern: Pattern
}

impl Material {
    pub fn new(color: [f64; 3], ambient: f64, diffuse: f64, specular: f64, shininess: f64, pattern: Pattern, reflective: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            reflective,
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
            reflective: 0.0,
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
