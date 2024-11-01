# TTS Archiver

A comprehensive backup tool for Tabletop Simulator games that downloads and archives all game assets, including those embedded in Lua scripts.

## Features

- Deep asset extraction using regex pattern matching
- Finds and downloads assets from:
  - Game JSON files
  - Embedded Lua scripts
  - Workshop items
- Automatically updates old Steam CDN links to new working versions
- Organizes assets by type (Audio, Images, Models, etc.)
- Creates ZIP archives for easy sharing
- Supports batch processing multiple save files

## Installation

1. Download the latest release from the releases page
2. Extract the executable to your desired location
3. Run from command line/terminal

Or build from source, requires Rust 1.70.0 or later:

```sh
git clone https://github.com/yourusername/tts-archiver
cd tts-archiver
cargo build --release
```

## Usage

Basic usage:

```sh
tts-archiver "path/to/save.json"
```

With options:

```sh
tts-archiver --ignore-errors -o "output/path" "path/to/save.json"
```

### Options

```sh
FLAGS:
    -d, --dry-run          Only check files without downloading
    -i, --ignore-errors    Continue processing despite errors
    -p, --prefetch        Only download to cache, skip ZIP creation
        --pack            Create ZIP from cached files only
    -v, --verbose         Enable verbose logging
    -h, --help           Print help information
    -V, --version        Print version information

OPTIONS:
    -c, --cache <path>    Set custom cache directory 
    -o, --output <path>   Set custom output directory

ARGS:
    <TTS_SAVE_JSON>...    One or more save files to process
```

## How It Works

The archiver processes files in several steps:

1. **Save File Analysis**
     - Parses TTS save files and creates folders for different asset types:
       - Audio, Images, Models, PDF files
       - Workshop content and raw/processed assets

2. **Asset Discovery**
     - Finds URLs in the save file using regex matching
     - Removes duplicates and validates links
     - Downloads missing files with retry logic
     - Routes assets to appropriate folders based on type

3. **File Processing**
     - Identifies file types through:
       - HTTP headers
       - URL extensions
       - Content analysis
     - Handles special cases like:
       - Embedded Lua scripts
       - Steam Workshop content
       - Old CDN links

4. **Workshop Support**
     - Downloads workshop thumbnails
     - Updates outdated Steam CDN links
     - Tracks missing workshop content

5. **Reliability**
     - Retries failed downloads
     - Optional dry-run mode
     - Detailed error reporting

6. **Results**
     - Clean folder structure
     - Fixed CDN links in save file
     - Missing assets report
     - Ready for offline play

## Future Improvements

- [ ] Improved dry-run functionality
- [ ] Improved pack-only mode
- [ ] FTP support
- [ ] Auto-detection of URLs without protocol prefix
- [ ] Option to replace URLs with local file:// paths for offline play

## Building

Requires Rust 1.70.0 or later.

```sh
cargo build --release
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.