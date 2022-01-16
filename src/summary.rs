use crate::fs::File;

#[derive(Debug, PartialEq)]
pub struct Summary {
    pub n_dirs: usize,
    pub n_files: usize,
    pub line_count: u64,
    pub size: u64,
}

impl Summary {
    pub fn new() -> Summary {
        Summary {
            n_dirs: 0,
            n_files: 0,
            line_count: 0,
            size: 0,
        }
    }

    pub fn add(&mut self, s: &Summary) {
        self.n_dirs += s.n_dirs;
        self.n_files += s.n_files;
        self.line_count += s.line_count;
        self.size += s.size;
    }

    pub fn incr(&mut self, f: &File) {
        match f {
            File::RegularFile(f) => {
                self.n_files += 1;
                self.line_count += f.line_count;
                self.size += f.size;
            }
            File::Directory(_) => self.n_dirs += 1,
            _ => (),
        }
    }
}
