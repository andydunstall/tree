use crate::formatter::Formatter;
use crate::fs::File;
use crate::summary::Summary;
use crate::UI;

use colored::*;

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
    fn file(&self, file: &File, depth: usize, is_last: bool) {
        match file {
            File::RegularFile(f) => {
                if f.executable {
                    print!(
                        "{}{}\n",
                        self.formatter.prefix(depth, is_last),
                        self.formatter.file(file).bright_green()
                    );
                } else if f.accessible {
                    print!(
                        "{}{}\n",
                        self.formatter.prefix(depth, is_last),
                        self.formatter.file(file)
                    );
                } else {
                    print!(
                        "{}{}\n",
                        self.formatter.prefix(depth, is_last),
                        self.formatter.file(file).red()
                    );
                }
            }
            File::Directory(dir) => {
                if dir.accessible {
                    print!(
                        "{}{}\n",
                        self.formatter.prefix(depth, is_last),
                        self.formatter.file(file)
                    );
                } else {
                    print!(
                        "{}{}\n",
                        self.formatter.prefix(depth, is_last),
                        self.formatter.file(file).red()
                    );
                }
            }
            File::Symlink(symlink) => {
                if symlink.accessible {
                    print!(
                        "{}{}\n",
                        self.formatter.prefix(depth, is_last),
                        self.formatter.file(file).cyan(),
                    );
                } else {
                    print!(
                        "{}{}\n",
                        self.formatter.prefix(depth, is_last),
                        self.formatter.file(file).red()
                    );
                }
            }
        }
    }

    fn summary(&self, summary: &Summary) {
        print!(
            "\n{}\n",
            self.formatter.summary(summary.n_dirs, summary.n_files)
        );
    }

    fn add_dir(&mut self, depth: usize) {
        self.formatter.add_dir(depth)
    }

    fn remove_dir(&mut self, depth: usize) {
        self.formatter.remove_dir(depth)
    }
}
