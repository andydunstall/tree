use std::path::Path;

use crate::error::Result;
use crate::os_dir::OSDir;

pub trait FS {
    fn list_dir(&self, dir: &Path) -> Result<OSDir>; // TODO(AD) Return trait
}
