use std::{io::ErrorKind, process::Command};

use crate::error::AvalancheError;

pub fn check_installed() -> Result<(), AvalancheError> {
    let result = Command::new("ffmpeg").arg("-version").output();
    match result {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(AvalancheError::FfmpegNotFound),
            _ => Err(AvalancheError::FfmpegLaunchFailed {
                reason: e.to_string(),
            }),
        },
    }
}
