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
use geometry::{rotation_x, rotation_y, rotation_z, scaling, sphere_normal_at, translation, Shape};
use intersections::{
    intersect_sphere, intersect_world, prepare_computations, Comps, Intersection, Intersections,
};
use light::PointLight;
use material::{color_from_rgb, Material, Pattern};
use nalgebra::{Matrix4, Matrix4x1, Point};
use noise::Perlin;
use ray::{position, Ray};
use renderer::{color_at, ray_for_pixel, render};
use world::{view_transform, World};

use crate::{
    export::_save_png,
    renderer::{lighting, shade_hit, Camera},
};
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
    let mut sphere_1 = Shape::default_sphere();
    sphere_1.material.color = color_from_rgb(43, 48, 58);
    world.objects.push(sphere_1);

    let mut sphere_2 = Shape::default_sphere();
    sphere_2.material.color = color_from_rgb(146, 220, 229);
    sphere_2.transform = translation(2.25, 0.0, 0.0);
    world.objects.push(sphere_2);

    let mut sphere_3 = Shape::default_sphere();
    sphere_3.material.color = color_from_rgb(214, 73, 51);
    sphere_3.transform = translation(-2.25, 0.0, 0.0);
    world.objects.push(sphere_3);

    let mut floor = Shape::plane();
    floor.material.color = color_from_rgb(100, 150, 100);
    floor.transform = translation(0.0, -1.0, -3.2);
    floor.material.pattern =
        Pattern::checker(color_from_rgb(124, 124, 124), color_from_rgb(238, 229, 233));
    world.objects.push(floor);

    for object in world.objects.iter() {
        println!("id: {}", object.shape_id);
    }
    // --------- Camera ----------
    let mut cam = Camera::default(1080, 1080, PI / 6.0);
    cam.transform = translation(0.0, 0.0, -15.0);

    // --------- Testing ray_for_pixel ----------

    let cam_ray = ray_for_pixel(&cam, 5, 5);
    let test_color = color_at(&world, &cam_ray);
    println!("test color: {:?}", test_color);
    println!("ray origin: {:?}", cam_ray.origin);
    println!("ray direction: {:?}", cam_ray.direction);

    // --------- Test ray ----------
    let test_ray = Ray {
        origin: Matrix4x1::new(0.0, 0.0, 0.75, 1.0),
        direction: Matrix4x1::new(0.0, 0.0, -1.0, 0.0),
    };

    // --------- Test shape intersection ---------
    // if let Some(i) = intersect_plane(object, ray) {
    //     // if let Some(i) = intersect_sphere(ray, object) {
    //         intersections.collection.push(Intersection::new(i.0, &object));
    //         intersections.collection.push(Intersection::new(i.1, &object));
    //     }

    // --------- Testing render() ----------

    let image = render(&cam, &world);

    // --------- Saving render ----------
    _save_png("test_render.png", image);
}
