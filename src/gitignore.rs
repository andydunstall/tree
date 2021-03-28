use std::fs;
use std::path::Path;
use std::vec::Vec;

pub use crate::error::Result;
pub use crate::ignore_config::IgnoreConfig;
pub use crate::rule::{OverrideRule, PathRule, PriorityRule, Rule};

const GITIGNORE: &str = ".gitignore";

pub fn open_gitignores(dir: &Path) -> Vec<IgnoreConfig> {
    let mut rulesets = vec![];

    if let Ok(mut path) = fs::canonicalize(dir) {
        loop {
            if path.join(GITIGNORE).is_file() {
                if let Ok(gitignore) = fs::read_to_string(path.join(GITIGNORE)) {
                    rulesets.push(IgnoreConfig::new(&gitignore));
                }
            }

            // Once reached the top of the git workspace return all the
            // gitignores.
            if is_git_workspace(&path) {
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

fn is_git_workspace(dir: &Path) -> bool {
    dir.join(".git").is_dir()
}
