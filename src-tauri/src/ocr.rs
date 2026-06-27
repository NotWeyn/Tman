use crate::config::AppConfig;
use image::DynamicImage;
use std::sync::Mutex;

/// Lazily-initialized OAR-OCR engine. Stays resident in memory so
/// subsequent calls skip the ~200 ms ONNX session creation.
/// Models are auto-downloaded to ~/.oar on first use.
static OAR_ENGINE: Mutex<Option<oar_ocr::oarocr::OAROCR>> = Mutex::new(None);

pub fn clear_engine() {
    if let Ok(mut guard) = OAR_ENGINE.lock() {
        *guard = None;
        log::debug!("OCR engine unloaded from memory");
    }
}

pub fn extract_text(image: &DynamicImage, cfg: &AppConfig) -> Result<(String, String), String> {
    let raw_text = extract_text_oar(image)?;

    let char_count = raw_text.chars().filter(|c| !c.is_whitespace()).count();
    if char_count < cfg.ocr_min_chars as usize {
        log::debug!(
            "OCR text too short ({} chars < threshold {}), skipping",
            char_count,
            cfg.ocr_min_chars
        );
        return Err(format!(
            "Text length {} below threshold {}",
            char_count, cfg.ocr_min_chars
        ));
    }

    let mut processed_text = raw_text;
    if cfg.ocr_merge_lines {
        processed_text = processed_text.replace("-\n", "").replace('\n', " ");
        log::debug!("OCR post-processing: lines merged");
    }
    if cfg.ocr_merge_paragraphs {
        processed_text = processed_text.replace("  ", " "); // Simple heuristic after line merge
        log::debug!("OCR post-processing: paragraphs merged");
    }

    let detected_lang = if cfg.ocr_auto_detect_lang {
        let lang = whichlang::detect_language(&processed_text);
        let lang_str = format!("{:?}", lang).to_lowercase();
        log::debug!("Language auto-detected: '{}'", lang_str);
        lang_str
    } else {
        log::debug!(
            "Using configured source language: '{}'",
            cfg.ocr_source_lang
        );
        cfg.ocr_source_lang.clone()
    };

    Ok((processed_text.trim().to_string(), detected_lang))
}

/// Native Rust OCR via oar-ocr (PaddleOCR v5 ONNX models).
/// Engine is initialized once and kept in memory for fast repeated calls.
fn extract_text_oar(image: &DynamicImage) -> Result<String, String> {
    use oar_ocr::oarocr::OAROCRBuilder;
    use oar_ocr::utils::load_image;

    let shm_dir = std::path::Path::new("/dev/shm");
    let temp_dir = if shm_dir.exists() && shm_dir.is_dir() {
        shm_dir.to_path_buf()
    } else {
        std::env::temp_dir()
    };
    let temp_path = temp_dir.join("tman_oar_capture.bmp");
    image
        .save_with_format(&temp_path, image::ImageFormat::Bmp)
        .map_err(|e| {
            log::error!("Failed to save temp image for OCR: {}", e);
            format!("Failed to save temp image: {}", e)
        })?;

    let mut guard = OAR_ENGINE.lock().map_err(|e| {
        log::error!("OCR engine lock poisoned: {}", e);
        format!("OAR engine lock poisoned: {}", e)
    })?;

    // Initialize engine on first use
    if guard.is_none() {
        log::info!("Initializing OCR engine (first use, may download models)...");
        let ocr = OAROCRBuilder::new(
            "pp-ocrv5_mobile_det.onnx",
            "pp-ocrv5_mobile_rec.onnx",
            "ppocrv5_dict.txt",
        )
        .build()
        .map_err(|e| {
            log::error!("OCR engine initialization failed: {}", e);
            format!("OAR-OCR init failed: {}", e)
        })?;
        *guard = Some(ocr);
        log::info!("OCR engine initialized successfully");
    }

    let ocr = guard.as_ref().ok_or_else(|| "OCR engine not initialized".to_string())?;

    let oar_image = load_image(&temp_path).map_err(|e| {
        log::error!("Failed to load image for OCR processing: {}", e);
        format!("Failed to load image for OAR: {}", e)
    })?;

    let results = ocr.predict(vec![oar_image]).map_err(|e| {
        log::error!("OCR prediction failed: {}", e);
        format!("OAR-OCR prediction failed: {}", e)
    })?;

    let mut text_parts = Vec::new();
    if let Some(result) = results.first() {
        log::debug!("OCR found {} text regions", result.text_regions.len());
        for region in &result.text_regions {
            if let Some(ref text) = region.text {
                text_parts.push(text.clone());
            }
        }
    }

    Ok(text_parts.join("\n"))
}
