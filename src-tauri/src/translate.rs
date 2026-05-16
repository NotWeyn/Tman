use crate::config::{AppConfig, get_secret};
use serde_json::json;

pub async fn translate_text(text: &str, source_lang: &str, cfg: &AppConfig) -> Result<String, String> {
    if text.trim().is_empty() {
        return Ok("".to_string());
    }

    match cfg.trans_provider.as_str() {
        "openai" => translate_openai(text, source_lang, cfg).await,
        "deepl" => translate_deepl(text, source_lang, cfg).await,
        "libre" => translate_libre(text, source_lang, cfg).await,
        _ => translate_google(text, source_lang, cfg).await,
    }
}

async fn translate_google(text: &str, _source_lang: &str, cfg: &AppConfig) -> Result<String, String> {
    // For now using free web api
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl={}&dt=t&q={}",
        cfg.trans_target_lang,
        urlencoding::encode(text)
    );

    let res = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !res.status().is_success() {
        return Err(format!("Google API error: {}", res.status()));
    }
    
    let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    
    let mut translated = String::new();
    if let Some(sentences) = body.get(0).and_then(|v| v.as_array()) {
        for sentence in sentences {
            if let Some(trans) = sentence.get(0).and_then(|v| v.as_str()) {
                translated.push_str(trans);
            }
        }
    }
    
    Ok(translated)
}

async fn translate_openai(text: &str, source_lang: &str, cfg: &AppConfig) -> Result<String, String> {
    let api_key = get_secret("openai_key");
    if api_key.is_empty() {
        return Err("OpenAI API key not found in keyring".to_string());
    }

    let client = reqwest::Client::new();
    
    let system_prompt = format!("You are a translator. Translate the text from {} to {}. Reply ONLY with the translated text without quotes.", source_lang, cfg.trans_target_lang);
    
    let body = json!({
        "model": cfg.trans_openai_model,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": text}
        ],
        "temperature": 0.3
    });

    let res = client.post(&cfg.trans_openai_endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("OpenAI API error: {}", res.status()));
    }

    let res_json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    
    if let Some(content) = res_json["choices"][0]["message"]["content"].as_str() {
        Ok(content.trim().to_string())
    } else {
        Err("Invalid OpenAI response".to_string())
    }
}

async fn translate_deepl(text: &str, _source_lang: &str, cfg: &AppConfig) -> Result<String, String> {
    let api_key = get_secret("deepl_key");
    if api_key.is_empty() {
        return Err("DeepL API key not found in keyring".to_string());
    }

    let client = reqwest::Client::new();
    let url = if api_key.ends_with(":fx") {
        "https://api-free.deepl.com/v2/translate"
    } else {
        "https://api.deepl.com/v2/translate"
    };

    let body = json!({
        "text": [text],
        "target_lang": cfg.trans_target_lang.to_uppercase()
    });

    let res = client.post(url)
        .header("Authorization", format!("DeepL-Auth-Key {}", api_key))
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("DeepL API error: {}", res.status()));
    }

    let res_json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    
    if let Some(translations) = res_json.get("translations").and_then(|t| t.as_array()) {
        if let Some(trans) = translations.get(0).and_then(|t| t.get("text")).and_then(|t| t.as_str()) {
            return Ok(trans.to_string());
        }
    }
    
    Err("Invalid DeepL response".to_string())
}

async fn translate_libre(text: &str, source_lang: &str, cfg: &AppConfig) -> Result<String, String> {
    let client = reqwest::Client::new();
    let sl = if source_lang == "auto" { "auto" } else { source_lang };
    
    let body = json!({
        "q": text,
        "source": sl,
        "target": cfg.trans_target_lang,
        "format": "text"
    });

    let res = client.post(&cfg.trans_libre_url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("LibreTranslate API error: {}", res.status()));
    }

    let res_json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    
    if let Some(translated) = res_json.get("translatedText").and_then(|t| t.as_str()) {
        Ok(translated.to_string())
    } else {
        Err("Invalid LibreTranslate response".to_string())
    }
}
