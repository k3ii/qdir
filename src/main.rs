use clap::{command, Arg};
use qdir::make_dir;

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .default_value("0"),
        )
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .default_value("6"),
        )
        .get_matches();

    let length = matches
        .get_one::<String>("length")
        .expect("Length is required")
        .parse::<usize>()
        .expect("Length must be a valid number");

    let depth = matches
        .get_one::<String>("depth")
        .expect("Depth is required")
        .parse::<u8>()
        .expect("Depth must be a valid number");

    make_dir(depth, length);
}
