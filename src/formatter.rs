use std::collections::HashSet;

use crate::entry::Entry;

// TODO(AD) Add Formatter trait and CLIFormatter impl
pub struct Formatter {
    nested: HashSet<usize>,
}

impl Formatter {
    pub fn new() -> Formatter {
        Formatter {
            nested: HashSet::new(),
        }
    }

    pub fn file(&self, entry: &Entry) -> String {
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
            format!("{}{}{}", s, prefix, entry.file_name)
        } else {
            format!("{}", entry.file_name)
        }
    }

    pub fn summary(&self, n_dirs: usize, n_files: usize) -> String {
        format!("\n{} directories, {} files", n_dirs, n_files)
    }

    pub fn add_indent(&mut self, depth: usize) {
        self.nested.insert(depth);
    }

    pub fn remove_indent(&mut self, depth: usize) {
        self.nested.remove(&depth);
    }
}
