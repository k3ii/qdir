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
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .action(clap::ArgAction::SetTrue)
                .help("Use names instead of random string"),
        )
        .arg(
            Arg::new("pet")
                .short('p')
                .long("pet")
                .action(clap::ArgAction::SetTrue)
                .help("Use pets instead of random string"),
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

    let use_name = matches.contains_id("name");
    let use_pet = matches.contains_id("pet");

    make_dir(depth, length, use_name, use_pet);
}
