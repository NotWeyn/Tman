/// Centralized logging initialization.
/// Uses env_logger with direct LevelFilter API for reliable filtering.
/// Uses exact TrueColor (24-bit RGB) ANSI escape codes to ensure vibrant colors 
/// regardless of terminal theme defaults.

use std::io::Write;

pub fn init_logging(config_level: &str) {
    let level_filter = match config_level {
        "error" => log::LevelFilter::Error,
        "debug" => log::LevelFilter::Debug,
        _ => log::LevelFilter::Info,
    };

    let result = env_logger::Builder::new()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("tman_lib", level_filter)
        .format(|buf, record| {
            let level = record.level();
            
            // TrueColor RGB ANSI Escape sequences
            let colored_level = match level {
                // Red (OrangeRed / Bright Red)
                log::Level::Error => "\x1b[38;2;255;69;0mERROR\x1b[0m",
                // Gold / Yellow
                log::Level::Warn  => "\x1b[38;2;255;215;0mWARN\x1b[0m",
                // DodgerBlue / Bright Blue
                log::Level::Info  => "\x1b[38;2;30;144;255mINFO\x1b[0m",
                // LimeGreen / Bright Green
                log::Level::Debug => "\x1b[38;2;50;205;50mDEBUG\x1b[0m",
                // SlateGray
                log::Level::Trace => "\x1b[38;2;112;128;144mTRACE\x1b[0m",
            };

            let module = record.module_path()
                .unwrap_or("")
                .strip_prefix("tman_lib::")
                .unwrap_or("tman");

            writeln!(buf, "[{} {}] {}", colored_level, module, record.args())
        })
        .try_init();

    if let Err(e) = result {
        eprintln!("[tman] Logger already initialized or failed: {}", e);
    }
}
