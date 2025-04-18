use base64::{Engine, engine::general_purpose};
use reqwest::Client;
use serde_json::json;
use web_sys::console::log_1;
use std::env;

pub async fn push_to_github(path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = option_env!("GITHUB_TOKEN").unwrap_or("unfound env".into());
    log_1(&format!("t {}" , token).into());
    let repo ="about-me";
    let branch ="main";
    let api_url = format!("https://api.github.com/repos/{}/contents/{}", repo, path);
    let encoded = general_purpose::STANDARD.encode(content);
    let client = Client::new();

    // 👇 ลองดึง SHA ก่อนว่ามีไฟล์นี้ไหม
    let sha = {
        let res = client
            .get(&api_url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "rust-uploader")
            .send()
            .await?;

        if res.status().is_success() {
            // ไฟล์มีอยู่แล้ว → extract sha
            let json: serde_json::Value = res.json().await?;
            Some(json["sha"].as_str().unwrap_or("").to_string())
        } else {
            None
        }
    };

    // 👇 เตรียม body JSON
    let mut body = json!({
        "message": format!("Upload or update file {}", path),
        "content": encoded,
        "branch": branch
    });

    // ถ้ามี sha → ใส่เพื่อทำ overwrite
    if let Some(sha_value) = sha {
        body["sha"] = json!(sha_value);
    }

    // 👇 PUT ไปยัง GitHub
    let res = client
        .put(&api_url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "rust-uploader")
        .json(&body)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        let text = res.text().await?;
        Err(format!("GitHub error: {}", text).into())
    }
}
