mod error;

use std::{path::PathBuf, process};

use clap::{Parser, Subcommand};

use crate::error::AvalancheError;
fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(&cli) {
        eprintln!("error: {e}");
        process::exit(e.exit_code());
    }
}

fn run(cli: &Cli) -> Result<(), AvalancheError> {
    match &cli.command {
        Command::Convert { input, output } => {
            if !input.exists() {
                return Err(AvalancheError::FileNotFound {
                    path: input.clone(),
                });
            }

            let Some(ext) = output.extension().and_then(|e| e.to_str()) else {
                return Err(AvalancheError::NoExtension {
                    path: output.clone(),
                });
            };

            let ext = ext.to_lowercase();
            println!(
                "convert {} -> {} (format: {ext})",
                input.display(),
                output.display()
            );
        }

        Command::Info { file } => {
            if !file.exists() {
                return Err(AvalancheError::FileNotFound { path: file.clone() });
            }

            println!("File information: {}", file.display());
        }
    }
    Ok(())
}

#[derive(Parser)]
#[command(name = "avalanche", version, about = "Convert media files")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
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
