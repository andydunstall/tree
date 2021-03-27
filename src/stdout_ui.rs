use std::collections::HashSet;

use crate::entry::Entry;
use crate::ui::UI;

pub struct StdoutUI {
    nested: HashSet<usize>,
}

impl StdoutUI {
    pub fn new() -> StdoutUI {
        StdoutUI {
            nested: HashSet::new(),
        }
    }
}

impl UI for StdoutUI {
    fn file(&self, entry: &Entry) {
        if entry.depth > 0 {
            let mut s = "".to_string();
            let indent = entry.depth - 1;
            for depth in 0..indent {
                if self.nested.contains(&depth) {
                    let tmp = format!("{}{}", s, "│   ");
                    s = tmp;
                } else {
                    let tmp = format!("{}{}", s, "    ");
                    s = tmp;
                }
            }
            let prefix = if entry.is_last {
                "└── "
            } else {
                "├── "
            };
            println!("{}{}{}", s, prefix, entry.file_name);
        } else {
            println!("{}", entry.file_name);
        }
    }

    fn add_indent(&mut self, depth: usize) {
        self.nested.insert(depth);
    }

    fn remove_indent(&mut self, depth: usize) {
        self.nested.remove(&depth);
    }
}
