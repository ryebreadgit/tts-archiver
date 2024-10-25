use crate::extract;
use std::io::Write;
use std::fs;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use log::{info, error, warn, debug};
use tree_magic;
use bytes::Bytes;
use std::time::Duration;
use tokio::time::sleep;
use regex::Regex;

pub async fn process_tts_save(tts_json_path: &str, cached_files: &HashMap<String, String>, cache_path: &str, ignore_error: bool, dry_run: bool) -> Result<(Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
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

    let mut successful_paths: Vec<String> = Vec::new();
    let mut failed_urls: Vec<String> = Vec::new();

    let shortlink_pattern = get_url_shortener_regex().await?;

    while let Some(url) = unique_urls.pop() {
        let filename = url.replace(|c: char| !c.is_ascii_alphanumeric(), "");
        if let Some(full_path) = cached_files.get(&filename) {
            debug!("Skipping already cached file {} at path: {:?}", filename, full_path);
            successful_paths.push(full_path.clone());
            continue;
        }

        if shortlink_pattern.is_match(&url) {
            warn!("Skipping shortlink: {}", url);
            continue;
        }

        let (bytes, _, extension) = match fetch_content(&url, dry_run).await {
            Ok(result) => result,
            Err(e) => {
                failed_urls.push(url.clone());
                error!("Failed to fetch content from {}: {}", url, e);
                if ignore_error || dry_run {
                    continue;
                } else {
                    return Err(e)?;
                }
            }
        };
        let mut file_path = "";
        let mut ext = extension.unwrap_or_else(|| {
            // Check if there is more than one dot in the URL
            if url.matches('.').count() > 1 {
                // Try to get the extension from the URL
                let detected_ext = url.split('.').last().map(|ext| ext.to_lowercase()).unwrap_or_else(|| "".to_string());
                if detected_ext.len() > 10 { // limit extension length to 10 characters to prevent issues with misconfigured URLs
                    return "".to_string();
                }
                detected_ext
            } else {
                "".to_string()
            }
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
                    failed_urls.push(url.clone());
                    error!("Failed to extract urls from lua file: {}", e);
                    continue;
                }
            };
            for lua_url in lua_urls {
                let lua_filename = lua_url.replace(|c: char| !c.is_ascii_alphanumeric(), "");
                if !unique_urls.contains(&lua_url) && !cached_files.contains_key(&lua_filename) {
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
            warn!("Unsupported file extension for {}: {:?}", url, ext);
            // failed_paths.push(url.clone()); // Don't add to failed paths as these are common files that are not supported (mostly web pages)
            continue;
        }
        
        let mut file_ext = ext.to_string();
        if !file_ext.is_empty() {
            file_ext = format!(".{}", ext);
        }

        let file_path = format!("{}{}{}", file_path, &filename, file_ext);

        if dry_run {
            info!("Dry run: Would save content from '{}' to '{}'", url, file_path);
            continue;
        }

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

        successful_paths.push(file_path);
    }

    // Check for a .png or .jpg file in the parent folder of tts_json_path and copy it to the Workshop folder
    let tts_path = PathBuf::from(tts_json_path);
    let parent_folder = match tts_path.parent() {
        Some(parent) => parent,
        None => {
            Err(format!("Failed to get parent folder of {}", tts_json_path))?
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
        let workshop_image_path = format!("{}{}", workshop_path, workshop_image_filename);

        if dry_run {
            info!("Dry run: Would copy workshop image from {} to {}", workshop_image.display(), workshop_image_path);
        } else {
            if let Err(err) = fs::copy(&workshop_image, &workshop_image_path) {
                return Err(format!("Failed to copy workshop image from {} to {}: {}", workshop_image.display(), workshop_image_path, err))?;
            }
            debug!("Copied workshop image from {} to {}", workshop_image.display(), workshop_image_path);
            successful_paths.push(workshop_image_path);
        }
    }

    // We do this here instead of a copy to replace the cloud-3.steamusercontent.com urls in the tts json file with the new steamusercontent-a.akamaihd.net urls as this will reflect the new urls in the cache
    let mut tts_json_str = fs::read_to_string(tts_json_path)?;
    tts_json_str = tts_json_str.replace("http://cloud-3.steamusercontent.com/", "https://steamusercontent-a.akamaihd.net/");
    let mut tts_json_file = fs::File::create(format!("{}{}", workshop_path, tts_filename))?;
    tts_json_file.write_all(tts_json_str.as_bytes())?;
    successful_paths.push(format!("{}{}", workshop_path, tts_filename));

    if !failed_urls.is_empty() {
        let missing_json_path = format!("{}{}_missing.txt", workshop_path, tts_filestub);
        if dry_run {
            info!("Dry run: Would write missing urls to {}", missing_json_path);
        } else {
            let mut missing_json_file = fs::File::create(&missing_json_path)?;
            let missing_json_str = serde_json::to_string_pretty(&failed_urls)?;
            missing_json_file.write_all(missing_json_str.as_bytes())?;
        }
        successful_paths.push(missing_json_path.clone());
        debug!("Wrote missing urls to {}", missing_json_path);
    }

    successful_paths.append(&mut unique_urls);

    Ok((successful_paths, failed_urls))
}

async fn fetch_content(url: &str, dry_run: bool) -> Result<(Bytes, Option<String>, Option<String>), Box<dyn std::error::Error>> {
    let max_retries = 3;
    let mut attempts = 0;

    let client = reqwest::Client::new();
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0";
    let parsed_url = reqwest::Url::parse(url)?;
    let host = parsed_url.host_str().ok_or("")?;

    loop {
        attempts += 1;
        let res = match dry_run {
            true => client.head(url).header(reqwest::header::USER_AGENT, user_agent).header(reqwest::header::HOST, host).send().await,
            false => client.get(url).header(reqwest::header::USER_AGENT, user_agent).header(reqwest::header::HOST, host).send().await
        };
        
        match res {
            Ok(resp) => {
                if resp.status() != 200 {
                    if attempts >= max_retries {
                        return Err(format!("Invalid response status {}", resp.status()).into());
                    }
                    debug!("Failed to fetch content from {}: {}. Retrying in 2 seconds...", url, resp.status());
                    sleep(Duration::from_secs(2)).await;
                    continue;
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

async fn get_url_shortener_regex() -> Result<Regex, Box<dyn std::error::Error>> {
    let shortlink_pattern = r"(?ix)
        https?://
        (?:
            (?:
                (?:
                    (?:bit\.ly|tinyurl\.com|t\.co|goo\.gl|tiny\.cc|is\.gd|cli\.gs|
                       qr\.ae|po\.st|bc\.vc|twitthis\.com|u\.to|j\.mp|buzurl\.com|
                       cutt\.ly|u\.bb|yourls\.org|x\.co|prettylinkpro\.com|scrnch\.me|
                       filoops\.info|vzturl\.com|qr\.net|1url\.com|tweez\.me|v\.gd|tr\.im|
                       link\.zip|ow\.ly|tiny\.pl|url\.ie|short\.ie|n9\.cl|db\.tt|hop\.click|
                       buff\.ly|rurl\.me|s2r\.co|snip\.ly|w\.tc|trib\.al|snurl\.com|
                       surl\.co\.uk|wp\.me|go\.ly|sl\.ly|rb\.gy|dai\.ly)
                    |
                    (?:[a-z0-9-]+\.)?(?:short|tiny|shrtn|shor|shrt|go|red|opn|reduceln|sh)\.
                    [a-z]{2,6}
                )
                /
                [a-zA-Z0-9_-]{4,25}
                (?:\?[a-zA-Z0-9_=&%-]*)?
            )
            |
            (?:
                discord\.gg/[a-zA-Z0-9_-]{5,10}
            )
        )
        ";

    Regex::new(&shortlink_pattern).map_err(|e| e.into())
}