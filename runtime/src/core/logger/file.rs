use std::{
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
};

use time::{OffsetDateTime, macros::format_description};
use tracing_appender::non_blocking::{NonBlocking, NonBlockingBuilder, WorkerGuard};

pub struct Logger {
    pub(super) writer: NonBlocking,
    path: PathBuf,
    _guard: WorkerGuard,
}

impl Logger {
    pub(super) fn open(
        directory: &Path, environment: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        fs::create_dir_all(directory)?;

        let timestamp = OffsetDateTime::now_utc().format(format_description!(
            "[year]-[month]-[day]_[hour]-[minute]-[second]_[subsecond digits:9]Z"
        ))?;
        let path = directory.join(format!("{timestamp}_{environment}_{}.log", std::process::id()));
        let file = OpenOptions::new().write(true).create_new(true).open(&path)?;
        let (writer, guard) = NonBlockingBuilder::default().lossy(false).finish(file);

        Ok(Self { writer, path, _guard: guard })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
