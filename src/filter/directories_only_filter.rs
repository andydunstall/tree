use std::path::Path;

use crate::filter::Filter;

pub struct DirectoriesOnlyFilter;

impl DirectoriesOnlyFilter {
    pub fn new() -> DirectoriesOnlyFilter {
        DirectoriesOnlyFilter {}
    }
}

impl Filter for DirectoriesOnlyFilter {
    fn is_ignored(&self, path: &Path) -> bool {
        !path.is_dir()
    }

    fn is_override(&self, _path: &Path) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_dir() {
        let filter = DirectoriesOnlyFilter::new();
        assert!(filter.is_ignored(Path::new("myfile")));
    }

    #[test]
    fn test_dir() {
        let filter = DirectoriesOnlyFilter::new();
        assert!(!filter.is_ignored(Path::new("target")));
    }
}
