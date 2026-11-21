use std::{str::FromStr, time::Duration};

use time::Date;

use super::BaseConfig;
use crate::config::{ConfigError, ConfigField, ConfigReader};

impl ConfigReader for BaseConfig {
    fn read(&self, field: &ConfigField) -> Result<String, ConfigError> {
        self.values.read(field)
    }

    fn contains(&self, key: &str) -> bool {
        self.contains(key)
    }
}

impl BaseConfig {
    pub fn read(&self, field: &ConfigField) -> Result<String, ConfigError> {
        ConfigReader::read(self, field)
    }

    pub fn optional_read(&self, field: &ConfigField) -> Result<Option<String>, ConfigError> {
        ConfigReader::optional_read(self, field)
    }

    pub fn parse<T: FromStr>(&self, field: &ConfigField, expected: &str) -> Result<T, ConfigError> {
        ConfigReader::parse(self, field, expected)
    }

    pub fn parse_or<T: FromStr>(
        &self, field: &ConfigField, expected: &str, default: T,
    ) -> Result<T, ConfigError> {
        ConfigReader::parse_or(self, field, expected, default)
    }

    pub fn comma_separated<T: FromStr>(
        &self, field: &ConfigField, expected: &str,
    ) -> Result<Vec<T>, ConfigError> {
        ConfigReader::comma_separated(self, field, expected)
    }

    pub fn date(&self, field: &ConfigField) -> Result<Option<Date>, ConfigError> {
        ConfigReader::date(self, field)
    }

    pub fn duration(&self, field: &ConfigField, max_seconds: u32) -> Result<Duration, ConfigError> {
        ConfigReader::duration(self, field, max_seconds)
    }
}
