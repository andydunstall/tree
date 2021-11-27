use std::path::Path;
use std::vec::Vec;

pub use crate::error::Result;
pub use crate::filter::{Filter, OverrideFilter, PathFilter, PriorityFilter};

// See https://git-scm.com/docs/gitignore#_pattern_format.
// Note order of returned priority filter important (deeper directories and lower
// within file take priority).
// TODO(AD) Add gitignore patterns.
pub struct IgnoreConfig {
    content: String,
}

impl IgnoreConfig {
    pub fn new(content: &str, _root: &Path) -> IgnoreConfig {
        IgnoreConfig {
            content: content.to_string(),
        }
    }

    pub fn filter(&self) -> impl Filter {
        let mut filters: Vec<Box<dyn Filter>> = vec![];
        for line in self.content.lines() {
            if let Some(filter) = IgnoreConfig::parse_line(line) {
                filters.push(filter);
            }
        }

        // Reverse since the filters are in priority order from last to first.
        filters.reverse();
        PriorityFilter::new(filters)
    }

    fn parse_line(line: &str) -> Option<Box<dyn Filter>> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }
        if line.starts_with("#") {
            return None;
        }

        if line.starts_with("!") {
            if let Some(line) = line.strip_prefix("!") {
                if let Some(filter) = IgnoreConfig::parse_line(line) {
                    return Some(Box::new(OverrideFilter::new(filter)));
                }
            }
        } else {
            return IgnoreConfig::parse_path_filter(line);
        }

        return None;
    }

    fn parse_path_filter(line: &str) -> Option<Box<dyn Filter>> {
        let path = if Path::new(line).is_absolute() {
            if let Ok(rel) = Path::new(line).strip_prefix("/") {
                rel
            } else {
                Path::new(line)
            }
        } else {
            Path::new(line)
        };

        return Some(Box::new(PathFilter::new(&path)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute() {
        let content = r#"
        /myfile
        "#;

        let filter = IgnoreConfig::new(content, Path::new("")).filter();
        assert!(filter.is_ignored(Path::new("myfile")));
        assert!(filter.is_ignored(Path::new("mydir/myfile")));
        assert!(filter.is_ignored(Path::new("myfile/myotherfile")));
    }

    #[test]
    fn test_override_ignore() {
        let content = r#"
        myfile
        !myfile
        "#;

        let filter = IgnoreConfig::new(content, Path::new("")).filter();
        assert!(!filter.is_ignored(Path::new("myfile")));
        assert!(!filter.is_ignored(Path::new("mydir/myfile")));
        assert!(!filter.is_ignored(Path::new("myfile/myotherfile")));
    }

    #[test]
    fn test_ignore_file() {
        let filter = IgnoreConfig::new("myfile", Path::new("")).filter();
        assert!(filter.is_ignored(Path::new("myfile")));
        assert!(filter.is_ignored(Path::new("mydir/myfile")));
        assert!(filter.is_ignored(Path::new("myfile/myotherfile")));
        assert!(!filter.is_ignored(Path::new("notmyfile")));
        assert!(!filter.is_ignored(Path::new("notmydir/notmyfile")));
    }

    #[test]
    fn test_ignore_comments() {
        let content = r#"
# commented


#commented
ignored
  # commented
"#;
        let filter = IgnoreConfig::new(content, Path::new("")).filter();
        assert!(filter.is_ignored(Path::new("ignored")));
        assert!(!filter.is_ignored(Path::new("commented")));
        assert!(!filter.is_ignored(Path::new("#commented")));
    }

    #[test]
    fn test_empty() {
        let filter = IgnoreConfig::new("", Path::new("")).filter();
        assert!(!filter.is_ignored(Path::new("myfile")));
    }
}
