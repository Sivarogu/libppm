extern crate ppm;
use std::path::Path;
use ppm::{new_with_file, grayscale};

fn main() {
    let path = Path::new("./images/input/P3/snail.ascii.ppm");
    let mut image = new_with_file(path);
    let gray_1 = grayscale(&mut image, 1);
    let gray_2 = grayscale(&mut image, 2);
    let gray_3 = grayscale(&mut image, 3);
    let output_path_1 = Path::new("./images/output/P3/grayscale_1.ascii.ppm");
    let output_path_2 = Path::new("./images/output/P3/grayscale_2.ascii.ppm");
    let output_path_3 = Path::new("./images/output/P3/grayscale_3.ascii.ppm");
    gray_1.save(output_path_1).unwrap();
    gray_2.save(output_path_2).unwrap();
    gray_3.save(output_path_3).unwrap();
}
