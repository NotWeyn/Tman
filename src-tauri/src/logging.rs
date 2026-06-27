use std::sync::OnceLock;
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{self, format::Writer, FmtContext, FormatEvent, FormatFields},
    prelude::*,
    registry::LookupSpan,
    reload, EnvFilter, Registry,
};
use tracing::{Event, Subscriber};

type ReloadHandle = reload::Handle<EnvFilter, Registry>;
static RELOAD_HANDLE: OnceLock<ReloadHandle> = OnceLock::new();

struct CustomFormatter;

impl<S, N> FormatEvent<S, N> for CustomFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let meta = event.metadata();
        
        let level = *meta.level();
        let colored_level = match level {
            tracing::Level::ERROR => "\x1b[38;2;255;69;0mERROR\x1b[0m",
            tracing::Level::WARN => "\x1b[38;2;255;215;0mWARN\x1b[0m",
            tracing::Level::INFO => "\x1b[38;2;30;144;255mINFO\x1b[0m",
            tracing::Level::DEBUG => "\x1b[38;2;50;205;50mDEBUG\x1b[0m",
            tracing::Level::TRACE => "\x1b[38;2;112;128;144mTRACE\x1b[0m",
        };
        
        let target = meta.target();
        let display_target = if target.starts_with("tman_lib") {
            target.strip_prefix("tman_lib::").unwrap_or("tman")
        } else {
            target
        };
        
        write!(writer, "[{} {}] ", colored_level, display_target)?;
        
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        
        writeln!(writer)
    }
}

fn parse_level(level: &str) -> LevelFilter {
    match level {
        "error" => LevelFilter::ERROR,
        "debug" => LevelFilter::DEBUG,
        _ => LevelFilter::INFO,
    }
}

pub fn init_logging(config_level: &str) {
    let level = parse_level(config_level);
    
    // Create base filter for tman and tman_lib crates
    let filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .parse_lossy(format!("tman={},tman_lib={}", level, level));

    let (filter_layer, reload_handle) = reload::Layer::new(filter);

    let format_layer = fmt::layer().event_format(CustomFormatter);

    let result = tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .try_init();

    if result.is_ok() {
        let _ = RELOAD_HANDLE.set(reload_handle);
    } else {
        eprintln!("[tman] Logger already initialized or failed.");
    }
}

pub fn set_log_level(config_level: &str) {
    let level = parse_level(config_level);
    if let Some(handle) = RELOAD_HANDLE.get() {
        let new_filter = EnvFilter::builder()
            .with_default_directive(level.into())
            .parse_lossy(format!("tman={},tman_lib={}", level, level));
        
        if let Err(e) = handle.modify(|filter| *filter = new_filter) {
            eprintln!("[tman] Failed to reload log level: {}", e);
        } else {
            tracing::debug!("Log level dynamically changed to {}", config_level);
        }
    }
}
