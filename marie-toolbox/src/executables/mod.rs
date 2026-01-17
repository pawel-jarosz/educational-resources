use std::collections::HashMap;
use std::fmt::Display;

use clap::{Parser, Args, Subcommand, ValueEnum};

pub mod objdump;
pub mod asm;
pub mod emulator;


pub struct Context {
    pub command: String,
    pub global_strings: HashMap<String, String>,
    pub global_booleans: HashMap<String, bool>,
    pub command_strings: HashMap<String, String>,
    pub command_booleans: HashMap<String, bool>
}

fn pack_command(command: Commands, context: &mut Context) {
    match command {
        Commands::Emulator { executable_file } => {
            context.command = "emulator".to_string();
            context.command_strings.insert("executable_file".to_string(), executable_file);
        },
        Commands::Asm { input_file, output_file } => {
            context.command = "asm".to_string();
            context.command_strings.insert("input_file".to_string(), input_file);
            if let Some(filename) = output_file {
                context.command_strings.insert("output_file".to_string(), filename);
            }
        }
        Commands::Objdump { object_file } => {
            context.command = "objdump".to_string();
            context.command_strings.insert("object_file".to_string(), object_file);
        }
    }
}

fn pack_globals(global_opts: GlobalOpts, context: &mut Context) {
    context.global_strings.insert("loglevel".to_string(), global_opts.loglevel.to_string());
}

impl Context {
    pub fn new() -> Self {
        Context {
            command: "".to_string(),
            global_strings: HashMap::new(),
            global_booleans: HashMap::new(),
            command_strings: HashMap::new(),
            command_booleans: HashMap::new()
        }
    }

    pub fn create_context(cli: Cli) -> Self {
        let mut context = Context::new();
        pack_command(cli.command, &mut context);
        pack_globals(cli.global_opts, &mut context);
        context
    }
}

/// Toolbox for experiments with MARIE architecture.
/// It is simplistic architecture designed for educational reasons to teach
/// assembly language and basics of computer design.
#[derive(Parser)]
#[command(name = "marie-tool")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(flatten)]
    global_opts: GlobalOpts,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute binary file for MARIE architecture
    #[command(arg_required_else_help=true)]
    Emulator {
        /// file to be executed on emulated CPU
        executable_file: String
    },
    /// Assemble codes from human readeable format to the executable format
    #[command(arg_required_else_help=true)]
    Asm {
        /// file with assembly codes
        input_file: String,
        /// file ready to be executed by emulator
        #[arg(long, short='o', default_value="output.bin")]
        output_file: Option<String>
    },
    /// Presents executable file in human readable format
    #[command(arg_required_else_help=true)]
    Objdump {
        /// file to be disassembled
        object_file: String
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum LogLevel {
    Info,
    Debug,
    Warn,
    Verbose,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "LogLevel::info"),
            LogLevel::Debug => write!(f, "LogLevel::debug"),
            LogLevel::Warn => write!(f, "LogLevel::warn"),
            LogLevel::Verbose => write!(f, "LogLevel::verbose"),
        }
    }
}

#[derive(Debug, Args)]
struct GlobalOpts {
    #[arg(long, num_args = 0..=1,
        default_value_t=LogLevel::Debug,
        default_missing_value = "debug",
        value_enum)]
    loglevel: LogLevel
}