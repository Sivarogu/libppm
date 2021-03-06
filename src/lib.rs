use std::fmt::{self, Formatter, Display};
use std::path::Path;
use std::fs::{File, create_dir_all};
use std::io::{Write, BufReader};
use std::io::prelude::*;

mod pixel;
pub use self::pixel::Pixel;

#[derive(Debug, Copy, Clone)]
pub enum PPMType {
    // P1, // ascii Portable Bit Map
    // P2, // ascii Portable Gray Map
    P3, // ascii Portable Pixel Map
    // P4, // binary Portable Bit Map
    // P5, // binary Portable Gray Map
    // P6, // binary Portable Pixel Map
}

/// # Image is compose of [Pixel](struct.Pixel.html)
/// An Image can have different type of PPM, Ascii or binary. Here we focus on P3, which is an Ascii Portable Pixel Map. 
/// It is composed of a width, a height and the number of color in this image.
pub struct Image {
    image_type: PPMType,
    height: usize,
    width: usize,
    nb_color: usize,
    content: Vec<Pixel>,
}

impl Image {
    /// # Example:
    /// ```
    /// let output_path = Path::new("./path_to_image.ppm");
    /// image.save(output_path).unwrap();
    /// ```
    pub fn new(image_type: PPMType, height: usize, width: usize, nb_color: usize, content: Vec<Pixel>) -> Image {
        Image {
            image_type: image_type,
            height: height,
            width: width,
            nb_color: nb_color,
            content: content
        }
    }

    pub fn save(&self, filename: &Path) -> std::io::Result<()> {
        create_dir_all(filename.parent().unwrap())?;
        let mut file = File::create(filename)?;
        let header = format!("{:?}\n# {:?}\n{} {}\n{}\n",
            self.image_type,
            filename.file_name().unwrap(),
            self.width,
            self.height,
            self.nb_color
        );
        file.write(header.as_bytes())?;

        for pixel in &self.content {
            file.write(format!("{} {} {}\n",
                                  pixel.red(),
                                  pixel.green(),
                                  pixel.blue()
                              ).as_bytes())?;
        }

        Ok(())
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Type: {:?}, Width: {1}, Height: {2}, Nb of colors: {3}",
            self.image_type, self.width, self.height, self.nb_color)
    }
}

///Put an image in memory and stock it in an Image instance, this object will be change with other methods.
/// # Example:
/// ```
/// let path = Path::new("./path_to_image.ppm");
/// let mut image = new_with_file(path);
/// println!("{}", image);
/// ```
pub fn new_with_file(filename: &Path) -> Image {
    let f = File::open(filename);
    let mut buffer = BufReader::new(f.unwrap());

    let mut line = String::new();
    let _ = buffer.read_line(&mut line).unwrap();
    let mut new_image: Image;

    match &clean_line(line)[..] {
        "P3" => new_image = read_p3_file(&mut buffer),
        _ => panic!("This format is not handled yet..."),
    }

    loop {
        let mut new_line = String::new();
        let num_bytes = buffer.read_line(&mut new_line).unwrap();
        if num_bytes == 0 {
            break;
        } else {
            let cleaned = clean_line(new_line);
            let new_pixels = cleaned.split_whitespace()
                .filter(|val| val.len() != 0)
                .collect::<Vec<&str>>()
                .chunks(3)
                .map(|pix| {
                    let mut red: usize = pix[0].parse().unwrap();
                    let mut green: usize = pix[1].parse().unwrap();
                    let mut blue: usize = pix[2].parse().unwrap();
                    if new_image.nb_color > 255 {
                        let ratio = (new_image.nb_color + 1) / (255 + 1);
                        red /= ratio;
                        green /= ratio;
                        blue /= ratio;
                    } else if new_image.nb_color < 255 {
                        let ratio = (255 + 1) / (new_image.nb_color + 1);
                        red *= ratio;
                        green *= ratio;
                        blue *= ratio;
                    }

                    Pixel::new(
                        red as u8, green as u8, blue as u8
                    )
                })
                .collect::<Vec<Pixel>>();
            new_image.content.extend(new_pixels);
        }
    }

    new_image
}

///Take an Image in parameter. 
/// Fetch the Image and invert it color.
/// # Example:
/// ```
/// let mut image = new_with_file(path);
/// image = invert(&mut image);
/// ```
pub fn invert(image: &mut Image) -> Image {
    let mut inverted: Vec<Pixel> = Vec::new();
    for c in image.content.iter_mut() {
        c.invert();
        inverted.push(*c);
    }

    Image {
        image_type: image.image_type,
        content: inverted,
        height: image.height,
        width: image.width,
        nb_color: image.nb_color
    }
}

/// Take an Image in parameter.
/// Fetch the Image and put it in grayscale.
/// # Example:
/// ```
/// let mut image = new_with_file(path);
/// image = greyscale(&mut image);
/// ```
pub fn grayscale(image: &mut Image, method: u8) -> Image {
    let mut inverted: Vec<Pixel> = Vec::new();
    for c in image.content.iter_mut() {
        c.grayscale(method);
        inverted.push(*c);
    }

    Image {
        image_type: image.image_type,
        content: inverted,
        height: image.height,
        width: image.width,
        nb_color: image.nb_color
    }
}

///Take an Image in parameter. 
/// Fetch The Image and devide size and quality of the Image by 4.
/// # Example:
/// ```
/// let mut image = new_with_file(path);
/// image = downscale(&mut image);
/// ```
pub fn downscale(image: &mut Image) -> Image {
    let mut i = 0;
    let mut downscaled: Vec<Pixel> = image.content.clone();
    downscaled.retain(|_| ((i / image.width) % 2 == 1 && i % 2 == 0, i += 1).0);

    Image {
        image_type: image.image_type,
        content: downscaled,
        height: (image.height / 2) + image.height % 2,
        width: (image.width / 2) + image.width % 2,
        nb_color: image.nb_color
    }
}

///Take an Image in parameter.
/// Fetch the Image and put upside down.
/// # Example:
/// ```
/// let mut image = new_with_file(path);
/// image = flip(&mut image);
/// ```
pub fn flip(image: &mut Image) -> Image {
    let mut flipped: Vec<Pixel> = image.content.clone();
    let mut i = 0;
    while image.height / 2 > i
    {
        let (left, right) = flipped.split_at_mut((i + 1) * image.width);
        let temp = image.width * (image.height - ((i + 1) * 2));
        left[(i * image.width)..].swap_with_slice(&mut right[temp..(temp + image.width)]);
        i += 1;
    }

    Image {
        image_type: image.image_type,
        content: flipped,
        height: image.height,
        width: image.width,
        nb_color: image.nb_color
    }
}

fn clean_line(line: String) -> String {
    line.split('#').collect::<Vec<&str>>()[0].trim().to_string()
}

fn read_p3_file(buffer: &mut BufReader<File>) -> Image {
    let mut height = 0;
    let mut width = 0;
    let mut nb_color = 0;
    let mut step = 1;

    while step < 3 {
        let mut line = String::new();
        let _ = buffer.read_line(&mut line).unwrap();
        let cleaned = clean_line(line);
        if cleaned.len() == 0 {
            continue;
        } 
        match step {
            1 => {
                let dims = cleaned.split_whitespace().collect::<Vec<&str>>();
                width = dims[0].parse().unwrap();
                height = dims[1].parse().unwrap();
                step = 2;
            },
            2 => {
                nb_color = cleaned.split_whitespace()
                    .collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap();
                step = 3;
            },
            _ => {}
        } 
    }

    Image {
        image_type: PPMType::P3,
        content: Vec::new(),
        height: height,
        width: width,
        nb_color: nb_color
    }
}
