mod space;

use rand::distributions::{Distribution, Uniform};
use png::{BitDepth, ColorType, Encoder, Writer};
use std::fs::File;
use space::Space;

fn main() {
    let width = 1000;
    let height = 100;
    let density = 25;

    let mut space = Space::new(width, height);
    space.fill_randomly(density);
    let data = space.to_data();

    let fout = File::create("out.png").expect("Failed to create output file!");
    let mut pngcoder = Encoder::new(fout, width as u32, height as u32);
    pngcoder.set_color(ColorType::Grayscale);
    pngcoder.set_depth(BitDepth::Eight);

    let mut writer = pngcoder.write_header().expect("Failed to write PNG header!");
    writer.write_image_data(&data.as_slice()).expect("Failed to write PNG image data!");
}
