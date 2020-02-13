use std::fmt::{self, Formatter, Display};
use std::path::Path;
use std::fs::File;
use std::io::{Write, BufReader};

#[derive(Clone, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

pub struct Image {
    content: Vec<Pixel>,
    height: usize,
    width: usize
}

impl Pixel {
    fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            r: red,
            g: green,
            b: blue
        }
    }

    fn red(&self) -> u8 {
        self.r
    }

    fn green(&self) -> u8 {
        self.g
    }

    fn blue(&self) -> u8 {
        self.b
    }

    fn invert(&mut self) {
        self.r = !self.r;
        self.g = !self.g;
        self.b = !self.b;
    }

    fn grayscale(&mut self) {
        let gray = (0.21 * self.r as f32 + 0.72 * self.g as f32 + 0.07 * self.b as f32) as u8;
        self.r = gray;
        self.g = gray;
        self.b = gray;
    }
    
    fn to_slice(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.red() && self.g == other.green() && self.b == other.blue()
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{3}{4}{5}",
            self.r, self.g, self.b,
            format!("{:01$x}", self.r, 2),
            format!("{:01$x}", self.g, 2),
            format!("{:01$x}", self.b, 2))
    }
}

impl Image {
    fn save(&self, filename: &Path) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
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

    // Check if file has good ppm format: "P6" then height width then max color value then data
    // and ignore comments

    Image {content: Vec::new(), height: 0, width: 0}
}

fn main() {

}