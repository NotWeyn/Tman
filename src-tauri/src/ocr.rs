use crate::config::AppConfig;
use image::DynamicImage;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

/// Lazily-initialized OAR-OCR engine. Stays resident in memory so
/// subsequent calls skip the ~200 ms ONNX session creation.
/// Models are auto-downloaded to ~/.oar on first use.
static OAR_ENGINE: Mutex<Option<oar_ocr::oarocr::OAROCR>> = Mutex::new(None);

pub fn extract_text(image: &DynamicImage, cfg: &AppConfig) -> Result<(String, String), String> {
    let raw_text = match cfg.ocr_engine.as_str() {
        "oar" => extract_text_oar(image),
        "paddle" | "easy" | "rapidocr" => extract_text_sidecar(image, cfg),
        other => return Err(format!("Desteklenmeyen OCR motoru seçildi: {}. Lütfen ayarlardan çalışan bir OCR motoru seçin.", other)),
    }?;

    let char_count = raw_text.chars().filter(|c| !c.is_whitespace()).count();
    if char_count < cfg.ocr_min_chars as usize {
        return Err(format!(
            "Text length {} below threshold {}",
            char_count, cfg.ocr_min_chars
        ));
    }

    let mut processed_text = raw_text;
    if cfg.ocr_merge_lines {
        processed_text = processed_text.replace("-\n", "").replace('\n', " ");
    }
    if cfg.ocr_merge_paragraphs {
        processed_text = processed_text.replace("  ", " "); // Simple heuristic after line merge
    }

    let detected_lang = if cfg.ocr_auto_detect_lang {
        let lang = whichlang::detect_language(&processed_text);
        format!("{:?}", lang).to_lowercase()
    } else {
        cfg.ocr_source_lang.clone()
    };

    Ok((processed_text.trim().to_string(), detected_lang))
}

/// Native Rust OCR via oar-ocr (PaddleOCR v5 ONNX models).
/// Engine is initialized once and kept in memory for fast repeated calls.
fn extract_text_oar(image: &DynamicImage) -> Result<String, String> {
    use oar_ocr::oarocr::OAROCRBuilder;
    use oar_ocr::utils::load_image;

    // Save image to temp file (oar-ocr's load_image reads from disk)
    let temp_path = std::env::temp_dir().join("tman_oar_capture.png");
    image
        .save_with_format(&temp_path, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save temp image: {}", e))?;

    let mut guard = OAR_ENGINE
        .lock()
        .map_err(|e| format!("OAR engine lock poisoned: {}", e))?;

    // Initialize engine on first use
    if guard.is_none() {
        let ocr = OAROCRBuilder::new(
            "pp-ocrv5_mobile_det.onnx",
            "pp-ocrv5_mobile_rec.onnx",
            "ppocrv5_dict.txt",
        )
        .build()
        .map_err(|e| format!("OAR-OCR init failed: {}", e))?;
        *guard = Some(ocr);
    }

    let ocr = guard.as_ref().unwrap();

    let oar_image = load_image(&temp_path)
        .map_err(|e| format!("Failed to load image for OAR: {}", e))?;

    let results = ocr
        .predict(vec![oar_image])
        .map_err(|e| format!("OAR-OCR prediction failed: {}", e))?;

    let mut text_parts = Vec::new();
    if let Some(result) = results.first() {
        for region in &result.text_regions {
            if let Some(ref text) = region.text {
                text_parts.push(text.clone());
            }
        }
    }

    Ok(text_parts.join("\n"))
}

fn extract_text_sidecar(image: &DynamicImage, cfg: &AppConfig) -> Result<String, String> {
    let temp_path = std::env::temp_dir().join("tman_capture.png");
    image
        .save_with_format(&temp_path, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save temp image: {}", e))?;

    // Use binary path from config
    let bin_path = match cfg.ocr_engine.as_str() {
        "paddle" => &cfg.ocr_paddle_path,
        "easy" => &cfg.ocr_easy_path,
        "rapidocr" => &cfg.ocr_rapid_path,
        _ => "",
    };

    // Fallback script mode if no binary is configured
    let mut cmd = if bin_path.is_empty() {
        let mut c = Command::new("python");
        let script_path = PathBuf::from("../sidecars/ocr_engine.py");
        let active_script = if script_path.exists() {
            script_path
        } else {
            PathBuf::from("sidecars/ocr_engine.py")
        };
        c.arg(active_script).arg(&temp_path);
        c
    } else {
        let mut c = Command::new(bin_path);
        c.arg(&temp_path);
        // Add flags if binary supports them
        if cfg.ocr_use_gpu {
            c.arg("--use-gpu");
        }
        c.arg("--lang").arg(&cfg.ocr_source_lang);
        c
    };

    let output = cmd
        .output()
        .map_err(|e| format!("Command execution failed: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
