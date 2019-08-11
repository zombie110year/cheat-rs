#![allow(non_snake_case)]

use cheat::sheet;
use cheat::search::{displaySheets, searchSheet};
use clap::{self, App, Arg, SubCommand};

fn main() {
    let args = getParser().get_matches();
    match args.subcommand_name() {
        Some("edit") => {
            let name = args
                .subcommand_matches("edit")
                .unwrap()
                .value_of("EDITNAME")
                .unwrap();
            let s = sheet::Sheet::new(String::from(name));
            s.edit();
        },
        Some("read") => {
            let name = args
                .subcommand_matches("read")
                .unwrap()
                .value_of("READNAME")
                .unwrap();
            let s = sheet::Sheet::new(String::from(name));
            s.read();
        },
        Some("list") => {
            displaySheets();
        },
        Some("search") => {

        }
        Some(&_) | None => {
            panic!("SubcommandParseError");
        }
    }
}

fn getParser() -> App<'static, 'static> {
    App::new("Cheat Sheet Manager")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(
            SubCommand::with_name("edit")
                .about("edit target sheet")
                .author(clap::crate_authors!())
                .version(clap::crate_version!())
                .arg(Arg::with_name("EDITNAME").help("which cheat sheet to display")),
        )
        .subcommand(
            SubCommand::with_name("read")
                .about("show target sheet")
                .author(clap::crate_authors!())
                .version(clap::crate_version!())
                .arg(Arg::with_name("READNAME").help("which cheat sheet to display")),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("list all sheets")
                .author(clap::crate_authors!())
                .version(clap::crate_version!()),
        )
        .subcommand(
            SubCommand::with_name("search")
                .about("search sheets match regex pattern")
                .author(clap::crate_authors!())
                .version(clap::crate_version!())
                .arg(Arg::with_name("PATTERN").help("regex pattern")),
        )
}

#[cfg(test)]
#[test]
fn test_getArgs() {
    let parser = getParser();
    let m = parser.get_matches_from(vec!["cheat", "show", "hello"]);
    assert_eq!(m.value_of("NAME").unwrap(), "hello");
    if let Some(sub_m) = m.subcommand_matches("show") {
        sub_m.value_of("NAME");
    }
}
