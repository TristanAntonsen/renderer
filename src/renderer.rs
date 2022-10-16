use crate::ray::{Ray,reflect};
use crate::material::{Material, Pattern, stripe_at, pattern_at_shape};
use crate::light::PointLight;
use crate::world::World;
use crate::intersections::{Comps, intersect_world, prepare_computations};
use crate::geometry::{Shape, norm_3, cross_4};
use crate::constants::Canvas;
use image::imageops::colorops;
use nalgebra::{Matrix4x1, Matrix4};

pub fn render(camera: &Camera, world: &World, remaining: &u8) -> Canvas {
    let mut image = Canvas::new(camera.hsize as usize, camera.vsize as usize);
    let X_RES = camera.hsize;
    let Y_RES = camera.vsize;

    let (mut ray, mut color);
    for y in 0..Y_RES - 1 {
            for x in 0..X_RES - 1 {
            ray = ray_for_pixel(camera, x, y);
            color = color_at(world, &ray, &remaining);
            // println!("color: {:?}",&color);
            image.write_pixel(x as usize, y as usize, color);
        }
    }

    image
}


pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    pub field_of_view: f64,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub transform: Matrix4<f64>

}

impl Camera {
    pub fn default(hsize: u32, vsize: u32, fov: f64) -> Self {
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
        let aspect = (self.hsize as f64/ self.vsize as f64);
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
        self.pixel_size = (half_width * 2.0) / self.hsize as f64;

    }
}

pub fn ray_for_pixel(camera: &Camera, px: u32, py: u32) -> Ray {
    let pixel_size = camera.pixel_size;

    // offset from edge of canvas to pixel
    let x_offset = (0.5 + px as f64) * pixel_size;
    let y_offset = (0.5 + py as f64) * pixel_size;

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

pub fn is_shadowed(world: &World, point: Matrix4x1<f64>) -> bool {

    let v = world.lights[0].position - point;
    let distance = v.magnitude();
    let direction = v.normalize();

    let ray = Ray {
        origin: point,
        direction: direction
    };

    let intersections = intersect_world(&ray, world);

    if let Some(h) = intersections.hit() {
        if h.t < distance {
            return true
        }
    }

    false

}

pub fn color_at(world: &World, ray: &Ray, remaining: &u8) -> [f64; 3] {
    let world_ints = intersect_world(ray, &world);
    let world_ints_collection = world_ints.collection.clone(); // revisit this later -- probably don't need to copy
    let color : [f64; 3];
    if let Some(h) = world_ints.hit() {
        // if there is a valid intersection, compute the colOr
        let comps = prepare_computations(&h, ray, &world_ints_collection);
        color = shade_hit(world, &comps, remaining)
    } else {
        // if no valid interse  ction, return black
        color = [0.0,0.0,0.0];
    }
    return color
}

pub fn reflected_color(world: &World, comps: &Comps, remaining: &u8) -> [f64; 3] {
    if remaining <= &0 {
        return [0.0,0.0,0.0]
    }
    if comps.object.material.reflective == 0.0 {
        return [0.0,0.0,0.0]
    }
    let reflected_ray = Ray {
        origin: comps.over_point,
        direction: comps.reflectv
    };
    let mut color = color_at(world, &reflected_ray, &(remaining - 1));
    let refl = comps.object.material.reflective;
    color.iter_mut().for_each(|c| *c *= refl);

    color
}

pub fn shade_hit(world: &World, comps: &Comps, remaining: &u8) -> [f64; 3] {

    let shadowed = is_shadowed(world, comps.over_point);
    // let shadowed = false;

    let surface = lighting(
        &comps.object.material,
        comps.object,
        &world.lights[0],
        comps.over_point,
        comps.eyev,
        comps.normalv,
        shadowed
    );

    let reflected = reflected_color(world, comps, remaining);

    return [surface[0] + reflected[0],
            surface[1] + reflected[1],
            surface[2] + reflected[2]
    ]
}


pub fn lighting(material: &Material, object: &Shape, light: &PointLight, point: Matrix4x1<f64>, eyev: Matrix4x1<f64>, normalv: Matrix4x1<f64>, is_shadowed: bool) -> [f64; 3] {
    
    //color to turn into final color
    let mut color = material.color;

    if material.pattern.colors.len() != 0 {
        color = pattern_at_shape(&material.pattern, object, point);
    }

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
    let reflectv: Matrix4x1<f64>;
    let reflect_dot_eye: f64;
    let factor: f64;

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
        if is_shadowed {
            [
                ambient[0],
                ambient[1],
                ambient[2],
            ]
        } else {
            [
                ambient[0] + diffuse[0] + specular[0],
                ambient[1] + diffuse[1] + specular[1],
                ambient[2] + diffuse[2] + specular[2]
            ]   
        }
    }

    
}