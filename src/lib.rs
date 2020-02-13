use std::path::Path;
use std::fs::File;
use std::io::{Write, BufReader};

mod pixel;
use self::pixel::Pixel;

pub struct Image {
    content: Vec<Pixel>,
    height: usize,
    width: usize
}

impl Image {
    fn save(&self, filename: &Path) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        let header = format!("P3 {} {} 255\n", self.width, self.height);
        file.write(header.as_bytes())?;

        // TODO - Vérifier si c'est assez rapide et si le format est ok
        for pixel in &self.content {
            file.write(&pixel.to_slice())?;
        }

        Ok(())
    }
}

fn new_with_file(filename: &Path) -> Image {
    let f = File::open(filename);
    let buffer = BufReader::new(f.unwrap());

    // Check if file has good ppm format: "P3" then height width then max color value then data
    // and ignore comments

    Image {content: Vec::new(), height: 0, width: 0}
}
