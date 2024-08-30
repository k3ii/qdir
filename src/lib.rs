use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;
use std::path::PathBuf;

pub mod name;
pub mod pet;

pub use name::get_random_name;
pub use pet::get_random_pet;

pub fn random(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn make_dir(depth: u8, name_length: usize, name: bool, pet: bool, tmp: bool) {
    let mut path = if tmp {
        std::env::temp_dir()
    } else {
        PathBuf::from(".")
    };

    let depth = if depth == 0 { 1 } else { depth };

    for _ in 0..depth {
        let dir_name = if name {
            get_random_name().unwrap().to_string()
        } else if pet {
            get_random_pet().unwrap().to_string()
        } else {
            random(name_length)
        };
        path.push(dir_name);
    }
    fs::create_dir_all(&path).expect("Failed to create directory");

    println!("{}", path.display());

    let name = get_random_name().unwrap_or_else(|| String::from("default_name"));
    let pet = get_random_pet()
        .unwrap_or_else(|| "default_pet")
        .to_string();

    println!("Name: {}", name);
    println!("Pet: {}", pet);
}
