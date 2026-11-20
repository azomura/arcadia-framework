use std::{str::FromStr, time::Duration};

use time::{Date, macros::format_description};

use super::{ConfigError, ConfigField};

pub trait ConfigReader {
    fn read(&self, field: &ConfigField) -> Result<String, ConfigError>;
    fn contains(&self, key: &str) -> bool;

    fn optional_read(&self, field: &ConfigField) -> Result<Option<String>, ConfigError> {
        self.contains(field.name).then(|| self.read(field)).transpose()
    }

    fn parse<T: FromStr>(&self, field: &ConfigField, expected: &str) -> Result<T, ConfigError> {
        self.read(field)?.parse().map_err(|_| ConfigError::new(field.name, expected))
    }

    fn parse_or<T: FromStr>(
        &self, field: &ConfigField, expected: &str, default: T,
    ) -> Result<T, ConfigError> {
        self.optional_read(field)?
            .map(|value| value.parse().map_err(|_| ConfigError::new(field.name, expected)))
            .transpose()
            .map(|value| value.unwrap_or(default))
    }

    fn comma_separated<T: FromStr>(
        &self, field: &ConfigField, expected: &str,
    ) -> Result<Vec<T>, ConfigError> {
        self.read(field)?
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.parse().map_err(|_| ConfigError::new(field.name, expected)))
            .collect()
    }

    fn date(&self, field: &ConfigField) -> Result<Option<Date>, ConfigError> {
        self.optional_read(field)?
            .map(|value| {
                Date::parse(&value, format_description!("[year]-[month]-[day]"))
                    .map_err(|_| ConfigError::new(field.name, "expected a valid date: YYYY-MM-DD"))
            })
            .transpose()
    }

    fn number(&self, field: &ConfigField, max: u32) -> Result<u32, ConfigError> {
        let value = self.parse::<u32>(field, "expected a positive integer")?;
        match value == 0 || value > max {
            true => Err(ConfigError::new(field.name, format!("must be between 1 and {max}"))),
            false => Ok(value),
        }
    }

    fn duration(&self, field: &ConfigField, max_seconds: u32) -> Result<Duration, ConfigError> {
        Ok(Duration::from_secs(self.number(field, max_seconds)?.into()))
    }
}
