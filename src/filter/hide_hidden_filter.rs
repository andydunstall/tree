use std::path::Path;

use crate::filter::Filter;

pub struct HideHiddenFilter;

impl HideHiddenFilter {
    pub fn new() -> HideHiddenFilter {
        HideHiddenFilter {}
    }
}

impl HideHiddenFilter {
    fn is_hidden(path: &Path) -> bool {
        if let Some(file_name) = path.file_name() {
            if let Some(file_name) = file_name.to_str() {
                file_name.starts_with(".")
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Filter for HideHiddenFilter {
    fn is_ignored(&self, path: &Path) -> bool {
        let mut path = path;
        loop {
            if HideHiddenFilter::is_hidden(path) {
                return true;
            }

            // Keep walking up the path until a hidden file found.
            if let Some(parent) = path.parent() {
                path = parent;
            } else {
                return false;
            }
        }
    }

    fn is_override(&self, _path: &Path) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_hidden() {
        let filter = HideHiddenFilter::new();
        assert!(!filter.is_ignored(Path::new("myfile")));
    }

    #[test]
    fn test_hidden_file() {
        let filter = HideHiddenFilter::new();
        assert!(filter.is_ignored(Path::new(".ignored")));
    }

    #[test]
    fn test_nested_in_hidden() {
        let filter = HideHiddenFilter::new();
        assert!(filter.is_ignored(Path::new("mydir/.ignored")));
        assert!(filter.is_ignored(Path::new(".ignored/myfile")));
        assert!(filter.is_ignored(Path::new("mydir/.ignored/myfile")));
    }

    #[test]
    fn test_current_path_not_hidden() {
        let filter = HideHiddenFilter::new();
        assert!(!filter.is_ignored(Path::new("./myfile")));
        assert!(!filter.is_ignored(Path::new("a/./b/myfile")));
    }

    #[test]
    fn test_current_path_hidden() {
        let filter = HideHiddenFilter::new();
        assert!(filter.is_ignored(Path::new("./.myfile")));
        assert!(filter.is_ignored(Path::new("./.git/objects/9c/32")));
    }

    #[test]
    fn test_parent_path_not_hidden() {
        let filter = HideHiddenFilter::new();
        assert!(!filter.is_ignored(Path::new("../myfile")));
    }

    #[test]
    fn test_parent_path_hidden() {
        let filter = HideHiddenFilter::new();
        assert!(filter.is_ignored(Path::new("../.myfile")));
    }
}
