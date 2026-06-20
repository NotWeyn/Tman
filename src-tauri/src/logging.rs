use log::{Level, LevelFilter, Metadata, Record};
use std::sync::atomic::{AtomicUsize, Ordering};

static LOG_LEVEL: AtomicUsize = AtomicUsize::new(LevelFilter::Info as usize);

struct AppLogger;

impl log::Log for AppLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let target = metadata.target();
        if target.starts_with("tman_lib") || target.starts_with("tman") {
            metadata.level() as usize <= LOG_LEVEL.load(Ordering::Relaxed)
        } else {
            metadata.level() <= Level::Warn
        }
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let colored_level = match level {
                Level::Error => "\x1b[38;2;255;69;0mERROR\x1b[0m",
                Level::Warn => "\x1b[38;2;255;215;0mWARN\x1b[0m",
                Level::Info => "\x1b[38;2;30;144;255mINFO\x1b[0m",
                Level::Debug => "\x1b[38;2;50;205;50mDEBUG\x1b[0m",
                Level::Trace => "\x1b[38;2;112;128;144mTRACE\x1b[0m",
            };

            let target = record.target();
            let display_module = if target.starts_with("tman_lib") {
                target.strip_prefix("tman_lib::").unwrap_or("tman")
            } else {
                target
            };

            eprintln!("[{} {}] {}", colored_level, display_module, record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: AppLogger = AppLogger;

fn parse_level(level: &str) -> LevelFilter {
    match level {
        "error" => LevelFilter::Error,
        "debug" => LevelFilter::Debug,
        _ => LevelFilter::Info,
    }
}

pub fn init_logging(config_level: &str) {
    let level = parse_level(config_level);
    LOG_LEVEL.store(level as usize, Ordering::Relaxed);

    let result = log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace));

    if let Err(e) = result {
        eprintln!("[tman] Logger already initialized or failed: {}", e);
    }
}

pub fn set_log_level(config_level: &str) {
    let level = parse_level(config_level);
    LOG_LEVEL.store(level as usize, Ordering::Relaxed);
    log::debug!("Log level dynamically changed to {}", config_level);
}
