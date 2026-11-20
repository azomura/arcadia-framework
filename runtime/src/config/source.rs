use std::{collections::HashMap, ffi::OsString, path::PathBuf};

use super::{ConfigError, ConfigField};

#[derive(Clone, Default)]
pub struct ConfigValues(HashMap<OsString, OsString>);

impl ConfigValues {
    pub fn new(values: impl IntoIterator<Item = (OsString, OsString)>) -> Self {
        Self(values.into_iter().collect())
    }

    pub fn with_files(self, files: &[PathBuf]) -> Result<Self, ConfigError> {
        let mut merged = HashMap::new();

        for file in files {
            let entries = dotenvy::from_path_iter(file)
                .map_err(|_| ConfigError::new("ENV_FILE", "cannot read env file"))?;
            for entry in entries {
                let pair = entry.map_err(|_| ConfigError::new("ENV_FILE", "invalid syntax"))?;
                merged.insert(OsString::from(pair.0), OsString::from(pair.1));
            }
        }

        merged.extend(self.0);

        Ok(Self(merged))
    }

    pub fn optional(&self, key: &str) -> Result<Option<String>, ConfigError> {
        self.0
            .get(&OsString::from(key))
            .map(|value| {
                value
                    .to_str()
                    .map(str::to_owned)
                    .ok_or_else(|| ConfigError::new(key, "must contain valid Unicode"))
            })
            .transpose()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.0.contains_key(&OsString::from(key))
    }

    pub fn read(&self, field: &ConfigField) -> Result<String, ConfigError> {
        self.optional(field.name)?
            .or_else(|| field.default.map(str::to_owned))
            .ok_or_else(|| ConfigError::new(field.name, "required value is missing"))
    }
}
