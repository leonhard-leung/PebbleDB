use std::path::PathBuf;

pub const VERSION: &str = "0.1.0";

pub fn root() -> PathBuf {
    PathBuf::from(
        std::env::var("LOCALAPPDATA")
            .expect("LOCALAPPDATA environment variable not found")
    )
        .join("PebbleDB")
        .join("data")
}