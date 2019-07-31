#![allow(non_snake_case)]

use cheat::sheet;
use clap::{App, Arg, ArgMatches};

fn main() {
    let args = getArgs();
    let name = args.value_of("NAME").unwrap();
    let s = sheet::Sheet::new(String::from(name));
    s.read();
}

fn getArgs() -> ArgMatches<'static> {
    let matches = App::new("Cheat Sheet Manager")
        .version("0.1.0")
        .author("Zombie110year <zombie110year@outlook.com>")
        .about("works as what the name described.")
        .arg(
            Arg::with_name("NAME")
                .help("which cheat sheet to display")
                .required(true)
                .index(1),
        )
        .get_matches();

    return matches;
}
