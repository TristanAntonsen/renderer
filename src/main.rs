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
    renderer::{lighting, shade_hit, Camera, reflected_color},
};
extern crate image;

fn main() {
    // -----------------------------------
    // --------- Preparing scene ---------
    // -----------------------------------

    // --------- World -----------
    let mut world = World::default();

    // --------- Light ----------
    let mut light = PointLight::new(1.0, Matrix4x1::new(10.0, 5.0, 5.0, 1.0));
    light.intensity = 1.0;
    world.lights.push(light);

    // --------- Test ray ----------
    let rt2_2 = (2.0 as f64).sqrt() / 2.0;
    let test_ray = Ray {
        origin: Matrix4x1::new(0.0, 0.0, -3.0, 1.0),
        direction: Matrix4x1::new(0.0,-rt2_2, rt2_2, 0.0),
    };

    let mut floor = Shape::plane();
    floor.material.color = color_from_rgb(255, 255, 255);
    floor.material.reflective = 0.5;
    floor.material.pattern = Pattern::checker([0.0,0.0,0.0], [1.0,1.0,1.0]);
    floor.transform = translation(0.0, -1.0, 0.0);
    
    let test_int = Intersection::new((2.0 as f64).sqrt(), &floor);

    let test_comps = prepare_computations(&test_int, &test_ray);
    let remaining = 0;
    let color = shade_hit(&world, &test_comps, &remaining);

    println!("color: {:?}",color);

    world.objects.push(floor);
    // --------- Material testing ----------

    let material = Material::default();

    // --------- Objects -----------
    // let mut sphere_1 = Shape::default_sphere();
    // sphere_1.material.color = color_from_rgb(43, 48, 58);
    // world.objects.push(sphere_1);

    // let mut sphere_2 = Shape::default_sphere();
    // sphere_2.material.color = color_from_rgb(146, 220, 229);
    // sphere_2.transform = translation(2.25, 0.0, 0.0);
    // world.objects.push(sphere_2);

    // let mut sphere_3 = Shape::default_sphere();
    // sphere_3.material.color = color_from_rgb(214, 73, 51);
    // sphere_3.transform = translation(-2.25, 0.0, 0.0);
    // world.objects.push(sphere_3);


    // --------- Camera ----------
    let mut cam = Camera::default(1080, 1080, PI / 6.0);
    cam.transform = translation(0.0, 0.0, -15.0);


    // --------- Testing render() ----------
    let bounces = 4;
    let image = render(&cam, &world, &bounces);

    // --------- Saving render ----------
    _save_png("test_render.png", image);
}
