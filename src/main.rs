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
use intersections::{
    intersect_sphere, intersect_cube, intersect_world, prepare_computations, Comps, Intersection, Intersections,
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
    renderer::{lighting, shade_hit, Camera, reflected_color},
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
        origin: Matrix4x1::new(2.0, 2.0, 0.0, 1.0),
        direction: Matrix4x1::new(-1.0, 0.0, 0.0, 0.0),
    };

    // --------- Cube intersection testing ----------
    
    let test_cube = Shape::default_cube();
    let t = intersect_cube(&test_ray, &test_cube);

    println!("intersection: {:?}", t);
    // --------- Cube normal testing ----------
    
    let test_pt = Matrix4x1::new(
        1.0,
        1.0,
        0.5,
        1.0
    );

    let n = cube_normal_at(&test_cube, test_pt);
    println!("normal: {}",n);


    // --------- Objects -----------
    // let mut sphere_1 = Shape::default_sphere();
    // sphere_1.material.color = color_from_rgb(43, 48, 58);
    // sphere_1.transform = translation(0.0, 0.25, 0.0);
    // sphere_1.material.reflective = 0.35;
    // world.objects.push(sphere_1);

    // let mut sphere_2 = Shape::default_sphere();
    // sphere_2.material.color = color_from_rgb(146, 220, 229);
    // sphere_2.material.reflective = 0.25;
    // sphere_2.transform = translation(2.25, 0.25, 0.0);
    // world.objects.push(sphere_2);

    // let mut sphere_3 = Shape::default_sphere();
    // sphere_3.material.color = color_from_rgb(214, 73, 51);
    // sphere_3.material.reflective = 0.25;
    // sphere_3.transform = translation(-2.25, 0.25, 0.0);
    // world.objects.push(sphere_3);

    let mut cube = Shape::default_cube();
    // cube.transform = translation(0.0, 0.0, -1.0);
    // cube.transform = scaling(0.5, 0.5, 0.5);
    cube.transform = rotation_y(PI / 3.0) * rotation_x(PI / 3.0) * translation(0.0, -1.0, 0.0);
    world.objects.push(cube);

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
