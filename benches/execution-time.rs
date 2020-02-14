#![feature(test)]
extern crate test;
extern crate ppm;

use test::Bencher;
use ppm::dummy;
use ppm::Pixel;


#[bench]
fn bench_dummy(b: &mut Bencher) {
    b.iter(|| dummy());
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

