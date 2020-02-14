use std::fmt::{self, Formatter, Display};

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            r: red,
            g: green,
            b: blue
        }
    }

    pub fn red(&self) -> u8 {
        self.r
    }

    pub fn green(&self) -> u8 {
        self.g
    }

    pub fn blue(&self) -> u8 {
        self.b
    }

    pub fn invert(&mut self) {
        self.r = !self.r;
        self.g = !self.g;
        self.b = !self.b;
    }

    pub fn grayscale(&mut self, methode: u8) {

        if methode == 1 {

            let gray = (0.21 * self.r as f32 + 0.72 * self.g as f32 + 0.07 * self.b as f32) as u8;
            self.r = gray;
            self.g = gray;
            self.b = gray;

        }
        else if methode == 2 {

            let gray = (0.33 * self.r as f32 + 0.33 * self.g as f32 + 0.33 * self.b as f32) as u8;
            self.r = gray;
            self.g = gray;
            self.b = gray;

        }
        else {

            let items = [self.r, self.g, self.b];

            let (min, max) = items
                             .iter()
                             .fold((items[0], items[0]), |acc, &x| (acc.0.min(x), acc.1.max(x)));

            let gray = ((max as f32 + min as f32) * 0.50) as u8;
            self.r = gray;
            self.g = gray;
            self.b = gray;

        }
    }
    
    pub fn to_slice(&self) -> [u8; 3] {
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
