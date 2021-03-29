pub trait UI {
    fn file(&self, file_name: String, depth: usize, is_last: bool);
    fn summary(&self, n_dirs: usize, n_files: usize);
    fn add_dir(&mut self, depth: usize);
    fn remove_dir(&mut self, depth: usize);
}
