use crate::ray::{Ray,reflect};
use crate::material::Material;
use crate::light::PointLight;
use crate::world::World;
use crate::intersections::{Comps, intersect_world, prepare_computations};
use crate::geometry::{norm_3, cross_4};
use crate::constants::Canvas;
use image::imageops::colorops;
use nalgebra::{Matrix4x1, Matrix4};


pub fn render(camera: &Camera, world: &World) -> Canvas {
    let mut image = Canvas::new(camera.hsize as usize, camera.vsize as usize);
    let X_RES = camera.hsize;
    let Y_RES = camera.vsize;

    let (mut ray, mut color);

    for y in 0..Y_RES - 1 {
            for x in 0..X_RES - 1 {
            ray = ray_for_pixel(camera, x, y);
            color = color_at(world, &ray);
            // println!("color: {:?}",&color);
            image.write_pixel(x as usize, y as usize, color);
        }
    }

    image
}


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
    pub fn default(hsize: u32, vsize: u32, fov: f32) -> Self {
        let mut default_cam = Self {
            hsize,
            vsize,
            field_of_view: fov,
            pixel_size: 0.0,  //init
            half_width: 0.0,  //init
            half_height: 0.0, //init
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
        let aspect = (self.hsize as f32/ self.vsize as f32);
        let (half_width, half_height);
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view
        };

        self.half_width = half_width;
        self.half_height = half_height;
        self.pixel_size = (half_width * 2.0) / self.hsize as f32;

    }
}

pub fn ray_for_pixel(camera: &Camera, px: u32, py: u32) -> Ray {
    let pixel_size = camera.pixel_size;

    // offset from edge of canvas to pixel
    let x_offset = (0.5 + px as f32) * pixel_size;
    let y_offset = (0.5 + py as f32) * pixel_size;

    // untransformed coordinates of the pixel in world space
    // (camera looks toward -z, so +x is to the left)
    let world_x = camera.half_width - x_offset; 
    let world_y = camera.half_height - y_offset;

    // transforming the canvas point & origin using the camera matrix
    // & computing the ray's direction vector
    // canvas @ z = -1
    let cam_transf_inv = camera.transform.try_inverse().unwrap();
    let pixel = cam_transf_inv * Matrix4x1::new(world_x, world_y, -1.0, 1.0);
    
    let origin = cam_transf_inv * Matrix4x1::new(0.0,0.0,0.0,1.0);
    // let direction = norm_3(&(pixel - origin)); // need to normalize without the 4th element
    let direction = (pixel - origin).normalize(); // need to normalize without the 4th element
    Ray {
        origin,
        direction
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
    let lightv = norm_3(&(light.position - point));

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