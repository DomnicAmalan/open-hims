use log::{info, warn, error};

/// HIMS SDK Logger
pub struct HimsLogger;

impl HimsLogger {
    pub fn init() {
        env_logger::init();
    }

    pub fn log_info(message: &str) {
        info!("{}", message);
    }

    pub fn log_warning(message: &str) {
        warn!("{}", message);
    }

    pub fn log_error(message: &str) {
        error!("{}", message);
    }
}