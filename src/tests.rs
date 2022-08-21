use nalgebra::Matrix1x4;

pub fn is_point(tuple: Matrix1x4<f32>) -> bool {
    match tuple[3] {
        1.0 => return true,
        _ => return false
    }
}

pub fn is_vector(tuple: Matrix1x4<f32>) -> bool {
    match tuple[3] {
        0.0 => return true,
        _ => return false
    }
}