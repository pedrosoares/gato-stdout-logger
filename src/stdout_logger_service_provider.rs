use gato_core::kernel::{Provider, Logger};
use crate::stdout_logger::StdoutLogger;

pub struct StdoutLoggerServiceProvider {}

impl StdoutLoggerServiceProvider {
    pub fn new() -> Box<Self> {
        return Box::new(StdoutLoggerServiceProvider {  });
    }
}

impl Provider for StdoutLoggerServiceProvider {
    fn boot(&self) {
        let stdout_logger = StdoutLogger::new();
        Logger::set_driver(Box::new(stdout_logger));
    }
}
