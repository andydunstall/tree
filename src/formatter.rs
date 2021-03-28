use crate::entry::Entry;

pub trait Formatter {
    fn file(&self, entry: &Entry) -> String;
    fn summary(&self, n_dirs: usize, n_files: usize) -> String;
    fn add_dir(&mut self, depth: usize);
    fn remove_dir(&mut self, depth: usize);
}
