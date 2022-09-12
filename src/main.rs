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
use ray::Ray;
use renderer::camera_ray;
use geometry::{Sphere, scaling, translation, normal_at};
use nalgebra::{Matrix4x1, Matrix4};
use light::PointLight;
use material::Material;

use crate::{export::_save_png, renderer::lighting};
extern crate image;


fn main() {

    // -----------------------------------
    // --------- Preparing scene ---------
    // -----------------------------------

    // --------- Image ----------
    const X_RES : u32 = 1080;
    const Y_RES : u32 = 1080;
    let mut canvas = Canvas::new(X_RES as usize, Y_RES as usize);
    let camera_origin = Matrix4x1::new(0.0, 0.0, 5.0, 1.0);

    // --------- Objects ----------
    // wall
    let wall_x : f32 = 6.0;
    let wall_y : f32 = 6.0;
    let wall_z : f32 = 10.0;

    let mut sphere = Sphere::new(0.0, 0.0, 0.0, 1.0);
    sphere.set_transform(scaling(2.0, 2.0, 2.0));
    
    
    // --------- Initializing other variables ----------
    let x_inc = wall_x / X_RES as f32;
    let y_inc = wall_y / Y_RES as f32;
    let mut ray: Ray;
    let mut canvas_x = 0.0;
    let mut canvas_y = 0.0;
    
    
    // ------------------------------------
    // --------- Main loop start ----------
    // ------------------------------------
    for x in 0..X_RES {
        for y in 0..Y_RES {
            canvas_x = x as f32 * x_inc;
            canvas_y = y as f32 * y_inc;
            ray = camera_ray(canvas_x, canvas_y, camera_origin, wall_z, wall_x, wall_y);
            if let Some(i) = intersect_sphere(&ray, &sphere) {
                // println!("{:?}",i.0);
                canvas.write_pixel(x as usize, y as usize, [1.0,0.0,0.0])
           }
        }
    }
    // -----------------------------------------
    // --------- Main render loop end ----------
    // -----------------------------------------

    // --------- Saving render ---------- d
    _save_png("sphere.png", canvas);


}
