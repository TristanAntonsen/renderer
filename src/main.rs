mod constants;
mod export;
mod intersections;
mod geometry;
mod ray;
mod renderer;
mod light;
mod material;


use intersections::{Intersection,Intersections,intersect_sphere};
use constants::{Canvas};
use ray::{Ray, position};
use renderer::camera_ray;
use geometry::{Sphere, scaling, translation, normal_at};
use nalgebra::{Matrix4x1, Matrix4, Point};
use light::PointLight;
use material::{Material, color_from_rgb};

use crate::{export::_save_png, renderer::lighting};
extern crate image;
use std::time::{Duration, Instant};


fn main() {

    // -----------------------------------
    // --------- Preparing scene ---------
    // -----------------------------------

    // --------- Image ----------
    const X_RES : u32 = 1080;
    const Y_RES : u32 = 1080;
    let mut canvas = Canvas::new(X_RES as usize, Y_RES as usize);
    let camera_origin = Matrix4x1::new(0.0, 0.0, -5.0, 1.0);

    // --------- Wall ----------
    let wall_x : f32 = 6.0;
    let wall_y : f32 = 6.0;
    let wall_z : f32 = 10.0;

    // --------- Sphere ----------
    let mut sphere = Sphere::new(0.0, 0.0, 0.0, 1.0);
    // sphere.set_transform(scaling(2.0, 2.0, 2.0));
    sphere.set_transform(scaling(0.5, 0.5, 0.5));
    // sphere.material.color = [1.0, 0.2, 1.0];
    sphere.material.color = color_from_rgb(181, 126, 220);


    // --------- Light ----------
    let mut light = PointLight::new(1.0, Matrix4x1::new(
        -10.0,-10.0, -10.0, 1.0
    ));
    light.intensity = 1.0;
    
    // --------- Initializing other variables ----------
    let x_inc = wall_x / X_RES as f32;
    let y_inc = wall_y / Y_RES as f32;
    let mut ray: Ray;
    let mut canvas_x = 0.0;
    let mut canvas_y = 0.0;
    let mut intersections: Intersections;
    let (mut color, mut point, mut normal, mut eye);
    let mut t;
    // Initializing intersections with 1 intersection (update later with objects)

    // ------------------------------------
    // --------- Main loop start ----------
    // ------------------------------------
    let start = Instant::now();
    for x in 0..X_RES {
        for y in 0..Y_RES {
            canvas_x = x as f32 * x_inc;
            canvas_y = y as f32 * y_inc;
            ray = camera_ray(canvas_x, canvas_y, camera_origin, wall_z, wall_x, wall_y);

            if let Some(i) = intersect_sphere(&ray, &sphere) {
                t = i.0;
                // println!("{}",t);
                // intersections
                if t > 0.0 { //if the intersection is visible
                    point = position(&ray, t);
                    normal = normal_at(&sphere, point);
                    eye = -ray.direction;
                    color = lighting(&mut sphere.material, &light, point, eye, normal);
                    canvas.write_pixel(x as usize, y as usize, color);
                }

            }
        }
    }
    let duration = start.elapsed();
    println!("Elapsed time: {:?}", duration);
    // -----------------------------------------
    // --------- Main render loop end ----------
    // -----------------------------------------

    // --------- Saving render ---------- d
    _save_png("sphere.png", canvas);

}
