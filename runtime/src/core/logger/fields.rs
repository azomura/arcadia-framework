use std::fmt;

use tracing::field::Visit;

#[derive(Default)]
pub(super) struct EventFields {
    pub(super) kind: String,
    pub(super) message: String,
    pub(super) details: String,
}

impl Visit for EventFields {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "log_kind" => self.kind = value.to_owned(),
            "message" => self.message = value.to_owned(),
            _ => self.record_debug(field, &value),
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        use fmt::Write;

        if field.name() == "message" {
            self.message = format!("{value:?}");
        } else {
            let _ = write!(self.details, " {}={value:?}", field.name());
        }
    }
}
