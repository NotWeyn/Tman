pub mod broadcaster;
pub mod capture;
pub mod config;
pub mod db;
pub mod ocr;
pub mod server;
pub mod translate;
pub mod utils;

use base64::Engine;
use image::Luma;
use qrcode::QrCode;
use serde::Serialize;
use std::sync::Arc;
use tauri::Manager;
use tauri::State;
use tokio::sync::Mutex;

pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub broadcaster: broadcaster::Broadcaster,
    pub config: Mutex<config::AppConfig>,
    pub actual_port: Mutex<u16>,
    pub server_shutdown_tx: Mutex<Option<tokio::sync::oneshot::Sender<()>>>,
    pub last_text: Mutex<Option<String>>,
}

#[tauri::command]
async fn capture_and_translate(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    // 1. Capture Region
    let cfg = state.config.lock().await.clone();
    let (original_image, processed_image, region) = capture::capture_region(&cfg)?;

    if cfg.capture_last_region != region {
        let mut cfg_mut = state.config.lock().await;
        cfg_mut.capture_last_region = region.clone();
        crate::config::save_config(&cfg_mut);
    }

    // Encode images for Web UI
    let captured_b64 = crate::utils::dynamic_image_to_base64(&original_image).ok();
    let processed_b64 = crate::utils::dynamic_image_to_base64(&processed_image).ok();

    // 2. OCR (Using processed image for better accuracy)
    let (original_text, detected_lang) = ocr::extract_text(&processed_image, &cfg)?;

    if cfg.capture_mode == "degisim" {
        let mut last_text_lock = state.last_text.lock().await;
        if let Some(last_txt) = last_text_lock.as_ref() {
            if last_txt == &original_text {
                return Ok("No significant text change".to_string());
            }
        }
        *last_text_lock = Some(original_text.clone());
    }

    // 3. Translation
    let target_lang = cfg.trans_target_lang.clone();

    let mut translated_text = String::new();
    let mut from_cache = false;

    if cfg.trans_cache_enabled {
        if let Ok(Some(cached)) =
            db::get_cached_translation(&state.db, &original_text, &target_lang).await
        {
            translated_text = cached;
            from_cache = true;
        }
    }

    if !from_cache {
        translated_text = translate::translate_text(&original_text, &detected_lang, &cfg).await?;
        if cfg.history_save {
            let _ = db::insert_history(
                &state.db,
                &original_text,
                &translated_text,
                &target_lang,
                cfg.history_max_records,
            )
            .await;
        }
    }

    // 4. Broadcast to Web UI (Full event with images)
    let event = broadcaster::TranslationEvent {
        original_text: original_text.clone(),
        translated_text: translated_text.clone(),
        source_lang: detected_lang,
        target_lang: target_lang.clone(),
        region: region.clone(),
        captured_image: captured_b64,
        processed_image: processed_b64,
    };
    let _ = state.broadcaster.send(event.clone());

    // 5. Emit to Tauri frontend (Lightweight event WITHOUT images to avoid overloading)
    use tauri::Emitter;
    let mut tauri_event = event.clone();
    tauri_event.captured_image = None;
    tauri_event.processed_image = None;
    let _ = app.emit("translation-update", tauri_event);

    Ok(translated_text)
}

#[tauri::command]
async fn pick_region(state: State<'_, Arc<AppState>>) -> Result<String, String> {
    let region = capture::pick_region()?;
    let mut cfg = state.config.lock().await;
    cfg.capture_last_region = region.clone();
    crate::config::save_config(&cfg);
    Ok(region)
}

#[tauri::command]
async fn get_history(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<db::TranslationHistory>, String> {
    db::get_history(&state.db).await
}

#[tauri::command]
async fn clear_history(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    db::clear_history(&state.db).await
}

#[tauri::command]
async fn export_history_to_file(
    format: String,
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let history = db::get_history(&state.db).await?;
    let download_dir = dirs::download_dir().ok_or("Cannot find Downloads directory")?;

    let mut filename = "tman_history.txt";
    let mut data_str = String::new();

    if format == "JSON" {
        filename = "tman_history.json";
        data_str = serde_json::to_string_pretty(&history).map_err(|e| e.to_string())?;
    } else if format == "CSV" {
        filename = "tman_history.csv";
        data_str.push_str("id,original_text,translated_text,source_lang,target_lang,timestamp\n");
        for r in history {
            data_str.push_str(&format!(
                "{},\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                r.id,
                r.original_text.replace("\"", "\"\""),
                r.translated_text.replace("\"", "\"\""),
                r.source_lang,
                r.target_lang,
                r.timestamp
            ));
        }
    } else {
        for r in history {
            data_str.push_str(&format!(
                "[{}] {}->{}\nOriginal: {}\nTranslated: {}\n---\n",
                r.timestamp, r.source_lang, r.target_lang, r.original_text, r.translated_text
            ));
        }
    }

    let file_path = download_dir.join(filename);
    std::fs::write(&file_path, data_str).map_err(|e| format!("Dosya kaydedilemedi: {}", e))?;

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn save_settings(
    api_key: String,
    target_lang: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let mut cfg = state.config.lock().await;
    cfg.trans_target_lang = target_lang;
    config::save_config(&cfg);
    config::set_secret("openai_key", &api_key);
    Ok(())
}

#[tauri::command]
async fn get_config(state: State<'_, Arc<AppState>>) -> Result<config::AppConfig, String> {
    Ok(state.config.lock().await.clone())
}

#[tauri::command]
async fn save_config(
    new_config: config::AppConfig,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    config::save_config(&new_config);
    *state.config.lock().await = new_config;
    Ok(())
}

#[tauri::command]
async fn get_config_path() -> Result<String, String> {
    Ok(crate::config::get_config_path()
        .to_string_lossy()
        .to_string())
}

#[tauri::command]
async fn set_config_path(new_path: String, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    crate::config::set_config_path_pointer(&new_path);
    let cfg = state.config.lock().await.clone();
    crate::config::save_config(&cfg);
    Ok(())
}

#[tauri::command]
async fn get_secret(key: String) -> Result<String, String> {
    Ok(config::get_secret(&key))
}

#[tauri::command]
async fn set_secret(key: String, secret: String) -> Result<(), String> {
    config::set_secret(&key, &secret);
    Ok(())
}

#[derive(Serialize)]
struct ServerInfo {
    ip: String,
    port: u16,
    qr_code_base64: String,
}

#[tauri::command]
async fn get_server_info(state: State<'_, Arc<AppState>>) -> Result<ServerInfo, String> {
    let cfg = state.config.lock().await.clone();
    let port = *state.actual_port.lock().await;

    let ip = if cfg.server_local_only {
        "127.0.0.1".to_string()
    } else {
        local_ip_address::local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| "127.0.0.1".to_string())
    };

    let url = format!("http://{}:{}", ip, port);

    let code = QrCode::new(url.as_bytes()).map_err(|e| e.to_string())?;
    let image = code.render::<Luma<u8>>().build();
    let mut buf = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    use image::ImageEncoder;
    encoder
        .write_image(
            &image,
            image.width(),
            image.height(),
            image::ColorType::L8.into(),
        )
        .map_err(|e| e.to_string())?;

    let base64 = base64::engine::general_purpose::STANDARD.encode(&buf);

    Ok(ServerInfo {
        ip,
        port,
        qr_code_base64: format!("data:image/png;base64,{}", base64),
    })
}

#[tauri::command]
async fn toggle_server(active: bool, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let mut tx_guard = state.server_shutdown_tx.lock().await;

    if active {
        if tx_guard.is_some() {
            return Ok(()); // Already running
        }

        let cfg = state.config.lock().await.clone();
        let mut port = cfg.server_port;
        let bind_ip = if cfg.server_local_only {
            "127.0.0.1"
        } else {
            "0.0.0.0"
        };

        let listener = loop {
            match tokio::net::TcpListener::bind(format!("{}:{}", bind_ip, port)).await {
                Ok(l) => break l,
                Err(_) => port += 1,
            }
        };

        *state.actual_port.lock().await = port;
        let tx_clone = state.broadcaster.tx.clone();
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            server::start_server(listener, tx_clone, shutdown_rx).await;
        });

        *tx_guard = Some(shutdown_tx);
    } else {
        if let Some(tx) = tx_guard.take() {
            let _ = tx.send(());
        }
    }

    Ok(())
}

#[derive(Serialize)]
struct UpdateInfo {
    available: bool,
    version: String,
    body: String,
}

#[tauri::command]
async fn check_for_update(app: tauri::AppHandle) -> Result<UpdateInfo, String> {
    use tauri_plugin_updater::UpdaterExt;
    let updater = app.updater().map_err(|e| format!("Updater init failed: {}", e))?;
    match updater.check().await {
        Ok(Some(update)) => Ok(UpdateInfo {
            available: true,
            version: update.version.clone(),
            body: update.body.clone().unwrap_or_default(),
        }),
        Ok(None) => Ok(UpdateInfo {
            available: false,
            version: String::new(),
            body: String::new(),
        }),
        Err(e) => Err(format!("Update check failed: {}", e)),
    }
}

#[tauri::command]
async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_updater::UpdaterExt;
    let updater = app.updater().map_err(|e| format!("Updater init failed: {}", e))?;
    let update = updater.check().await
        .map_err(|e| format!("Update check failed: {}", e))?
        .ok_or("No update available".to_string())?;
    update.download_and_install(|_, _| {}, || {}).await
        .map_err(|e| format!("Update install failed: {}", e))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                // Load Config
                let app_config = config::load_config();

                // Set log level from config
                let log_level = match app_config.app_log_level.as_str() {
                    "hata" => "error",
                    "hata_ayiklama" => "debug",
                    _ => "info", // "bilgi" or any other
                };
                std::env::set_var("RUST_LOG", format!("tman={},oar_ocr={}", log_level, log_level));

                // Initialize Database
                let app_dir = dirs::data_local_dir()
                    .unwrap_or_else(|| std::path::PathBuf::from("."))
                    .join("tman");
                std::fs::create_dir_all(&app_dir).ok();
                let db_path = app_dir.join("history.db");
                let db_path_str = db_path.to_string_lossy().to_string();
                let db_url = format!("sqlite://{}?mode=rwc", db_path_str);

                if !db_path.exists() {
                    std::fs::File::create(&db_path).unwrap();
                }

                let db = db::init_db(&db_url).await.expect("Failed to init database");

                // Initialize Broadcaster
                let broadcaster = broadcaster::Broadcaster::new();

                // Start Local Web Server
                let tx_clone = broadcaster.tx.clone();
                let mut port = app_config.server_port;
                let bind_ip = if app_config.server_local_only {
                    "127.0.0.1"
                } else {
                    "0.0.0.0"
                };

                let listener = loop {
                    match tokio::net::TcpListener::bind(format!("{}:{}", bind_ip, port)).await {
                        Ok(l) => break l,
                        Err(_) => port += 1,
                    }
                };

                let actual_port = port;

                let mut server_shutdown_tx = None;

                if app_config.server_enabled {
                    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
                    tokio::spawn(async move {
                        server::start_server(listener, tx_clone, shutdown_rx).await;
                    });
                    server_shutdown_tx = Some(shutdown_tx);
                }

                // Manage State
                let state = Arc::new(AppState {
                    db,
                    broadcaster,
                    config: Mutex::new(app_config),
                    actual_port: Mutex::new(actual_port),
                    server_shutdown_tx: Mutex::new(server_shutdown_tx),
                    last_text: Mutex::new(None),
                });

                app.manage(state);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            capture_and_translate,
            pick_region,
            get_history,
            clear_history,
            save_settings,
            get_config,
            save_config,
            get_secret,
            set_secret,
            get_server_info,
            toggle_server,
            get_config_path,
            set_config_path,
            export_history_to_file,
            check_for_update,
            install_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
