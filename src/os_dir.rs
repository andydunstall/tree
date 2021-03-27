use crate::error::{Error, Result};
use std::fs::ReadDir;
use std::path::PathBuf;

pub struct OSDir {
    dir: ReadDir,
}

impl OSDir {
    pub fn new(dir: ReadDir) -> OSDir {
        OSDir { dir: dir }
    }
}

impl Iterator for OSDir {
    type Item = Result<PathBuf>;

    fn next(&mut self) -> Option<Result<PathBuf>> {
        if let Some(entry) = self.dir.next() {
            match entry {
                Ok(entry) => Some(Ok(entry.path())),
                Err(err) => Some(Err(Error::from(err))),
            }
        } else {
            None
        }
    }
}
