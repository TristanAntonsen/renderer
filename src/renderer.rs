use crate::ray::Ray;
use nalgebra::{Matrix4x1};

pub fn camera_ray(x: usize, y: usize, camera_origin: Matrix4x1<f32>, canvas_distance: f32) -> Ray {

    let pixel_location = Matrix4x1::new(
        x as f32,
        y as f32,
        canvas_distance,
        1.0
    );

    let ray_direction = (pixel_location - camera_origin).normalize();

    Ray {
        origin: camera_origin,
        direction: ray_direction
    }

}