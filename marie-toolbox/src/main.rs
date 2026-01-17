#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::*;
use clap::Parser;
use log::{info};

mod executables;

use executables::{Cli, Context};

fn configure_logger(context: &Context) {
    let level_filter = match context.global_strings["loglevel"].as_str() {
        "LogLevel::info" => LevelFilter::Info,
        "LogLevel::debug" => LevelFilter::Debug,
        "LogLevel::warn" =>  LevelFilter::Warn,
        "LogLevel::verbose" => LevelFilter::Trace,
        _ => { unreachable!("Because it is provided on the base of LogLevel parsing");}
    };

    TermLogger::init(level_filter, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
}

fn main() {
    let args = Cli::parse();
    let context = Context::create_context(args);

    configure_logger(&context);

    info!("Starting MARIE-Tool");

    match context.command.as_str() {
        "objdump" => {
            executables::objdump::objdump_main(context);
        },
        "asm" => {
            executables::asm::asm_main(context);
        },
        "emulator" => {
            executables::emulator::emulator_main(context);
        },
        _ => {
            unreachable!("This should never happen as clap should prevent this showing help instead of panicking.");
        }
    }
}
