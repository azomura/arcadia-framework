use std::{fs, path::PathBuf};

use arcadia_framework::core::logger::{LogEnvironment, LogOptions, init_with_options};

fn test_directory() -> PathBuf {
    std::env::temp_dir().join(format!(
        "arcadia-logger-{}-{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ))
}

#[test]
fn logger_creates_unique_environment_file_without_ansi() {
    let directory = test_directory();
    let _ = fs::remove_dir_all(&directory);
    let logger = init_with_options(
        &directory,
        LogOptions { ansi: false, environment: LogEnvironment::Test },
    )
    .unwrap();
    let path = logger.path().to_owned();
    assert!(path.file_name().unwrap().to_string_lossy().contains("_test_"));
    assert!(path.exists());

    tracing::info!(log_kind = "SUCCESS", answer = 42, "logger is ready");
    drop(logger);
    let contents = fs::read_to_string(path).unwrap();
    assert!(contents.contains("SUCCESS"));
    assert!(contents.contains("logger is ready"));
    assert!(!contents.contains("\u{1b}["));
    fs::remove_dir_all(directory).unwrap();
}
