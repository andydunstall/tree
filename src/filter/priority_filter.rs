use std::path::Path;
use std::vec::Vec;

use crate::filter::Filter;

pub struct PriorityFilter {
    filters: Vec<Box<dyn Filter>>,
}

impl PriorityFilter {
    pub fn new(filters: Vec<Box<dyn Filter>>) -> PriorityFilter {
        PriorityFilter { filters: filters }
    }
}

impl Filter for PriorityFilter {
    fn is_ignored(&self, path: &Path) -> bool {
        for filter in &self.filters {
            if filter.is_ignored(path) {
                return true;
            }
            if filter.is_override(path) {
                return false;
            }
        }
        false
    }

    fn is_override(&self, path: &Path) -> bool {
        for filter in &self.filters {
            if filter.is_override(path) {
                return true;
            }
        }
        false
    }
}

// TODO(AD) Unittest
