use crate::ray::Ray;
use nalgebra::{Matrix4x1};

pub fn camera_ray(x: f32, y: f32, camera_origin: Matrix4x1<f32>, canvas_distance: f32, width: f32, height: f32) -> Ray {

    let canvas_point = Matrix4x1::new(
        x - width as f32 / 2.0,
        y - height as f32 / 2.0,
        canvas_distance,
        1.0
    );

    let ray_direction = (canvas_point - camera_origin).normalize();

    Ray {
        origin: camera_origin,
        direction: ray_direction
    }

}

