#[macro_export]
macro_rules! success {
    ($($args:tt)*) => {
        tracing::info!(log_kind = "SUCCESS", $($args)*)
    };
}

#[macro_export]
macro_rules! critical {
    ($($args:tt)*) => {
        tracing::error!(log_kind = "CRITICAL", $($args)*)
    };
}
