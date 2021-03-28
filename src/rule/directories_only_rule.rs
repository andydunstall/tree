use std::path::Path;

use crate::rule::Rule;

pub struct DirectoriesOnlyRule;

impl DirectoriesOnlyRule {
    pub fn new() -> DirectoriesOnlyRule {
        DirectoriesOnlyRule {}
    }
}

impl Rule for DirectoriesOnlyRule {
    fn is_ignored(&self, path: &Path) -> bool {
        !path.is_dir()
    }

    fn is_override(&self, _path: &Path) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_dir() {
        let rule = DirectoriesOnlyRule::new();
        assert!(rule.is_ignored(Path::new("myfile")));
    }

    #[test]
    fn test_dir() {
        let rule = DirectoriesOnlyRule::new();
        assert!(!rule.is_ignored(Path::new("target")));
    }
}
