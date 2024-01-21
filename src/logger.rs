use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError, error, warn, info, debug, trace};
use colored::*;

pub struct Logger;

impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(LevelFilter::Trace);
        log::set_boxed_logger(Box::new(self))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {

            // Find the correct color for the level of the log
            let level_string = match record.level() {
                Level::Error => format!("{:<5}", record.level().to_string()).red().to_string(),
                Level::Warn => format!("{:<4}", record.level().to_string()).yellow().to_string(),
                Level::Info => format!("{:<4}", record.level().to_string()).normal().to_string(),
                Level::Debug => format!("{:<5}", record.level().to_string()).blue().to_string(),
                Level::Trace => format!("{:<5}", record.level().to_string()).cyan().to_string()
            };

            // Construct the message and print it
            let message = format!("[{}] {}", level_string, record.args());

            println!("{}", message);
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError>{
    Logger::new().init()
}

pub fn trace(message: String) {
    trace!("{}", message)
}

pub fn debug(message: String) {
    debug!("{}", message)
}

pub fn info(message: String) {
    info!("{}", message)
}

pub fn warn(message: String) {
    warn!("{}", message)
}

pub fn error(message: String) {
    error!("{}", message)
}