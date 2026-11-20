use std::{fs, path::PathBuf};

use arcadia_framework::core::logger::init_global;

fn directory(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("arcadia-{name}-{}", std::process::id()))
}

#[test]
fn init_global_returns_shared_logger() {
    let directory = directory("global");
    let _ = fs::remove_dir_all(&directory);
    let first = init_global(&directory).unwrap();
    let second = init_global(&directory).unwrap();

    assert_eq!(first.path(), second.path());
    assert!(first.path().exists());
}
