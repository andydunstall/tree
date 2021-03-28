use std::fs;
use std::path::Path;

pub use crate::error::Result;
pub use crate::ruleset::Ruleset;

const GITIGNORE: &str = ".gitignore";

// See https://git-scm.com/docs/gitignore#_pattern_format.
pub struct GitignoreRuleset {}

impl GitignoreRuleset {
    pub fn new(gitignore: &str) -> GitignoreRuleset {
        GitignoreRuleset {}
    }

    // open returns all gitignore rulesets in order of deepest to the top of
    // the workspace.
    // Note order is important as the deepest gitignores override the upper
    // levels (TODO(AD) Not handling overriding block with an allow).
    pub fn open(dir: &Path) -> Result<Vec<GitignoreRuleset>> {
        let mut rulesets = vec![];

        let mut path = fs::canonicalize(dir)?;
        loop {
            if path.join(GITIGNORE).is_file() {
                let gitignore = fs::read_to_string(path.join(GITIGNORE))?;
                rulesets.push(GitignoreRuleset::new(&gitignore));
            }

            // Once reached the top of the git workspace return all the
            // gitignores.
            if GitignoreRuleset::is_git_workspace(&path) {
                return Ok(rulesets);
            }

            // If no parent exists can return any gitignores found so far.
            if let Some(p) = path.parent() {
                path = p.to_path_buf();
            } else {
                return Ok(rulesets);
            }
        }
    }

    fn is_git_workspace(dir: &Path) -> bool {
        dir.join(".git").is_dir()
    }
}

impl Ruleset for GitignoreRuleset {
    fn is_ignored(&self, path: &Path) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO(AD)
}
