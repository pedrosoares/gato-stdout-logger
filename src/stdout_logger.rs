use gato_core::kernel::Log;
use chrono::{Utc, Datelike, Timelike};

pub struct StdoutLogger {}

impl StdoutLogger {
    pub fn new() -> Self {
        return StdoutLogger{};
    }

    fn get_date(&self) -> String {
        let now = Utc::now();
        let date = format!("{}-{:02}-{:02} {:02}:{:02}:{:02}",
           now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second()
        );
        return date;
    }
}

impl Log for StdoutLogger {
    fn info(&self, text: &str) {
        println!("[{}] {}", self.get_date(), text);
    }

    fn error(&self, text: &str) {
        eprintln!("[{}] {}", self.get_date(), text);
    }
}
