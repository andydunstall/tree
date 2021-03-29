use std::collections::HashSet;

pub struct Formatter {
    dirs: HashSet<usize>,
}

impl Formatter {
    pub fn new() -> Formatter {
        Formatter {
            dirs: HashSet::new(),
        }
    }

    pub fn file(&self, file_name: String, depth: usize, is_last: bool) -> String {
        if depth > 0 {
            self.file_nested(file_name, depth, is_last)
        } else {
            format!("{}\n", file_name)
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

    fn file_nested(&self, file_name: String, depth: usize, is_last: bool) -> String {
        format!(
            "{}{}{}\n",
            self.indent(depth),
            self.prefix(is_last),
            file_name
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_dir() {
        let mut fmt = Formatter::new();

        let out = fmt.file("myfile.txt".to_string(), 2, true);
        assert_eq!(out, "    └── myfile.txt\n");

        fmt.add_dir(0);

        let out = fmt.file("myfile.txt".to_string(), 2, true);
        assert_eq!(out, "│   └── myfile.txt\n");

        fmt.remove_dir(0);

        let out = fmt.file("myfile.txt".to_string(), 2, true);
        assert_eq!(out, "    └── myfile.txt\n");
    }

    #[test]
    fn test_depth_1_last() {
        let fmt = Formatter::new();
        let out = fmt.file("myfile.txt".to_string(), 1, true);
        assert_eq!(out, "└── myfile.txt\n");
    }

    #[test]
    fn test_depth_1_not_last() {
        let fmt = Formatter::new();
        let out = fmt.file("myfile.txt".to_string(), 1, false);
        assert_eq!(out, "├── myfile.txt\n");
    }

    #[test]
    fn test_top_level_file() {
        let fmt = Formatter::new();
        let out = fmt.file("myfile.txt".to_string(), 0, true);
        assert_eq!(out, "myfile.txt\n");
    }

    #[test]
    fn test_summary() {
        let fmt = Formatter::new();
        assert_eq!(fmt.summary(72, 428), "72 directories, 428 files\n");
    }
}
