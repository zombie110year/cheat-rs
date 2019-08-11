#![allow(non_snake_case)]

use cheat::init;
use cheat::search::{displaySheets, listSheets, searchSheet};
use cheat::sheet;
use clap::{self, App, Arg, SubCommand};

fn main() {
    init();
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
        }
        Some("read") => {
            let name = args
                .subcommand_matches("read")
                .unwrap()
                .value_of("READNAME")
                .unwrap();
            let s = sheet::Sheet::new(String::from(name));
            s.read();
        }
        Some("list") => {
            let list = listSheets();
            displaySheets(&list);
        }
        Some("search") => {
            let pattern = args
                .subcommand_matches("search")
                .unwrap()
                .value_of("PATTERN")
                .unwrap();
            let result = searchSheet(String::from(pattern));
            displaySheets(&result);
        }
        Some(&_) | None => (),
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
    let m = parser.get_matches_from(vec!["cheat", "read", "hello"]);
    assert_eq!(m.subcommand_name(), Some("read"));
    let a = m.subcommand_matches("read").unwrap().value_of("READNAME");
    assert_eq!(a, Some("hello"));
}
