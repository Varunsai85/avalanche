use std::{error::Error, fmt, path::PathBuf};

#[derive(Debug, PartialEq)]
pub enum AvalancheError {
    /// A file the user gave doesn't exist
    FileNotFound { path: PathBuf },

    /// The output already exists
    OutputAlreadyExists { path: PathBuf },

    /// The output path has no extension, so no format can be chosen
    NoExtension { path: PathBuf },

    /// Unsupported output format
    UnsupportedFormat { ext: String },

    /// ffmpeg is not found
    FfmpegNotFound,

    /// ffmpeg exists but couldn't be started (e.g. permissions)
    FfmpegLaunchFailed { reason: String },

    /// ffmpeg unable to convert the media
    FfmpegFailed { stderr: String },
}

impl fmt::Display for AvalancheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AvalancheError::FileNotFound { path } => {
                write!(f, "file '{}' not found", path.display())
            }
            AvalancheError::OutputAlreadyExists { path } => {
                write!(f, "output file '{}' already exists", path.display())
            }
            AvalancheError::NoExtension { path } => {
                write!(f, "file '{}' has no file extension", path.display())
            }
            AvalancheError::UnsupportedFormat { ext } => {
                write!(f, "unsupported file extension: '{}'", ext)
            }
            AvalancheError::FfmpegNotFound => {
                write!(
                    f,
                    "ffmpeg not found - install it with `brew install ffmpeg`"
                )
            }
            AvalancheError::FfmpegLaunchFailed { reason } => {
                write!(f, "ffmpeg failed to launch: '{}'", reason)
            }
            AvalancheError::FfmpegFailed { stderr } => {
                write!(f, "ffmpeg unable to convert the media: '{}'", stderr)
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
            AvalancheError::FfmpegNotFound => 69,
            AvalancheError::FfmpegLaunchFailed { .. } => 71,
            AvalancheError::FfmpegFailed { .. } => 70,
            AvalancheError::OutputAlreadyExists { .. } => 73,
        }
    }
}
