use log::{Level, Log, Metadata, Record};

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color = match record.level() {
                Level::Debug => "200;200;200",
                Level::Info => "100;255;100",
                Level::Warn => "255;100;100",
                Level::Error => "100;100;255",
                Level::Trace => "100;255;255",
            };
            let level = match record.level() {
                Level::Debug => "DEBG",
                Level::Info => "INFO",
                Level::Warn => "WARN",
                Level::Error => "ERROR",
                Level::Trace => "TRACE",
            };
            eprintln!(
                "\x1b[38;2;{}m{} {} {:42}  {}\x1b[0m",
                color,
                chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S"),
                level,
                format!(
                    "{} ({})",
                    record.module_path().unwrap(),
                    record.line().unwrap()
                ),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
