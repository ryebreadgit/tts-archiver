use regex::Regex;
use std::error::Error;

async fn get_url_regex() -> Result<Regex, Box<dyn Error>> {
    Regex::new(r"https?://[a-zA-Z0-9.-]+(:[0-9]+)?(/[a-zA-Z0-9&%_.~+-]*)+(\?[a-zA-Z0-9&%_.~+=-]*)?(#[a-zA-Z0-9&%_.~+=-]*)?").map_err(|e| e.into())
}

pub async fn get_urls_from_str(s: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut urls: Vec<String> = Vec::new();
    let url_regex = get_url_regex().await?;

    let s = s.replace("http://cloud-3.steamusercontent.com/", "https://steamusercontent-a.akamaihd.net/"); // fix for updated url format, must be updated in json manually

    for cap in url_regex.captures_iter(&s) {
        if let Some(url) = cap.get(0) {
            let url_str = url.as_str().to_string();
            urls.push(url_str);
        }
    }

    Ok(urls)
}