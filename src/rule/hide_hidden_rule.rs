use std::path::Path;

use crate::rule::Rule;

pub struct HideHiddenRule;

impl HideHiddenRule {
    pub fn new() -> HideHiddenRule {
        HideHiddenRule {}
    }
}

impl HideHiddenRule {
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

impl Rule for HideHiddenRule {
    fn is_ignored(&self, path: &Path) -> bool {
        let mut path = path;
        loop {
            if HideHiddenRule::is_hidden(path) {
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
        let rule = HideHiddenRule::new();
        assert!(!rule.is_ignored(Path::new("myfile")));
    }

    #[test]
    fn test_hidden_file() {
        let rule = HideHiddenRule::new();
        assert!(rule.is_ignored(Path::new(".ignored")));
    }

    #[test]
    fn test_nested_in_hidden() {
        let rule = HideHiddenRule::new();
        assert!(rule.is_ignored(Path::new("mydir/.ignored")));
        assert!(rule.is_ignored(Path::new(".ignored/myfile")));
        assert!(rule.is_ignored(Path::new("mydir/.ignored/myfile")));
    }

    #[test]
    fn test_current_path_not_hidden() {
        let rule = HideHiddenRule::new();
        assert!(!rule.is_ignored(Path::new("./myfile")));
        assert!(!rule.is_ignored(Path::new("a/./b/myfile")));
    }

    #[test]
    fn test_current_path_hidden() {
        let rule = HideHiddenRule::new();
        assert!(rule.is_ignored(Path::new("./.myfile")));
        assert!(rule.is_ignored(Path::new("./.git/objects/9c/32")));
    }

    #[test]
    fn test_parent_path_not_hidden() {
        let rule = HideHiddenRule::new();
        assert!(!rule.is_ignored(Path::new("../myfile")));
    }

    #[test]
    fn test_parent_path_hidden() {
        let rule = HideHiddenRule::new();
        assert!(rule.is_ignored(Path::new("../.myfile")));
    }
}
