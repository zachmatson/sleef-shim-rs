use std::path::{Path, PathBuf};

pub fn get_header_dir() -> PathBuf {
    Path::new(env!("OUT_DIR")).join("include")
}
