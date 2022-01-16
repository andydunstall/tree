use std::collections::HashSet;

use crate::fs::File;
use crate::summary::Summary;

pub struct Formatter {
    dirs: HashSet<usize>,
    long_format: bool,
    count_lines: bool,
}

impl Formatter {
    pub fn new(long_format: bool, count_lines: bool) -> Formatter {
        Formatter {
            dirs: HashSet::new(),
            long_format: long_format,
            count_lines: count_lines,
        }
    }

    pub fn file(&self, file: &File) -> String {
        match file {
            File::RegularFile(file) => {
                format!(
                    "{}{}{}",
                    file.name,
                    self.long_format(file.size),
                    self.line_count_format(file.line_count),
                )
            }
            File::Directory(dir) => format!("{}", dir.name),
            File::Symlink(file) => format!("{} -> {}", file.name, file.target),
        }
    }

    pub fn summary(&self, summary: &Summary) -> String {
        let mut s = format!("{} directories, {} files", summary.n_dirs, summary.n_files);
        if self.long_format {
            s += &format!(", {} bytes", summary.size);
        }
        if self.count_lines {
            s += &format!(", {} lines", summary.line_count);
        }
        format!("{}\n", s)
    }

    pub fn add_dir(&mut self, depth: usize) {
        self.dirs.insert(depth);
    }

    pub fn remove_dir(&mut self, depth: usize) {
        self.dirs.remove(&depth);
    }

    pub fn prefix(&self, depth: usize, is_last: bool) -> String {
        if depth == 0 {
            return "".to_string();
        }

        if is_last {
            self.indent(depth) + "└── "
        } else {
            self.indent(depth) + "├── "
        }
    }

    fn indent(&self, depth: usize) -> String {
        let mut s = "".to_string();
        if depth == 0 {
            return s;
        }

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

    fn long_format(&self, size: u64) -> String {
        if self.long_format {
            format!(" ({}B)", size)
        } else {
            "".to_string()
        }
    }

    fn line_count_format(&self, line_count: u64) -> String {
        if self.count_lines {
            format!(" ({}L)", line_count)
        } else {
            "".to_string()
        }
    }
}
