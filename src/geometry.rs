use nalgebra::{Matrix4x1, Matrix4};
use crate::material::Material;

// struct for Sphere object used for calculating intersections
pub struct Sphere {
    pub origin: Matrix4x1<f32>,
    pub radius: f32,
    pub transform: Matrix4<f32>,
    pub material: Material
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32) -> Self {
        Self {
            origin: Matrix4x1::new(x,y,z,1.0),
            radius: radius,
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ),
            material: Material::default()
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4<f32>) {
        self.transform = transform
    }
}


//assumes form of [x,y,z,1] (w = 1 means Point)
pub struct _Point {
    pub pos: [f32; 4],
}

//assumes form of [i,j,k,1] (w = 0 means Vector)
pub struct _Vector {
    pub dir: [f32; 4],
}

impl _Point {
    pub fn _new(x: f32, y: f32, z: f32) -> Self {
        Self {
            pos: [x, y, z, 1.0],
        }
    }
}

impl _Vector {
    pub fn _new(x: f32, y: f32, z: f32) -> Self {
        Self {
            dir: [x, y, z, 0.0],
        }
    }
}

// // ---------- Normals ----------

pub fn normal_at(sphere: &Sphere, world_point: Matrix4x1<f32>) -> Matrix4x1<f32> {
    // inverse of sphere transformation * the point in world space
    let object_point = sphere.transform.try_inverse().unwrap() * world_point;

    // from sphere origin to point in object space
    let object_normal = object_point - Matrix4x1::new(0.0,0.0,0.0,1.0);

    // normal in world space
    let mut world_normal = sphere.transform
                            .try_inverse()
                            .unwrap()
                            .transpose() * object_normal;
    // setting world normal w = 0. Technically should use submatrix
    world_normal[3] = 0.0;

    return world_normal.normalize()
}



// ---------- Transformations ----------
// to do: shear and rotate

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4<f32> {

    Matrix4::new(
        x, 0.0, 0.0, 0.0,
        0.0, y, 0.0, 0.0,
        0.0, 0.0, z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn translation(x: f32, y: f32, z: f32) -> Matrix4<f32> {

    Matrix4::new(
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0,
    )
}