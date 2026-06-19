use crate::config::AppConfig;
use image::{DynamicImage, ImageFormat};
use std::process::Command;

pub fn pick_region() -> Result<String, String> {
    log::debug!("Launching slurp for region selection...");
    let slurp_output = Command::new("slurp").output().map_err(|e| {
        log::error!("Failed to launch slurp: {}", e);
        format!("Failed to run slurp: {}", e)
    })?;

    if !slurp_output.status.success() {
        log::debug!("Slurp cancelled by user or failed");
        return Err("Slurp was cancelled or failed".to_string());
    }

    let region = String::from_utf8_lossy(&slurp_output.stdout)
        .trim()
        .to_string();
    log::debug!("Region selected: '{}'", region);
    Ok(region)
}

pub fn capture_region(cfg: &AppConfig) -> Result<(DynamicImage, DynamicImage, String), String> {
    let mut region = cfg.capture_last_region.clone();

    if region.is_empty() {
        log::debug!("No saved region, launching slurp...");
        let slurp_output = Command::new("slurp").output().map_err(|e| {
            log::error!("Failed to launch slurp: {}", e);
            format!("Failed to run slurp: {}", e)
        })?;

        if !slurp_output.status.success() {
            log::debug!("Slurp cancelled by user");
            return Err("Slurp was cancelled or failed".to_string());
        }

        region = String::from_utf8_lossy(&slurp_output.stdout)
            .trim()
            .to_string();
        log::debug!("New region selected: '{}'", region);
    }

    log::debug!("Capturing screen region '{}' via grim...", region);
    let grim_output = Command::new("grim")
        .arg("-g")
        .arg(&region)
        .arg("-") // write to stdout
        .output()
        .map_err(|e| {
            log::error!("Failed to launch grim: {}", e);
            format!("Failed to run grim: {}", e)
        })?;

    if !grim_output.status.success() {
        let stderr = String::from_utf8_lossy(&grim_output.stderr);
        log::error!("Grim capture failed: {}", stderr);
        return Err("Grim failed to capture region".to_string());
    }

    log::debug!("Grim output: {} bytes", grim_output.stdout.len());

    let original_image = image::load_from_memory_with_format(&grim_output.stdout, ImageFormat::Png)
        .map_err(|e| {
            log::error!("Failed to decode captured image: {}", e);
            format!("Failed to parse image: {}", e)
        })?;

    let mut processed_image = original_image.clone();

    // Preprocess: Scale
    if cfg.pre_scale > 1.0 {
        let nwidth = (processed_image.width() as f32 * cfg.pre_scale) as u32;
        let nheight = (processed_image.height() as f32 * cfg.pre_scale) as u32;
        processed_image =
            processed_image.resize(nwidth, nheight, image::imageops::FilterType::Lanczos3);
        log::debug!(
            "Preprocessing: scaled {}x → {}x{}",
            cfg.pre_scale,
            nwidth,
            nheight
        );
    }

    // Preprocess: Grayscale
    if cfg.pre_grayscale {
        processed_image = processed_image.grayscale();
        log::debug!("Preprocessing: grayscale applied");
    }

    // Preprocess: Contrast
    match cfg.pre_contrast.as_str() {
        "light" => {
            processed_image = processed_image.adjust_contrast(15.0);
            log::debug!("Preprocessing: light contrast (+15)");
        }
        "strong" => {
            processed_image = processed_image.adjust_contrast(40.0);
            log::debug!("Preprocessing: strong contrast (+40)");
        }
        _ => {} // "off", "none", ""
    }

    Ok((original_image, processed_image, region))
}
