// extern crate image;

// use image::GenericImageView;

// fn main() {
//     // Use the open function to load an image from a Path.
//     // `open` returns a `DynamicImage` on success.
//     let img = image::open("deer.png").unwrap();

//     let brighter = img.blur(10.0);

//     // The dimensions method returns the images width and height.
//     println!("dimensions {:?}", img.dimensions());

//     // The color method returns the image's `ColorType`.
//     println!("{:?}", img.color());

//     // Write the contents of this image to the Writer in PNG format.
//     brighter.save("test.png").unwrap();
// }

extern crate image;

use image::DynamicImage::ImageRgba8;
use image::{DynamicImage, ImageBuffer, Pixel, Rgba};

pub struct Picture {
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Picture {
    fn new() -> Self {
        let img = image::open("deer.png").unwrap().to_rgba8();
        Picture { buffer: img }
    }
    fn export(&self) {
        let usabl = self.buffer.clone();
        let dyne = ImageRgba8(usabl);
        dyne.save("test.png").unwrap()
    }
}

pub fn blur(picture: &mut Picture, radius: i32) {
    let usable = picture.buffer.clone();
    let dyne = ImageRgba8(usable);
    let blurry = dyne.blur(radius as f32);
    let img = blurry.to_rgba8();
    picture.buffer = img;
}

fn main() {
    let mut pic = Picture::new();
    blur(&mut pic, 2);
    pic.export()
}
