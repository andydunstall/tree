use crate::entry::Entry;

pub trait Formatter {
    fn file(&self, entry: &Entry) -> String;
    fn summary(&self, n_dirs: usize, n_files: usize) -> String;
    fn add_indent(&mut self, depth: usize);
    fn remove_indent(&mut self, depth: usize);
}
