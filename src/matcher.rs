use std::path::Path;

pub struct Matcher {
    show_hidden: bool,
    directories_only: bool,
}

impl Matcher {
    pub fn new(show_hidden: bool, directories_only: bool) -> Matcher {
        Matcher {
            show_hidden: show_hidden,
            directories_only: directories_only,
        }
    }

    pub fn is_match(&self, path: &Path) -> bool {
        if self.directories_only && !path.is_dir() {
            return false;
        }

        if !self.show_hidden && self.is_hidden(path) {
            return false;
        }

        true
    }

    fn is_file_name_match(&self, file_name: &str) -> bool {
        let hidden = file_name.starts_with(".");
        if hidden && !self.show_hidden {
            false
        } else {
            true
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_hidden() {
        let matcher = Matcher::new(true, false);
        assert!(matcher.is_match(Path::new("myfile")));
        assert!(matcher.is_match(Path::new("myfile/.ignored")));
        assert!(matcher.is_match(Path::new(".ignore")));
    }

    #[test]
    fn test_not_show_hidden() {
        let matcher = Matcher::new(false, false);
        assert!(matcher.is_match(Path::new("myfile")));
        assert!(!matcher.is_match(Path::new("myfile/.ignored")));
        assert!(!matcher.is_match(Path::new(".ignore")));
    }

    #[test]
    fn test_directories_only() {
        let matcher = Matcher::new(false, true);
        assert!(matcher.is_match(Path::new("target")));
        assert!(!matcher.is_match(Path::new("a/b/c")));
    }

    #[test]
    fn test_not_directories_only() {
        let matcher = Matcher::new(false, false);
        assert!(matcher.is_match(Path::new("target")));
        assert!(matcher.is_match(Path::new("a/b/c")));
    }
}
