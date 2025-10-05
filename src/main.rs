use clap::Command;
use my_cli::{
    commands::{goodbye, greet},
    utils::cli_utils::{clear_terminal, print_elapsed, print_header},
};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    clear_terminal();
    print_header();

    let matches = Command::new("myCli")
        .version("1.0")
        .about("A simple CLI application")
        .subcommand(greet::new_command())
        .subcommand(goodbye::new_command())
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("greet") {
        let name = matches.get_one::<String>("n").unwrap();
        let surname = matches.get_one::<String>("s").unwrap();
        greet::execute(name, surname);
    }

    if let Some(matches) = matches.subcommand_matches("goodbye") {
        let name = matches.get_one::<String>("n").unwrap();
        let surname = matches.get_one::<String>("s").unwrap();
        goodbye::execute(name, surname);
    }

    print_elapsed(start);
}
