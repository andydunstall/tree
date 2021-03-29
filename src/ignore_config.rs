use std::path::Path;
use std::vec::Vec;

pub use crate::error::Result;
pub use crate::rule::{OverrideRule, PathRule, PriorityRule, Rule};

// See https://git-scm.com/docs/gitignore#_pattern_format.
// Note order of returned priority rule important (deeper directories and lower
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

    pub fn rule(&self) -> impl Rule {
        let mut rules: Vec<Box<dyn Rule>> = vec![];
        for line in self.content.lines() {
            if let Some(rule) = IgnoreConfig::parse_line(line) {
                rules.push(rule);
            }
        }

        // Reverse since the rules are in priority order from last to first.
        rules.reverse();
        PriorityRule::new(rules)
    }

    fn parse_line(line: &str) -> Option<Box<dyn Rule>> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }
        if line.starts_with("#") {
            return None;
        }

        if line.starts_with("!") {
            if let Some(line) = line.strip_prefix("!") {
                if let Some(rule) = IgnoreConfig::parse_line(line) {
                    return Some(Box::new(OverrideRule::new(rule)));
                }
            }
        } else {
            return IgnoreConfig::parse_path_rule(line);
        }

        return None;
    }

    fn parse_path_rule(line: &str) -> Option<Box<dyn Rule>> {
        let path = if Path::new(line).is_absolute() {
            if let Ok(rel) = Path::new(line).strip_prefix("/") {
                rel
            } else {
                Path::new(line)
            }
        } else {
            Path::new(line)
        };

        return Some(Box::new(PathRule::new(&path)));
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

        let rule = IgnoreConfig::new(content, Path::new("")).rule();
        assert!(rule.is_ignored(Path::new("myfile")));
        assert!(rule.is_ignored(Path::new("mydir/myfile")));
        assert!(rule.is_ignored(Path::new("myfile/myotherfile")));
    }

    #[test]
    fn test_override_ignore() {
        let content = r#"
        myfile
        !myfile
        "#;

        let rule = IgnoreConfig::new(content, Path::new("")).rule();
        assert!(!rule.is_ignored(Path::new("myfile")));
        assert!(!rule.is_ignored(Path::new("mydir/myfile")));
        assert!(!rule.is_ignored(Path::new("myfile/myotherfile")));
    }

    #[test]
    fn test_ignore_file() {
        let rule = IgnoreConfig::new("myfile", Path::new("")).rule();
        assert!(rule.is_ignored(Path::new("myfile")));
        assert!(rule.is_ignored(Path::new("mydir/myfile")));
        assert!(rule.is_ignored(Path::new("myfile/myotherfile")));
        assert!(!rule.is_ignored(Path::new("notmyfile")));
        assert!(!rule.is_ignored(Path::new("notmydir/notmyfile")));
    }

    #[test]
    fn test_ignore_comments() {
        let content = r#"
# commented


#commented
ignored
  # commented
"#;
        let rule = IgnoreConfig::new(content, Path::new("")).rule();
        assert!(rule.is_ignored(Path::new("ignored")));
        assert!(!rule.is_ignored(Path::new("commented")));
        assert!(!rule.is_ignored(Path::new("#commented")));
    }

    #[test]
    fn test_empty() {
        let rule = IgnoreConfig::new("", Path::new("")).rule();
        assert!(!rule.is_ignored(Path::new("myfile")));
    }
}
