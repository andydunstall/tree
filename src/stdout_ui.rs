use crate::formatter::Formatter;
use crate::UI;

pub struct StdoutUI {
    formatter: Formatter,
}

impl StdoutUI {
    pub fn new(formatter: Formatter) -> StdoutUI {
        StdoutUI {
            formatter: formatter,
        }
    }
}

impl UI for StdoutUI {
    fn file(&self, file_name: String, depth: usize, is_last: bool) {
        print!("{}", self.formatter.file(file_name, depth, is_last));
    }

    fn summary(&self, n_dirs: usize, n_files: usize) {
        print!("\n{}", self.formatter.summary(n_dirs, n_files));
    }

    fn add_dir(&mut self, depth: usize) {
        self.formatter.add_dir(depth)
    }

    fn remove_dir(&mut self, depth: usize) {
        self.formatter.remove_dir(depth)
    }
}
