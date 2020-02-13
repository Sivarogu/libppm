extern crate ppm;
use ppm::Pixel;

#[test]
fn test_pixel_getters() {
    let red = 100;
    let blue = 50;
    let green = 150;
    let pix = Pixel::new(red, green, blue);
    assert_eq!(red, pix.red());
    assert_eq!(blue, pix.blue());
    assert_eq!(green, pix.green());
}