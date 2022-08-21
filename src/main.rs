mod constants;
mod common;
use constants::{point, vector};
use nalgebra::{Matrix1x4};
use common::{is_point,is_vector};


fn main() {
    println!("Hello, world!");

    let test_point = Matrix1x4::new(0.0, 0.0, 3.0,1.0);
    let test_vector = Matrix1x4::new(1.0,0.0,0.0,0.0);

    println!("(true){}",is_point(test_point));
    println!("(false){}",is_point(test_vector));
    println!("(false){}",is_vector(test_point));
    println!("(true){}",is_vector(test_vector));
}
 