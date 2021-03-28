use std::path::Path;

use crate::rule::Rule;

pub struct OverrideRule {
    rule: Box<dyn Rule>,
}

impl OverrideRule {
    pub fn new(rule: Box<dyn Rule>) -> OverrideRule {
        OverrideRule { rule: rule }
    }
}

impl Rule for OverrideRule {
    fn is_ignored(&self, _path: &Path) -> bool {
        false
    }

    fn is_override(&self, path: &Path) -> bool {
        self.rule.is_ignored(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub use crate::rule::PathRule;

    #[test]
    fn override_path() {
        let rule = OverrideRule::new(Box::new(PathRule::new(Path::new("myfile"))));
        assert!(!rule.is_ignored(Path::new("myfile")));
        assert!(rule.is_override(Path::new("myfile")));
    }
}
