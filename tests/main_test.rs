use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_multiple_directories() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("qdir").unwrap();
    cmd.current_dir(temp_path)
        .arg("-m")
        .arg("3")
        .assert()
        .success()
        .stdout(predicates::str::contains("./"));

    let entries: Vec<_> = fs::read_dir(temp_path).unwrap().collect();
    assert_eq!(entries.len(), 3);
}

#[test]
fn test_pet_named_directories() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("qdir").unwrap();
    cmd.current_dir(temp_path)
        .arg("-p")
        .assert()
        .success()
        .stdout(predicates::str::contains("./"));

    let entries: Vec<_> = fs::read_dir(temp_path).unwrap().collect();
    assert_eq!(entries.len(), 1);

    let dir_name = entries[0].as_ref().unwrap().file_name();
    let name_str = dir_name.to_str().unwrap();
    assert!(name_str.chars().all(char::is_alphabetic));
}

#[test]
fn test_named_directories() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("qdir").unwrap();
    cmd.current_dir(temp_path)
        .arg("-n")
        .assert()
        .success()
        .stdout(predicates::str::contains("./"));

    let entries: Vec<_> = fs::read_dir(temp_path).unwrap().collect();
    assert_eq!(entries.len(), 1);

    let dir_name = entries[0].as_ref().unwrap().file_name();
    let name_str = dir_name.to_str().unwrap();
    assert!(name_str.chars().all(char::is_alphabetic));
}

#[test]
fn test_depth_directories() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("qdir").unwrap();
    cmd.current_dir(temp_path)
        .arg("-d")
        .arg("2")
        .assert()
        .success()
        .stdout(predicates::str::contains("./"));

    let entries: Vec<_> = fs::read_dir(temp_path).unwrap().collect();
    assert_eq!(entries.len(), 1);

    let first_entry = &entries[0].as_ref().unwrap().path();
    assert!(first_entry.is_dir());
    let nested_entries: Vec<_> = fs::read_dir(first_entry).unwrap().collect();
    assert_eq!(nested_entries.len(), 1);
}

#[test]
fn test_default_directory_creation() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("qdir").unwrap();
    cmd.current_dir(temp_path)
        .assert()
        .success()
        .stdout(predicates::str::contains("./"));

    let entries: Vec<_> = fs::read_dir(temp_path).unwrap().collect();
    assert_eq!(entries.len(), 1);
}
