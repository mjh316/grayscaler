use std::f64;

use opencv::{
    self,
    core::Vec3b,
    prelude::{MatTrait, MatTraitConst},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut x = opencv::imgcodecs::imread(&args[1], 1).unwrap();
    for i in 0..x.rows() {
        for j in 0..x.cols() {
            let pixel = x.at_2d_mut::<Vec3b>(i, j).unwrap();
            let blue = pixel[0] as f64 / 255.;
            let green = pixel[1] as f64 / 255.;
            let red = pixel[2] as f64 / 255.;
            let c_linear = 0.2126 * red + 0.7152 * green + 0.0722 * blue;
            let c_srgb = (if c_linear <= 0.0031308 {
                12.92 * c_linear
            } else {
                1.055 * c_linear.powf(1. / 2.4) - 0.055
            } * 255.) as u8;
            // let avg = (((pixel[0] as f64) + (pixel[1] as f64) + (pixel[2] as f64)) / 3.) as u8;
            // println!("{:?}", &pixel);
            // Blue Green Red order!
            pixel[0] = c_srgb;
            pixel[1] = c_srgb;
            pixel[2] = c_srgb;
        }
    }

    let mut params = opencv::core::Vector::<i32>::default();
    params.push(opencv::imgcodecs::IMWRITE_PNG_COMPRESSION);
    params.push(9);
    let _output_write_result =
        opencv::imgcodecs::imwrite(&args[2], &x, &params).expect("Whoops, failed to write file.");
    println!("Write result: {}", _output_write_result);
    // println!("{:#?}", x);
}
