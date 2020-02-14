#![feature(test)]
extern crate test;
extern crate ppm;

use test::Bencher;
use ppm::dummy;
use ppm::Pixel;
use std::path::Path;


#[bench]
fn bench_dummy(b: &mut Bencher) {
    b.iter(|| dummy());
}

#[bench]
fn create_low_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/custom.ascii.ppm");
    b.iter(|| ppm::new_with_file(path));
}

#[bench]
fn create_medium_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/blackbuck.ascii.ppm");
    b.iter(|| ppm::new_with_file(path));
}

#[bench]
fn invert_small_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/custom.ascii.ppm");
    let mut image = ppm::new_with_file(path);
    b.iter(|| ppm::invert(&mut image));
}

#[bench]
fn invert_medium_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/blackbuck.ascii.ppm");
    let mut image = ppm::new_with_file(path);
    b.iter(|| ppm::invert(&mut image));
}

#[bench]
fn grayscale_one_small_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/custom.ascii.ppm");
    let mut image = ppm::new_with_file(path);
    b.iter(|| ppm::grayscale(&mut image, 1));
}

#[bench]
fn grayscale_one_medium_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/blackbuck.ascii.ppm");
    let mut image = ppm::new_with_file(path);
    b.iter(|| ppm::grayscale(&mut image, 1));
}

#[bench]
fn grayscale_second_small_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/custom.ascii.ppm");
    let mut image = ppm::new_with_file(path);
    b.iter(|| ppm::grayscale(&mut image, 2));
}

#[bench]
fn grayscale_second_medium_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/blackbuck.ascii.ppm");
    let mut image = ppm::new_with_file(path);
    b.iter(|| ppm::grayscale(&mut image, 2));
}

#[bench]
fn grayscale_three_small_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/custom.ascii.ppm");
    let mut image = ppm::new_with_file(path);
    b.iter(|| ppm::grayscale(&mut image, 3));
}

#[bench]
fn grayscale_three_medium_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/blackbuck.ascii.ppm");
    let mut image = ppm::new_with_file(path);
    b.iter(|| ppm::grayscale(&mut image, 3));
}

#[bench]
fn flip_small_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/custom.ascii.ppm");
    let mut image = ppm::new_with_file(path);
    b.iter(|| ppm::flip(&mut image));
}

#[bench]
fn flip_medium_image(b: &mut Bencher) {
    let path = Path::new("./images/input/P3/blackbuck.ascii.ppm");
    let mut image = ppm::new_with_file(path);
    b.iter(|| ppm::flip(&mut image));
}

#[bench]
fn bench_invert_method(b: &mut Bencher) {

    let mut pixel = Pixel {
        r: 201,
        g: 230,
        b: 210
    };

    b.iter(|| pixel.invert());
}

#[bench]
fn bench_grayscale_method_one(b: &mut Bencher) {

    let mut pixel = Pixel {
        r: 201,
        g: 230,
        b: 210
    };

    b.iter(|| pixel.grayscale(1));
}

#[bench]
fn bench_grayscale_method_two(b: &mut Bencher) {

    let mut pixel = Pixel {
        r: 201,
        g: 230,
        b: 210
    };

    b.iter(|| pixel.grayscale(2));
}

#[bench]
fn bench_grayscale_method_three(b: &mut Bencher) {

    let mut pixel = Pixel {
        r: 201,
        g: 230,
        b: 210
    };

    b.iter(|| pixel.grayscale(3));
}

#[bench]
fn bench_create_pixel(b: &mut Bencher) {
    b.iter(|| Pixel::new(201, 230, 210));
}

