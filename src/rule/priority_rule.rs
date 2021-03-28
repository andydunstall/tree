use std::path::Path;
use std::vec::Vec;

use crate::rule::Rule;

pub struct PriorityRule {
    rules: Vec<Box<dyn Rule>>,
}

impl PriorityRule {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> PriorityRule {
        PriorityRule { rules: rules }
    }
}

impl Rule for PriorityRule {
    fn is_ignored(&self, path: &Path) -> bool {
        for rule in &self.rules {
            if rule.is_ignored(path) {
                return true;
            }
            if rule.is_override(path) {
                return false;
            }
        }
        false
    }

    fn is_override(&self, path: &Path) -> bool {
        for rule in &self.rules {
            if rule.is_override(path) {
                return true;
            }
        }
        false
    }
}

// TODO(AD) Unittest
