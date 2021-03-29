use mockall::automock;

use crate::summary::Summary;

#[automock]
pub trait UI {
    fn file(&self, file_name: String, depth: usize, is_last: bool);
    fn summary(&self, summary: &Summary);
    fn add_dir(&mut self, depth: usize);
    fn remove_dir(&mut self, depth: usize);
}
