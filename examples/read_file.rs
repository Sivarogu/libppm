extern crate ppm;
use std::path::Path;
use ppm::{new_with_file, flip};

fn main() {
    let path = Path::new("./images/input/P3/snail.ascii.ppm");
    let mut image = new_with_file(path);
    image = flip(&mut image);
    println!("{}", image);
    let output_path = Path::new("./images/output/P3/snail.ascii.ppm");
    image.save(output_path).unwrap();
}
