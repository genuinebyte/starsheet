use rand::distributions::{Distribution, Uniform};
use png::{BitDepth, ColorType, Encoder, Writer};
use std::fs::File;

fn main() {
    let width = 1600;
    let height = 400;
    let starcount = 25;

    let mut data: Vec<u8> = vec![0; width * height];

    let mut rng = rand::thread_rng();
    let rand_x = Uniform::from(0..width);
    let rand_y = Uniform::from(0..height);

    for _ in 0..starcount {
        *data.get_mut(rand_y.sample(&mut rng) * width + rand_x.sample(&mut rng)).expect("How did we exceed the iamge bounds?") = 255;
    }

    let fout = File::create("out.png").expect("Failed to create output file!");
    let mut pngcoder = Encoder::new(fout, width as u32, height as u32);
    pngcoder.set_color(ColorType::Grayscale);
    pngcoder.set_depth(BitDepth::Eight);

    let mut writer = pngcoder.write_header().expect("Failed to write PNG header!");
    writer.write_image_data(&data.as_slice()).expect("Failed to write PNG image data!");
}
