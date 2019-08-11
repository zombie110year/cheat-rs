#![allow(non_snake_case)]
//! # search
//!
//! list or search sheets

use crate::{CHEAT_DIR, SHEET_DIR};
use crate::Sheet;
use dirs::home_dir;
use fuzzy_matcher::skim::fuzzy_match;

/// display sheet list
pub fn displaySheets(list: &Vec<Sheet>) {
    let mut max_length: (usize, usize) = (0, 0);
    for item in list.iter() {
        let (name, mtime) = (
            format!("{}", item.name()),
            format!("{}", item.mtime()),
        );
        let length = (name.len(), mtime.len());
        if length.0 > max_length.0 {
            max_length.0 = length.0;
        }
        if length.1 > max_length.1 {
            max_length.1 = length.1;
        }
    }
    println!(
        "{1:<w2$}    {0:<w1$}",
        "Name",
        "Last Modified",
        w1 = max_length.0,
        w2 = max_length.1
    );
    println!(
        "{1:<w2$}    {0:<w1$}",
        "----",
        "-------------",
        w1 = max_length.0,
        w2 = max_length.1
    );
    for item in list.iter() {
        let (name, mtime) = (
            format!("{}", item.name()),
            format!("{}", item.mtime()),
        );
        println!(
            "{1:<w2$}    {0:<w1$}",
            name,
            mtime,
            w1 = max_length.0,
            w2 = max_length.1
        );
    }
}

/// list all existing sheets
pub fn listSheets() -> Vec<Sheet> {
    let mut info: Vec<Sheet> = Vec::new();
    let dir = home_dir().unwrap().join(CHEAT_DIR).join(SHEET_DIR);
    for file in dir
        .read_dir()
        .expect("directory didn't exist: ~/.cheat/sheet")
    {
        match file {
            Ok(entry) => {
                let path = entry.path();
                let filename = path.file_name().unwrap().to_str().unwrap();
                let s = Sheet::new(String::from(filename));
                info.push(s);
            }
            Err(_) => {
                panic!("error in listSheets");
            }
        }
    }
    return info;
}

pub fn searchSheet(pattern: String) -> Vec<Sheet> {
    let list = listSheets();
    let mut target: Vec<Sheet> = Vec::new();
    for item in list {
        let name = item.name();
        if matchSheet(name.as_str(), &pattern.as_str()) {
            target.push(item);
        }
    }
    return target;
}

fn matchSheet(name: &str, pattern: &str) -> bool {
    return fuzzy_match(name, pattern).is_some();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matches() {
        assert!(matchSheet("hello", "hel"));
        assert!(!matchSheet("vector", "vocter"));
        assert!(matchSheet("zombie110year", "Zombie110year"));
    }
}
