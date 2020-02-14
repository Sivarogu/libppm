use std::fmt::{self, Formatter, Display};
extern crate colored;
use colored::*;
///The pixel a component of the image
#[derive(Debug, Clone, Copy)]

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8
}
///The pixel is compose of 3 shades of color, red, blue and green (RGB)
impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            r: red,
            g: green,
            b: blue
        }
    }
///Fonction to return red color
    pub fn red(&self) -> u8 {
        self.r
    }
///Fonction to return green color
    pub fn green(&self) -> u8 {
        self.g
    }
///Fonction to return blue color
    pub fn blue(&self) -> u8 {
        self.b
    }
///Fonction to invert the color
    pub fn invert(&mut self) {
        self.r = !self.r;
        self.g = !self.g;
        self.b = !self.b;
    }
///Fonction to put the image in gray scale, it take an argument and use different methods depend of the argument
    pub fn grayscale(&mut self, methode: u8) {
        let gray;
        if methode == 1 {
            gray = (0.21 * self.r as f32 + 0.72 * self.g as f32 + 0.07 * self.b as f32) as u8;
        }
        else if methode == 2 {
            gray = (self.r as f32 / 3. + self.g as f32 / 3. + self.b as f32 / 3.) as u8;
        }
        else {
            let mut min = if self.r > self.g { self.g } else { self.r };
            let mut max = if self.r > self.g { self.r } else { self.g };

            if self.b > max {
                max = self.b
            } else if self.b < min {
                min = self.b
            }

            gray = ((max as f32 + min as f32) / 2.) as u8;
        }
        self.r = gray;
        self.g = gray;
        self.b = gray;
    }
}
///return True if a pixel is equal to another 
impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.red() && self.g == other.green() && self.b == other.blue()
    }
}
///return the pixel structur pixel in RGB and it's reference in the memory
impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{0}{1}{2} ({3}, {4}, {5}) 0x{6}{7}{8}",
            "R".red().bold(),
            "G".green().bold(),
            "B".blue().bold(),
            self.r, self.g, self.b,
            format!("{:01$x}", self.r, 2),
            format!("{:01$x}", self.g, 2),
            format!("{:01$x}", self.b, 2))
    }
}
