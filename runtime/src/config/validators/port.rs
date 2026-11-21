use super::super::{ConfigError, ConfigField};

pub fn validate_port(field: &ConfigField, value: u32) -> Result<u16, ConfigError> {
    (value <= u16::MAX as u32)
        .then_some(value as u16)
        .ok_or_else(|| ConfigError::new(field.name, "expected a port between 0 and 65535"))
}
