use super::super::{ConfigError, ConfigField};

pub fn validate_u32_range(
    field: &ConfigField, value: u32, minimum: u32, maximum: u32,
) -> Result<u32, ConfigError> {
    (minimum..=maximum).contains(&value).then_some(value).ok_or_else(|| {
        ConfigError::new(field.name, format!("must be between {minimum} and {maximum}"))
    })
}
