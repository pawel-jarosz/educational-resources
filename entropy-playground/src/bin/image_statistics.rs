
use image::ImageReader;
use clap::Parser;

use std::collections::HashSet;

use entropy_playground::FileStatistics;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_name="FILE")]
    file_name: String,
}

fn main() {
    let args = Args::parse();
    let mut statistics = FileStatistics::new();

    let img = ImageReader::open(args.file_name).expect("File cannot be open")
        .decode().expect("Image cannot be parsed");

    let bitmap = img.to_rgb8();
    let pixels = bitmap.pixels();
    let mut color_set: HashSet<String> = HashSet::new();

    for pixel in pixels {
        statistics.update(&pixel.0, 3);
        color_set.insert(format!("{}_{}_{}", pixel[0] as u8, pixel[1] as u8, pixel[2] as u8));
    }
    println!("Bitmap entropy:   {}", statistics.get_entropy());
    println!("Used colors: {}\n", color_set.len());
}
