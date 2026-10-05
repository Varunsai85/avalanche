use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "avalanche", version, about = "Convert media files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Convert a file into another format
    Convert {
        /// The file to convert
        input: PathBuf,

        /// Where to write the result (format comes from its extension)
        output: PathBuf,
    },

    /// Show information about a media file
    Info {
        /// The file to inspect
        file: PathBuf,
    },
}
