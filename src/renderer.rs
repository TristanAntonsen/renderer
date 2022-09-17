use std::f32::consts::PI;

use crate::ray::{Ray,reflect};
use crate::material::Material;
use crate::light::PointLight;
use crate::world::World;
use crate::intersections::{Comps, intersect_world, prepare_computations};
use nalgebra::{Matrix4x1, Matrix4};

pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    pub field_of_view: f32,
    pub pixel_size: f32,
    pub half_width: f32,
    pub half_height: f32,
    pub transform: Matrix4<f32>

}

impl Camera {
    pub fn default() -> Self {
        let mut default_cam = Self {
            hsize: 160,
            vsize: 120,
            field_of_view: PI / 2.0,
            pixel_size: 0.0,
            half_width: 0.0,
            half_height: 0.0,
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            )
        };

        Self::pixel_size(&mut default_cam);

        default_cam

    }
    
    pub fn pixel_size(&mut self) {
        let half_view = (self.field_of_view / 2.0).tan();
        let aspect = (self.hsize / self.vsize) as f32;
        let (half_width, half_height);
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view / aspect;
            half_height = half_view
        };

        self.half_width = half_width;
        self.half_height = half_height;
        self.pixel_size = (half_width * 2.0) / self.hsize as f32;

    }
}

// pub fn ray_for_pixel(camera: Camera, px: u32, py: u32) -> Ray {
//     let pixel_size = camera.pixel_size();
//     // offset from edge of canvas to pixel *center*
//     let x_offset = (0.5 * px as f32) * pixel_size;
//     let y_offset = (0.5 * py as f32) * pixel_size;
//     // untransformed coordinates of the pixel in world space
//     // (camera looks toward -z, so +x is to the left)
//     let world_x = camera.half_width - x_offset;

// }

pub fn camera_ray(x: f32, y: f32, camera_origin: Matrix4x1<f32>, canvas_distance: f32, width: f32, height: f32) -> Ray {

    let canvas_point = Matrix4x1::new(
        x - width as f32 / 2.0,
        y - height as f32 / 2.0,
        canvas_distance,
        1.0
    );

    let ray_direction = (canvas_point - camera_origin).normalize();

    Ray {
        origin: camera_origin,
        direction: ray_direction
    }

}
pub fn color_at(world: &World, ray: &Ray) -> [f32; 3] {
    let world_ints = intersect_world(ray, &world);
    let color : [f32; 3];
    if let Some(h) = world_ints.hit() {
        // if there is a valid intersection, compute the color
        let comps = prepare_computations(&h, ray);
        color = shade_hit(world, &comps)
    } else {
        // if no valid intersection, return black
        color = [0.0,0.0,0.0];
    }
    return color
}

pub fn shade_hit(world: &World, comps: &Comps) -> [f32; 3] {
    lighting(
        &comps.object.material,
        &world.lights[0],
        comps.point,
        comps.eyev,
        comps.normalv
    )
}

pub fn lighting(material: &Material, light: &PointLight, point: Matrix4x1<f32>, eyev: Matrix4x1<f32>, normalv: Matrix4x1<f32>) -> [f32; 3] {
    
    //color to turn into final color
    let mut color = material.color;

    // combining surface color with light's intensity
    color.iter_mut().for_each(|c| *c *= light.intensity);

    // find direction from point to light source
    let lightv = (light.position - point).normalize();

    // combining color with ambient color
    let mut ambient = color.clone();
    ambient.iter_mut().for_each(|c| *c *= material.ambient);

    // cos of angle bettween lightv and normal (black if negative)
    let light_dot_normal = lightv.dot(&normalv);

    let mut diffuse = color.clone();    
    let mut specular = [0.0,0.0,0.0];
    let reflectv: Matrix4x1<f32>;
    let reflect_dot_eye: f32;
    let factor: f32;

    if light_dot_normal < 0.0 { // not visible
        // specular & diffuse are [0,0,0]
        return ambient
    } else {
        // diffuse contribution
        diffuse.iter_mut().for_each(|c| *c *= material.diffuse * light_dot_normal);

        reflectv = reflect(-lightv, normalv);

        // cos of angle between reflection v and eye v. Negative means reflecting away from eye
        reflect_dot_eye = reflectv.dot(&eyev);

        if reflect_dot_eye <= 0.0 {
            specular = [0.0, 0.0, 0.0];
        } else {
            // specular contribution
            factor = reflect_dot_eye.powf(material.shininess);
            // specular = light.intensity * material.specular * factor;
            specular.iter_mut().for_each(|c| *c = light.intensity * material.specular * factor);            

        };
        
        [
            ambient[0] + diffuse[0] + specular[0],
            ambient[1] + diffuse[1] + specular[1],
            ambient[2] + diffuse[2] + specular[2],
        ]
    }

    
}