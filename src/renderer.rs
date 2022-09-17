use crate::ray::{Ray,reflect};
use crate::material::Material;
use crate::light::PointLight;
use crate::world::World;
use crate::intersections::Comps;


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
)

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