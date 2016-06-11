extern crate image;

use std::io::{self, Read};
use std::fs::File;
use std::path::Path;

use image::ImageBuffer;

fn main() {

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
    .expect("Could not read from stdin");

    let input = String::from(input.trim());
    let img_width = get_image_width(&input);
    let input = input.as_bytes();

    let img = ImageBuffer::from_fn(img_width, img_width, |x, y| {
        let mut index = x as usize + (y as usize * img_width as usize);
        index = index * 4;

        let empty_value = 0;

        let rchar;
        let gchar;
        let bchar;
        let achar;
        if (index + 0) < input.len() {
            rchar = input[index + 0];
        } else {
            rchar = empty_value;
        }

        if (index + 1) < input.len() {
            gchar = input[index + 1];
        } else {
            gchar =empty_value;
        }

        if (index + 2) < input.len() {
            bchar = input[index + 2];
        } else {
            bchar = empty_value;
        }

        if (index + 3) < input.len() {
            achar = input[index + 3];
        } else {
            achar = empty_value;
        }

        image::Rgba([rchar, gchar, bchar, achar])
    });

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    let _ = image::ImageRgba8(img).save(fout, image::PNG);
}

fn get_image_width(text: &String) -> u32 {
    let width = (text.len() / 4) as f64;
    let width: u32 = width.sqrt().ceil() as u32;

    return width;
}
