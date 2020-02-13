extern crate ppm;
use std::path::Path;

fn main() {
    let path = Path::new("./images/input/P3/feep.ascii.ppm");
    let image = ppm::new_with_file(path);
    println!("{}", image);
    let output_path = Path::new("./images/output/P3/feep.ascii.ppm");
    image.save(output_path).unwrap();
}