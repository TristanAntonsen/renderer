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
use geometry::{rotation_x, rotation_y, rotation_z, scaling, cube_normal_at, translation, Shape};
use intersections::{intersect_sphere, intersect_cube, intersect_caps};
use light::PointLight;
use material::{color_from_rgb, Material, Pattern};
use nalgebra::Matrix4x1;
use ray::Ray;
use renderer::render;
use world::World;

use crate::{
    export::_save_png,
    renderer::Camera,
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

    // --------- Test ray ----------
    let test_ray = Ray {
        origin: Matrix4x1::new(0.0, 0.0, 0.0, 1.0),
        direction: Matrix4x1::new(0.0, 0.0, 1.0, 0.0),
    };

    // --------- Cap intersection testing ----------
    let mut test_cyl = Shape::default_cylinder();
    test_cyl.transform = rotation_x(PI / 2.5);
    if let Some(xs) = intersect_caps(test_ray, &test_cyl) {  
        println!("hey");
        println!("{:?}",xs.collection[0].t);
        println!("{:?}",xs.collection[1].t);
    };

    // --------- Objects -----------
    let mut sphere_1 = Shape::default_sphere();
    sphere_1.material.color = color_from_rgb(43, 48, 58);
    sphere_1.transform = translation(1.5, 0.25, 0.0);
    // sphere_1.material.reflective = 0.35;
    // world.objects.push(sphere_1);

    let mut cyl_1 = Shape::default_cylinder();
    cyl_1.transform = translation(0.0, -1.0, 0.0) * rotation_x(PI / 2.50) * rotation_z(PI / 6.0);
    world.objects.push(cyl_1);

    let mut cube_1 = Shape::default_cube();
    cube_1.material.color = color_from_rgb(214, 73, 51);
    // cube_1.material.reflective = 0.25;
    cube_1.transform = 
        translation(-1.5, 0.75, 0.0) *
        rotation_y(PI / 4.0) *
        rotation_x(PI / 4.0);
    // world.objects.push(cube_1);

    let mut floor = Shape::plane();
    floor.material.color = color_from_rgb(255, 255, 255);
    // floor.material.reflective = 0.5;
    floor.material.pattern = Pattern::checker([0.0,0.0,0.0], [1.0,1.0,1.0]);
    floor.transform = translation(0.0, -0.75, 0.0);
    // world.objects.push(floor);

    // --------- Camera ----------s
    let mut cam = Camera::default(1080, 1080, PI / 6.0);
    cam.transform = translation(0.0, 0.0, -15.0);


    // --------- Testing render() ----------
    let bounces = 4;
    let image = render(&cam, &world, &bounces);

    // --------- Saving render ----------
    _save_png("test_render.png", image);

}
