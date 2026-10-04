use std::{path::PathBuf, process};

use clap::{Parser, Subcommand};
fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(&cli) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(cli: &Cli) -> Result<(), String> {
    match &cli.command {
        Command::Convert { input, output } => {
            if !input.exists() {
                return Err(format!("input file '{}' not found", input.display()));
            }

            let Some(ext) = output.extension().and_then(|e| e.to_str()) else {
                return Err(format!(
                    "output file '{}' has no file extension",
                    output.display()
                ));
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
                return Err(format!("file '{}' not found", file.display()));
            }

            println!("The file information: {}", file.display());
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