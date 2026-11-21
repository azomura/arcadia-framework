use std::{ffi::OsString, sync::Arc};

use super::{ConfigError, ConfigValues};

mod project_paths;
mod reader;

pub use project_paths::{Environment, ProjectPaths};

#[derive(Clone)]
pub struct BaseConfig {
    paths: Arc<ProjectPaths>,
    pub(super) values: Arc<ConfigValues>,
}

impl BaseConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_values(std::env::vars_os(), ProjectPaths::discover())
    }

    pub fn from_values(
        values: impl IntoIterator<Item = (OsString, OsString)>, paths: ProjectPaths,
    ) -> Result<Self, ConfigError> {
        if paths.env_files().is_empty() {
            tracing::error!(
                log_kind = "CRITICAL",
                environment = %paths.environment(),
                root = %paths.root().display(),
                "No environment configuration file found; expected .env.{0} or .env",
                paths.environment()
            );
            return Err(ConfigError::new(
                "ENV_FILE",
                "critical: expected environment-specific .env file or .env",
            ));
        }
        let values = ConfigValues::new(values).with_files(paths.env_files())?;
        Ok(Self { paths: Arc::new(paths), values: Arc::new(values) })
    }

    pub fn from_values_at(
        values: impl IntoIterator<Item = (OsString, OsString)>, root: impl Into<std::path::PathBuf>,
    ) -> Result<Self, ConfigError> {
        Self::from_values(values, ProjectPaths::discover_at(root))
    }

    pub fn paths(&self) -> &ProjectPaths {
        &self.paths
    }

    pub fn optional(&self, key: &str) -> Result<Option<String>, ConfigError> {
        self.values.optional(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.values.contains(key)
    }
}
