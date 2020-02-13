use std::fmt::{self, Formatter, Display};
use std::path::Path;
use std::fs::{File, create_dir_all};
use std::io::{Write, BufReader};
use std::io::prelude::*;

mod pixel;
use self::pixel::Pixel;

#[derive(Debug)]
enum PPMType {
    P2,
    P3,
    P6
}

pub struct Image {
    image_type: PPMType,
    height: usize,
    width: usize,
    nb_color: usize,
    content: Vec<Pixel>,
}

impl Image {
    pub fn save(&self, filename: &Path) -> std::io::Result<()> {
        create_dir_all(filename.parent().unwrap())?;
        let mut file = File::create(filename)?;
        let header = format!("{:?}\n# {:?}\n{} {}\n{}\n", self.image_type, filename.file_name().unwrap(), self.width, self.height, self.nb_color);
        file.write(header.as_bytes())?;

        // TODO - Vérifier si c'est assez rapide et si le format est ok
        for pixel in &self.content {
            file.write(&pixel.to_slice())?;
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

pub fn new_with_file(filename: &Path) -> Image {
    let f = File::open(filename);
    let mut buffer = BufReader::new(f.unwrap());

    let mut line = String::new();
    let _ = buffer.read_line(&mut line).unwrap();
    let new_image: Image;

    match &clean_line(line)[..] {
        "P3" => new_image = read_p3_file(&mut buffer),
        _ => panic!("Unhandled format !"),
    }
    // Check if file has good ppm format: "P3" then height width then max color value then data
    // and ignore comments

    let mut new_line = String::new();
    let _ = buffer.read_line(&mut new_line).unwrap();

    new_image
    //Image {content: Vec::new(), height: 0, width: 0}
}

fn clean_line(line: String) -> String {
    line.splitn(1, '#').collect::<Vec<&str>>()[0].trim().to_string()
}

fn read_p3_file(buffer: &mut BufReader<File>) -> Image {
    let mut height = 0;
    let mut width = 0;
    let mut nb_color = 0;
    let mut step = 1;

    // TODO - Add number of colors to handle case != 255
    // let nb_colors: usize;
    
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
                // TODO - Handle nb_color
                nb_color = cleaned.split_whitespace().collect::<Vec<&str>>()[0].parse().unwrap();
                step = 3;
            },
            _ => {}
        } 
    }

    Image {image_type: PPMType::P3, content: Vec::new(), height: height, width: width, nb_color: nb_color}
}
