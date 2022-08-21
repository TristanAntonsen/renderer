use std::fs;
use crate::Canvas;
extern crate image;
use image::{ImageBuffer, Rgb, RgbImage};

pub fn save_ppm(path: &str, canvas: Canvas) {
    let mut data: String = "".to_string();
    let width = canvas.pixels.len().to_string();
    let height = canvas.pixels[0].len().to_string();

    data.push_str("P3\n");
    data.push_str(&width);
    data.push_str(" ");
    data.push_str(&height);
    data.push_str("\n");

    let mut pixel_int;

    for row in canvas.pixels {
        for pixel in row {
            for val in pixel {
                pixel_int = (val * 255.0).round() as i32;
                data.push_str(&pixel_int.to_string());
                data.push_str(" ");

            }
        }
        data.push_str("\n");
    }
    
    fs::write(path, data).expect("Unable to write file");

}

pub fn save_png(path: &str, canvas: Canvas) {
    let width = canvas.pixels.len() as u32;
    let height = canvas.pixels[0].len() as u32;

    let mut img = RgbImage::new(width, height);
    let mut r;
    let mut g;
    let mut b;
    let mut color: [f32; 3];
    for x in 0..width {
        for y in 0..height {
            color = canvas.pixels[x as usize][y as usize];
            r = (color[0] * 255.0).round() as u8;
            g = (color[1] * 255.0).round() as u8;
            b = (color[2] * 255.0).round() as u8;
            
            img.put_pixel(x, y, Rgb([r,g,b]));            
        }
    }
    println!("{} exported.", path);
    
    img.save(path).expect("Could not save png");
}
