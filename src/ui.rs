use mockall::automock;

use crate::summary::Summary;

#[automock]
pub trait UI {
    fn file(
        &self,
        file_name: String,
        file_size: u64,
        line_count: u64,
        depth: usize,
        is_last: bool,
        is_dir: bool,
    );
    fn summary(&self, summary: &Summary);
    fn add_dir(&mut self, depth: usize);
    fn remove_dir(&mut self, depth: usize);
}
