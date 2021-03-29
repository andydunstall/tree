use std::fs;
use std::path::Path;
use std::vec::Vec;

pub use crate::error::Result;
pub use crate::ignore_config::IgnoreConfig;
pub use crate::rule::{OverrideRule, PathRule, PriorityRule, Rule};

const GITIGNORE: &str = ".gitignore";

// Returns the ignore configuration of all `.gitignore`s upto the repository
// root (directory containing `.git/`) or an empty vector if not a git
// repository.
//
// Note the order is important as deeper gitignores can override higher level,
// so returned in order from deepest to top.
pub fn open_gitignores(dir: &Path) -> Vec<IgnoreConfig> {
    let mut rulesets = vec![];
    if let Ok(mut path) = fs::canonicalize(dir) {
        loop {
            if path.join(GITIGNORE).is_file() {
                if let Ok(gitignore) = fs::read_to_string(path.join(GITIGNORE)) {
                    rulesets.push(IgnoreConfig::new(&gitignore, &path));
                }
            }

            // Once reached the top of the git repository return all the
            // gitignores.
            if is_git_repository(&path) {
                return rulesets;
            }

            if let Some(p) = path.parent() {
                path = p.to_path_buf();
            } else {
                // If no parent exists (ie reached root without finding a git
                // repository) then return nothing.
                return vec![];
            }
        }
    } else {
        vec![]
    }
}

fn is_git_repository(dir: &Path) -> bool {
    dir.join(".git").is_dir()
}
