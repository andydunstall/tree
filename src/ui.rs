use crate::entry::Entry;

pub trait UI {
    fn file(&self, entry: &Entry);
    fn add_indent(&mut self, depth: usize);
    fn remove_indent(&mut self, depth: usize);
}
