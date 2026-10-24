use std::{io::ErrorKind, path::PathBuf, process::Command};

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

pub fn convert_media(args: Vec<&PathBuf>) -> Result<(), AvalancheError> {
    let result = Command::new("ffmpeg").arg("-i").args(args).output();
    match result {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let last_line = stderr.lines().last().unwrap_or("").to_string();
                return Err(AvalancheError::FfmpegFailed { stderr: last_line });
            }
            Ok(())
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(AvalancheError::FfmpegNotFound),
            _ => Err(AvalancheError::FfmpegLaunchFailed {
                reason: e.to_string(),
            }),
        },
    }
}
