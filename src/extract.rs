use regex::Regex;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

async fn get_url_regex() -> Result<Regex, Box<dyn Error>> {
    Regex::new(r"https?://[a-zA-Z0-9.-]+(:[0-9]+)?(/[a-zA-Z0-9&%_.~+-]*)+(\?[a-zA-Z0-9&%_.~+=-]*)?(#[a-zA-Z0-9&%_.~+=-]*)?").map_err(|e| e.into())
}

pub async fn get_urls_from_str(s: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut urls: Vec<String> = Vec::new();
    let url_regex = get_url_regex().await?;
    
    /* Find a better method for this
    let json_url_regex = get_json_url_regex().await?;

    // Fix URLs in JSON that don't start with http/s://
    let s = json_url_regex.replace_all(s, |caps: &regex::Captures| {
        let matched = caps.get(1).unwrap().as_str();
        if !matched.starts_with("http://") && !matched.starts_with("https://") {
            format!(r#""URL":"https://{}""#, matched)
        } else {
            format!(r#""URL":"{}""#, matched)
        }
    });
     */

    // Replace specific URL patterns
    let s = s.replace(
        "http://cloud-3.steamusercontent.com/",
        "https://steamusercontent-a.akamaihd.net/"
    );

    // Collect all URLs
    for cap in url_regex.captures_iter(&s) {
        if let Some(url) = cap.get(0) {
            let mut url_str = url.as_str().to_string();
            
            // Add https:// to URLs that don't have a protocol
            if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
                url_str = format!("https://{}", url_str);
            }
            
            urls.push(url_str);
        }
    }

    Ok(urls)
}

pub fn get_json_files_from_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
    fs::read_dir(dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension().unwrap_or_default() == "json" {
                Some(Ok(path))
            } else {
                None
            }
        })
        .collect()
}