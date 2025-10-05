use clap::{Arg, Command};
use spinach::Spinner;

pub fn new_command() -> Command {
    Command::new("goodbye")
        .about("Executes goodbye")
        .arg(
            Arg::new("n")
                .short('n')
                .long("name")
                .required(true)
                .help("Name"),
        )
        .arg(
            Arg::new("s")
                .short('s')
                .long("surname")
                .required(true)
                .help("Surname"),
        )
}

pub fn execute(name: &str, surname: &str) {
    let spinner = Spinner::new("getting lists...").start();
    spinner.text("  DONE").success();
    println!("Goodbye, {} {}!", name, surname);
}
