extern crate ppm;
use std::path::Path;

fn main() {
    let path = Path::new("./images/input/P3/custom.ascii.ppm");
    let mut image = ppm::new_with_file(path);
    image = ppm::flip(&mut image);
    println!("{}", image);
    let output_path = Path::new("./images/output/P3/custom.ascii.ppm");
    image.save(output_path).unwrap();
}