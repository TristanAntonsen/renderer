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
    sphere_1.material.color = color_from_rgb(255, 150, 0);
    sphere_1.transform = scaling(1.5, 1.5, 1.5);
    sphere_1.material.pattern =
        Pattern::perlin(color_from_rgb(255, 255, 255));
    world.objects.push(sphere_1);

    let mut sphere2 = Shape::default_sphere();
    sphere2.material.color = color_from_rgb(255, 0, 255);
    sphere2.transform = translation(2.0, 1.0, 1.5) * scaling(0.5, 0.5, 0.5);
    world.objects.push(sphere2);

    let mut floor = Shape::plane();
    floor.material.color = color_from_rgb(100, 150, 100);
    floor.transform = translation(0.0, -2.0, -3.2);
    floor.material.pattern =
        Pattern::checker(color_from_rgb(100, 255, 100), color_from_rgb(50, 50, 50));
    world.objects.push(floor);

    let mut wall = Shape::plane();
    wall.material.color = color_from_rgb(100, 150, 100);
    wall.transform = translation(0.0, -1.75, -3.0) * rotation_y(PI / 8.0) * rotation_x(PI / 2.0);
    wall.material.pattern =
        Pattern::checker(color_from_rgb(100, 255, 100), color_from_rgb(50, 50, 50));
    world.objects.push(wall);

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
