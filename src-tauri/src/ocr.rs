use crate::config::AppConfig;
use image::DynamicImage;
use std::io::Cursor;
use std::process::Command;
use std::path::PathBuf;

pub fn extract_text(image: &DynamicImage, cfg: &AppConfig) -> Result<(String, String), String> {
    let raw_text = match cfg.ocr_engine.as_str() {
        "leptess" => extract_text_leptess(image, cfg),
        _ => extract_text_sidecar(image, cfg),
    }?;

    let char_count = raw_text.chars().filter(|c| !c.is_whitespace()).count();
    if char_count < cfg.ocr_min_chars as usize {
        return Err(format!("Text length {} below threshold {}", char_count, cfg.ocr_min_chars));
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

fn extract_text_sidecar(image: &DynamicImage, cfg: &AppConfig) -> Result<String, String> {
    let temp_path = std::env::temp_dir().join("tman_capture.png");
    image.save_with_format(&temp_path, image::ImageFormat::Png)
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
        let active_script = if script_path.exists() { script_path } else { PathBuf::from("sidecars/ocr_engine.py") };
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

    let output = cmd.output().map_err(|e| format!("Command execution failed: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn extract_text_leptess(image: &DynamicImage, cfg: &AppConfig) -> Result<String, String> {
    use leptess::LepTess;
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Tiff)
        .map_err(|e| format!("Failed to encode image for OCR: {}", e))?;

    // Leptess takes tesseract language codes like "eng", "tur", "jpn"
    let lang = if cfg.ocr_source_lang == "auto" { "eng+jpn".to_string() } else { cfg.ocr_source_lang.clone() };
    
    let mut lt = LepTess::new(None, &lang).map_err(|e| format!("Failed to initialize Tesseract: {}", e))?;
    lt.set_image_from_mem(&bytes).map_err(|e| format!("Failed to load image into Tesseract: {}", e))?;
    let text = lt.get_utf8_text().map_err(|e| format!("Failed to extract text: {}", e))?;
    Ok(text)
}
