use super::super::{ConfigError, ConfigField};

pub fn validate_bool(field: &ConfigField, value: &str) -> Result<bool, ConfigError> {
    value.parse().map_err(|_| ConfigError::new(field.name, "expected true or false"))
}
