use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{filter, prelude::*};

pub struct Logger {
    file_path: PathBuf,
}

// https://stackoverflow.com/questions/70013172/how-to-use-the-tracing-library
impl Logger {
    pub fn new(file_path: impl AsRef<Path>) -> Logger {
        let file_path = file_path.as_ref().to_path_buf();
        let logger = Logger { file_path };
        logger.set_rust_log_variable();
        logger.create_log_directory();
        logger.set_tracing_subscribers();
        logger
    }

    fn set_rust_log_variable(&self) {
        std::env::set_var(
            "RUST_LOG",
            std::env::var("RUST_LOG").unwrap_or("info".to_string()),
        );
    }

    fn create_log_directory(&self) {
        let log_directory = self.file_path.parent().unwrap_or_else(|| {
            panic!(
                "Error getting parent directory of file: \"{}\"",
                self.file_path.display()
            )
        });
        if !log_directory.exists() {
            match fs::create_dir_all(log_directory) {
                Ok(_) => {}
                Err(error) => panic!(
                    "Error creating directory: \"{}\": {}",
                    log_directory.display(),
                    error
                ),
            }
        }
    }

    fn create_log_file(&self) -> fs::File {
        let file = fs::File::create(&self.file_path);
        match file {
            Ok(file) => file,
            Err(error) => panic!(
                "Error creating file: \"{}\": {}",
                self.file_path.display(),
                error
            ),
        }
    }

    ///  Set up the tracing subscribers.
    ///
    /// By default the `info`, `warn`, and `error` events will be seen by both the
    /// stdout log layer and the debug log file layer.
    /// While the `debug` event will only be seen by the debug log file layer.
    ///
    /// If a `RUST_LOG` environment variable is set, the `env_filter` layer will
    /// take it into account.
    /// But the `stdout_log` layer will only log events with a level greater than or equal to
    /// `INFO`.
    fn set_tracing_subscribers(&self) {
        // A layer that logs events to stdout.
        let stdout_log = tracing_subscriber::fmt::layer().compact().without_time(); // .pretty();

        // A layer that logs events to a file.
        let file = self.create_log_file();
        let debug_log = tracing_subscriber::fmt::layer()
            .with_writer(Arc::new(file))
            .with_ansi(false);

        // A filter that takes the `RUST_LOG` environment variable into account.
        let env_filter = EnvFilter::from_default_env();

        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                stdout_log
                    // Add an `INFO` filter to the stdout logging layer
                    .with_filter(filter::LevelFilter::INFO)
                    // Combine the filtered `stdout_log` layer with the
                    // `debug_log` layer, producing a new `Layered` layer.
                    .and_then(debug_log),
            )
            .init();
    }
}
