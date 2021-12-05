use std::path::{Path, PathBuf};
use std::vec::Vec;

use mockall::automock;

#[derive(Debug, PartialEq)]
pub struct RegularFile {
    pub name: String,
    pub size: u64,
    pub line_count: u64,
    pub executable: bool,
    pub accessible: bool,
}

#[derive(Debug, PartialEq)]
pub struct Directory {
    pub name: String,
    pub contents: Vec<PathBuf>,
    pub accessible: bool,
}

#[derive(Debug, PartialEq)]
pub struct Symlink {
    pub name: String,
    pub target: String,
    pub accessible: bool,
}

#[derive(Debug, PartialEq)]
pub enum File {
    RegularFile(RegularFile),
    Directory(Directory),
    Symlink(Symlink),
}

#[automock]
pub trait FS {
    fn open(&self, path: &Path) -> File;
}
