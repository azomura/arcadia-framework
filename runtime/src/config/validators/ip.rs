use std::net::IpAddr;

use super::super::{ConfigError, ConfigField};

pub fn validate_ip(field: &ConfigField, value: &str) -> Result<IpAddr, ConfigError> {
    value.parse().map_err(|_| ConfigError::new(field.name, "expected an IPv4 or IPv6 address"))
}
