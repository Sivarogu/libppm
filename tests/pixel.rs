extern crate ppm;
use ppm::Pixel;

#[test]
///test the fonction that return red, green and blue shape 
fn test_struct_pixel()
{
    let pixel = Pixel::new(201, 230, 210);

    assert_eq!(pixel.red(), 201);
    assert_eq!(pixel.green(), 230);
    assert_eq!(pixel.blue(), 210);

}

#[test]
///test the fonction that invert pixel color
fn test_invert_pixel()
{

    let mut pixel = Pixel::new(201, 230, 210);

    pixel.invert();

    assert_eq!(54, pixel.red());
    assert_eq!(45, pixel.blue());
    assert_eq!(25, pixel.green());

}

#[test]
///test functions that put pixel in gray scale
fn test_grayscale_one_pixel()
{

    let mut pixel = Pixel::new(201, 230, 210);

    pixel.grayscale(1);

    assert_eq!(222, pixel.red());
    assert_eq!(222, pixel.blue());
    assert_eq!(222, pixel.green());

}

#[test]
fn test_grayscale_two_pixel()
{

    let mut pixel = Pixel::new(201, 230, 210);

    pixel.grayscale(2);

    assert_eq!(213, pixel.red());
    assert_eq!(213, pixel.blue());
    assert_eq!(213, pixel.green());

}

#[test]
fn test_grayscale_three_pixel()
{

    let mut pixel = Pixel::new(201, 230, 210);

    pixel.grayscale(0);

    assert_eq!(215, pixel.red());
    assert_eq!(215, pixel.blue());
    assert_eq!(215, pixel.green());

}

#[test]
fn test_create_pixel() {
    let pixel = Pixel::new(201, 230, 210);
    let pixel2 = Pixel::new(201, 230, 210);

    assert_eq!(pixel2, pixel);
}
