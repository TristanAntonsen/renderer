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
    let mut world = World::new();

    // --------- Light ----------
    let mut light = PointLight::new(1.0, Matrix4x1::new(10.0, 5.0, 5.0, 1.0));
    light.intensity = 1.0;
    world.lights.push(light);

    // --------- Test ray ----------
    let test_ray = Ray {
        origin: Matrix4x1::new(0.0, 0.0, -4.0, 1.0),
        direction: Matrix4x1::new(0.0, 0.0, 1.0, 0.0),
    };

    // --------- Material testing ----------


    // --------- Objects -----------
    // A
    let mut sphere_A = Shape::glass_sphere();
    sphere_A.transform = scaling(2.0, 2.0, 2.0);
    sphere_A.material.refractive_index = 1.5;
    // world.objects.push(sphere_A);
    println!("A transform: {}", sphere_A.transform);
    println!("A index: {}", sphere_A.material.refractive_index);
    
    // B
    let mut sphere_B = Shape::glass_sphere();
    sphere_B.transform = translation(0.0, 0.0, -0.25);
    sphere_B.material.refractive_index = 2.0;
    // world.objects.push(sphere_B);
    println!("B transform: {}", sphere_B.transform);
    println!("B index: {}", sphere_B.material.refractive_index);

    // C
    let mut sphere_C = Shape::glass_sphere();
    sphere_C.transform = translation(0.0, 0.0, 0.25);
    sphere_C.material.refractive_index = 2.5;
    // world.objects.push(sphere_C);
    println!("C transform: {}", sphere_C.transform);
    println!("C index: {}", sphere_C.material.refractive_index);


    let i0 = Intersection::new(2.0, &sphere_A);
    let i1 = Intersection::new(2.75, &sphere_B);
    let i2 = Intersection::new(3.25, &sphere_C);
    let i3 = Intersection::new(4.75, &sphere_B);
    let i4 = Intersection::new(5.25, &sphere_C);
    let i5 = Intersection::new(6.0, &sphere_A);
    let intersections = Intersections::collect(vec![i0, i1, i2, i3, i4, i5]);
    let mut comps;
    println!("index: | n1  | n2  |");
    for index in 0..6{
        comps = prepare_computations(&intersections.collection[index], &test_ray, &intersections.collection);
        // println!("{}:     | {:?} | {:?} | point z: {:?}",index, comps.n1, comps.n2, comps.point[2]);
        println!("");
    }
    

    // let mut floor = Shape::plane();
    // floor.material.color = color_from_rg¸b(255, 255, 255);
    // floor.material.reflective = 0.5;
    // floor.material.pattern = Pattern::checker([0.0,0.0,0.0], [1.0,1.0,1.0]);
    // floor.transform = translation(0.0, -1.0, 0.0);
    // world.objects.push(floor);


    // --------- Camera ----------
    let mut cam = Camera::default(1080, 1080, PI / 6.0);
    cam.transform = translation(0.0, 0.0, -15.0);


    // --------- Testing render() ----------
    let bounces = 4;
    // let image = render(&cam, &world, &bounces);

    // --------- Saving render ----------
    // _save_png("test_render.png", image);

}
