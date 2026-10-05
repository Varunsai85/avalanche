pub mod cli;
pub mod error;
pub mod format;

use cli::{Cli, Command};
use error::AvalancheError;

use crate::format::Format;

pub fn run(cli: &Cli) -> Result<(), AvalancheError> {
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

            let format = Format::from_extension(ext)?;
            println!(
                "convert {} -> {} ({:?}, {:?})",
                input.display(),
                output.display(),
                format,
                format.kind()
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
