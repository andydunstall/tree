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
    pub fn new(name: String, size: u64, line_count: u64) -> File {
        File {
            name,
            size,
            line_count,
        }
    }
}

#[automock]
pub trait FS {
    fn list_dir(&self, dir: &Path) -> Result<Vec<PathBuf>>;
    fn file_size(&self, path: &Path) -> Result<u64>;
    fn line_count(&self, path: &Path) -> Result<u64>;
}
