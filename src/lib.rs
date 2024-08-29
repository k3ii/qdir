use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;
use std::path::PathBuf;

pub fn random(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn make_dir(depth: u8, name_length: usize) {
    let mut path = PathBuf::new();
    let depth = if depth == 0 { 1 } else { depth };

    for _ in 0..depth {
        let dir_name = random(name_length);
        path.push(dir_name);
    }
    fs::create_dir_all(&path).expect("Failed to create directory");

    println!("{}", path.display());
}
