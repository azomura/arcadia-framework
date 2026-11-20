use super::fields::EventFields;

use std::fmt;

use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{
    fmt::{
        FmtContext, FormatEvent, FormatFields,
        format::Writer,
        time::{FormatTime, SystemTime},
    },
    registry::LookupSpan,
};

pub(super) struct ColorFormatter;

impl<S, N> FormatEvent<S, N> for ColorFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self, context: &FmtContext<'_, S, N>, mut writer: Writer<'_>, event: &Event<'_>,
    ) -> fmt::Result {
        let mut fields = EventFields::default();
        event.record(&mut fields);

        let (label, color) = match (event.metadata().level(), fields.kind.as_str()) {
            (&Level::ERROR, "CRITICAL") => ("CRITICAL", "1;97;41"),
            (&Level::INFO, "SUCCESS") => ("SUCCESS", "32"),
            (&Level::ERROR, _) => ("ERROR", "31"),
            (&Level::WARN, _) => ("WARN", "33"),
            (&Level::INFO, _) => ("INFO", "36"),
            (&Level::DEBUG, _) => ("DEBUG", "35"),
            (&Level::TRACE, _) => ("TRACE", "90"),
        };

        if writer.has_ansi_escapes() {
            write!(writer, "\x1b[{color}m")?;
        }
        SystemTime.format_time(&mut writer)?;
        write!(writer, " {label} [{}]", event.metadata().target())?;
        if let Some(scope) = context.event_scope() {
            for span in scope.from_root() {
                write!(writer, " [{}]", span.name())?;
            }
        }
        write!(writer, " {}{}", fields.message, fields.details)?;
        if writer.has_ansi_escapes() {
            write!(writer, "\x1b[0m")?;
        }
        writeln!(writer)
    }
}
