use std::{io::ErrorKind, path::Path, process::Command};

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

pub fn build_args(input: &Path, output: &Path) -> Vec<String> {
    vec![
        String::from("-i"),
        input.display().to_string(),
        output.display().to_string(),
    ]
}

pub fn run_ffmpeg(args: &[String]) -> Result<(), AvalancheError> {
    let result = Command::new("ffmpeg").args(args).output();
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn builds_input_then_output() {
        let args = build_args(Path::new("in.mov"), Path::new("out.mkv"));
        assert_eq!(args, vec!["-i", "in.mov", "out.mkv"]);
    }
}
