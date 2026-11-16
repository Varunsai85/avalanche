use std::fmt;

#[derive(Debug, Default, PartialEq)]
pub struct StreamInfo {
    kind: String,
    codec: String,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Debug, Default, PartialEq)]
pub struct MediaInfo {
    container: Option<String>,
    duration: Option<f64>,
    bit_rate: Option<u64>,
    streams: Vec<StreamInfo>,
}

pub fn parse_media_info(output: &str) -> MediaInfo {
    let mut media_info = MediaInfo::default();
    for line in output.lines() {
        let Some((key, mut value)) = line.split_once('=') else {
            continue;
        };

        value = value.trim_matches('"');

        if let Some(field) = key.strip_prefix("format.") {
            match field {
                "format_name" => media_info.container = Some(value.to_string()),
                "duration" => media_info.duration = value.parse().ok(),
                "bit_rate" => media_info.bit_rate = value.parse().ok(),
                _ => {}
            }
        } else if let Some(rest) = key.strip_prefix("streams.stream.") {
            let Some((index, field)) = rest.split_once(".") else {
                continue;
            };

            let Ok(index) = index.parse::<usize>() else {
                continue;
            };

            if index == media_info.streams.len() {
                media_info.streams.push(StreamInfo::default());
            }

            let Some(stream) = media_info.streams.get_mut(index) else {
                continue;
            };

            match field {
                "codec_name" => stream.codec = value.to_string(),
                "codec_type" => stream.kind = value.to_string(),
                "height" => stream.height = value.parse().ok(),
                "width" => stream.width = value.parse().ok(),
                _ => {}
            }
        }
    }
    media_info
}

impl fmt::Display for MediaInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.container {
            Some(container) => write!(f, "container: {container}")?,
            None => write!(f, "container: N/A")?,
        }

        match self.duration {
            Some(duration) => {
                let total_seconds = duration as u32;
                let hrs = total_seconds / 3600;
                let mins = (total_seconds % 3600) / 60;
                let secs = total_seconds % 60;
                write!(f, "\nduration: {hrs:02}:{mins:02}:{secs:02}")?;
            }
            None => write!(f, "\nduration: N/A")?,
        }

        match self.bit_rate {
            Some(bit_rate) => write!(f, "\nbitrate: {} kb/s", bit_rate / 1000)?,
            None => write!(f, "\nbitrate: N/A")?,
        }

        write!(f, "\nstreams:")?;
        for stream in &self.streams {
            write!(f, "\n {stream}")?;
        }

        Ok(())
    }
}

impl fmt::Display for StreamInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:", self.kind)?;
        write!(f, "\n  codec: {}", self.codec)?;

        if let (Some(width), Some(height)) = (self.width, self.height) {
            write!(f, "\n  width: {width}")?;
            write!(f, "\n  height: {height}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MP4_OUTPUT: &str = r#"streams.stream.0.codec_name="h264"
streams.stream.0.codec_type="video"
streams.stream.0.width=640
streams.stream.0.height=360
streams.stream.1.codec_name="aac"
streams.stream.1.codec_type="audio"
format.format_name="mov,mp4,m4a,3gp,3g2,mj2"
format.duration="2.000000"
format.bit_rate="139644""#;

    const PNG_OUTPUT: &str = r#"streams.stream.0.codec_name="png"
streams.stream.0.codec_type="video"
streams.stream.0.width=640
streams.stream.0.height=360
format.format_name="png_pipe"
format.duration="N/A"
format.bit_rate="N/A""#;

    #[test]
    fn parsing_ffprobe_mp4_result() {
        let media_info = MediaInfo {
            container: Some(String::from("mov,mp4,m4a,3gp,3g2,mj2")),
            duration: Some(2.0),
            bit_rate: Some(139644),
            streams: vec![
                StreamInfo {
                    kind: String::from("video"),
                    codec: String::from("h264"),
                    width: Some(640),
                    height: Some(360),
                },
                StreamInfo {
                    kind: String::from("audio"),
                    codec: String::from("aac"),
                    width: None,
                    height: None,
                },
            ],
        };

        assert_eq!(media_info, parse_media_info(MP4_OUTPUT));
    }

    #[test]
    fn parsing_ffprobe_png_result() {
        let info = parse_media_info(PNG_OUTPUT);
        assert_eq!(None, info.duration);
        assert_eq!(Some(String::from("png_pipe")), info.container);
    }

    #[test]
    fn displaying_mp4_media_info() {
        let expected = "\
container: mov,mp4,m4a,3gp,3g2,mj2
duration: 00:00:02
bitrate: 139 kb/s
streams:
 video:
  codec: h264
  width: 640
  height: 360
 audio:
  codec: aac";
        assert_eq!(expected, parse_media_info(MP4_OUTPUT).to_string())
    }

    #[test]
    fn displaying_png_media_info() {
        let expected = "\
container: png_pipe
duration: N/A
bitrate: N/A
streams:
 video:
  codec: png
  width: 640
  height: 360";
        assert_eq!(expected, parse_media_info(PNG_OUTPUT).to_string())
    }
}
