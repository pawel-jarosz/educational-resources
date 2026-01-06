
use std::fs::File;
use std::io::{Read, BufReader};

use clap::Parser;

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

    let file = File::open(args.file_name).expect("File cannot be opened");
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 64];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        statistics.update(&buffer, bytes_read);
    }

    println!("File size: {}", statistics.length);
    println!("Entropy:   {}", statistics.get_entropy());
    println!("Used bytes: {}\n", statistics.get_used_byte_value());
    println!("Statistics:");
    println!("===========");

    for byte in 0..256_usize {
        if statistics.stats[byte] == 0 {
            continue
        }
        println!("{} used {} times", byte, statistics.stats[byte]);
    }    
}
