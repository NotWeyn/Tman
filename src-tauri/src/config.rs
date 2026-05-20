use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use keyring::Entry;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub server_enabled: bool,
    pub server_port: u16,
    pub server_local_only: bool,
    pub server_auto_start: bool,

    pub capture_mode: String,
    pub capture_interval_sec: u32,
    pub capture_change_threshold: u32,
    pub capture_last_region: String,

    pub pre_grayscale: bool,
    pub pre_binarize: bool,
    pub pre_contrast: String,
    pub pre_scale: f32,


    pub ocr_source_lang: String,
    pub ocr_auto_detect_lang: bool,
    pub ocr_merge_lines: bool,
    pub ocr_merge_paragraphs: bool,
    pub ocr_min_chars: u32,

    pub trans_provider: String,
    pub trans_target_lang: String,
    pub trans_cache_enabled: bool,
    pub trans_openai_endpoint: String,
    pub trans_openai_model: String,
    pub trans_libre_url: String,

    pub history_save: bool,
    pub history_max_records: u32,

    pub app_log_level: String,
    pub app_lang: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_enabled: true,
            server_port: 4000,
            server_local_only: false,
            server_auto_start: true,

            capture_mode: "manual".to_string(),
            capture_interval_sec: 5,
            capture_change_threshold: 5,
            capture_last_region: "".to_string(),

            pre_grayscale: false,
            pre_binarize: false,
            pre_contrast: "kapali".to_string(),
            pre_scale: 1.0,


            ocr_source_lang: "eng".to_string(),
            ocr_auto_detect_lang: true,
            ocr_merge_lines: true,
            ocr_merge_paragraphs: false,
            ocr_min_chars: 1,

            trans_provider: "google".to_string(),
            trans_target_lang: "tr".to_string(),
            trans_cache_enabled: true,
            trans_openai_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            trans_openai_model: "gpt-4o-mini".to_string(),
            trans_libre_url: "http://localhost:5000/translate".to_string(),

            history_save: true,
            history_max_records: 1000,

            app_log_level: "info".to_string(),
            app_lang: "en".to_string(),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("tman");
        
    if !data_dir.exists() {
        let _ = fs::create_dir_all(&data_dir);
    }
    
    let pointer_path = data_dir.join("config_pointer.txt");
    if pointer_path.exists() {
        if let Ok(content) = fs::read_to_string(&pointer_path) {
            let path = PathBuf::from(content.trim());
            if !content.trim().is_empty() {
                return path;
            }
        }
    }
    
    data_dir.join("config.json")
}

pub fn set_config_path_pointer(new_path: &str) {
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("tman");
        
    let pointer_path = data_dir.join("config_pointer.txt");
    
    if new_path.is_empty() || new_path == "config.json" {
        let _ = fs::remove_file(&pointer_path);
    } else {
        let _ = fs::write(&pointer_path, new_path);
    }
}

pub fn load_config() -> AppConfig {
    let path = get_config_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
    }
    let default_config = AppConfig::default();
    save_config(&default_config);
    default_config
}

pub fn save_config(config: &AppConfig) {
    let path = get_config_path();
    if let Ok(content) = serde_json::to_string_pretty(config) {
        let _ = fs::write(path, content);
    }
}

// Secret Management
pub fn get_secret(key: &str) -> String {
    if let Ok(entry) = Entry::new("tman", key) {
        match entry.get_password() {
            Ok(password) => password,
            Err(_) => "".to_string(),
        }
    } else {
        "".to_string()
    }
}

pub fn set_secret(key: &str, secret: &str) {
    if let Ok(entry) = Entry::new("tman", key) {
        if secret.is_empty() {
            let _ = entry.delete_credential();
        } else {
            let _ = entry.set_password(secret); // Actually set_password might still exist, let's keep it or change it?
            // keyring v3 still has set_password, but just to be safe... wait, the error only said `delete_password` was missing!
        }
    }
}
