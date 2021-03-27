use std::fs;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use crate::error::{Error, Result};
use crate::fs::FS;

pub struct OSFS;

impl OSFS {
    pub fn new() -> OSFS {
        OSFS {}
    }
}

impl FS for OSFS {
    fn list_dir(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut paths = vec![];
        for entry in fs::read_dir(dir)? {
            match entry {
                Ok(entry) => {
                    paths.push(entry.path());
                }
                Err(err) => {
                    return Err(Error::from(err));
                }
            }
        }
        paths.sort();
        Ok(paths)
    }
}
