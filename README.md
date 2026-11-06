# Avalanche

A fast media conversion CLI for macOS, written in Rust. Linux and Windows support are planned.

Avalanche gives you a simple, safe command line for everyday media jobs: converting videos, extracting audio, converting images and inspecting files. The encoding work is done by [FFmpeg](https://ffmpeg.org/). Avalanche handles the user-facing parts: argument parsing, picking the format from the file extension, checking that a conversion makes sense before it starts, and clear error messages with distinct exit codes.

```console
$ avalanche convert holiday.mov holiday.mp4
converted: holiday.mov -> holiday.mp4 (mp4, video)

$ avalanche convert holiday.mp4 soundtrack.mp3
converted: holiday.mp4 -> soundtrack.mp3 (mp3, audio)

$ avalanche convert song.mp3 song.mov
error: cannot convert mp3(audio) to mov(video)
```

> **Status:** early development (v0.1.0). `convert` works. `info` currently prints raw `ffprobe` output; formatted output is in progress.

---

## Contents

- [Features](#features)
- [Platform support](#platform-support)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
  - [convert](#convert)
  - [info](#info)
- [Supported formats](#supported-formats)
- [Conversion rules](#conversion-rules)
- [License](#license)

---

## Features

- **Convert between formats.** `avalanche convert in.mov out.mp4`
- **The output format comes from the file extension.** You don't need a `--to` flag.
- **Extract audio from video.** `avalanche convert video.mp4 song.mp3`
- **Convert images.** PNG, JPG, WebP and AVIF.
- **Validation before conversion.** Missing files, unknown extensions and impossible conversions (such as audio → video) are rejected before FFmpeg is started.
- **No overwriting.** If the output file already exists, Avalanche refuses to replace it.
- **Clear errors.** For example: `` ffmpeg not found - install it with `brew install ffmpeg` ``.
- **A distinct exit code for each kind of failure**, so scripts can tell failures apart.

---

## Platform support

| Platform | Status |
|---|---|
| macOS | ✅ Supported. This is the platform Avalanche is developed and tested on |
| Linux | 🔜 Planned next |
| Windows | 🔜 Planned after Linux |

Avalanche may already build and run on Linux and Windows, but it hasn't been tested there, and some messages (such as the FFmpeg install hint) are macOS-specific.

---

## Requirements

| Requirement | Why |
|---|---|
| macOS | See [Platform support](#platform-support) |
| [Rust](https://www.rust-lang.org/tools/install) 1.85 or newer | The crate uses the 2024 edition |
| [FFmpeg](https://ffmpeg.org/download.html) (`ffmpeg` and `ffprobe` on your `PATH`) | Does the actual conversion and probing |

Install FFmpeg with [Homebrew](https://brew.sh/):

```sh
brew install ffmpeg
```

Check that it's available:

```sh
ffmpeg -version
ffprobe -version
```

---

## Installation

Avalanche isn't published on crates.io yet. Build it from source:

```sh
git clone https://github.com/Varunsai85/avalanche.git
cd avalanche
cargo install --path .
```

This puts the `avalanche` binary in `~/.cargo/bin`. Make sure that directory is on your `PATH`.

To run it without installing:

```sh
cargo run -- convert input.mov output.mp4
```

---

## Usage

```text
Usage: avalanche <COMMAND>

Commands:
  convert  Convert a file into another format
  info     Show information about a media file
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### convert

```text
Usage: avalanche convert <INPUT> <OUTPUT>

Arguments:
  <INPUT>   The file to convert
  <OUTPUT>  Where to write the result (format comes from its extension)
```

Before calling FFmpeg, Avalanche checks, in this order:

1. The input file exists.
2. The input has a supported extension.
3. The output file does **not** already exist.
4. The output has a supported extension.
5. The conversion between the two media kinds is allowed (see [Conversion rules](#conversion-rules)).
6. FFmpeg is installed.

Examples:

```sh
# Video → video
avalanche convert clip.mov clip.mp4
avalanche convert clip.mkv clip.webm

# Video → audio (extract the soundtrack)
avalanche convert concert.mp4 concert.mp3

# Audio → audio
avalanche convert voice.wav voice.m4a

# Image → image
avalanche convert photo.png photo.webp
avalanche convert photo.jpeg photo.avif
```

Extensions are case-insensitive, so `CLIP.MOV` works the same as `clip.mov`.

### info

```sh
avalanche info clip.mp4
```

Runs `ffprobe` on the file and shows its container, duration, bitrate and streams.

> **In progress:** for now `info` prints the raw `ffprobe` output. Readable output (for example `Duration: 0:00:02`, `Stream 0: video h264 640x360`) is the next milestone.

---

## Supported formats

| Kind | Extensions |
|---|---|
| Video | `mp4`, `mov`, `mkv`, `avi`, `webm`, `flv` |
| Audio | `mp3`, `wav`, `m4a` |
| Image | `png`, `jpg` / `jpeg`, `webp`, `avif` |

Any other extension fails with exit code `65`.

---

## Conversion rules

Each format belongs to a media kind. Only these conversions are allowed:

| From ↓ / To → | Video | Audio | Image |
|---|:-:|:-:|:-:|
| **Video** | ✅ | ✅ | ❌ |
| **Audio** | ❌ | ✅ | ❌ |
| **Image** | ❌ | ❌ | ✅ |

Video → image (thumbnails) and video → GIF are planned as separate commands.

---

## License

Avalanche is released under the [MIT License](LICENSE).

FFmpeg isn't bundled with Avalanche. It's a separate program with its own license (LGPL/GPL), installed and run by you.
