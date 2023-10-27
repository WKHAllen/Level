use chrono::Local;
use common::format_log_message;
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// The directory where log files are stored.
pub(crate) const LOGS_DIR: &str = "logs";

/// The file extension for log files.
pub(crate) const LOG_FILE_EXT: &str = "log";

/// Creates the logs directory if it does not already exist.
pub(crate) fn init_logs_dir() -> io::Result<()> {
    let root_path = project_root::get_project_root()?;
    let logs_dir = root_path.join(LOGS_DIR);

    if !logs_dir.exists() {
        fs::create_dir(&logs_dir)?;
    }

    Ok(())
}

/// Gets the path to the logs directory.
pub(crate) fn get_logs_path() -> String {
    let root_path = project_root::get_project_root().unwrap();
    let logs_path = format!("{}/{}", root_path.display(), LOGS_DIR);
    logs_path
}

/// The logger for the app backend.
#[derive(Debug, Clone)]
struct AppLogger {
    /// The file where logs are written.
    out: Arc<Mutex<File>>,
}

impl AppLogger {
    /// Creates a new app logger.
    pub fn new() -> Self {
        init_logs_dir().expect("Failed to initialize logs directory");

        let today = Local::now().naive_local().date();
        let file_date_prefix = today.format("%Y-%m-%d_").to_string();

        let logs_path = get_logs_path();
        let entries = fs::read_dir(&logs_path).expect("Failed to read log directory");
        let mut log_index = 0;

        for entry in entries {
            let entry = entry.expect("Failed to read log entry");

            if let Some(file_identifier) = entry.path().file_stem() {
                if let Some(file_identifier_str) = file_identifier.to_str() {
                    if let Some(suffix) = file_identifier_str.strip_prefix(&file_date_prefix) {
                        if let Ok(index) = suffix.parse::<usize>() {
                            if index > log_index {
                                log_index = index;
                            }
                        }
                    }
                }
            }
        }

        let path = PathBuf::from(&logs_path).join(format!(
            "{}{}.{}",
            file_date_prefix,
            log_index + 1,
            LOG_FILE_EXT
        ));
        let out = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open(path)
            .expect("Failed to open new log file");

        Self {
            out: Arc::new(Mutex::new(out)),
        }
    }
}

impl Log for AppLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format_log_message(&format!("{}", record.args()));

            print!("{}", message);

            let mut out = self
                .out
                .lock()
                .expect("Failed to acquire a lock on the log file");
            out.write_all(message.as_bytes())
                .expect("Failed to write to the log file");
        }
    }

    fn flush(&self) {
        let mut out = self
            .out
            .lock()
            .expect("Failed to acquire a lock on the log file");
        out.flush().expect("Failed to flush the log file");
    }
}

/// Initializes the global logger.
pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(AppLogger::new()))
        .map(|()| log::set_max_level(LevelFilter::Trace))
}
