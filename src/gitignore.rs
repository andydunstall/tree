use std::fs;
use std::path::Path;
use std::vec::Vec;

pub use crate::error::Result;
pub use crate::rule::{PriorityRule, Rule};

const GITIGNORE: &str = ".gitignore";

// See https://git-scm.com/docs/gitignore#_pattern_format.
// Note order of returned priority rule important (deeper directories and lower
// within file take priority).
pub struct Gitignore {}

impl Gitignore {
    pub fn new(_gitignore: &str) -> Gitignore {
        Gitignore {}
    }

    pub fn workspace(dir: &Path) -> Vec<Gitignore> {
        let mut rulesets = vec![];

        let mut path = fs::canonicalize(dir).unwrap(); // TODO(AD) unwrap -> ignore
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
    }

    pub fn rule(&self) -> impl Rule {
        PriorityRule::new(vec![])
    }

    fn is_git_workspace(dir: &Path) -> bool {
        dir.join(".git").is_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let rule = Gitignore::new("").rule();
        assert!(rule.is_ignored(Path::new("myfile")));
    }
}
