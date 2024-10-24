use crate::{schema, extract};
use std::io::Write;
use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;
use log::{info, error, debug};
use tree_magic;
use bytes::Bytes;
use std::time::Duration;
use tokio::time::sleep;

pub async fn process_tts_save(tts_json_path: &str, cached_files: &HashSet<String>, cache_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = &format!("{}/Mods/Audio/", cache_path);
    let image_path = &format!("{}/Mods/Images/", cache_path);
    let pdf_path = &format!("{}/Mods/PDF/", cache_path);
    let model_path = &format!("{}/Mods/Models/", cache_path);
    let workshop_path = &format!("{}/Mods/Workshop/", cache_path);

    // Create folders if they don't exist
    fs::create_dir_all(&audio_path)?;
    fs::create_dir_all(&image_path)?;
    fs::create_dir_all(&pdf_path)?;
    fs::create_dir_all(&model_path)?;
    fs::create_dir_all(&workshop_path)?;

    let file = fs::File::open(&tts_json_path)?;
    let tts: schema::TTSSave = serde_json::from_reader(file)?;

    let urls = extract::get_tts_urls(tts).await?;
    let unique_urls: HashSet<_> = urls.into_iter().collect();

    for url in unique_urls {
        let filename = url.replace(|c: char| !c.is_ascii_alphanumeric(), "");
        if cached_files.contains(&filename) {
            debug!("Skipping already cached file {}", filename);
            continue;
        }

        let (bytes, _, extension) = match fetch_content(&url).await {
            Ok(result) => result,
            Err(e) => {
                error!("Failed to fetch content from {}: {}", url, e);
                continue;
            }
        };
        let mut file_path = "";
        let mut ext = extension.unwrap_or_else(|| {
            // Try to get the extension from the content
            let detected_ext = tree_magic::from_u8(&bytes).split('/').last().map(|ext| ext.to_lowercase()).unwrap_or_else(|| "".to_string());
            detected_ext
        });

        if ext.is_empty() {
            // Try to get the extension from the url
            ext = tree_magic::from_u8(&bytes).split('.').last().map(|ext| ext.to_lowercase()).unwrap_or_else(|| "".to_string());
        }

        let ext = ext.as_str();

        // Check for audio extensions, if so put in the Audio folder.
        if ["mp3", "wav", "ogg", "flac", "m4a", "aac", "wma", "aiff", "alac", "dsd", "pcm"].contains(&ext) {
            file_path = audio_path;
        }
        else if ["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "svg", "ico"].contains(&ext) {
            file_path = image_path;
        }
        else if ["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "rtf"].contains(&ext) {
            file_path = pdf_path;
        }
        else if ["obj"].contains(&ext) {
            file_path = model_path;
        }
        else if ["html"].contains(&ext) {
            // ignore html files
            continue;
        }

        if file_path.is_empty() {
            error!("Unsupported file extension for {}: {:?}", url, ext);
            continue;
        }
        
        let mut file_ext = ext.to_string();
        if !file_ext.is_empty() {
            file_ext = format!(".{}", ext);
        }

        let file_path = format!("{}{}{}", file_path, &filename, file_ext);

        // create folder
        match fs::create_dir_all(&file_path.rsplitn(2, '/').last().unwrap()) {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to create folder for {}: {}", file_path, e);
                continue;
            }
        }
        let mut file = match fs::File::create(&file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to create file {}: {}", file_path, e);
                continue;
            }
        };

        match file.write(&bytes) {
            Ok(_) => info!("Successfully saved content from '{}' to '{}'", url, file_path),
            Err(e) => error!("Failed to save content from '{}' to '{}': {}", url, file_path, e),
        }
    }

    // Check for a .png or .jpg file in the parent folder of tts_json_path and copy it to the Workshop folder
    let tts_path = PathBuf::from(tts_json_path);
    let parent_folder = match tts_path.parent() {
        Some(parent) => parent,
        None => {
            error!("Failed to get parent folder of {}", tts_json_path);
            return Ok(());
        }
    };

    let tts_filename = tts_path.file_name().unwrap().to_string_lossy().to_string();
    let tts_filestub = tts_filename.split('.').next().unwrap();
    
    let workshop_image = parent_folder.read_dir()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let extension = path.extension()?.to_string_lossy().to_string();
            if ["png", "jpg"].contains(&extension.as_str()) {
                let file_stem = path.file_stem()?.to_string_lossy().to_string();
                if file_stem == tts_filestub {
                    Some(path)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next();

    if let Some(workshop_image) = workshop_image {
        let workshop_image_filename = workshop_image.file_name().unwrap().to_string_lossy().to_string();
        let workshop_image_path = format!("{}/{}", workshop_path, workshop_image_filename);
        if let Err(err) = fs::copy(&workshop_image, &workshop_image_path) {
            return Err(format!("Failed to copy workshop image from {} to {}: {}", workshop_image.display(), workshop_image_path, err))?;
        }
        debug!("Copied workshop image from {} to {}", workshop_image.display(), workshop_image_path);
    }

    fs::copy(tts_json_path, format!("{}/{}", workshop_path, tts_filename))?;

    Ok(())
}

async fn fetch_content(url: &str) -> Result<(Bytes, Option<String>, Option<String>), Box<dyn std::error::Error>> {
    let max_retries = 3;
    let mut attempts = 0;

    loop {
        attempts += 1;
        match reqwest::get(url).await {
            Ok(resp) => {
                // get file name and extension using regex
                let content_disposition = match resp.headers().get(reqwest::header::CONTENT_DISPOSITION) {
                    Some(content_disposition) => content_disposition.to_str().ok(),
                    None => None,
                };

                let (filename, extension) = if let Some(content_disposition) = content_disposition {
                    let re = regex::Regex::new(r#"filename\*?=(?:UTF-8''|)([^;]+)"#)?;
                    if let Some(captures) = re.captures(content_disposition) {
                        if let Some(filename) = captures.get(1) {
                            let filename = percent_encoding::percent_decode_str(filename.as_str()).decode_utf8()?.to_string();
                            let filename = filename.trim_matches('"').to_string();
                            let extension = filename.split('.').last().map(|ext| ext.to_lowercase());
                            (Some(filename), extension)
                        } else {
                            (None, None)
                        }
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                };

                return Ok((resp.bytes().await?, filename, extension));
            }
            Err(e) => {
                if attempts >= max_retries {
                    return Err(Box::new(e));
                }
                debug!("Failed to fetch content from {}: {}. Retrying in 2 seconds...", url, e);
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
}
