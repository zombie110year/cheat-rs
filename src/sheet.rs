#![allow(dead_code)]
#![allow(non_snake_case)]
use super::config::Configure;
use crate::{CHEAT_DIR, SHEET_DIR};
use chrono::prelude::*;
/// # Sheet
///
/// Discover and Read sheet file.
///
/// The sheet file stored at ~/.cheat/sheet with out extension name
/// and use Markdown Syntax.
///
use dirs::home_dir;
use std::fs::read_to_string;
use std::fs::write;
use std::path::PathBuf;
use std::process::Command;

/// ## Sheet
///
/// The struct of Sheet
///
/// ### Examples
///
/// ```rust
/// use crate::cheat::sheet::Sheet;
/// // initialize a instance
/// let s = Sheet::new(String::from("no-one-will-call-this-name.jfksjdkfksdjkfjkadf"));
/// // you can get its name as a ref
/// assert_eq!(s.name(), &String::from("no-one-will-call-this-name.jfksjdkfksdjkfjkadf"));
/// // and check if it exists:
/// assert_eq!(s.exists(), false);
/// // and get its path
/// s.path();
/// ```
#[derive(Debug)]
pub struct Sheet {
    name: String,
    mtime: DateTime<Local>,
}

impl Sheet {
    /// return a new Sheet instance, initialize with `name` argument.
    /// if sheet file exists, read its metadata and set to instance,
    /// else init with values:
    ///
    /// | field | value |
    /// |:-----:|:-----:|
    /// | `name` | `name` |
    /// | `mtime` | `chrono::Local::now()` |
    ///
    /// ### Example
    ///
    /// ```rust
    /// use crate::cheat::sheet::Sheet;
    /// let s = Sheet::new(String::from("no-one-will-call-this-name.jfksjdkfksdjkfjkadf"));
    /// ```
    pub fn new(name: String) -> Self {
        let tmp = Sheet {name: name.clone(), mtime: Local::now()};
        if tmp.exists() {
            let meta = tmp
            .path()
            .metadata()
            .expect("crate::sheet/ impl Sheet / fn exists: cannot read file metadata");

            let mtime = DateTime::from(meta.modified().expect("crate::sheet/ impl Sheet / fn exists: cannot transfer SystemTime to DateTime<Local>"));
            return Sheet {name: name.clone(), mtime};
        }
        return tmp;
    }
    /// get reference of Sheet's `name`
    ///
    /// ### Example
    ///
    /// ```rust
    /// use crate::cheat::sheet::Sheet;
    /// let s = Sheet::new(String::from("helloworld"));
    /// assert_eq!(s.name(), &String::from("helloworld"));
    /// ```
    pub fn name(&self) -> &String {
        return &self.name;
    }
    /// string of mtime
    ///
    /// ### Example
    ///
    /// ```rust
    /// use cheat::sheet::Sheet;
    /// use chrono::prelude::*;
    /// let s = Sheet::new(String::from("no-one-call-this-name.faklsdgngihaqeutb"));
    /// assert_eq!(s.mtime(), format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S")));
    /// ```
    pub fn mtime(&self) -> String {
        return format!("{}", self.mtime.format("%Y-%m-%d %H:%M:%S"));
    }
    /// check weather the Sheet's file exists
    ///
    /// ### Example
    ///
    /// ```rust
    /// use crate::cheat::sheet::Sheet;
    /// let s = Sheet::new(String::from("no-one-will-call-this-name.jfksjdkfksdjkfjkadf"));
    /// assert_eq!(s.exists(), false);
    /// ```
    pub fn exists(&self) -> bool {
        return self.path().exists();
    }
    /// get path of the Sheet's file
    ///
    /// ### Example
    ///
    /// ```rust
    /// use crate::cheat::sheet::Sheet;
    /// use dirs::home_dir;
    /// let CHEAT_DIR = ".cheat";
    /// let s = Sheet::new(String::from("no-one-will-call-this-name.jfksjdkfksdjkfjkadf"));
    /// let cheat_dir = home_dir().expect("dirs::home_dir error").join(CHEAT_DIR);
    /// assert_eq!(s.path(), cheat_dir.join(s.name()));
    /// ```
    pub fn path(&self) -> PathBuf {
        let home: PathBuf = home_dir().expect("crate::sheet/ impl Sheet / fn path: home_dir error");
        let path = home.join(CHEAT_DIR).join(SHEET_DIR).join(self.name());
        return path;
    }
    /// edit sheet's file(todo)
    ///
    /// It will call the editor which depends on the system.
    /// In **Windows**, It's *notepad.exe*,
    /// In **Linux**, It's *vim*
    ///
    /// Wherever you edit, it will save content as utf-8 string.
    pub fn edit(&self) {
        if !self.exists() {
            write(self.path(), b"").expect("FileWriteError");
            println!(
                "file created: {}",
                self.path()
                    .to_str()
                    .expect("can't transfer PathBuf to &str")
            );
        }
        let config = Configure::new();
        Command::new(config.get("EDITOR"))
            .arg(self.path())
            .status()
            .expect("failed to execute subprocess");
    }
    /// read sheet's content
    pub fn read(&self) {
        if self.exists() {
            println!("{}", read_to_string(self.path()).expect("read file error"));
        } else {
            println!("FileNotFoundError: {}", self.name());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sheet() {
        let s = Sheet::new(String::from(
            "no-one-will-call-this-name.jfksjdkfksdjkfjkadf",
        ));
        assert_eq!(
            s.name(),
            &String::from("no-one-will-call-this-name.jfksjdkfksdjkfjkadf")
        );
        assert_eq!(s.exists(), false);
    }
    #[test]
    fn test_path() {
        let s = Sheet::new(String::from(
            "no-one-will-call-this-name.jfksjdkfksdjkfjkadf",
        ));
        let cheat_dir = home_dir()
            .expect("dirs::home_dir error")
            .join(CHEAT_DIR)
            .join(SHEET_DIR);
        assert_eq!(
            s.path(),
            cheat_dir.join("no-one-will-call-this-name.jfksjdkfksdjkfjkadf")
        );
    }
}
