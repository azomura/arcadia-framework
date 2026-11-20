use super::{Logger, subscriber::subscriber};

use std::path::Path;

use tracing_subscriber::{EnvFilter, util::SubscriberInitExt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogEnvironment {
    Test,
    Dev,
    Release,
}

impl LogEnvironment {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Test => "test",
            Self::Dev => "dev",
            Self::Release => "release",
        }
    }
}

pub struct LogOptions {
    pub ansi: bool,
    pub environment: LogEnvironment,
}

impl Default for LogOptions {
    fn default() -> Self {
        Self { ansi: std::env::var_os("NO_COLOR").is_none(), environment: environment() }
    }
}

pub fn init(
    directory: impl AsRef<Path>,
) -> Result<Logger, Box<dyn std::error::Error + Send + Sync>> {
    init_with_options(directory, LogOptions::default())
}

pub fn init_with_options(
    directory: impl AsRef<Path>, options: LogOptions,
) -> Result<Logger, Box<dyn std::error::Error + Send + Sync>> {
    let logger = Logger::open(directory.as_ref(), options.environment.as_str())?;
    let level = match options.environment {
        LogEnvironment::Dev => "debug",
        LogEnvironment::Test | LogEnvironment::Release => "info",
    };
    subscriber(logger.writer.clone(), EnvFilter::new(level), options.ansi).try_init()?;
    tracing::info!(
        log_kind = "SUCCESS",
        environment = options.environment.as_str(),
        file = %logger.path().display(),
        "File logging initialized"
    );
    if let Some(path) = logger.path().parent() {
        tracing::debug!(root = %path.display(), "Logger paths resolved");
    }
    Ok(logger)
}

fn environment() -> LogEnvironment {
    if cfg!(test) || std::env::var_os("RUST_TEST_THREADS").is_some() {
        LogEnvironment::Test
    } else if cfg!(debug_assertions) {
        LogEnvironment::Dev
    } else {
        LogEnvironment::Release
    }
}
