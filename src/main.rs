mod constants;
mod tests;
use constants::{point, vector};
use nalgebra::{Matrix1x4};
use tests::{is_point,is_vector};


fn main() {
    println!("Hello, world!");

    let test_point = Matrix1x4::new(0.0, 0.0, 3.0,1.0);
    let test_vector = Matrix1x4::new(1.0,0.0,0.0,0.0);
    let test_vector_2 = Matrix1x4::new(1.0,1.0,2.0,0.0);

    let result = test_vector + test_vector_2;

    println!("{}",result);
}
 