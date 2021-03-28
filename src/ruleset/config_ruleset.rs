use std::path::Path;

pub use crate::ruleset::Ruleset;

pub struct ConfigRuleset {
    show_hidden: bool,
    directories_only: bool,
}

impl ConfigRuleset {
    pub fn new(show_hidden: bool, directories_only: bool) -> ConfigRuleset {
        ConfigRuleset {
            show_hidden: show_hidden,
            directories_only: directories_only,
        }
    }

    fn is_hidden(&self, path: &Path) -> bool {
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

impl Ruleset for ConfigRuleset {
    fn is_ignored(&self, path: &Path) -> bool {
        if self.directories_only && !path.is_dir() {
            return false;
        }
        if !self.show_hidden && self.is_hidden(path) {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_hidden() {
        let rs = ConfigRuleset::new(true, false);
        assert!(rs.is_ignored(Path::new("myfile")));
        assert!(rs.is_ignored(Path::new("myfile/.ignored")));
        assert!(rs.is_ignored(Path::new(".ignore")));
    }

    #[test]
    fn test_not_show_hidden() {
        let rs = ConfigRuleset::new(false, false);
        assert!(rs.is_ignored(Path::new("myfile")));
        assert!(!rs.is_ignored(Path::new("myfile/.ignored")));
        assert!(!rs.is_ignored(Path::new(".ignore")));
    }

    #[test]
    fn test_directories_only() {
        let rs = ConfigRuleset::new(false, true);
        assert!(rs.is_ignored(Path::new("target")));
        assert!(!rs.is_ignored(Path::new("a/b/c")));
    }

    #[test]
    fn test_not_directories_only() {
        let rs = ConfigRuleset::new(false, false);
        assert!(rs.is_ignored(Path::new("target")));
        assert!(rs.is_ignored(Path::new("a/b/c")));
    }
}
