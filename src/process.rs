use crate::extract;
use std::io::Write;
use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;
use log::{info, error, debug};
use tree_magic;
use bytes::Bytes;
use std::time::Duration;
use tokio::time::sleep;
use regex::Regex;

pub async fn process_tts_save(tts_json_path: &str, cached_files: &HashSet<String>, cache_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = &format!("{}/Mods/Audio/", cache_path);
    let image_path = &format!("{}/Mods/Images/", cache_path);
    let pdf_path = &format!("{}/Mods/PDF/", cache_path);
    let model_path = &format!("{}/Mods/Models/", cache_path);
    let workshop_path = &format!("{}/Mods/Workshop/", cache_path);
    let image_raw_path = &format!("{}/Mods/Images Raw/", cache_path);
    let model_raw_path = &format!("{}/Mods/Models Raw/", cache_path);
    let assetbundles_path = &format!("{}/Mods/Assetbundles/", cache_path);
    let text_path = &format!("{}/Mods/Text/", cache_path);

    // Create folders if they don't exist
    fs::create_dir_all(&audio_path)?;
    fs::create_dir_all(&image_path)?;
    fs::create_dir_all(&pdf_path)?;
    fs::create_dir_all(&model_path)?;
    fs::create_dir_all(&workshop_path)?;
    fs::create_dir_all(&image_raw_path)?;
    fs::create_dir_all(&model_raw_path)?;
    fs::create_dir_all(&assetbundles_path)?;
    fs::create_dir_all(&text_path)?;

    let file_str = fs::read_to_string(&tts_json_path)?;
    let urls = extract::get_urls_from_str(&file_str).await?;
    let unique_urls: HashSet<_> = urls.into_iter().collect();
    let mut unique_urls: Vec<String> = unique_urls.into_iter().collect();

    while let Some(url) = unique_urls.pop() {
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
            // Try to get the extension from the url
            let detected_ext = url.split('.').last().map(|ext| ext.to_lowercase()).unwrap_or_else(|| "".to_string());
            if detected_ext.len() > 10 { // limit extension length to 10 characters to prevent issues with misconfigured urls
                return "".to_string();
            }
            detected_ext
        });

        if ext.is_empty() {
            // Try to get the extension from the content
            ext = tree_magic::from_u8(&bytes).split('/').last().map(|ext| ext.to_lowercase()).unwrap_or_else(|| "".to_string());
        }

        let mut ext = ext.as_str();
        let mut bytes = bytes.to_vec();

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
        else if ["unity3d"].contains(&ext) {
            file_path = assetbundles_path;
        }
        else if ["rawm"].contains(&ext) {
            file_path = model_raw_path;
        }
        else if ["rawt"].contains(&ext) {
            file_path = image_raw_path;
        }
        else if ["ttslua"].contains(&ext) {
            file_path = text_path;
            // Run url extraction on the lua file and append the results to the urls list
            let mut lua_str = String::from_utf8_lossy(&bytes).to_string();
            lua_str = lua_str.replace("http://cloud-3.steamusercontent.com/", "https://steamusercontent-a.akamaihd.net/");
            bytes = lua_str.as_bytes().to_vec();
            let lua_urls = match extract::get_urls_from_str(&lua_str).await {
                Ok(urls) => urls,
                Err(e) => {
                    error!("Failed to extract urls from lua file: {}", e);
                    continue;
                }
            };
            for lua_url in lua_urls {
                let lua_filename = lua_url.replace(|c: char| !c.is_ascii_alphanumeric(), "");
                if !unique_urls.contains(&lua_url) && !cached_files.contains(&lua_filename) {
                    unique_urls.push(lua_url);
                }
            }
        }
        else if ["plain"].contains(&ext) {
            let obj_pattern = Regex::new(r"(?m)^(?:v\s+[-\d.]+\s+[-\d.]+\s+[-\d.]+|vt\s+[-\d.]+\s+[-\d.]+|vn\s+[-\d.]+\s+[-\d.]+\s+[-\d.]+|f\s+\d+(?:[/\\]\d*)*(?:\s+\d+(?:[/\\]\d*)*)+)").unwrap();
            let obj_str = String::from_utf8_lossy(&bytes);
            if obj_pattern.is_match(&obj_str) {
                file_path = model_path;
                ext = "obj";
            }
            else {
                file_path = text_path;
                ext = "txt";
            }
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

    // Replace the cloud-3.steamusercontent.com urls in the tts json file with the new steamusercontent-a.akamaihd.net urls as this will reflect the new urls in the cache
    let mut tts_json_str = fs::read_to_string(tts_json_path)?;
    tts_json_str = tts_json_str.replace("http://cloud-3.steamusercontent.com/", "https://steamusercontent-a.akamaihd.net/");
    let mut tts_json_file = fs::File::create(format!("{}/{}", workshop_path, tts_filename))?;
    tts_json_file.write_all(tts_json_str.as_bytes())?;

    Ok(())
}

async fn fetch_content(url: &str) -> Result<(Bytes, Option<String>, Option<String>), Box<dyn std::error::Error>> {
    let max_retries = 3;
    let mut attempts = 0;

    loop {
        attempts += 1;
        match reqwest::get(url).await {
            Ok(resp) => {
                if resp.status() != 200 {
                    return Err(format!("non-200 response from {}: {}", url, resp.status()).into());
                }
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
