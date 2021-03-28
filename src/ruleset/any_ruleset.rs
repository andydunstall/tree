use std::path::Path;
use std::vec::Vec;

pub use crate::ruleset::Ruleset;

// AnyRuleset implements Ruleset where a file is ignored if any contained
// rulesets ignore it.
pub struct AnyRuleset {
    rulesets: Vec<Box<dyn Ruleset>>,
}

impl AnyRuleset {
    pub fn new(rulesets: Vec<Box<dyn Ruleset>>) -> AnyRuleset {
        AnyRuleset { rulesets: rulesets }
    }
}

impl Ruleset for AnyRuleset {
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
