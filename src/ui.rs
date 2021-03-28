use crate::entry::Entry;

pub trait UI {
    fn file(&self, entry: &Entry);
    fn summary(&self, n_dirs: usize, n_files: usize);
    fn add_dir(&mut self, depth: usize);
    fn remove_dir(&mut self, depth: usize);
}
