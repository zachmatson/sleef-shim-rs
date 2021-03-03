use std::path::{Path, PathBuf};

const BASE_DIR: &'static str = env!("OUT_DIR");

pub fn get_lib_dir() -> PathBuf {
    Path::new(BASE_DIR).join("lib*")
}

pub fn get_header_dir() -> PathBuf {
    Path::new(BASE_DIR).join("include")
}
