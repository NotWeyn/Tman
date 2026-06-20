pub mod broadcaster;
pub mod capture;
pub mod config;
pub mod db;
pub mod logging;
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
    pub db: Mutex<Option<sqlx::SqlitePool>>,
    pub broadcaster: broadcaster::Broadcaster,
    pub config: Mutex<config::AppConfig>,
    pub actual_port: Mutex<u16>,
    pub server_shutdown_tx: Mutex<Option<tokio::sync::oneshot::Sender<()>>>,
    pub last_text: Mutex<Option<String>>,
    pub last_broadcast: Mutex<Option<String>>,
    pub http_client: reqwest::Client,
}

/// Helper: get DB pool from lazy state, returns error if not yet initialized
async fn get_db(state: &AppState) -> Result<sqlx::SqlitePool, String> {
    state
        .db
        .lock()
        .await
        .clone()
        .ok_or_else(|| "Database not ready yet, please wait a few seconds.".to_string())
}

#[tauri::command]
async fn capture_and_translate(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let pipeline_start = std::time::Instant::now();
    log::info!("──── Translation pipeline started ────");

    // 1. Capture Region + Preprocessing
    let t0 = std::time::Instant::now();
    let cfg = state.config.lock().await.clone();
    log::debug!(
        "Config loaded — provider={}, target_lang={}, cache={}",
        cfg.trans_provider,
        cfg.trans_target_lang,
        cfg.trans_cache_enabled
    );

    let (original_image, processed_image, region) = capture::capture_region(&cfg)?;
    log::debug!(
        "Screen captured — region='{}', image={}x{}",
        region,
        original_image.width(),
        original_image.height()
    );

    if cfg.capture_last_region != region {
        let mut cfg_mut = state.config.lock().await;
        cfg_mut.capture_last_region = region.clone();
        crate::config::save_config(&cfg_mut);
        log::debug!("Region changed, saved to config");
    }

    // Encode images for Web UI concurrently in the background
    let orig_clone = original_image.clone();
    let proc_clone = processed_image.clone();
    let b64_task = tokio::task::spawn_blocking(move || {
        let cap = crate::utils::dynamic_image_to_base64(&orig_clone).ok();
        let proc = crate::utils::dynamic_image_to_base64(&proc_clone).ok();
        (cap, proc)
    });

    let capture_ms = t0.elapsed().as_millis();
    log::debug!("Capture phase completed in {}ms", capture_ms);

    // 2. OCR
    let t1 = std::time::Instant::now();
    let (original_text, detected_lang) = ocr::extract_text(&processed_image, &cfg)?;
    let ocr_ms = t1.elapsed().as_millis();
    log::debug!(
        "OCR completed in {}ms — detected_lang='{}', text_len={} chars\n  text_preview=\"{}\"",
        ocr_ms,
        detected_lang,
        original_text.len(),
        &original_text.chars().take(80).collect::<String>()
    );

    // 3. Translation
    let t2 = std::time::Instant::now();
    let target_lang = cfg.trans_target_lang.clone();

    let mut translated_text = String::new();
    let mut from_cache = false;

    if cfg.trans_cache_enabled {
        if let Ok(pool) = get_db(&state).await {
            if let Ok(Some(cached)) =
                db::get_cached_translation(&pool, &original_text, &target_lang).await
            {
                translated_text = cached;
                from_cache = true;
                log::info!(
                    "Cache HIT — \"{}...\" → {}",
                    &original_text.chars().take(40).collect::<String>(),
                    target_lang
                );
            }
        } else {
            log::error!("Cache lookup failed — DB not ready yet");
        }
    }

    if !from_cache {
        if cfg.trans_cache_enabled {
            log::debug!(
                "Cache MISS — \"{}...\" → {}",
                &original_text.chars().take(40).collect::<String>(),
                target_lang
            );
        }
        log::debug!("Calling translation provider '{}' ...", cfg.trans_provider);
        translated_text =
            translate::translate_text(&original_text, &detected_lang, &cfg, &state.http_client)
                .await?;
        log::debug!("Translation received — {} chars", translated_text.len());
    }

    // In change-detection mode, compare translated text with previous translation
    if cfg.capture_mode == "change" {
        let mut last_text_lock = state.last_text.lock().await;
        if let Some(last_txt) = last_text_lock.as_ref() {
            if last_txt == &translated_text {
                return Ok("No significant text change".to_string());
            }
        }
        *last_text_lock = Some(translated_text.clone());
    }

    // Save to history (only if not from cache and not skipped by change-detection)
    if !from_cache && cfg.history_save {
        if let Ok(pool) = get_db(&state).await {
            match db::insert_history(
                &pool,
                &original_text,
                &translated_text,
                &target_lang,
                cfg.history_max_records,
            )
            .await
            {
                Ok(_) => log::debug!("History saved successfully"),
                Err(e) => log::error!("Failed to save history: {}", e),
            }
        } else {
            log::error!("Cannot save history — DB not initialized");
        }
    }

    // 4. Broadcast to Web UI — debounce: skip if identical to last broadcast
    let should_broadcast = {
        let mut last_bc = state.last_broadcast.lock().await;
        if last_bc.as_deref() == Some(&translated_text) {
            false
        } else {
            *last_bc = Some(translated_text.clone());
            true
        }
    };

    if should_broadcast {
        let (captured_b64, processed_b64) = b64_task.await.unwrap_or((None, None));

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

        // 5. Emit to Tauri frontend (Lightweight event WITHOUT images)
        use tauri::Emitter;
        let mut tauri_event = event.clone();
        tauri_event.captured_image = None;
        tauri_event.processed_image = None;
        let _ = app.emit("translation-update", tauri_event);
    }

    let translate_ms = t2.elapsed().as_millis();
    let total_ms = pipeline_start.elapsed().as_millis();
    log::debug!(
        "──── Pipeline Performance ────\n\
         ├─ Capture:   {:>5}ms\n\
         ├─ OCR:       {:>5}ms\n\
         ├─ Translate: {:>5}ms  (cache={})\n\
         └─ Total:     {:>5}ms",
        capture_ms,
        ocr_ms,
        translate_ms,
        if from_cache { "hit" } else { "miss" },
        total_ms
    );
    log::info!("──── Translation completed in {}ms ────", total_ms);

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
    let pool = get_db(&state).await?;
    db::get_history(&pool).await
}

#[tauri::command]
async fn clear_history(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let pool = get_db(&state).await?;
    db::clear_history(&pool).await
}

#[tauri::command]
async fn export_history_to_file(
    format: String,
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let pool = get_db(&state).await?;
    let history = db::get_history(&pool).await?;
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
    std::fs::write(&file_path, data_str).map_err(|e| format!("Failed to save file: {}", e))?;

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
    crate::logging::set_log_level(&new_config.app_log_level);
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

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
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
    let updater = app
        .updater()
        .map_err(|e| format!("Updater init failed: {}", e))?;
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
    let updater = app
        .updater()
        .map_err(|e| format!("Updater init failed: {}", e))?;
    let update = updater
        .check()
        .await
        .map_err(|e| format!("Update check failed: {}", e))?
        .ok_or("No update available".to_string())?;
    update
        .download_and_install(|_, _| {}, || {})
        .await
        .map_err(|e| format!("Update install failed: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn unload_ocr() -> Result<(), String> {
    crate::ocr::clear_engine();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let start = std::time::Instant::now();

            // ── Sync-fast: config load (~1 ms) ──
            let app_config = config::load_config();

            // Initialize structured logging from config
            logging::init_logging(&app_config.app_log_level);
            log::info!("Tman v{} starting...", env!("CARGO_PKG_VERSION"));
            log::debug!("Log level set to '{}'", app_config.app_log_level);

            // Initialize Broadcaster (instant — just channel creation)
            let broadcaster = broadcaster::Broadcaster::new();

            // Create shared HTTP client (reused across all translation calls)
            let http_client = reqwest::Client::new();

            // Create state with lazy DB (None initially, filled by background task)
            let state = Arc::new(AppState {
                db: Mutex::new(None),
                broadcaster,
                config: Mutex::new(app_config.clone()),
                actual_port: Mutex::new(app_config.server_port),
                server_shutdown_tx: Mutex::new(None),
                last_text: Mutex::new(None),
                last_broadcast: Mutex::new(None),
                http_client,
            });

            let state_clone = state.clone();

            // ── Heavy init → background task ──
            tauri::async_runtime::spawn(async move {
                let bg_start = std::time::Instant::now();

                // Database init
                let app_dir = dirs::data_local_dir()
                    .unwrap_or_else(|| std::path::PathBuf::from("."))
                    .join("tman");
                std::fs::create_dir_all(&app_dir).ok();
                let db_path = app_dir.join("history.db");
                let db_url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());

                if !db_path.exists() {
                    std::fs::File::create(&db_path).ok();
                }

                match db::init_db(&db_url).await {
                    Ok(pool) => {
                        *state_clone.db.lock().await = Some(pool);
                        log::info!("Database ready ({}ms)", bg_start.elapsed().as_millis());
                        log::debug!("DB path: {}", db_path.display());
                    }
                    Err(e) => {
                        log::error!("Database initialization failed: {}", e);
                        log::error!("  Attempted DB URL: {}", db_url);
                    }
                }

                // Server init
                let cfg = state_clone.config.lock().await.clone();
                if cfg.server_enabled {
                    let tx_clone = state_clone.broadcaster.tx.clone();
                    let bind_ip = if cfg.server_local_only {
                        "127.0.0.1"
                    } else {
                        "0.0.0.0"
                    };
                    let mut port = cfg.server_port;
                    log::debug!("Starting web server on {}:{}", bind_ip, port);

                    let listener = loop {
                        match tokio::net::TcpListener::bind(format!("{}:{}", bind_ip, port)).await {
                            Ok(l) => break l,
                            Err(e) => {
                                log::debug!("Port {} busy ({}), trying next...", port, e);
                                port += 1;
                                if port > cfg.server_port + 100 {
                                    log::error!(
                                        "Could not find free port after 100 attempts ({}–{})",
                                        cfg.server_port,
                                        port
                                    );
                                    return;
                                }
                            }
                        }
                    };

                    *state_clone.actual_port.lock().await = port;

                    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
                    tokio::spawn(async move {
                        server::start_server(listener, tx_clone, shutdown_rx).await;
                    });
                    *state_clone.server_shutdown_tx.lock().await = Some(shutdown_tx);

                    log::info!(
                        "Web server ready on {}:{} ({}ms)",
                        bind_ip,
                        port,
                        bg_start.elapsed().as_millis()
                    );
                } else {
                    log::debug!("Web server disabled in config, skipping");
                }
            });

            log::info!(
                "UI ready ({}ms) — DB/server loading in background",
                start.elapsed().as_millis()
            );

            app.manage(state);
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
            install_update,
            get_app_version,
            unload_ocr
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
