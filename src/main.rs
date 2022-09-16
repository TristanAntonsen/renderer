mod constants;
mod export;
mod geometry;
mod intersections;
mod light;
mod material;
mod ray;
mod renderer;
mod world;

use constants::Canvas;
use geometry::{normal_at, scaling, translation, Sphere};
use intersections::{intersect_sphere, Intersection, Intersections};
use light::PointLight;
use material::{color_from_rgb, Material};
use nalgebra::{Matrix4, Matrix4x1, Point};
use ray::{position, Ray};
use renderer::camera_ray;
use world::World;

use crate::{export::_save_png, renderer::lighting};
extern crate image;

fn main() {
    // -----------------------------------
    // --------- Preparing scene ---------
    // -----------------------------------

    // --------- Image ----------
    const X_RES: u32 = 1080;
    const Y_RES: u32 = 1080;
    let mut canvas = Canvas::new(X_RES as usize, Y_RES as usize);
    let camera_origin = Matrix4x1::new(0.0, 0.0, -5.0, 1.0);

    // --------- Wall ----------
    let wall_x: f32 = 6.0;
    let wall_y: f32 = 6.0;
    let wall_z: f32 = 10.0;


    // --------- Light ----------
    let mut light = PointLight::new(1.0, Matrix4x1::new(-10.0, -10.0, -10.0, 1.0));
    light.intensity = 1.0;

    // --------- World -----------
    let mut world = World::default();
    world.objects[0].material.color = color_from_rgb(52, 235, 158);
    world.objects[0].set_transform(scaling(0.5, 0.5, 0.5));

    println!("World: ");
    println!("Sphere: {:?}", world.objects[0].material.color);
    println!("Sphere: {:?}", world.objects[1].radius);
    println!("Lights: {:?}", world.lights[0].position);

    // --------- Initializing other variables ----------
    let x_inc = wall_x / X_RES as f32;
    let y_inc = wall_y / Y_RES as f32;
    let mut ray: Ray;
    let mut canvas_x = 0.0;
    let mut canvas_y = 0.0;
    let mut intersections: Intersections;
    let (mut color, mut point, mut normal, mut eye);
    let mut t;

    // ------------------------------------
    // --------- Main loop start ----------
    // ------------------------------------
    for x in 0..X_RES {
        for y in 0..Y_RES {
            canvas_x = x as f32 * x_inc;
            canvas_y = y as f32 * y_inc;
            ray = camera_ray(canvas_x, canvas_y, camera_origin, wall_z, wall_x, wall_y);
            
            if let Some(i) = intersect_sphere(&ray, &world.objects[0]) {
                t = i.0;
                // println!("{}",t);
                // intersections
                if t > 0.0 {
                    //if the intersection is visible
                    point = position(&ray, t);
                    normal = normal_at(&world.objects[0], point);
                    eye = -ray.direction;
                    color = lighting(&mut world.objects[0].material, &light, point, eye, normal);
                    canvas.write_pixel(x as usize, y as usize, color);
                }
            }
        }
    }
    // -----------------------------------------
    // --------- Main render loop end ----------
    // -----------------------------------------

    // --------- Saving render ---------- d
    _save_png("sphere.png", canvas);
}
