#[derive(Clone)]
pub struct Summary {
    pub n_dirs: usize,
    pub n_files: usize,
}

impl Summary {
    pub fn add(&mut self, s: &Summary) -> Summary {
        self.n_dirs += s.n_dirs;
        self.n_files += s.n_files;
        return self.clone();
    }
}
