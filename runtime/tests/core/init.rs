use std::{fs, path::PathBuf};

use arcadia_framework::core::logger::init;

fn directory(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("arcadia-{name}-{}", std::process::id()))
}

#[test]
fn init_creates_logger_file() {
    let directory = directory("init");
    let _ = fs::remove_dir_all(&directory);
    let logger = init(&directory).unwrap();

    assert!(logger.path().starts_with(&directory));
    assert!(logger.path().exists());
    drop(logger);
    fs::remove_dir_all(directory).unwrap();
}
