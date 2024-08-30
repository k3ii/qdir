use clap::{command, Arg};
use qdir::make_dir;

fn main() {
    let matches = command!()
        .version("0.1.0")
        .author("Jain Ramchurn")
        .about("Quick Directory Generator")
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .value_parser(clap::value_parser!(u8))
                .default_value("0"),
        )
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .value_parser(clap::value_parser!(usize))
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
        .arg(
            Arg::new("tmp")
                .short('t')
                .long("tmp")
                .action(clap::ArgAction::SetTrue)
                .help("Use the system's temporary directory"),
        )
        .get_matches();

    let depth = matches
        .get_one::<u8>("depth")
        .copied()
        .expect("Default depth should be provided");
    let length = matches
        .get_one::<usize>("length")
        .copied()
        .expect("Default length should be provided");

    let use_name = matches.get_flag("name");
    let use_pet = matches.get_flag("pet");
    let use_tmp = matches.get_flag("tmp");

    make_dir(depth, length, use_name, use_pet, use_tmp);
}
