use crate::config::AppConfig;
use std::process::Command;
use image::{DynamicImage, ImageFormat};

pub fn pick_region() -> Result<String, String> {
    let slurp_output = Command::new("slurp")
        .output()
        .map_err(|e| format!("Failed to run slurp: {}", e))?;

    if !slurp_output.status.success() {
        return Err("Slurp was cancelled or failed".to_string());
    }

    Ok(String::from_utf8_lossy(&slurp_output.stdout).trim().to_string())
}

pub fn capture_region(cfg: &AppConfig) -> Result<(DynamicImage, DynamicImage, String), String> {
    let mut region = cfg.capture_last_region.clone();

    if region.is_empty() {
        let slurp_output = Command::new("slurp")
            .output()
            .map_err(|e| format!("Failed to run slurp: {}", e))?;

        if !slurp_output.status.success() {
            return Err("Slurp was cancelled or failed".to_string());
        }

        region = String::from_utf8_lossy(&slurp_output.stdout).trim().to_string();
    }

    let grim_output = Command::new("grim")
        .arg("-g")
        .arg(&region)
        .arg("-") // write to stdout
        .output()
        .map_err(|e| format!("Failed to run grim: {}", e))?;

    if !grim_output.status.success() {
        return Err("Grim failed to capture region".to_string());
    }

    let original_image = image::load_from_memory_with_format(&grim_output.stdout, ImageFormat::Png)
        .map_err(|e| format!("Failed to parse image: {}", e))?;

    let mut processed_image = original_image.clone();

    // Preprocess: Scale
    if cfg.pre_scale > 1.0 {
        let nwidth = (processed_image.width() as f32 * cfg.pre_scale) as u32;
        let nheight = (processed_image.height() as f32 * cfg.pre_scale) as u32;
        processed_image = processed_image.resize(nwidth, nheight, image::imageops::FilterType::Lanczos3);
    }

    // Preprocess: Grayscale
    if cfg.pre_grayscale || cfg.pre_binarize {
        processed_image = processed_image.grayscale();
    }

    // Preprocess: Contrast
    if cfg.pre_contrast != "normal" && !cfg.pre_contrast.is_empty() {
        processed_image = processed_image.adjust_contrast(20.0);
    }

    // Preprocess: Binarize
    if cfg.pre_binarize {
        let mut luma = processed_image.into_luma8();
        for p in luma.pixels_mut() {
            if p[0] > 128 {
                p[0] = 255;
            } else {
                p[0] = 0;
            }
        }
        processed_image = DynamicImage::ImageLuma8(luma);
    }

    Ok((original_image, processed_image, region))
}
