mod space;

use rand::distributions::{Distribution, Uniform};
use png::{BitDepth, ColorType, Encoder, Writer};
use std::fs::File;
use space::Space;
use getopts::Options;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = &args[0];
    let print_usage = |opts: Options| {
        let brief = format!("Usage: {} OUTPUT [options]", program);
        print!("{}", opts.usage(&brief));
    };

    let mut opts = Options::new();
    opts.reqopt("w", "width", "set the width of the output image", "WIDTH");
    opts.reqopt("h", "height", "set the height of the output image", "HEIGHT");
    opts.reqopt("s", "stars", "set the number of stars you want per 100 square pixels", "STARS");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_e) => {
            print_usage(opts);
            return;
        }
    };

    // TODO: Macro for this pattern.
    let width: u32 = match matches.opt_get("width") {
        Ok(arg) => arg.unwrap(),
        Err(_e) => {
            print_usage(opts);
            return;
        }
    };
    let height: u32 = match matches.opt_get("height") {
        Ok(arg) => arg.unwrap(),
        Err(_e) => {
            print_usage(opts);
            return;
        }
    };
    let density: u32 = match matches.opt_get("stars") {
        Ok(arg) => arg.unwrap(),
        Err(_e) => {
            print_usage(opts);
            return;
        }
    };
    let outname = if !matches.free.is_empty() {
        format!("{}.png", matches.free[0])
    } else {
        print_usage(opts);
        return;
    };

    let mut space = Space::new(width, height);
    space.fill_randomly(density);
    let data = space.to_data();

    let fout = File::create(outname).expect("Failed to create output file!");
    let mut pngcoder = Encoder::new(fout, width as u32, height as u32);
    pngcoder.set_color(ColorType::Grayscale);
    pngcoder.set_depth(BitDepth::Eight);

    let mut writer = pngcoder.write_header().expect("Failed to write PNG header!");
    writer.write_image_data(&data.as_slice()).expect("Failed to write PNG image data!");
}
