pub mod config;
pub mod search;
pub mod sheet;

pub const CHEAT_DIR: &'static str = ".cheat";
/// `~/.cheat/sheet` store sheet files
pub const SHEET_DIR: &'static str = "sheet";
/// `~/.cheat/cache` store cache files
pub const CACHE_DIR: &'static str = "cache";

pub fn init() {
    // mkdir
    use std::fs;
    let home = dirs::home_dir().expect("crate/ fn init: dirs::home_dir error");
    let cheat = home.join(CHEAT_DIR);
    let sheet = cheat.join(SHEET_DIR);
    let cache = cheat.join(CACHE_DIR);
    let builder = fs::DirBuilder::new();
    for dir in [cheat, sheet, cache].iter() {
        if !dir.exists() {
            builder
                .create(dir.as_path())
                .expect("crate/ fn init: DirBuilder.create error");
            println!("created dir {}", dir.to_str().unwrap());
        }
    }
}
