pub mod cli;
pub mod error;
pub mod ffmpeg;
pub mod format;
pub mod probe;

use std::path::Path;

use cli::{Cli, Command};
use error::AvalancheError;

use crate::format::Format;

pub fn run(cli: Cli) -> Result<(), AvalancheError> {
    match cli.command {
        Command::Convert { input, output } => {
            if !input.exists() {
                return Err(AvalancheError::FileNotFound { path: input });
            }

            let input_format = check_extension(&input)?;

            if output.exists() {
                return Err(AvalancheError::OutputAlreadyExists { path: output });
            }

            let output_format = check_extension(&output)?;

            if !input_format.kind().can_convert_to(&output_format.kind()) {
                return Err(AvalancheError::UnsupportedConversion {
                    from: input_format,
                    to: output_format,
                });
            }

            ffmpeg::check_installed()?;

            let args = ffmpeg::build_args(&input, &output);
            ffmpeg::run_ffmpeg(&args)?;
            println!(
                "converted: {} -> {} ({}, {})",
                input.display(),
                output.display(),
                output_format,
                output_format.kind()
            );
        }

        Command::Info { file } => {
            if !file.exists() {
                return Err(AvalancheError::FileNotFound { path: file });
            }

            let result = ffmpeg::run_ffprobe(&file)?;
            println!("File information: {result}");
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

    #[test]
    fn unsupported_extension_error() {
        let path = Path::new("Cargo.toml");
        assert_eq!(
            check_extension(path),
            Err(AvalancheError::UnsupportedFormat {
                ext: "toml".to_string()
            })
        )
    }
}
