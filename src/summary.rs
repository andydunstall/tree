#[derive(Debug, PartialEq)]
pub struct Summary {
    pub n_dirs: usize,
    pub n_files: usize,
}

impl Summary {
    pub fn new() -> Summary {
        Summary {
            n_dirs: 0,
            n_files: 0,
        }
    }

    pub fn add(&mut self, s: &Summary) {
        self.n_dirs += s.n_dirs;
        self.n_files += s.n_files;
    }

    pub fn incr(&mut self, is_dir: bool) {
        if is_dir {
            self.n_dirs += 1;
        } else {
            self.n_files += 1;
        }
    }
}
