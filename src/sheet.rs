#![allow(dead_code)]
#![allow(non_snake_case)]
/// # Sheet
///
/// Discover and Read sheet file.
///
/// The sheet file stored at ~/.cheat/ with out extension name
/// and use Markdown Syntax.
///
use dirs::home_dir;
use std::path::PathBuf;

/// where to store cheat sheets' files
const CHEAT_DIR: &str = ".cheat";

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
}

impl Sheet {
    /// return a new Sheet instance, initialize with `name` argument
    ///
    /// ### Example
    ///
    /// ```rust
    /// use crate::cheat::sheet::Sheet;
    /// let s = Sheet::new(String::from("no-one-will-call-this-name.jfksjdkfksdjkfjkadf"));
    /// ```
    pub fn new(name: String) -> Self {
        return Sheet { name };
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
        let home: PathBuf = match home_dir() {
            Some(p) => p,
            None => {
                panic!("Cannot get HOME directory's path!");
            }
        };
        let path = home.join(CHEAT_DIR).join(self.name());
        return path;
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
        let cheat_dir = home_dir().expect("dirs::home_dir error").join(CHEAT_DIR);
        assert_eq!(
            s.path(),
            cheat_dir.join("no-one-will-call-this-name.jfksjdkfksdjkfjkadf")
        );
    }
}
