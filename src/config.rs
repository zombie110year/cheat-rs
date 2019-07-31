#![allow(non_snake_case)]
//! # config
//!
//! configures about cheat-rs
//!

use std::collections::hash_map::HashMap;

/// ## Configure
pub struct Configure {
    content: HashMap<&'static str, &'static str>
}

impl Configure {
    /// create a new Configure instance
    /// contains default value:
    ///
    /// ```rust
    /// use crate::cheat::config::Configure;
    /// let c = Configure::new();
    /// #[cfg(target_os = "windows")]
    /// assert_eq!(c.get("EDITOR"), "notepad.exe");
    /// #[cfg(target_os = "linux")]
    /// assert_eq!(c.get("EDITOR"), "vim");
    /// ```
    pub fn new() -> Self {
        let mut map: HashMap<&'static str, &'static str> = HashMap::new();
        // insert default value
        map.insert("EDITOR", getEditor());

        Configure{content: map}
    }
    /// get a value stored in Configure.
    /// copy data and move ownership.
    pub fn get(&self, key: &str) -> &str {
        let value = self.content.get(key).clone();
        match value {
            Some(v) => v,
            None => {
                panic!("CloneError: {}", key);
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn getEditor() -> &'static str {
    "notepad.exe"
}

#[cfg(target_os = "linux")]
fn getEditor() -> &'static str {
    "vim"
}
