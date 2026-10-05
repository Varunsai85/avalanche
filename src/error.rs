use std::{error::Error, fmt, path::PathBuf};

#[derive(Debug, PartialEq)]
pub enum AvalancheError {
    /// A file the user gave doesn't exist
    FileNotFound { path: PathBuf },

    /// The output path has no extension, so no format can be chosen
    NoExtension { path: PathBuf },

    /// Unsupported output format
    UnsupportedFormat { ext: String },
}

impl fmt::Display for AvalancheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AvalancheError::FileNotFound { path } => {
                write!(f, "file '{}' not found", path.display())
            }
            AvalancheError::NoExtension { path } => {
                write!(f, "output file '{}' has no file extension", path.display())
            }
            AvalancheError::UnsupportedFormat { ext } => {
                write!(f, "unsupported file extension '{}'", ext)
            }
        }
    }
}

impl Error for AvalancheError {}

impl AvalancheError {
    pub fn exit_code(&self) -> i32 {
        match self {
            AvalancheError::FileNotFound { .. } => 66,
            AvalancheError::NoExtension { .. } => 64,
            AvalancheError::UnsupportedFormat { .. } => 65,
        }
    }
}
