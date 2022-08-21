use nalgebra::Matrix1x4;
use crate::Canvas;
use crate::Env;
use crate::Projectile;

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