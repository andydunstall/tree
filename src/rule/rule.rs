use std::path::Path;

pub trait Rule {
    // Returns true if the given path should be allowed, false otherwise.
    fn is_ignored(&self, path: &Path) -> bool;

    // Returns true if the given path must always be allowed overriding any
    // future rules, false otherwise.
    fn is_override(&self, path: &Path) -> bool;
}
