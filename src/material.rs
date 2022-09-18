use nalgebra::Matrix4x1;

pub struct Material {
    pub color: [f64; 3],
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(color: [f64; 3], ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn default() -> Self {
        Self {
            color: [1.0, 1.0, 1.0],
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
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
