use std::fs;
use std::path::Path;

use crate::error::Result;
use crate::fs::FS;
use crate::os_dir::OSDir;

pub struct OSFS;

impl OSFS {
    pub fn new() -> OSFS {
        OSFS {}
    }
}

impl FS for OSFS {
    fn list_dir(&self, dir: &Path) -> Result<OSDir> {
        Ok(OSDir::new(fs::read_dir(dir)?))
    }
}
