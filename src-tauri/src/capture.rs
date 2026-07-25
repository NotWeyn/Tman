use crate::config::AppConfig;
use image::{DynamicImage, ImageFormat, RgbaImage};
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::io::AsFd;
use std::process::Command;

use wayland_client::{
    protocol::{
        wl_buffer::{self, WlBuffer},
        wl_output::{self, WlOutput},
        wl_registry::{self, WlRegistry},
        wl_shm::{self, Format, WlShm},
        wl_shm_pool::{self, WlShmPool},
    },
    Connection, Dispatch, QueueHandle, WEnum,
};
use wayland_protocols_wlr::screencopy::v1::client::{
    zwlr_screencopy_frame_v1::{self, ZwlrScreencopyFrameV1},
    zwlr_screencopy_manager_v1::{self, ZwlrScreencopyManagerV1},
};

#[derive(Default)]
struct WlrState {
    shm: Option<WlShm>,
    screencopy_manager: Option<ZwlrScreencopyManagerV1>,
    outputs: Vec<WlOutput>,
    buffer_info: Option<(Format, u32, u32, u32)>,
    flags: u32,
    ready: bool,
    failed: bool,
}

impl Dispatch<WlRegistry, ()> for WlrState {
    fn event(
        state: &mut Self,
        registry: &WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "wl_shm" => {
                    let shm = registry.bind::<WlShm, _, _>(name, version.min(1), qh, ());
                    state.shm = Some(shm);
                }
                "zwlr_screencopy_manager_v1" => {
                    let manager = registry.bind::<ZwlrScreencopyManagerV1, _, _>(
                        name,
                        version.min(3),
                        qh,
                        (),
                    );
                    state.screencopy_manager = Some(manager);
                }
                "wl_output" => {
                    let output = registry.bind::<WlOutput, _, _>(name, version.min(4), qh, ());
                    state.outputs.push(output);
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<WlShm, ()> for WlrState {
    fn event(
        _: &mut Self,
        _: &WlShm,
        _: wl_shm::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlOutput, ()> for WlrState {
    fn event(
        _: &mut Self,
        _: &WlOutput,
        _: wl_output::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwlrScreencopyManagerV1, ()> for WlrState {
    fn event(
        _: &mut Self,
        _: &ZwlrScreencopyManagerV1,
        _: zwlr_screencopy_manager_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwlrScreencopyFrameV1, ()> for WlrState {
    fn event(
        state: &mut Self,
        _: &ZwlrScreencopyFrameV1,
        event: zwlr_screencopy_frame_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            zwlr_screencopy_frame_v1::Event::Buffer {
                format: WEnum::Value(fmt),
                width,
                height,
                stride,
            } => {
                state.buffer_info = Some((fmt, width, height, stride));
            }
            zwlr_screencopy_frame_v1::Event::Flags {
                flags: WEnum::Value(f),
            } => {
                state.flags = f.bits();
            }
            zwlr_screencopy_frame_v1::Event::Ready { .. } => {
                state.ready = true;
            }
            zwlr_screencopy_frame_v1::Event::Failed => {
                state.failed = true;
            }
            _ => {}
        }
    }
}

impl Dispatch<WlShmPool, ()> for WlrState {
    fn event(
        _: &mut Self,
        _: &WlShmPool,
        _: wl_shm_pool::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlBuffer, ()> for WlrState {
    fn event(
        _: &mut Self,
        _: &WlBuffer,
        _: wl_buffer::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

fn parse_region_coords(region: &str) -> Option<(i32, i32, u32, u32)> {
    let parts: Vec<&str> = region.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }
    let xy: Vec<&str> = parts[0].split(',').collect();
    let wh: Vec<&str> = parts[1].split('x').collect();
    if xy.len() != 2 || wh.len() != 2 {
        return None;
    }
    let x: i32 = xy[0].parse().ok()?;
    let y: i32 = xy[1].parse().ok()?;
    let w: u32 = wh[0].parse().ok()?;
    let h: u32 = wh[1].parse().ok()?;
    Some((x, y, w, h))
}

fn capture_region_wlr(region: &str) -> Result<DynamicImage, String> {
    let conn = Connection::connect_to_env().map_err(|e| format!("Wayland conn failed: {}", e))?;
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();
    let mut state = WlrState::default();

    let _registry = conn.display().get_registry(&qh, ());
    event_queue
        .roundtrip(&mut state)
        .map_err(|e| format!("Wayland registry roundtrip 1 failed: {}", e))?;
    event_queue
        .roundtrip(&mut state)
        .map_err(|e| format!("Wayland registry roundtrip 2 failed: {}", e))?;

    let shm = state
        .shm
        .clone()
        .ok_or_else(|| "Wayland wl_shm global not found".to_string())?;
    let manager = state
        .screencopy_manager
        .clone()
        .ok_or_else(|| "Wayland zwlr_screencopy_manager_v1 global not found".to_string())?;
    let output = state
        .outputs
        .first()
        .cloned()
        .ok_or_else(|| "No Wayland outputs found".to_string())?;

    let coords = parse_region_coords(region);
    let frame = match coords {
        Some((x, y, w, h)) => {
            manager.capture_output_region(0, &output, x, y, w as i32, h as i32, &qh, ())
        }
        None => manager.capture_output(0, &output, &qh, ()),
    };

    event_queue
        .roundtrip(&mut state)
        .map_err(|e| format!("Wayland frame roundtrip failed: {}", e))?;

    let (format, width, height, stride) = state
        .buffer_info
        .ok_or_else(|| "No buffer info received from screencopy frame".to_string())?;

    let size = (stride * height) as usize;
    let mut file = tempfile::tempfile().map_err(|e| format!("Failed to create shm file: {}", e))?;
    file.set_len(size as u64)
        .map_err(|e| format!("Failed to resize shm file: {}", e))?;

    let pool = shm.create_pool(file.as_fd(), size as i32, &qh, ());
    let buffer = pool.create_buffer(
        0,
        width as i32,
        height as i32,
        stride as i32,
        format,
        &qh,
        (),
    );

    frame.copy(&buffer);

    while !state.ready && !state.failed {
        event_queue
            .blocking_dispatch(&mut state)
            .map_err(|e| format!("Wayland dispatch failed: {}", e))?;
    }

    if state.failed {
        return Err("Screencopy frame dispatch reported failure".to_string());
    }

    file.seek(SeekFrom::Start(0))
        .map_err(|e| format!("Seek failed: {}", e))?;
    let mut raw_bytes = vec![0u8; size];
    file.read_exact(&mut raw_bytes)
        .map_err(|e| format!("Read shm bytes failed: {}", e))?;

    let mut rgba_bytes = vec![0u8; (width * height * 4) as usize];
    for y_idx in 0..height {
        let src_row = (y_idx * stride) as usize;
        let dst_row = (y_idx * width * 4) as usize;
        for x_idx in 0..width {
            let src_px = src_row + (x_idx * 4) as usize;
            let dst_px = dst_row + (x_idx * 4) as usize;
            if src_px + 3 < raw_bytes.len() && dst_px + 3 < rgba_bytes.len() {
                match format {
                    Format::Argb8888 | Format::Xrgb8888 => {
                        let b = raw_bytes[src_px];
                        let g = raw_bytes[src_px + 1];
                        let r = raw_bytes[src_px + 2];
                        let a = raw_bytes[src_px + 3];
                        rgba_bytes[dst_px] = r;
                        rgba_bytes[dst_px + 1] = g;
                        rgba_bytes[dst_px + 2] = b;
                        rgba_bytes[dst_px + 3] = if format == Format::Xrgb8888 { 255 } else { a };
                    }
                    _ => {
                        rgba_bytes[dst_px] = raw_bytes[src_px];
                        rgba_bytes[dst_px + 1] = raw_bytes[src_px + 1];
                        rgba_bytes[dst_px + 2] = raw_bytes[src_px + 2];
                        rgba_bytes[dst_px + 3] = raw_bytes[src_px + 3];
                    }
                }
            }
        }
    }

    let rgba_img = RgbaImage::from_raw(width, height, rgba_bytes)
        .ok_or_else(|| "Failed to construct RgbaImage from buffer".to_string())?;

    Ok(DynamicImage::ImageRgba8(rgba_img))
}

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

    let original_image = match capture_region_wlr(&region) {
        Ok(img) => {
            log::debug!("Screen captured via native Wayland wlr-screencopy");
            img
        }
        Err(e) => {
            let grim_output = Command::new("grim")
                .arg("-t")
                .arg("ppm")
                .arg("-g")
                .arg(&region)
                .arg("-") // write to stdout
                .output()
                .map_err(|e| {
                    log::error!("Failed to launch grim: {}", e);
                    format!("Failed to run grim (wlr-screencopy error: {}): {}", e, e)
                })?;

            if !grim_output.status.success() {
                let stderr = String::from_utf8_lossy(&grim_output.stderr);
                log::error!("Grim capture failed: {} (wlr-screencopy error: {})", stderr, e);
                return Err("Grim failed to capture region".to_string());
            }

            log::debug!("Screen captured via grim fallback");

            image::load_from_memory_with_format(&grim_output.stdout, ImageFormat::Pnm).map_err(
                |e| {
                    log::error!("Failed to decode captured image: {}", e);
                    format!("Failed to parse image: {}", e)
                },
            )?
        }
    };

    let mut processed_image = original_image.clone();

    // Preprocess: Scale
    if cfg.pre_scale > 1.0 {
        let nwidth = (processed_image.width() as f32 * cfg.pre_scale) as u32;
        let nheight = (processed_image.height() as f32 * cfg.pre_scale) as u32;
        processed_image =
            processed_image.resize(nwidth, nheight, image::imageops::FilterType::Triangle);
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
