use std::collections::HashSet;

use crate::entry::Entry;
use crate::formatter::Formatter;

pub struct CLIFormatter {
    dirs: HashSet<usize>,
}

impl CLIFormatter {
    pub fn new() -> CLIFormatter {
        CLIFormatter {
            dirs: HashSet::new(),
        }
    }

    fn file_nested(&self, entry: &Entry) -> String {
        format!(
            "{}{}{}\n",
            self.indent(entry.depth),
            self.prefix(entry.is_last),
            entry.file_name
        )
    }

    fn prefix(&self, is_last: bool) -> String {
        if is_last {
            "└── ".to_string()
        } else {
            "├── ".to_string()
        }
    }

    fn indent(&self, depth: usize) -> String {
        let mut s = "".to_string();
        let indent = depth - 1;
        for depth in 0..indent {
            if self.dirs.contains(&depth) {
                s = format!("{}{}", s, "│   ");
            } else {
                s = format!("{}{}", s, "    ");
            }
        }
        s
    }
}

impl Formatter for CLIFormatter {
    fn file(&self, entry: &Entry) -> String {
        if entry.depth > 0 {
            self.file_nested(entry)
        } else {
            format!("{}\n", entry.file_name)
        }
    }

    fn summary(&self, n_dirs: usize, n_files: usize) -> String {
        format!("{} directories, {} files\n", n_dirs, n_files)
    }

    fn add_dir(&mut self, depth: usize) {
        self.dirs.insert(depth);
    }

    fn remove_dir(&mut self, depth: usize) {
        self.dirs.remove(&depth);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_dir() {
        let mut fmt = CLIFormatter::new();

        let out = fmt.file(&Entry {
            file_name: "myfile.txt".to_string(),
            depth: 2,
            is_last: true,
        });
        assert_eq!(out, "    └── myfile.txt\n");

        fmt.add_dir(0);

        let out = fmt.file(&Entry {
            file_name: "myfile.txt".to_string(),
            depth: 2,
            is_last: true,
        });
        assert_eq!(out, "│   └── myfile.txt\n");

        fmt.remove_dir(0);

        let out = fmt.file(&Entry {
            file_name: "myfile.txt".to_string(),
            depth: 2,
            is_last: true,
        });
        assert_eq!(out, "    └── myfile.txt\n");
    }

    #[test]
    fn test_depth_1_last() {
        let fmt = CLIFormatter::new();
        let out = fmt.file(&Entry {
            file_name: "myfile.txt".to_string(),
            depth: 1,
            is_last: true,
        });
        assert_eq!(out, "└── myfile.txt\n");
    }

    #[test]
    fn test_depth_1_not_last() {
        let fmt = CLIFormatter::new();
        let out = fmt.file(&Entry {
            file_name: "myfile.txt".to_string(),
            depth: 1,
            is_last: false,
        });
        assert_eq!(out, "├── myfile.txt\n");
    }

    #[test]
    fn test_top_level_file() {
        let fmt = CLIFormatter::new();
        let out = fmt.file(&Entry {
            file_name: "myfile.txt".to_string(),
            depth: 0,
            is_last: true,
        });
        assert_eq!(out, "myfile.txt\n");
    }

    #[test]
    fn test_summary() {
        let fmt = CLIFormatter::new();
        assert_eq!(fmt.summary(72, 428), "72 directories, 428 files\n");
    }
}
