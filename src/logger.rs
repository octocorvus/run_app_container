use colored::{ColoredString, Colorize};

pub struct Logger;

impl Logger {
    pub fn set_log_level(level: log::LevelFilter) {
        log::set_max_level(level);
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let msg = format!("{} - {}", record.level(), record.args());
            eprintln!("{}", colourise_level_name(level, &msg));
        }
    }

    fn flush(&self) {}
}

fn colourise_level_name(level: log::Level, msg: &str) -> ColoredString {
    match level {
        log::Level::Error => msg.red(),
        log::Level::Warn => msg.yellow(),
        log::Level::Info => msg.green(),
        log::Level::Debug | log::Level::Trace => msg.normal(),
    }
}
