use super::super::{ConfigError, ConfigField};

pub fn validate_optional_text(
    field: &ConfigField, value: Option<&str>,
) -> Result<(), ConfigError> {
    value.map_or(Ok(()), |value| super::super::validate_text(field, value))
}
