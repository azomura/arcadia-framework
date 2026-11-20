/// Public schema describes settings without exposing their current values.
#[derive(Clone, Copy, Debug)]
pub struct ConfigField {
    pub name: &'static str,
    pub description: &'static str,
    pub default: Option<&'static str>,
    pub secret: bool,
}

impl ConfigField {
    pub const fn new(name: &'static str, description: &'static str) -> Self {
        Self { name, description, default: None, secret: false }
    }

    pub const fn with_default(mut self, value: &'static str) -> Self {
        self.default = Some(value);
        self
    }

    pub const fn secret(mut self) -> Self {
        self.secret = true;
        self
    }
}
