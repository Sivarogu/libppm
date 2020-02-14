extern crate ppm;
use std::path::Path;
use ppm::{new_with_file, flip, invert, downscale};

fn main() {
    let path = Path::new("./images/input/P3/blackbuck.ascii.ppm");
    let mut image = new_with_file(path);
    image = flip(&mut image);
    image = invert(&mut image);
    image = downscale(&mut image);
    let output_path = Path::new("./images/output/P3/blackbuck.ascii.ppm");
    image.save(output_path).unwrap();
}
