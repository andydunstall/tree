use crate::entry::Entry;

pub trait UI {
    fn file(&self, entry: &Entry);
}
