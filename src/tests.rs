use nalgebra::{Matrix1x4,Matrix4x1, Matrix4};
use crate::Canvas;
use crate::Env;
use crate::Projectile;
use crate::constants::point;
use std::f64::consts::PI;

pub fn is_point(tuple: Matrix1x4<f32>) -> bool {
    match tuple[3] {
        1.0 => return true,
        _ => return false
    }
}

pub fn is_vector(tuple: Matrix1x4<f32>) -> bool {
    match tuple[3] {
        0.0 => return true,
        _ => return false
    }
}

pub fn clock(mut canvas: Canvas) -> Canvas{

    let divisions = 12;
    let tau = 2.0 * PI;
    let theta_increment = tau / divisions as f64;

    let radius: f32 = canvas.pixels.len() as f32 * 0.375;
    let mut point = Matrix4x1::new(radius,0.0,0.0,1.0);
    let mut temp_point = Matrix4x1::new(0.0,0.0,0.0,1.0);
    
    let translation_matrix = Matrix4::new(
        1.0, 0.0, 0.0, canvas.pixels.len() as f32 / 2.0,
        0.0, 1.0, 0.0, canvas.pixels.len() as f32 / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    let mut theta = theta_increment as f32;
    let mut x;
    let mut y;

    let mut rotation_matrix = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    for _ in 0..divisions{
        //creating new rotation matrix
        rotation_matrix[0] = theta.cos(); //M[0,0]
        rotation_matrix[1] = -theta.sin(); //M[0,1]
        rotation_matrix[4] = theta.sin(); //M[1,0]
        rotation_matrix[5] = theta.cos(); //M[1,1]

        point = rotation_matrix * point;
        temp_point = translation_matrix * point;
        x = temp_point[0].round();
        y = temp_point[1].round();

        println!("x: {}, y: {}, theta: {}",x,y,(theta * 100.0).round() / 100.0);

        canvas.write_pixel(x as usize, y as usize, [1.0,1.0,1.0]);

        // theta += theta_increment;
    };

    return canvas

}


pub fn launch(mut canvas: Canvas, env: Env, position: Matrix1x4<f32>, velocity: Matrix1x4<f32>) -> Canvas {

    // let mut point: &position;
    let mut projectile = Projectile {
        position: position,
        velocity: velocity
    };
    let max_ticks = 1000;
    let offset = canvas.pixels.len() as i32;
    let mut x;
    let mut y;
    for _ in 0..max_ticks {
        projectile = tick(&env, projectile);
        x = projectile.position[0].round() as i32;
        y = projectile.position[1].round() as i32;
        y = offset - y;
        if y < 500 && y >= 0 && x < 500 && x >= 0 {
            println!("{}, {}", x, y);
            canvas.write_pixel(x as usize, y as usize, [1.0,1.0,1.0]);
        }
    }

    return canvas

}


fn tick(env: &Env, mut proj: Projectile) -> Projectile {

    proj.position += proj.velocity;
    proj.velocity = proj.velocity + env.gravity;

    proj
}