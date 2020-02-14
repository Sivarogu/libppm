extern crate ppm;
use ppm::Pixel;

#[test]
fn test_struct_pixel()
{
    let pixel = Pixel {
        r: 201,
        g: 230,
        b: 210
    };

    assert_eq!(pixel.red(), 201);
    assert_eq!(pixel.green(), 230);
    assert_eq!(pixel.blue(), 210);

}

#[test]
fn test_invert_pixel()
{

    let mut pixel = Pixel {
        r: 201,
        g: 230,
        b: 210
    };

    pixel.invert();

    assert_eq!(54, pixel.red());
    assert_eq!(45, pixel.blue());
    assert_eq!(25, pixel.green());

}

#[test]
fn test_grayscale1_pixel()
{

    let mut pixel = Pixel {
        r: 201,
        g: 230,
        b: 210
    };

    pixel.grayscale(1);

    assert_eq!(222, pixel.red());
    assert_eq!(222, pixel.blue());
    assert_eq!(222, pixel.green());

}

#[test]
fn test_grayscale2_pixel()
{

    let mut pixel = Pixel {
        r: 201,
        g: 230,
        b: 210
    };

    pixel.grayscale(2);

    assert_eq!(211, pixel.red());
    assert_eq!(211, pixel.blue());
    assert_eq!(211, pixel.green());

}

#[test]
fn test_grayscale3_pixel()
{

    let mut pixel = Pixel {
        r: 201,
        g: 230,
        b: 210
    };

    pixel.grayscale(0);

    assert_eq!(215, pixel.red());
    assert_eq!(215, pixel.blue());
    assert_eq!(215, pixel.green());

}

#[test]
fn test_to_slice_pixel()
{

    let mut pixel = Pixel {
        r: 201,
        g: 230,
        b: 210
    };

    pixel.to_slice();

    assert_eq!(201, pixel.red());
    assert_eq!(210, pixel.blue());
    assert_eq!(230, pixel.green());

}

#[test]
fn test_create_pixel()
{

    let mut pixel = Pixel {
        r: 201,
        g: 230,
        b: 210
    };


    let pixel2 = Pixel::new(201, 230, 210);

    assert_eq!(pixel2, pixel);


}