use super::super::{ConfigError, ConfigField};

pub fn validate_url(field: &ConfigField, value: Option<&str>) -> Result<(), ConfigError> {
    let Some(value) = value else { return Ok(()) };
    super::super::validate_text(field, value)?;
    (value.starts_with("http://") || value.starts_with("https://"))
        .then_some(())
        .ok_or_else(|| ConfigError::new(field.name, "expected an HTTP(S) URL"))
}
