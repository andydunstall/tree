use std::path::{Path, PathBuf};
use std::vec::Vec;

use mockall::automock;

use crate::error::Result;

#[derive(Debug, PartialEq)]
pub struct File {
    pub name: String,
    pub size: u64,
    pub line_count: u64,
}

impl File {
    pub fn new(path: &Path, size: u64, line_count: u64) -> File {
        File {
            name: path_to_filename(path),
            size,
            line_count,
        }
    }
}

#[automock]
pub trait FS {
    fn metadata(&self, dir: &Path) -> Result<File>;
    fn list_dir(&self, dir: &Path) -> Result<Vec<PathBuf>>;
}

fn path_to_filename(dir: &Path) -> String {
    if let Some(file_name) = dir.file_name() {
        // Assume unicode path.
        return file_name.to_str().unwrap().to_string();
    } else {
        return ".".to_string();
    }
}
