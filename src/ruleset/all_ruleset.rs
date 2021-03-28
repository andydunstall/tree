use std::path::Path;
use std::vec::Vec;

pub use crate::ruleset::Ruleset;

pub struct AllRuleset {
    rulesets: Vec<Box<dyn Ruleset>>,
}

impl AllRuleset {
    pub fn new(rulesets: Vec<Box<dyn Ruleset>>) -> AllRuleset {
        AllRuleset { rulesets: rulesets }
    }
}

impl Ruleset for AllRuleset {
    fn is_ignored(&self, path: &Path) -> bool {
        for rs in &self.rulesets {
            if rs.is_ignored(path) {
                return true;
            }
        }
        false
    }
}

// TODO(AD) Unittest
