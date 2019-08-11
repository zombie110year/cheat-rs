#![allow(non_snake_case)]
//! # search
//!
//! list or search sheets

use crate::{CHEAT_DIR, SHEET_DIR};
use chrono::prelude::*;
use dirs::home_dir;

pub fn displaySheets() {
    let list = listSheets();
    let mut max_length: (usize, usize) = (0, 0);
    for item in list.iter() {
        let (name, mtime) = (
            format!("{}", item.0),
            format!("{}", item.1.format("%Y-%m-%d %H:%M:%S")),
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
            format!("{}", item.0),
            format!("{}", item.1.format("%Y-%m-%d %H:%M:%S")),
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

fn listSheets() -> Vec<(String, DateTime<Local>)> {
    let mut info: Vec<(String, DateTime<Local>)> = Vec::new();
    let dir = home_dir().unwrap().join(CHEAT_DIR).join(SHEET_DIR);
    for file in dir
        .read_dir()
        .expect("directory didn't exist: ~/.cheat/sheet")
    {
        match file {
            Ok(entry) => {
                let metadata = entry.metadata().unwrap();
                let path = entry.path();
                let filename = path.file_name().unwrap().to_str().unwrap();
                let mtime: DateTime<Local> = DateTime::from(metadata.modified().unwrap());
                info.push((format!("{}", filename), mtime));
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
