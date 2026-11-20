use std::fmt;

#[derive(Debug)]
pub struct ConfigError {
    key: String,
    message: String,
}

impl ConfigError {
    pub fn new(key: impl Into<String>, message: impl Into<String>) -> Self {
        Self { key: key.into(), message: message.into() }
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.key, self.message)
    }
}

impl std::error::Error for ConfigError {}
