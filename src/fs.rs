use std::path::{Path, PathBuf};
use std::vec::Vec;

use mockall::automock;

use crate::error::Result;

#[automock]
pub trait FS {
    fn list_dir(&self, dir: &Path) -> Result<Vec<PathBuf>>;
}
