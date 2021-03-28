use std::path::Path;

pub trait Ruleset {
    fn is_ignored(&self, path: &Path) -> bool;
}
