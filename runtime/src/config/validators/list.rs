use super::super::{ConfigError, ConfigField};

pub fn validate_non_empty_list<T>(field: &ConfigField, values: &[T]) -> Result<(), ConfigError> {
    (!values.is_empty())
        .then_some(())
        .ok_or_else(|| ConfigError::new(field.name, "must contain at least one value"))
}
