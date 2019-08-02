#![allow(non_snake_case)]
//! # search
//!
//! list or search sheets

use dirs::home_dir;
use chrono::prelude::*;

const CHEAT_DIR: &str = ".cheat";

pub fn listSheets() {
    let list = _listSheets();
    for item in list.iter() {
        println!("{}", item);
    }
}

fn _listSheets() -> Vec<String> {
    let mut info: Vec<String> = Vec::new();
    let dir = home_dir().unwrap().join(CHEAT_DIR);
    for file in dir.read_dir().unwrap() {
        match file {
            Ok(entry) => {
                let metadata = entry.metadata().unwrap();
                let path = entry.path();
                let filename = path.file_name().unwrap()
                    .to_str().unwrap();
                let mtime: DateTime<Local> = DateTime::from(metadata.modified().unwrap());
                let mtime = mtime.format("%Y-%m-%d %H:%M:%S");
                info.push(format!("{} {}", filename, mtime));
            },
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
