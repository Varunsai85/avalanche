use crate::error::AvalancheError;

#[derive(Debug, PartialEq)]
pub enum Format {
    Mp4,
    Mov,
    Mkv,
    Mp3,
    Wav,
    Png,
    Jpg,
    Webp,
    Avif,
}

#[derive(Debug, PartialEq)]
pub enum MediaKind {
    Video,
    Audio,
    Image,
}

impl Format {
    pub fn from_extension(ext: &str) -> Result<Format, AvalancheError> {
        match ext.to_lowercase().as_str() {
            "mp4" => Ok(Format::Mp4),
            "mov" => Ok(Format::Mov),
            "mkv" => Ok(Format::Mkv),
            "mp3" => Ok(Format::Mp3),
            "wav" => Ok(Format::Wav),
            "png" => Ok(Format::Png),
            "jpg" | "jpeg" => Ok(Format::Jpg),
            "webp" => Ok(Format::Webp),
            "avif" => Ok(Format::Avif),
            _ => Err(AvalancheError::UnsupportedFormat {
                ext: ext.to_string(),
            }),
        }
    }

    pub fn kind(&self) -> MediaKind {
        match self {
            Format::Mov | Format::Mkv | Format::Mp4 => MediaKind::Video,
            Format::Mp3 | Format::Wav => MediaKind::Audio,
            Format::Png | Format::Jpg | Format::Avif | Format::Webp => MediaKind::Image,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_extension() {
        let cases = [
            ("mp4", Format::Mp4),
            ("mov", Format::Mov),
            ("mkv", Format::Mkv),
            ("mp3", Format::Mp3),
            ("wav", Format::Wav),
            ("png", Format::Png),
            ("jpeg", Format::Jpg),
            ("jpg", Format::Jpg),
            ("webp", Format::Webp),
            ("avif", Format::Avif),
        ];

        for (ext, expected) in cases {
            assert_eq!(
                expected,
                Format::from_extension(ext).unwrap(),
                "extension '{ext}'"
            );
        }
    }

    #[test]
    fn case_insensitive_extension() {
        let ext = "WAv";
        assert_eq!(Format::Wav, Format::from_extension(ext).unwrap());
    }

    #[test]
    fn invalid_extension() {
        let ext = "xyz";
        assert!(Format::from_extension(ext).is_err());
    }

    #[test]
    fn has_right_kind() {
        let cases = [
            (Format::Mov, MediaKind::Video),
            (Format::Mkv, MediaKind::Video),
            (Format::Mp4, MediaKind::Video),
            (Format::Mp3, MediaKind::Audio),
            (Format::Wav, MediaKind::Audio),
            (Format::Png, MediaKind::Image),
            (Format::Jpg, MediaKind::Image),
            (Format::Webp, MediaKind::Image),
            (Format::Avif, MediaKind::Image),
        ];

        for (format, expected) in cases {
            assert_eq!(expected, format.kind(), "format {format:?}");
        }
    }
}
