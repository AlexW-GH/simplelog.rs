// Copyright 2016 Victor Brekenfeld
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Module providing the TestLogger Implementation

use log::{LevelFilter, Metadata, Record, SetLoggerError, set_max_level, set_boxed_logger, Log};
use ::{Config, SharedLogger};

/// The TestLogger struct. Provides a very basic Logger implementation
pub struct TestLogger {
    level: LevelFilter,
    config: Config,
}

impl TestLogger {

    /// init function. Globally initializes the TestLogger as the one and only used log facility.
    ///
    /// Takes the desired `Level` and `Config` as arguments. They cannot be changed later on.
    /// Fails if another Logger was already initialized.
    ///
    /// # Examples
    /// ```
    /// # extern crate testlog;
    /// # use testlog::*;
    /// # fn main() {
    /// let _ = TestLogger::init(LevelFilter::Info, Config::default());
    /// # }
    /// ```
    pub fn init(log_level: LevelFilter, config: Config) -> Result<(), SetLoggerError> {
        set_max_level(log_level.clone());
        set_boxed_logger(TestLogger::new(log_level, config))
    }

    /// allows to create a new logger, that can be independently used, no matter what is globally set.
    ///
    /// no macros are provided for this case and you probably
    /// dont want to use this function, but `init()`, if you dont want to build a `CombinedLogger`.
    ///
    /// Takes the desired `Level` and `Config` as arguments. They cannot be changed later on.
    ///
    /// # Examples
    /// ```
    /// # extern crate testlog;
    /// # use testlog::*;
    /// # fn main() {
    /// let test_logger = TestLogger::new(LevelFilter::Info, Config::default());
    /// # }
    /// ```
    pub fn new(log_level: LevelFilter, config: Config) -> Box<TestLogger> {
        Box::new(TestLogger { level: log_level, config })
    }
}

impl Log for TestLogger {

    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _ = log(&self.config, record);
        }
    }

    fn flush(&self) {}
}

impl SharedLogger for TestLogger {

    fn level(&self) -> LevelFilter {
        self.level
    }

    fn config(&self) -> Option<&Config>
    {
        Some(&self.config)
    }

    fn as_log(self: Box<Self>) -> Box<Log> {
        Box::new(*self)
    }

}

#[inline(always)]
pub fn log(config: &Config, record: &Record)
{

    if let Some(time) = config.time {
        if time <= record.level() {
            write_time(config);
        }
    }

    if let Some(level) = config.level {
        if level <= record.level() {
            write_level(record);
        }
    }

    if let Some(target) = config.target {
        if target <= record.level() {
            write_target(record);
        }
    }

    if let Some(location) = config.location {
        if location <= record.level() {
            write_location(record);
        }
    }

    write_args(record);
}

#[inline(always)]
pub fn write_time(config: &Config)
{
    let cur_time = chrono::Utc::now();
    print!("{} ", cur_time.format(
        config
            .time_format
            .unwrap_or("%H:%M:%S")
    ));
}

#[inline(always)]
pub fn write_level(record: &Record)
{
    print!("[{}] ", record.level());
}

#[inline(always)]
pub fn write_target(record: &Record)
{
    print!("{}: ", record.target());
}

#[inline(always)]
pub fn write_location(record: &Record)
{
    let file = record.file().unwrap_or("<unknown>");
    if let Some(line) = record.line() {
        print!("[{}:{}] ", file, line);
    } else {
        print!("[{}:<unknown>] ", file);
    }
}

#[inline(always)]
pub fn write_args(record: &Record)
{
    println!("{}", record.args());
}
