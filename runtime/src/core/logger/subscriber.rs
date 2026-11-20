use super::formatter::ColorFormatter;

use std::io;

use tracing::Subscriber;
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};

pub(super) fn subscriber(
    writer: NonBlocking, filter: EnvFilter, ansi: bool,
) -> impl Subscriber + Send + Sync {
    let console = tracing_subscriber::fmt::layer()
        .with_ansi(ansi)
        .event_format(ColorFormatter)
        .with_writer(io::stderr);
    let file = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .event_format(ColorFormatter)
        .with_writer(writer);

    tracing_subscriber::registry().with(console).with(file).with(filter)
}
