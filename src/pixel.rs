use std::fmt::{self, Formatter, Display};
extern crate colored;
use colored::*;
///The pixel a component of the image
#[derive(Debug, Clone, Copy)]

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
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

    pub fn to_slice(&self) -> [String; 3] {
        [self.r.to_string(), self.g.to_string(), self.b.to_string()]
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
