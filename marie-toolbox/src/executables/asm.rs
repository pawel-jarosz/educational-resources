use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use marie_toolbox::asm::assembly_helpers::AssemblerParser;
use marie_toolbox::asm::object_content_factory::ObjectContentFactory;
use super::Context;


fn assembly(input: &File, output_filename: &String) {
    let buffer_reader = BufReader::new(input);
    let mut parser = AssemblerParser::new();

    info!("Start pre-parsing: prepare map of memory sections");

    let mut object_content_factory = ObjectContentFactory::new();

    for line in buffer_reader.lines() {
        let result = parser.handle_line(line.unwrap().as_str());
        if result != assembly_helpers::HandleResult::Empty {
            object_content_factory.push_instruction(result);
        }
        debug!("Parsed: {:?}", result)
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