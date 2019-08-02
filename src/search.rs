#![allow(non_snake_case)]
//! # search
//!
//! list or search sheets

use dirs::home_dir;
use chrono::prelude::*;

const CHEAT_DIR: &str = ".cheat";

pub fn listSheets() {
    let list = _listSheets();
    let mut max_length: (usize, usize) = (0, 0);
    for item in list.iter() {
        let length = (item.0.len(), item.1.len());
        if length.0 > max_length.0 {
            max_length.0 = length.0;
        }
        if length.1 > max_length.1 {
            max_length.1 = length.1;
        }
    }
    println!("{:^w1$}\t{:<w2$}", "Name", "Last Modified", w1=max_length.0, w2=max_length.1);
    println!("{:<w1$}\t{:<w2$}", "----", "-------------", w1=max_length.0, w2=max_length.1);
    for item in list.iter() {
        println!("{:<w1$}\t{:<w2$}", item.0, item.1, w1=max_length.0, w2=max_length.1);
    }
}

fn _listSheets() -> Vec<(String, String)> {
    let mut info: Vec<(String, String)> = Vec::new();
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
                info.push((format!("{}", filename), format!("{}", mtime)));
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
