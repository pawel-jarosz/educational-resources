
use image::ImageReader;
use clap::Parser;

use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_name="INPUT FILE")]
    file_name: String,
    #[arg(value_name="OUTPUT FILE")]
    output: String,
    #[arg(short('r'), long("red"), num_args(1..6))]
    red: u8,
    #[arg(short('g'), long("green"), num_args(1..6))]
    green: u8,
    #[arg(short('b'), long("blue"), num_args(1..6))]
    blue: u8
}

fn main() {
    let args = Args::parse();
    let img = ImageReader::open(args.file_name).expect("File cannot be open")
        .decode().expect("Image cannot be parsed");
    let mut bitmap = img.to_rgb8();
    let pixels = bitmap.pixels_mut();

    let mut color_set: HashSet<String> = HashSet::new();
    for pixel in pixels {
        pixel[0] &= (0xFF >> args.red) << args.red;
        pixel[1] &= (0xFF >> args.green) << args.green;
        pixel[2] &= (0xFF >> args.blue) << args.blue;
        color_set.insert(format!("{}_{}_{}", pixel[0] as u8, pixel[1] as u8, pixel[2] as u8));
    }
    println!("Used colors: {}\n", color_set.len());
    bitmap.save(args.output).expect("Cannot save file");
}
