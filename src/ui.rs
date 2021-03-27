pub trait UI {
    fn file(&self, name: &str, depth: usize, is_last: bool);
}
