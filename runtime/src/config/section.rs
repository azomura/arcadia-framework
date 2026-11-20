use super::{ConfigError, ConfigField};

/// Common configuration contract: construction, schema and validation.
pub trait ConfigSection: Sized {
    type Source;
    const FIELDS: &'static [ConfigField];

    fn build(base: &Self::Source) -> Result<Self, ConfigError>;
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }

    fn load(base: &Self::Source) -> Result<Self, ConfigError> {
        let section = Self::build(base)?;
        section.validate()?;
        Ok(section)
    }
}
