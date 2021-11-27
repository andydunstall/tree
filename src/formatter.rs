use std::collections::HashSet;

use crate::fs::File;

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

    pub fn file(&self, file: File, depth: usize, is_last: bool, is_dir: bool) -> String {
        if depth > 0 {
            self.file_nested(file, depth, is_last, is_dir)
        } else {
            format!(
                "{}{}{}\n",
                file.name,
                self.long_format(is_dir, file.size),
                self.line_count_format(is_dir, file.line_count)
            )
        }
    }

    pub fn summary(&self, n_dirs: usize, n_files: usize) -> String {
        format!("{} directories, {} files\n", n_dirs, n_files)
    }

    pub fn add_dir(&mut self, depth: usize) {
        self.dirs.insert(depth);
    }

    pub fn remove_dir(&mut self, depth: usize) {
        self.dirs.remove(&depth);
    }

    fn file_nested(&self, file: File, depth: usize, is_last: bool, is_dir: bool) -> String {
        format!(
            "{}{}{}{}{}\n",
            self.indent(depth),
            self.prefix(is_last),
            file.name,
            self.long_format(is_dir, file.size),
            self.line_count_format(is_dir, file.line_count),
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

    fn long_format(&self, is_dir: bool, size: u64) -> String {
        if self.long_format && !is_dir {
            format!(" ({}B)", size)
        } else {
            "".to_string()
        }
    }

    fn line_count_format(&self, is_dir: bool, line_count: u64) -> String {
        if self.count_lines && !is_dir {
            format!(" ({}L)", line_count)
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_dir() {
        let mut fmt = Formatter::new(false, false);

        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            2,
            true,
            false,
        );
        assert_eq!(out, "    └── myfile.txt\n");

        fmt.add_dir(0);

        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            2,
            true,
            false,
        );
        assert_eq!(out, "│   └── myfile.txt\n");

        fmt.remove_dir(0);

        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            2,
            true,
            false,
        );
        assert_eq!(out, "    └── myfile.txt\n");
    }

    #[test]
    fn test_nested_dir_long_format() {
        let mut fmt = Formatter::new(true, false);

        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            2,
            true,
            false,
        );
        assert_eq!(out, "    └── myfile.txt (123B)\n");

        fmt.add_dir(0);

        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            2,
            true,
            false,
        );
        assert_eq!(out, "│   └── myfile.txt (123B)\n");

        fmt.remove_dir(0);

        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            2,
            true,
            false,
        );
        assert_eq!(out, "    └── myfile.txt (123B)\n");
    }

    #[test]
    fn test_nested_dir_line_count() {
        let mut fmt = Formatter::new(false, true);

        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            2,
            true,
            false,
        );
        assert_eq!(out, "    └── myfile.txt (456L)\n");

        fmt.add_dir(0);

        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            2,
            true,
            false,
        );
        assert_eq!(out, "│   └── myfile.txt (456L)\n");

        fmt.remove_dir(0);

        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            2,
            true,
            false,
        );
        assert_eq!(out, "    └── myfile.txt (456L)\n");
    }

    #[test]
    fn test_depth_1_last() {
        let fmt = Formatter::new(false, false);
        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            1,
            true,
            false,
        );
        assert_eq!(out, "└── myfile.txt\n");
    }

    #[test]
    fn test_depth_1_not_last() {
        let fmt = Formatter::new(false, false);
        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            1,
            false,
            false,
        );
        assert_eq!(out, "├── myfile.txt\n");
    }

    #[test]
    fn test_top_level_file() {
        let fmt = Formatter::new(false, false);
        let out = fmt.file(
            File::new("myfile.txt".to_string(), 123, 456),
            0,
            true,
            false,
        );
        assert_eq!(out, "myfile.txt\n");
    }

    #[test]
    fn test_summary() {
        let fmt = Formatter::new(false, false);
        assert_eq!(fmt.summary(72, 428), "72 directories, 428 files\n");
    }
}
