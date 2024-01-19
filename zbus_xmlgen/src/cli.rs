use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,

    /// Specify the destination for saving the output. If no argument is provided, the parsed
    /// interfaces will be stored in separate files. If a filename is provided, the output will
    /// be saved to that file. Use '-' to print the output to stdout.
    #[clap(short, long, allow_hyphen_values = true, global = true)]
    pub output: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub enum Command {
    /// Generate code for interfaces in the specified file.
    #[clap()]
    File { path: PathBuf },

    /// Generate code for interfaces from the specified system service.
    #[clap()]
    System {
        service: String,
        object_path: String,
    },

    /// Generate code for interfaces from the current users session.
    #[clap()]
    Session {
        service: String,
        object_path: String,
    },

    /// Generate code for interfaces from the specified address.
    #[clap()]
    Address {
        address: String,
        service: String,
        object_path: String,
    },
}
