pub mod cli;
pub mod error;

use cli::{Cli, Command};
use error::AvalancheError;

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
