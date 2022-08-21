mod constants;
mod tests;
use constants::{point, vector, Point, Vector, Canvas};
use nalgebra::{Matrix1x4};
use tests::{is_point,is_vector};


fn main() {
    println!("Hello, world!");

    let test_point = Matrix1x4::new(0.0, 0.0, 3.0,1.0);
    let test_vector_2 = Vector::new(1.0,1.0,1.0);

    let test_canvas = Canvas::new(10,20);

    println!("First row of pixels: {:?}",test_canvas.pixels[0])
}
 