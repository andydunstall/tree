use std::path::{Path, PathBuf};
use std::vec::Vec;

use mockall::automock;

use crate::error::Result;

#[automock]
pub trait FS {
    fn list_dir(&self, dir: &Path) -> Result<Vec<PathBuf>>;
    fn file_size(&self, path: &Path) -> Result<u64>;
    fn line_count(&self, path: &Path) -> Result<u64>;
}
