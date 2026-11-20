use super::{ConfigError, ConfigField};

pub fn validate_text(field: &ConfigField, value: &str) -> Result<(), ConfigError> {
    if value.trim().is_empty() {
        return Err(ConfigError::new(field.name, "must not be empty"));
    }

    Ok(())
}
