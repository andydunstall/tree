use std::path::{Path, PathBuf};
use std::vec::Vec;

use crate::error::Result;

pub trait FS {
    fn list_dir(&self, dir: &Path) -> Result<Vec<PathBuf>>;
}
