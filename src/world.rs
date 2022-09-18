use crate::geometry::{Shape,translation, cross_4};
use crate::light::PointLight;
use nalgebra::{Matrix4x1, Matrix4};
pub struct World {
    pub lights: Vec<PointLight>,
    pub objects: Vec<Shape>,
}

impl World {

    pub fn new() -> Self {
        Self { lights: Vec::new(), objects: Vec::new() }
    }

    pub fn default() -> Self {
        let outer_sphere = Shape::new(0.0, 0.0, 0.0, 1.0);
        let inner_sphere = Shape::new(0.0, 0.0, 0.0, 0.5);
        let light = PointLight::new(1.0, Matrix4x1::new(-10.0, 10.0, -10.0, 1.0));

        Self {
            lights: vec![light],
            objects: vec![outer_sphere, inner_sphere],
        }
    }
}

pub fn view_transform(from: Matrix4x1<f64>, to: Matrix4x1<f64>, up: Matrix4x1<f64>) -> Matrix4<f64>{

    // from: camera/eye
    // to: point to look at
    // up: up vector

    // 1. Compute forward vector = "to" - "from"
    // 2. Compute left vector = forward x up_norm
    // 3. Compute true_up vector = left x forward
    // 4. Construct a matrix representing the orientation transformation
    //    | left_x     left_y     left_z     0 |
    //    | true_up_x  true_up_y  true_up_z  0 |
    //    | -forward_x -forward_y -forward_z 0 |
    //    | 0          0          0          0 |
    // 5. Translate the scene into place by multiplying orientation by:
    //    translation(-from.x, -from.y, -from.z)

    let forward = (to - from).normalize();
    let up_norm = up.normalize();
    let left = cross_4(&forward, &up_norm);
    let true_up = cross_4(&left, &forward);

    let orientation = Matrix4::new(
        left.x,     left.y,      left.z,     0.0,
        true_up.x,  true_up.y,   true_up.z,  0.0,
        -forward.x, -forward.y , -forward.z, 0.0,
        0.0,        0.0,         0.0,        1.0
    );

    orientation * translation(-from.x, -from.y, -from.z)
}