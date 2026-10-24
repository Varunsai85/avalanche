pub mod cli;
pub mod error;
pub mod ffmpeg;
pub mod format;

use std::path::Path;

use cli::{Cli, Command};
use error::AvalancheError;

use crate::{ffmpeg::convert_media, format::Format};

pub fn run(cli: Cli) -> Result<(), AvalancheError> {
    match cli.command {
        Command::Convert { input, output } => {
            if !input.exists() {
                return Err(AvalancheError::FileNotFound { path: input });
            }
            check_extension(&input)?;
            if output.exists() {
                return Err(AvalancheError::OutputAlreadyExists { path: output });
            }

            let format = check_extension(&output)?;
            ffmpeg::check_installed()?;

            let args = vec![&input, &output];
            convert_media(args)?;

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
                return Err(AvalancheError::FileNotFound { path: file });
            }

            println!("File information: {}", file.display());
        }
    }
    Ok(())
}

fn check_extension(path: &Path) -> Result<Format, AvalancheError> {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        return Err(AvalancheError::NoExtension {
            path: path.to_path_buf(),
        });
    };
    Format::from_extension(ext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn check_formats() {
        let path = PathBuf::from("Cargo.toml");
        assert_eq!(
            check_extension(&path),
            Err(AvalancheError::UnsupportedFormat {
                ext: "toml".to_string()
            })
        )
    }
}
