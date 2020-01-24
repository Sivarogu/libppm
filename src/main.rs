use std::fmt::{self, Formatter, Display};

#[derive(Clone, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

pub struct Image {
    file: Vec<Pixel>,
    heigth: usize,
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

fn main() {

}