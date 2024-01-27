use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use colored::*;
use time::{format_description::FormatItem, OffsetDateTime};

const TIMESTAMP_FORMAT: &[FormatItem] =
    time::macros::format_description!("[hour]:[minute]:[second]");

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
        metadata.level() <= Level::Trace
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

            let timestamp = format!(
                "{}", 
                OffsetDateTime::now_utc().format(TIMESTAMP_FORMAT)
                    .expect("Error with time stamp format")
                );

            // Construct the message and print it
            let message = format!("[{}] [{}] {} - {}", timestamp, level_string, record.target(), record.args());

            println!("{}", message);
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError>{
    Logger::new().init()
}

// Core engine macros for the logs
#[allow(unused_macros)]
macro_rules! hds_core_error {
    ($($arg:tt)+) => (log::log!(target: "HADES", log::Level::Error, $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use hds_core_error;

#[allow(unused_macros)]
macro_rules! hds_core_warn {
    ($($arg:tt)+) => (log::log!(target: "HADES", log::Level::Warn, $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use hds_core_warn;

#[allow(unused_macros)]
macro_rules! hds_core_info {
    ($($arg:tt)+) => (log::log!(target: "HADES", log::Level::Info, $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use hds_core_info;

#[allow(unused_macros)]
macro_rules! hds_core_debug {
    ($($arg:tt)+) => (log::log!(target: "HADES", log::Level::Debug, $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use hds_core_debug;

#[allow(unused_macros)]
macro_rules! hds_core_trace {
    ($($arg:tt)+) => (log::log!(target: "HADES", log::Level::Trace, $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use hds_core_trace;

// App macros for the logs
#[allow(unused_macros)]
macro_rules! hds_error {
    ($($arg:tt)+) => (log::log!(target: "APP", log::Level::Error, $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use hds_error;

#[allow(unused_macros)]
macro_rules! hds_warn {
    ($($arg:tt)+) => (log::log!(target: "APP", log::Level::Warn, $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use hds_warn;

#[allow(unused_macros)]
macro_rules! hds_info {
    ($($arg:tt)+) => (log::log!(target: "APP", log::Level::Info, $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use hds_info;

#[allow(unused_macros)]
macro_rules! hds_debug {
    ($($arg:tt)+) => (log::log!(target: "APP", log::Level::Debug, $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use hds_debug;

#[allow(unused_macros)]
macro_rules! hds_trace {
    ($($arg:tt)+) => (log::log!(target: "APP", log::Level::Trace, $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use hds_trace;