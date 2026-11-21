use super::super::{ConfigError, ConfigField};

pub fn validate_email(field: &ConfigField, value: Option<&str>) -> Result<(), ConfigError> {
    let Some(value) = value else { return Ok(()) };
    super::super::validate_text(field, value)?;
    value
        .contains('@')
        .then_some(())
        .ok_or_else(|| ConfigError::new(field.name, "expected an email address"))
}
