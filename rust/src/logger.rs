use std::sync::Mutex;
use log::{Level, Log, Record, SetLoggerError};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use godot::log::{godot_print, godot_script_error, godot_warn};

static LOGGER: GodotLogger = GodotLogger;

struct GodotLogger;

impl Log for GodotLogger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        log(record);
    }

    fn flush(&self) {}
}

fn log(record: &Record) {
    match record.level() {
        Level::Error => godot_script_error!("{}", record.args()),
        Level::Warn => godot_warn!("{}", record.args()),
        Level::Info => godot_print!("INFO: {}", record.args()),
        Level::Debug => godot_print!("DEBUG: {}", record.args()),
        Level::Trace => godot_print!("TRACE: {}", record.args()),
    }
}

#[inline]
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

#[allow(dead_code)]
#[inline]
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Info)
}

static FILE_GUARD: Mutex<Option<WorkerGuard>> = Mutex::new(None);
#[inline]
pub fn init_tracing() {
    let mut lock = FILE_GUARD.lock().expect("Unable to lock mutex");
    let file_appender = tracing_appender::rolling::hourly("./logs", "my-game");
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let (file, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer().with_writer(file);
    let combined = Registry::default()
        .with(stdout_log)
        .with(file_layer);
    tracing::subscriber::set_global_default(combined)
        .expect("Unable to set global subscriber");
    *lock = Some(guard);
}

#[inline]
pub fn deinit_tracing() {
    let mut lock = FILE_GUARD.lock().expect("Unable to lock mutex");
    let Some(_) = lock.as_ref() else {
        eprintln!("de-init tracing but it not initialized");
        return
    };
    *lock = None;
}