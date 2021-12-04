use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::vec::Vec;

use crate::error::{Error, Result};
use crate::fs::{File, FS};

pub struct SystemFS;

impl SystemFS {
    pub fn new() -> SystemFS {
        SystemFS {}
    }
}

impl FS for SystemFS {
    fn metadata(&self, dir: &Path) -> Result<File> {
        Ok(File::new(dir, self.file_size(dir)?, self.line_count(dir)?))
    }

    fn list_dir(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        if !dir.is_dir() {
            return Ok(vec![]);
        }

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

impl SystemFS {
    fn file_size(&self, path: &Path) -> Result<u64> {
        if path.is_dir() {
            Ok(0)
        } else {
            Ok(fs::metadata(path)?.len())
        }
    }

    fn line_count(&self, path: &Path) -> Result<u64> {
        if path.is_dir() {
            return Ok(0);
        }
        let f = fs::File::open(path)?;
        let mut reader = BufReader::with_capacity(1024 * 32, f);
        let mut count = 0;
        loop {
            let len = {
                let buf = reader.fill_buf()?;
                if buf.is_empty() {
                    break;
                }
                count += bytecount::count(&buf, b'\n');
                buf.len()
            };
            reader.consume(len);
        }
        Ok(count as u64)
    }
}
