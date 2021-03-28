use std::fs;
use std::path::Path;
use std::vec::Vec;

pub use crate::error::Result;
pub use crate::rule::{PathRule, PriorityRule, Rule};

const GITIGNORE: &str = ".gitignore";

// See https://git-scm.com/docs/gitignore#_pattern_format.
// Note order of returned priority rule important (deeper directories and lower
// within file take priority).
pub struct Gitignore {
    content: String,
}

// TODO(AD) Maybe GitignoreRule needed to handle matching paths relative to
// the path of the gitignore
impl Gitignore {
    pub fn new(gitignore: &str) -> Gitignore {
        Gitignore {
            content: gitignore.to_string(),
        }
    }

    pub fn workspace(dir: &Path) -> Vec<Gitignore> {
        let mut rulesets = vec![];

        if let Ok(mut path) = fs::canonicalize(dir) {
            loop {
                if path.join(GITIGNORE).is_file() {
                    if let Ok(gitignore) = fs::read_to_string(path.join(GITIGNORE)) {
                        rulesets.push(Gitignore::new(&gitignore));
                    }
                }

                // Once reached the top of the git workspace return all the
                // gitignores.
                if Gitignore::is_git_workspace(&path) {
                    return rulesets;
                }

                // If no parent exists can return any gitignores found so far.
                if let Some(p) = path.parent() {
                    path = p.to_path_buf();
                } else {
                    return rulesets;
                }
            }
        } else {
            vec![]
        }
    }

    pub fn rule(&self) -> impl Rule {
        let mut rules: Vec<Box<dyn Rule>> = vec![];
        for line in self.content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with("#") {
                continue;
            }

            rules.push(Box::new(PathRule::new(Path::new(line))));
        }
        PriorityRule::new(rules)
    }

    fn is_git_workspace(dir: &Path) -> bool {
        dir.join(".git").is_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignore_file() {
        // TODO(AD)
        let rule = Gitignore::new("myfile").rule();
        assert!(rule.is_ignored(Path::new("myfile")));
        // assert!(rule.is_ignored(Path::new("mydir/myfile")));
        // assert!(rule.is_ignored(Path::new("myfile/myotherfile")));

        assert!(!rule.is_ignored(Path::new("notmyfile")));
        // assert!(!rule.is_ignored(Path::new("notmydir/notmyfile")));
    }

    #[test]
    fn test_ignore_comments() {
        let content = r#"
# commented


#commented
ignored
  # commented
"#;
        let rule = Gitignore::new(content).rule();
        assert!(rule.is_ignored(Path::new("ignored")));
        assert!(!rule.is_ignored(Path::new("commented")));
        assert!(!rule.is_ignored(Path::new("#commented")));
    }

    #[test]
    fn test_empty() {
        let rule = Gitignore::new("").rule();
        assert!(!rule.is_ignored(Path::new("myfile")));
    }
}
