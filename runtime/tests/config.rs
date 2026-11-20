use std::{
    ffi::OsString,
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use arcadia_framework::config::{
    ConfigError, ConfigField, ConfigReader, ConfigSection, ConfigValues, validate_text,
};

struct Reader(ConfigValues);

impl ConfigReader for Reader {
    fn read(&self, field: &ConfigField) -> Result<String, ConfigError> {
        self.0.read(field)
    }

    fn contains(&self, key: &str) -> bool {
        self.0.contains(key)
    }
}

struct Section {
    port: u32,
}

impl ConfigSection for Section {
    type Source = Reader;

    const FIELDS: &'static [ConfigField] = &[ConfigField::new("PORT", "HTTP port")];

    fn build(source: &Self::Source) -> Result<Self, ConfigError> {
        Ok(Self { port: source.number(&Self::FIELDS[0], 65_535)? })
    }

    fn validate(&self) -> Result<(), ConfigError> {
        (self.port > 0).then_some(()).ok_or_else(|| ConfigError::new("PORT", "must be positive"))
    }
}

fn values(entries: &[(&str, &str)]) -> ConfigValues {
    ConfigValues::new(
        entries.iter().map(|(key, value)| (OsString::from(key), OsString::from(value))),
    )
}

#[test]
fn config_values_read_optional_and_merge_files() {
    let root = std::env::temp_dir();
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = root.join(format!("arcadia-config-{suffix}.env"));
    fs::write(&path, "PORT=8080\nHOST=127.0.0.1\n").unwrap();

    let merged = values(&[("PORT", "9000")]).with_files(&[path.clone()]).unwrap();
    assert_eq!(merged.optional("HOST").unwrap().as_deref(), Some("127.0.0.1"));
    assert_eq!(merged.read(&ConfigField::new("PORT", "port")).unwrap(), "9000");
    assert!(merged.contains("HOST"));
    assert_eq!(merged.optional("MISSING").unwrap(), None);

    fs::remove_file(path).unwrap();
}

#[test]
fn config_reader_parses_supported_types_and_defaults() {
    let reader = Reader(values(&[
        ("PORT", "8080"),
        ("COUNT", "1, 2, 3"),
        ("DATE", "2026-11-20"),
        ("SECONDS", "12"),
    ]));
    let port = ConfigField::new("PORT", "port");
    let count = ConfigField::new("COUNT", "count");
    let date = ConfigField::new("DATE", "date");
    let seconds = ConfigField::new("SECONDS", "seconds");
    let missing = ConfigField::new("MISSING", "missing").with_default("fallback");

    assert_eq!(reader.parse::<u16>(&port, "invalid").unwrap(), 8080);
    assert_eq!(reader.comma_separated::<u32>(&count, "invalid").unwrap(), vec![1, 2, 3]);
    assert!(reader.date(&date).unwrap().is_some());
    assert_eq!(reader.number(&port, 9000).unwrap(), 8080);
    assert_eq!(reader.duration(&seconds, 60).unwrap().as_secs(), 12);
    assert_eq!(reader.optional_read(&missing).unwrap(), None);
    assert_eq!(reader.read(&missing).unwrap(), "fallback");
}

#[test]
fn config_rejects_invalid_values_and_validates_sections() {
    let reader = Reader(values(&[("PORT", "0"), ("DATE", "invalid"), ("BAD", "nope")]));
    let port = ConfigField::new("PORT", "port");
    let date = ConfigField::new("DATE", "date");
    let bad = ConfigField::new("BAD", "bad");
    let text = ConfigField::new("NAME", "name");

    assert!(reader.number(&port, 100).is_err());
    assert!(reader.date(&date).is_err());
    assert!(reader.parse::<u16>(&bad, "invalid").is_err());
    assert!(validate_text(&text, " ").is_err());
    assert!(validate_text(&text, "arcadia").is_ok());
    assert!(Section::load(&Reader(values(&[("PORT", "8080")]))).is_ok());
}

#[test]
fn config_schema_builders_and_errors_are_readable() {
    let field = ConfigField::new("TOKEN", "secret token").with_default("dev").secret();
    assert_eq!(field.name, "TOKEN");
    assert_eq!(field.description, "secret token");
    assert_eq!(field.default, Some("dev"));
    assert!(field.secret);

    let error = ConfigError::new("TOKEN", "is missing");
    assert_eq!(error.key(), "TOKEN");
    assert_eq!(error.to_string(), "TOKEN: is missing");
}
