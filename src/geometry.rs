use crate::material::Material;
use nalgebra::{Matrix3x1, Matrix4, Matrix4x1};

// struct for Sphere object used for calculating intersections
pub struct Shape {
    pub origin: Matrix4x1<f64>,
    pub transform: Matrix4<f64>,
    pub material: Material,
    pub shape_id: u8,
}

impl Shape {
    pub fn test_shape() -> Self {
        Self {
            origin: Matrix4x1::new(0.0, 0.0, 0.0, 1.0),
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            material: Material::default(),
            shape_id: 0,
        }
    }

    pub fn default_sphere() -> Self {
        Self {
            origin: Matrix4x1::new(0.0, 0.0, 0.0, 1.0),
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            material: Material::default(),
            shape_id: 0,
        }
    }

    pub fn glass_sphere() -> Self {
        Self {
            origin: Matrix4x1::new(0.0, 0.0, 0.0, 1.0),
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            material: Material::glass(),
            shape_id: 0,
        }
    }

    pub fn plane() -> Self {
        Self {
            origin: Matrix4x1::new(0.0, 0.0, 0.0, 1.0),
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            material: Material::default(),
            shape_id: 1,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4<f64>) {
        self.transform = transform
    }
}

//assumes form of [x,y,z,1] (w = 1 means Point)
pub struct _Point {
    pub pos: [f64; 4],
}

//assumes form of [i,j,k,1] (w = 0 means Vector)
pub struct _Vector {
    pub dir: [f64; 4],
}

impl _Point {
    pub fn _new(x: f64, y: f64, z: f64) -> Self {
        Self {
            pos: [x, y, z, 1.0],
        }
    }
}

impl _Vector {
    pub fn _new(x: f64, y: f64, z: f64) -> Self {
        Self {
            dir: [x, y, z, 0.0],
        }
    }
}

// ---------- Abstract normal functions ----------

pub fn normal_at(shape: &Shape, world_point: Matrix4x1<f64>) -> Matrix4x1<f64> {
    match shape.shape_id {
        0 => sphere_normal_at(shape, world_point), //add other cases later
        1 => plane_normal_at(shape, world_point),  //add other cases later
        _ => plane_normal_at(shape, world_point),  //add other cases later
    }
}

// ---------- Local normal functions ----------

// consolodate & move these into abstract function later
pub fn sphere_normal_at(sphere: &Shape, world_point: Matrix4x1<f64>) -> Matrix4x1<f64> {
    // inverse of sphere transformation * the point in world space
    let object_point = sphere.transform.try_inverse().unwrap() * world_point;

    // from sphere origin to point in object space
    let object_normal = object_point - Matrix4x1::new(0.0, 0.0, 0.0, 1.0);

    // normal in world space
    let mut world_normal = sphere.transform.try_inverse().unwrap().transpose() * object_normal;
    // setting world normal w = 0. Technically should use submatrix
    world_normal[3] = 0.0;

    return world_normal.normalize();
}

pub fn plane_normal_at(plane: &Shape, world_point: Matrix4x1<f64>) -> Matrix4x1<f64> {
    // always +Y
    let object_normal = Matrix4x1::new(0.0, 1.0, 0.0, 1.0);

    // normal in world space
    let mut world_normal = plane.transform.try_inverse().unwrap().transpose() * object_normal;
    // setting world normal w = 0. Technically should use submatrix
    world_normal[3] = 0.0;

    return world_normal.normalize();
}

// ---------- Transformations ----------
// to do: shear and rotate

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4<f64> {
    Matrix4::new(
        x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4<f64> {
    Matrix4::new(
        1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
    )
}

pub fn rotation_x(theta: f64) -> Matrix4<f64> {
    Matrix4::new(
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        theta.cos(),
        -theta.sin(),
        0.0,
        0.0,
        theta.sin(),
        theta.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub fn rotation_y(theta: f64) -> Matrix4<f64> {
    Matrix4::new(
        theta.cos(),
        0.0,
        theta.sin(),
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        -theta.sin(),
        0.0,
        theta.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub fn rotation_z(theta: f64) -> Matrix4<f64> {
    Matrix4::new(
        theta.cos(),
        -theta.sin(),
        0.0,
        0.0,
        theta.sin(),
        theta.cos(),
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub fn cross_4(v1: &Matrix4x1<f64>, v2: &Matrix4x1<f64>) -> Matrix4x1<f64> {
    let tmp_a = Matrix3x1::new(v1[0], v1[1], v1[2]); // not a good way to do this, fix later
    let tmp_b = Matrix3x1::new(v2[0], v2[1], v2[2]); // not a good way to do this, fix later

    let cross = tmp_a.cross(&tmp_b);

    Matrix4x1::new(cross[0], cross[1], cross[2], 0.0)
}

// may slow things down, revisit later
pub fn norm_3(v: &Matrix4x1<f64>) -> Matrix4x1<f64> {
    let v_sub = Matrix3x1::new(v[0], v[1], v[2]).normalize();

    Matrix4x1::new(v_sub[0], v_sub[1], v_sub[2], v[3])
}
