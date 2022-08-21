use std::fs;
mod constants;
mod export;
mod tests;
use constants::{point, vector, Canvas, Point, Vector};
use nalgebra::Matrix1x4;
use tests::{is_point, is_vector};
extern crate image;

fn main() {
    println!("Hello, world!");

    let test_point = Matrix1x4::new(0.0, 0.0, 3.0, 1.0);
    let test_vector_2 = Vector::new(1.0, 1.0, 1.0);

    let mut test_canvas = Canvas::new(720, 720);
    for x in 0..240 {
        for y in 0..720 {
            test_canvas.write_pixel(x, y, [1.0, 0.0, 0.0])
        }
    }
    for x in 240..480 {
        for y in 0..720 {
            test_canvas.write_pixel(x, y, [0.0, 1.0, 0.0])
        }
    }
    for x in 480..720 {
        for y in 0..720 {
            test_canvas.write_pixel(x, y, [0.0, 0.0, 1.0])
        }
    }
    test_canvas.write_pixel(2, 8, [1.0, 1.0, 1.0]);
    test_canvas.write_pixel(2, 2, [0.5, 0.5, 0.5]);

    println!("Pixel: {:?}", &test_canvas.pixels[2][8]);

    export::save_png("test_png.png", test_canvas);
}
