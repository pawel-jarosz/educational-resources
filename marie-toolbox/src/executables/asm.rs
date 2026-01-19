use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

use super::Context;

fn assembly(input: &File, output_filename: &String) {
    let mut buffer_reader = BufReader::new(input);
    for line in buffer_reader.lines() {
        let unpacked = line.unwrap();
        println!("{}", unpacked); 
    }  
}

pub fn asm_main(context: Context) {
    // availability of this keys is checked already by clap CLI parser
    let input_filename = context.command_strings.get("input_file").unwrap();
    let output_filename = context.command_strings.get("output_file").unwrap();

    info!("Generate assembly code for {}", input_filename);
    info!("Result will be in the {} file", output_filename);

    let input_file = File::open(input_filename);
    if let Ok(input) = &input_file {
        assembly(input, output_filename);
    }
    else {
        error!("{}", input_file.err().unwrap());
    }
}