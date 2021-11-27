use std::path::Path;

use mockall::automock;

#[automock]
pub trait Filter {
    // Returns true if the given path should be allowed, false otherwise.
    fn is_ignored(&self, path: &Path) -> bool;

    // Returns true if the given path must always be allowed overriding any
    // future filters, false otherwise.
    fn is_override(&self, path: &Path) -> bool;
}
