use std::{
    fmt,
    path::{Path, PathBuf},
};

use super::super::ConfigField;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Environment {
    Test,
    Dev,
    Release,
    Build,
}

impl Environment {
    pub fn current() -> Self {
        match std::env::var("APP_ENV").ok().as_deref() {
            Some("test") => Self::Test,
            Some("build") => Self::Build,
            Some("release") => Self::Release,
            _ if cfg!(test) => Self::Test,
            _ if cfg!(debug_assertions) => Self::Dev,
            _ => Self::Release,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Test => "test",
            Self::Dev => "dev",
            Self::Release => "release",
            Self::Build => "build",
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct ProjectPaths {
    root: PathBuf,
    environment: Environment,
    env_files: Vec<PathBuf>,
}

impl ProjectPaths {
    pub const FIELDS: &'static [ConfigField] = &[];

    pub fn discover() -> Self {
        Self::discover_at(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }

    pub fn discover_at(root: impl Into<PathBuf>) -> Self {
        Self::for_environment(root, Environment::current())
    }

    pub fn for_environment(root: impl Into<PathBuf>, environment: Environment) -> Self {
        let root = root.into();
        let named = root.join(format!(".env.{environment}"));
        let fallback = root.join(".env");
        let env_files = if named.is_file() {
            vec![named]
        } else if fallback.is_file() {
            vec![fallback]
        } else {
            Vec::new()
        };
        Self { root, environment, env_files }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
    pub fn environment(&self) -> Environment {
        self.environment
    }
    pub fn env_files(&self) -> &[PathBuf] {
        &self.env_files
    }
}
