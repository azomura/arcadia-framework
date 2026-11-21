use super::super::{ConfigError, ConfigField};

pub fn validate_choice<'a>(
    field: &ConfigField, value: &'a str, choices: &[&str],
) -> Result<&'a str, ConfigError> {
    choices.contains(&value).then_some(value).ok_or_else(|| {
        ConfigError::new(field.name, format!("expected one of: {}", choices.join(", ")))
    })
}
