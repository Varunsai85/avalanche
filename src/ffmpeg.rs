use std::{
    io::{self, ErrorKind},
    path::Path,
    process::Command,
};

use crate::error::AvalancheError;

pub fn check_installed() -> Result<(), AvalancheError> {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map_err(launch_error)?;
    Ok(())
}

pub fn build_args(input: &Path, output: &Path) -> Vec<String> {
    vec![
        String::from("-i"),
        input.display().to_string(),
        output.display().to_string(),
    ]
}

pub fn run_ffmpeg(args: &[String]) -> Result<(), AvalancheError> {
    let result = Command::new("ffmpeg")
        .args(args)
        .output()
        .map_err(launch_error)?;
    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        let last_line = stderr.lines().last().unwrap_or("").to_string();
        return Err(AvalancheError::FfmpegFailed { stderr: last_line });
    }
    Ok(())
}

pub fn run_ffprobe(file: &Path) -> Result<String, AvalancheError> {
    let args = [
        "-v",
        "error",
        "-show_entries",
        "format=duration,bit_rate,format_name:stream=codec_type,codec_name,width,height",
        "-of",
        "flat",
    ];
    let result = Command::new("ffprobe")
        .args(args)
        .arg(file)
        .output()
        .map_err(launch_error)?;
    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        let last_line = stderr.lines().last().unwrap_or("").to_string();
        return Err(AvalancheError::FfprobeFailed { stderr: last_line });
    }
    let output = String::from_utf8_lossy(&result.stdout);
    Ok(output.to_string())
}

fn launch_error(e: io::Error) -> AvalancheError {
    match e.kind() {
        ErrorKind::NotFound => AvalancheError::FfmpegNotFound,
        _ => AvalancheError::FfmpegLaunchFailed {
            reason: e.to_string(),
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
