use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::error::Error;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

pub async fn pack_files(files: Vec<String>, cache_folder: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let output_file = File::create(output)?;
    let mut zip = ZipWriter::new(output_file);

    for file_path in files {
        let mut buffer = Vec::new();
        let mut file = File::open(&file_path)?;
        file.read_to_end(&mut buffer)?;

        let relative_path = Path::new(&file_path)
            .strip_prefix(cache_folder)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid path"))?;
        let relative_path = relative_path.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid path"))?;

        let options = FileOptions::<'_, ()>::default()
            .compression_method(CompressionMethod::Stored)
            .unix_permissions(0o755);
        zip.start_file(relative_path, options)?;
        zip.write_all(&buffer)?;
    }

    zip.finish()?;
    Ok(())
}