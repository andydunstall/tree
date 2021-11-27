use std::path::Path;

use crate::filter::Filter;

pub struct OverrideFilter {
    filter: Box<dyn Filter>,
}

impl OverrideFilter {
    pub fn new(filter: Box<dyn Filter>) -> OverrideFilter {
        OverrideFilter { filter: filter }
    }
}

impl Filter for OverrideFilter {
    fn is_ignored(&self, _path: &Path) -> bool {
        false
    }

    fn is_override(&self, path: &Path) -> bool {
        self.filter.is_ignored(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub use crate::filter::PathFilter;

    #[test]
    fn override_path() {
        let filter = OverrideFilter::new(Box::new(PathFilter::new(Path::new("myfile"))));
        assert!(!filter.is_ignored(Path::new("myfile")));
        assert!(filter.is_override(Path::new("myfile")));
    }
}
