#![allow(non_snake_case)]
//! # search
//!
//! list or search sheets

use crate::{CHEAT_DIR, SHEET_DIR};
use crate::Sheet;
use dirs::home_dir;

pub fn displaySheets() {
    let list = listSheets();
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

fn listSheets() -> Vec<Sheet> {
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

pub fn searchSheet() -> Vec<String> {
    vec![]
}
