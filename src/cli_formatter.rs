use std::collections::HashSet;

use crate::entry::Entry;
use crate::formatter::Formatter;

// TODO(AD) Add Formatter trait and CLIFormatter impl
pub struct CLIFormatter {
    nested: HashSet<usize>,
}

impl CLIFormatter {
    pub fn new() -> CLIFormatter {
        CLIFormatter {
            nested: HashSet::new(),
        }
    }
}

impl Formatter for CLIFormatter {
    fn file(&self, entry: &Entry) -> String {
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
            format!("{}{}{}\n", s, prefix, entry.file_name)
        } else {
            format!("{}\n", entry.file_name)
        }
    }

    fn summary(&self, n_dirs: usize, n_files: usize) -> String {
        format!("\n{} directories, {} files\n", n_dirs, n_files)
    }

    fn add_indent(&mut self, depth: usize) {
        self.nested.insert(depth);
    }

    fn remove_indent(&mut self, depth: usize) {
        self.nested.remove(&depth);
    }
}
