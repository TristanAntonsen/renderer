mod constants;
mod export;
mod geometry;
mod intersections;
mod light;
mod material;
mod ray;
mod renderer;
mod world;

use std::f64::consts::PI;

use constants::Canvas;
use geometry::{normal_at, scaling, translation, Sphere, rotation_x,rotation_y};
use intersections::{intersect_sphere, intersect_world, Intersection, Intersections, prepare_computations, Comps};
use light::PointLight;
use material::{color_from_rgb, Material};
use nalgebra::{Matrix4, Matrix4x1, Point};
use ray::{position, Ray};
use renderer::{render, ray_for_pixel, color_at};
use world::{World, view_transform};

use crate::{export::_save_png, renderer::{lighting, shade_hit, Camera}};
extern crate image;

fn main() {
    // -----------------------------------
    // --------- Preparing scene ---------
    // -----------------------------------

    
    // --------- World -----------
    let mut world = World::new();

    // --------- Light ----------
    let mut light = PointLight::new(1.0, Matrix4x1::new(10.0, 5.0, 5.0, 1.0));
    light.intensity = 1.0;
    world.lights.push(light);

    // --------- Objects -----------
    let mut sphere_1 = Sphere::default();
    // sphere_1.material.color = color_from_rgb(255, 150, 0);
    world.objects.push(sphere_1);

    let mut sphere2 = Sphere::default();
    // sphere2.material.color = color_from_rgb(255, 255, 255);
    sphere2.transform = translation(0.0,3.0, 0.0);
    world.objects.push(sphere2);

    // let mut sphere3 = Sphere::default();
    // sphere3.material.color = color_from_rgb(200,0,200);
    // sphere3.transform = translation(0.0, 2.5, 0.0);
    // world.objects.push(sphere3);

    let mut floor = Sphere::default();
    floor.material.color = color_from_rgb(100,100,100);
    floor.transform = translation(0.0, -2.0, 0.0) * scaling(15.0, 0.01, 15.0);
    world.objects.push(floor);


    let mut wall_1 = Sphere::default();
    wall_1.material.color = color_from_rgb(100,100,100);
    wall_1.transform = 
        translation(0.0, 0.0, -5.0) * 
        rotation_y(-PI / 4.0) *
        rotation_x(PI / 2.0) *
        scaling(15.0, 0.01, 15.0);
    world.objects.push(wall_1);
    
    let mut wall_2 = Sphere::default();
    wall_2.material.color = color_from_rgb(100,100,100);

    wall_2.transform = 
        translation(0.0, 0.0, -5.0) * 
        rotation_y(PI / 4.0) *
        rotation_x(PI / 2.0) *
        scaling(15.0, 0.01, 15.0);
    world.objects.push(wall_2);

    // --------- Camera ----------
    let mut cam = Camera::default(1080, 1080, PI / 6.0);
    cam.transform = translation(0.0, 0.0, -15.0);

    // --------- Testing ray_for_pixel ----------

    let cam_ray = ray_for_pixel(&cam, 5, 5);
    let test_color = color_at(&world, &cam_ray);
    println!("test color: {:?}",test_color);
    println!("ray origin: {:?}", cam_ray.origin);
    println!("ray direction: {:?}", cam_ray.direction);

    // --------- Test ray ----------
    // let test_ray = Ray {
    //     origin: Matrix4x1::new(0.0, 0.0, 0.75, 1.0),
    //     direction: Matrix4x1::new(0.0, 0.0, -1.0, 0.0),
    // };

    // --------- Testing render() ----------

    let image = render(&cam, &world);

    // --------- Saving render ----------
    _save_png("test_render.png", image);
}
