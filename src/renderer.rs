use crate::ray::{Ray,reflect};
use crate::material::Material;
use crate::light::PointLight;


use nalgebra::{Matrix4x1};

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

pub fn lighting(material: &mut Material, light: PointLight, point: Matrix4x1<f32>, eyev: Matrix4x1<f32>, normalv: Matrix4x1<f32>) -> f32 {
    
    //color to turn into final color
    let mut color = material.color.clone();

    // combining surface color with light's intensity
    
    // color.iter_mut().for_each(|c| *c *= light.intensity);
    color *= light.intensity;
    // find direction from point to light source

    let lightv = (light.position - point).normalize();

    // combining color with ambient color

    let ambient = color * material.ambient;

    // cos of angle bettween lightv and normal (black if negative)
    let light_dot_normal = lightv.dot(&normalv);
    let diffuse: f32;
    let specular: f32;
    let reflectv: Matrix4x1<f32>;
    let reflect_dot_eye: f32;
    let factor: f32;

    if light_dot_normal < 0.0 { // not visible
        diffuse = 0.0;
        specular = 0.0;
        return ambient
    } else {
        // diffuse contribution
        diffuse = color * material.diffuse * light_dot_normal;

        reflectv = reflect(-lightv, normalv);

        // cos of angle between reflection v and eye v. Negative means reflecting away from eye
        reflect_dot_eye = reflectv.dot(&eyev);

        if reflect_dot_eye <= 0.0 {
            specular = 0.0;
            return ambient + diffuse
        } else {
            // specular contribution
            factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }

        ambient + diffuse + specular
    }

    
}