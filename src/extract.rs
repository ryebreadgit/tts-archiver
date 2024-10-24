use crate::schema;
use regex::Regex;
use std::error::Error;

pub async fn get_tts_urls(tts: schema::TTSSave) -> Result<Vec<String>, Box<dyn Error>> {
    let mut urls: Vec<String> = Vec::new();
    let url_regex = Regex::new(r"https?://[a-zA-Z0-9.-]+(:[0-9]+)?(/[a-zA-Z0-9&%_.~+-]*)+(\?[a-zA-Z0-9&%_.~+=-]*)?(#[a-zA-Z0-9&%_.~+=-]*)?")?;

    fn extract_urls(value: &serde_json::Value, regex: &Regex, urls: &mut Vec<String>) {
        match value {
            serde_json::Value::String(s) => {
                for cap in regex.captures_iter(s) {
                    if let Some(url) = cap.get(0) {
                        urls.push(url.as_str().to_string());
                    }
                }
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    extract_urls(item, regex, urls);
                }
            }
            serde_json::Value::Object(map) => {
                for (_, v) in map {
                    extract_urls(v, regex, urls);
                }
            }
            _ => {}
        }
    }

    let tts_json = serde_json::to_value(&tts)?;
    extract_urls(&tts_json, &url_regex, &mut urls);

    Ok(urls)
}